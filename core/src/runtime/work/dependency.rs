use super::{Cancelled, Observer, Operation, Shared, Work};
use parking_lot::Mutex;
use std::collections::{BTreeMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};
use std::task::{Context, Poll};

pub(crate) struct Dependencies<Value> {
    edges: Arc<Edges<Value>>,
}

struct Edges<Value> {
    next: AtomicU64,
    values: Mutex<BTreeMap<u64, Weak<Operation<Value>>>>,
}

pub(crate) struct DependencyObserver<Value> {
    observer: Observer<Value>,
    dependencies: Dependencies<Value>,
    ticket: u64,
}

enum Step<Value> {
    Poll(Arc<Operation<Value>>),
    Resume(Arc<Operation<Value>>),
}

impl<Value> Dependencies<Value> {
    pub(super) fn new() -> Self {
        Self {
            edges: Arc::new(Edges {
                next: AtomicU64::new(0),
                values: Mutex::new(BTreeMap::new()),
            }),
        }
    }

    pub(crate) fn observe(&self, work: &Work<Value>) -> DependencyObserver<Value> {
        DependencyObserver {
            observer: work.observe(),
            dependencies: self.clone(),
            ticket: self.edges.next.fetch_add(1, Ordering::Relaxed),
        }
    }

    fn active(&self) -> Vec<Arc<Operation<Value>>> {
        self.edges
            .values
            .lock()
            .values()
            .filter_map(Weak::upgrade)
            .collect()
    }

    fn remove(&self, ticket: u64) {
        self.edges.values.lock().remove(&ticket);
    }
}

impl<Value> Clone for Dependencies<Value> {
    fn clone(&self) -> Self {
        Self {
            edges: Arc::clone(&self.edges),
        }
    }
}

impl<Value> Future for DependencyObserver<Value> {
    type Output = Result<Shared<Value>, Cancelled>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let operation = &self.observer.work.operation;
        operation
            .wake
            .register(self.observer.ticket, cx.waker().clone());
        let result = operation.completed();
        if result.is_ready() {
            operation.wake.remove(self.observer.ticket);
            self.dependencies.remove(self.ticket);
        } else {
            self.dependencies
                .edges
                .values
                .lock()
                .insert(self.ticket, Arc::downgrade(operation));
        }
        result
    }
}

impl<Value> Drop for DependencyObserver<Value> {
    fn drop(&mut self) {
        self.dependencies.remove(self.ticket);
    }
}

impl<Value> Operation<Value> {
    pub(super) fn poll_graph(self: &Arc<Self>) -> Poll<Result<Shared<Value>, Cancelled>> {
        let mut steps = vec![Step::Poll(Arc::clone(self))];
        let mut visited = HashSet::new();
        while let Some(step) = steps.pop() {
            let operation = match step {
                Step::Poll(operation) => {
                    if !visited.insert(operation.identity) {
                        continue;
                    }
                    operation
                }
                Step::Resume(operation) => {
                    if !operation
                        .dependencies
                        .active()
                        .iter()
                        .any(|child| child.completed().is_ready())
                    {
                        continue;
                    }
                    operation
                }
            };
            let result = operation.poll();
            if result.is_ready() {
                continue;
            }
            let dependencies: Vec<_> = operation
                .dependencies
                .active()
                .into_iter()
                .filter(|child| !visited.contains(&child.identity))
                .collect();
            if !dependencies.is_empty() {
                steps.push(Step::Resume(operation));
                steps.extend(dependencies.into_iter().rev().map(Step::Poll));
            }
        }
        self.completed()
    }
}

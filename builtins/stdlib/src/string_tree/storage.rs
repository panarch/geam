use super::schema::StringTreeSchema;
use crate::{GleamStdlibHostProfile, stdlib_stores};
use crate::{
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalStorage,
    HostExternalStore,
};
use ecow::EcoString;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use std::rc::Rc;

#[derive(Default)]
pub(crate) struct Stores {
    values: HostExternalStore<StringTreePayload>,
}

pub struct StringTreePayload {
    pub(crate) tree: StringTree,
}

impl StringTreePayload {
    pub fn from_stored(tree: StringTree) -> Self {
        Self { tree }
    }
}

#[derive(Clone)]
pub struct StringTree {
    root: Rc<StringTreeNode>,
}

struct StringTreeNode {
    byte_len: usize,
    kind: StringTreeNodeKind,
}

enum StringTreeNodeKind {
    Text(EcoString),
    Sequence(Box<[Rc<StringTreeNode>]>),
}

impl StringTree {
    pub fn text(text: EcoString) -> Self {
        Self {
            root: Rc::new(StringTreeNode {
                byte_len: text.len(),
                kind: StringTreeNodeKind::Text(text),
            }),
        }
    }

    pub fn sequence(trees: impl IntoIterator<Item = Self>) -> Self {
        let children = trees
            .into_iter()
            .map(|tree| tree.root)
            .collect::<Box<[_]>>();
        let byte_len = children.iter().map(|child| child.byte_len).sum();
        Self {
            root: Rc::new(StringTreeNode {
                byte_len,
                kind: StringTreeNodeKind::Sequence(children),
            }),
        }
    }

    pub(super) fn append(&self, suffix: &Self) -> Self {
        Self::sequence([self.clone(), suffix.clone()])
    }

    pub(super) fn byte_len(&self) -> usize {
        self.root.byte_len
    }

    pub fn flatten(&self) -> EcoString {
        let mut output = String::with_capacity(self.byte_len());
        let mut pending = vec![self.root.as_ref()];
        while let Some(node) = pending.pop() {
            match &node.kind {
                StringTreeNodeKind::Text(text) => output.push_str(text),
                StringTreeNodeKind::Sequence(children) => {
                    pending.extend(children.iter().rev().map(Rc::as_ref));
                }
            }
        }
        output.into()
    }

    pub fn structurally_equal(&self, other: &Self) -> bool {
        if self.byte_len() != other.byte_len() {
            return false;
        }

        let mut pending = vec![(self.root.as_ref(), other.root.as_ref())];
        while let Some((left, right)) = pending.pop() {
            if std::ptr::eq(left, right) {
                continue;
            }
            match (&left.kind, &right.kind) {
                (StringTreeNodeKind::Text(left), StringTreeNodeKind::Text(right)) => {
                    if left != right {
                        return false;
                    }
                }
                (StringTreeNodeKind::Sequence(left), StringTreeNodeKind::Sequence(right))
                    if left.len() == right.len() =>
                {
                    pending.extend(
                        left.iter()
                            .zip(right.iter())
                            .rev()
                            .map(|(left, right)| (left.as_ref(), right.as_ref())),
                    );
                }
                _ => return false,
            }
        }
        true
    }

    pub fn structural_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        let mut pending = vec![self.root.as_ref()];
        while let Some(node) = pending.pop() {
            match &node.kind {
                StringTreeNodeKind::Text(text) => {
                    0_u8.hash(&mut hasher);
                    text.hash(&mut hasher);
                }
                StringTreeNodeKind::Sequence(children) => {
                    1_u8.hash(&mut hasher);
                    children.len().hash(&mut hasher);
                    pending.extend(children.iter().rev().map(Rc::as_ref));
                }
            }
        }
        hasher.finish()
    }

    pub(super) fn inspect(&self) -> EcoString {
        format!("string_tree.from_string({:?})", self.flatten()).into()
    }
}

impl Drop for StringTreeNode {
    fn drop(&mut self) {
        let mut pending = take_children(&mut self.kind);
        while let Some(child) = pending.pop() {
            let Ok(mut child) = Rc::try_unwrap(child) else {
                continue;
            };
            pending.extend(take_children(&mut child.kind));
        }
    }
}

fn take_children(kind: &mut StringTreeNodeKind) -> Vec<Rc<StringTreeNode>> {
    match mem::replace(kind, StringTreeNodeKind::Text(EcoString::new())) {
        StringTreeNodeKind::Text(_) => Vec::new(),
        StringTreeNodeKind::Sequence(children) => children.into_vec(),
    }
}

pub struct StringTreeExternalStorage;

impl<Profile> HostExternalStorage<Profile, StringTreeSchema> for StringTreeExternalStorage
where
    Profile: GleamStdlibHostProfile,
{
    type Payload = StringTreePayload;

    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &stdlib_stores::<Profile>(stores).string_tree.values
    }

    fn source_equal(
        _context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.tree.structurally_equal(&right.tree)
    }

    fn source_hash(_context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        value.tree.structural_hash()
    }

    fn inspect(_context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        value.tree.inspect()
    }
}

#[cfg(test)]
mod tests {
    use super::{StringTree, StringTreeNode, StringTreeNodeKind};
    use ecow::EcoString;
    use std::rc::Rc;

    fn children(tree: &StringTree) -> Option<&[Rc<StringTreeNode>]> {
        match &tree.root.kind {
            StringTreeNodeKind::Text(_) => None,
            StringTreeNodeKind::Sequence(children) => Some(children),
        }
    }

    #[test]
    fn preserves_structural_identity_separately_from_textual_content() {
        let flat = StringTree::text("ab".into());
        let segmented =
            StringTree::sequence([StringTree::text("a".into()), StringTree::text("b".into())]);
        let same =
            StringTree::sequence([StringTree::text("a".into()), StringTree::text("b".into())]);

        assert_eq!(flat.flatten(), segmented.flatten());
        assert!(!flat.structurally_equal(&segmented));
        assert!(segmented.structurally_equal(&same));
        assert!(segmented.structurally_equal(&segmented.clone()));
        assert!(!StringTree::text("a".into()).structurally_equal(&StringTree::text("bb".into())));
        assert!(!StringTree::text("a".into()).structurally_equal(&StringTree::text("b".into())));
        assert_eq!(segmented.structural_hash(), same.structural_hash());
        assert_eq!(segmented.byte_len(), 2);
        assert_eq!(segmented.inspect(), r#"string_tree.from_string("ab")"#);
    }

    #[test]
    fn append_shares_existing_subtrees_and_deep_chains_drop_iteratively() {
        let prefix = StringTree::text("a".into());
        let suffix = StringTree::text("b".into());
        let appended = prefix.append(&suffix);
        assert!(children(&prefix).is_none());
        let children = children(&appended).expect("append should create a sequence node");
        assert!(Rc::ptr_eq(&children[0], &prefix.root));
        assert!(Rc::ptr_eq(&children[1], &suffix.root));

        let mut deep = StringTree::text(EcoString::new());
        for _ in 0..50_000 {
            deep = deep.append(&StringTree::text("x".into()));
        }
        assert_eq!(deep.byte_len(), 50_000);
        assert_eq!(deep.flatten().len(), 50_000);
        drop(deep);
    }

    #[test]
    fn empty_text_and_empty_sequence_are_textually_but_not_structurally_equal() {
        let text = StringTree::text(EcoString::new());
        let sequence = StringTree::sequence([]);

        assert_eq!(text.flatten(), "");
        assert_eq!(sequence.flatten(), "");
        assert!(!text.structurally_equal(&sequence));
        assert_ne!(text.structural_hash(), sequence.structural_hash());
    }
}

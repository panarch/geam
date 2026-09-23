/// The service projection and initialization paths of one generated static host.
pub(crate) struct ServiceComposition {
    runtime: String,
    services: Vec<ServiceBinding>,
}

pub(crate) struct ServiceBinding {
    pub(crate) component: String,
    pub(crate) state_field: String,
}

impl ServiceComposition {
    pub(crate) fn new(
        runtime: &str,
        first: ServiceBinding,
        rest: impl IntoIterator<Item = ServiceBinding>,
    ) -> Self {
        Self {
            runtime: runtime.to_owned(),
            services: std::iter::once(first).chain(rest).collect(),
        }
    }

    pub(crate) fn type_expression(&self) -> String {
        let mut output = String::new();
        for service in &self.services {
            output.push_str(&self.runtime);
            output.push_str("::execution::ExecutionServices<");
            output.push_str(&self.state_type(service));
            output.push_str(", ");
        }
        output.push_str("()");
        output.extend(std::iter::repeat_n('>', self.services.len()));
        output
    }

    pub(crate) fn initialization(&self) -> String {
        let mut output = String::from(
            "    fn initialize_execution(state: &mut Self::RunState) -> Self::ExecutionState {\n",
        );
        for (index, service) in self.services.iter().enumerate() {
            let indent = "    ".repeat(index + 2);
            if index > 0 {
                output.push_str(&format!("{indent}rest: "));
            } else {
                output.push_str(&indent);
            }
            output.push_str(&format!(
                "{}::execution::ExecutionServices {{\n",
                self.runtime
            ));
            output.push_str(&format!("{indent}    first: <{} as {}::HostExecutionService>::initialize_service(&mut state.{}),\n", service.component, self.runtime, service.state_field));
        }
        let indent = "    ".repeat(self.services.len() + 2);
        output.push_str(&format!("{indent}rest: (),\n"));
        for index in (0..self.services.len()).rev() {
            let indent = "    ".repeat(index + 2);
            let comma = if index == 0 { "" } else { "," };
            output.push_str(&format!("{indent}}}{comma}\n"));
        }
        output.push_str("    }\n");
        output
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &ServiceBinding> {
        self.services.iter()
    }

    pub(crate) fn state_type(&self, service: &ServiceBinding) -> String {
        format!(
            "<{} as {}::HostExecutionService>::State",
            service.component, self.runtime
        )
    }

    pub(crate) fn projection(&self, index: usize) -> String {
        format!("&mut state.{}first", "rest.".repeat(index))
    }
}

#[cfg(test)]
mod tests {
    use super::{ServiceBinding, ServiceComposition};

    #[test]
    fn nested_service_types_initialization_and_projections_agree_in_order() {
        let services = ServiceComposition::new(
            "runtime",
            ServiceBinding {
                component: "process::Component<Profile>".into(),
                state_field: "process".into(),
            },
            [ServiceBinding {
                component: "tickets::Component".into(),
                state_field: "tickets".into(),
            }],
        );
        assert_eq!(
            services.type_expression(),
            "runtime::execution::ExecutionServices<<process::Component<Profile> as runtime::HostExecutionService>::State, runtime::execution::ExecutionServices<<tickets::Component as runtime::HostExecutionService>::State, ()>>"
        );
        assert_eq!(
            services
                .iter()
                .map(|service| service.state_field.as_str())
                .collect::<Vec<_>>(),
            ["process", "tickets"]
        );
        assert_eq!(services.projection(0), "&mut state.first");
        assert_eq!(services.projection(1), "&mut state.rest.first");
        assert_eq!(
            services.initialization(),
            "    fn initialize_execution(state: &mut Self::RunState) -> Self::ExecutionState {\n        runtime::execution::ExecutionServices {\n            first: <process::Component<Profile> as runtime::HostExecutionService>::initialize_service(&mut state.process),\n            rest: runtime::execution::ExecutionServices {\n                first: <tickets::Component as runtime::HostExecutionService>::initialize_service(&mut state.tickets),\n                rest: (),\n            },\n        }\n    }\n"
        );
    }
}

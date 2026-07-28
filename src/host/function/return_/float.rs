use super::{
    HostCallback, HostFunctionImplementation, HostReturn, HostValueFunctionImplementation,
};
use crate::host::function::HostValueType;
use crate::host::function::argument::HostCallArguments;
use crate::host::{HostCallError, HostProfile};
use std::sync::Arc;

pub(crate) struct HostFloatFunction<Profile: HostProfile> {
    implementation: Arc<HostCallback<Profile, f64>>,
}

impl<Profile: HostProfile> Clone for HostFloatFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            implementation: Arc::clone(&self.implementation),
        }
    }
}

impl<Profile: HostProfile> HostFloatFunction<Profile> {
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<f64, HostCallError> {
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for f64 {
    fn type_() -> HostValueType {
        HostValueType::Float
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::Value(HostValueFunctionImplementation::Float(
            HostFloatFunction {
                implementation: Arc::new(function),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostFloatFunction, HostFunctionImplementation, HostReturn, HostValueFunctionImplementation,
    };
    use crate::host::StatelessHostProfile;
    use crate::host::function::HostValueType;
    use crate::host::function::argument::{CallArguments, HostCallArguments, HostParameterLayout};

    #[test]
    fn float_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<f64>();
        assert_eq!(<f64 as HostReturn>::type_(), HostValueType::Float);
        let implementation =
            <f64 as HostReturn>::implementation::<StatelessHostProfile>(move |(), arguments| {
                Ok(arguments.float(slot) + 0.5)
            });
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            vec![1.0],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            0,
        );

        assert_eq!(
            float_implementation(implementation).call(&mut (), &arguments),
            Ok(1.5),
        );
    }

    #[test]
    #[should_panic(expected = "f64 return should create a Float implementation")]
    fn float_return_shape_guard_is_visible() {
        let callback = |(): &mut (), _: &dyn HostCallArguments| Ok(true);
        let implementation = <bool as HostReturn>::implementation::<StatelessHostProfile>(callback);
        float_implementation(implementation);
    }

    fn float_implementation(
        implementation: HostFunctionImplementation<StatelessHostProfile>,
    ) -> HostFloatFunction<StatelessHostProfile> {
        let HostFunctionImplementation::Value(HostValueFunctionImplementation::Float(
            implementation,
        )) = implementation
        else {
            panic!("f64 return should create a Float implementation");
        };
        implementation
    }
}

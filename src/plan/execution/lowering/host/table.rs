use super::super::function;
use super::super::specialization::{
    SpecializationKey, SpecializedFunctionShape, ValueInhabitation,
};
use super::super::{LoweredExecution, LoweringCompletion, LoweringContext, SpecializationOutcome};
use super::{parameter, return_, sealing};
use crate::host::{
    HostFunctionImplementation as RegisteredHostFunctionImplementation, HostProfile,
};
use crate::plan::execution::host::{
    HostFunctionTables, HostSpecializationError, HostedExecutionProfile, HostedFunction,
    HostedFunctionMetadata, HostedNeverFunction, HostedValueFunction,
};
use crate::plan::{HostFunctionTemplate, HostImplementationBinding};
use std::collections::HashMap;
use std::sync::Arc;

type HostedLoweredExecution = LoweredExecution<HostedExecutionProfile>;

pub(super) struct HostFunctionRegistry<Profile: HostProfile> {
    functions: HashMap<
        crate::plan::FunctionTemplateId,
        Arc<RegisteredHostFunctionImplementation<Profile>>,
    >,
}

pub(super) struct HostFunctionLowering<'registry, Profile: HostProfile> {
    registered: &'registry HostFunctionRegistry<Profile>,
    value_functions: Vec<HostedValueFunction<Profile>>,
    never_functions: Vec<HostedNeverFunction<Profile>>,
    additional: function::ProfiledFunctionEntries<HostedExecutionProfile>,
}

impl<Profile: HostProfile> HostFunctionRegistry<Profile> {
    pub(super) fn new(implementation_bindings: Vec<HostImplementationBinding<Profile>>) -> Self {
        Self {
            functions: implementation_bindings
                .into_iter()
                .map(HostImplementationBinding::into_parts)
                .collect(),
        }
    }

    pub(super) fn lowering(&self) -> HostFunctionLowering<'_, Profile> {
        HostFunctionLowering {
            registered: self,
            value_functions: Vec::new(),
            never_functions: Vec::new(),
            additional: function::ProfiledFunctionEntries {
                never: Vec::new(),
                custom: Vec::new(),
                external: Vec::new(),
                int: Vec::new(),
                float: Vec::new(),
                string: Vec::new(),
                bit_array: Vec::new(),
                utf_codepoint: Vec::new(),
                bool: Vec::new(),
                nil: Vec::new(),
                tuple: Vec::new(),
                parameter_list: Vec::new(),
                int_list: Vec::new(),
                string_list: Vec::new(),
                bit_array_list: Vec::new(),
                utf_codepoint_list: Vec::new(),
                custom_list: Vec::new(),
                external_list: Vec::new(),
                float_list: Vec::new(),
                bool_list: Vec::new(),
                nil_list: Vec::new(),
                tuple_list: Vec::new(),
                parameter_list_list: Vec::new(),
                list_list: Vec::new(),
                function_list: Vec::new(),
                int_function_functions: Vec::new(),
                float_function_functions: Vec::new(),
                string_function_functions: Vec::new(),
                bit_array_function_functions: Vec::new(),
                utf_codepoint_function_functions: Vec::new(),
                custom_function_functions: Vec::new(),
                external_function_functions: Vec::new(),
                bool_function_functions: Vec::new(),
                nil_function_functions: Vec::new(),
                tuple_function_functions: Vec::new(),
                generic_function_functions: Vec::new(),
                never_function_functions: Vec::new(),
                parameter_list_function_functions: Vec::new(),
                parameter_list_list_function_functions: Vec::new(),
                int_list_function_functions: Vec::new(),
                string_list_function_functions: Vec::new(),
                bit_array_list_function_functions: Vec::new(),
                utf_codepoint_list_function_functions: Vec::new(),
                custom_list_function_functions: Vec::new(),
                external_list_function_functions: Vec::new(),
                float_list_function_functions: Vec::new(),
                bool_list_function_functions: Vec::new(),
                nil_list_function_functions: Vec::new(),
                tuple_list_function_functions: Vec::new(),
                list_list_function_functions: Vec::new(),
                function_list_function_functions: Vec::new(),
                function_function_functions: Vec::new(),
            },
        }
    }
}

impl<Profile: HostProfile> HostFunctionLowering<'_, Profile> {
    pub(super) fn lower_specialized(
        &mut self,
        template: &HostFunctionTemplate,
        key: &SpecializationKey,
        context: &mut LoweringContext,
    ) -> Result<(), HostSpecializationError> {
        let index = context.specialization_index(key);
        let shape =
            SpecializedFunctionShape::instantiate(template.signature().shape(), key.substitution());
        let parameters = context.specialization_parameters(key).to_vec();
        let type_arguments = key
            .substitution()
            .arguments()
            .iter()
            .map(|argument| argument.to_module_shape().value_type())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let implementation = Arc::clone(&self.registered.functions[&template.id()]);
        let return_ = context.representations.inhabitation(shape.return_());

        match implementation.as_ref() {
            RegisteredHostFunctionImplementation::Value(implementation) => {
                let ValueInhabitation::Inhabited(return_) = return_ else {
                    return Err(HostSpecializationError::undetermined_return_storage(
                        template.package().clone(),
                        template.site().module().clone(),
                        template.site().function().clone(),
                        shape.to_module_shape().type_(),
                    ));
                };
                sealing::seal_callbacks(template, key, &shape, &context.representations, true)?;
                let constructions = sealing::seal_host_types(template, key, context);
                let parameters =
                    parameter::lower_host_parameters(&parameters, template.layout(), context);
                let type_ = context.lower_concrete_function_type(&shape);
                let host_index = self.value_functions.len();
                self.value_functions.push(HostedFunction::new(
                    HostedFunctionMetadata::new(
                        template.package().clone(),
                        template.site().clone(),
                        shape.to_module_shape().type_(),
                        type_arguments,
                        parameters,
                        constructions,
                        type_,
                    ),
                    implementation.clone(),
                ));
                return_::lower_host_return(
                    index,
                    key,
                    return_,
                    return_::HostTargetIndex::Value(host_index),
                    &mut self.additional,
                    context,
                );
            }
            RegisteredHostFunctionImplementation::Never(implementation) => {
                sealing::seal_callbacks(template, key, &shape, &context.representations, false)?;
                let constructions = sealing::seal_host_types(template, key, context);
                let parameters =
                    parameter::lower_host_parameters(&parameters, template.layout(), context);
                let type_ = context.lower_concrete_function_type(&shape);
                let host_index = self.never_functions.len();
                self.never_functions.push(HostedFunction::new(
                    HostedFunctionMetadata::new(
                        template.package().clone(),
                        template.site().clone(),
                        shape.to_module_shape().type_(),
                        type_arguments,
                        parameters,
                        constructions,
                        type_,
                    ),
                    implementation.clone(),
                ));
                match return_ {
                    ValueInhabitation::Inhabited(return_) => return_::lower_host_return(
                        index,
                        key,
                        return_,
                        return_::HostTargetIndex::Never(host_index),
                        &mut self.additional,
                        context,
                    ),
                    ValueInhabitation::Uninhabited(_) => {
                        return_::lower_uninhabited_never_return(
                            index,
                            key,
                            host_index,
                            &mut self.additional,
                        );
                    }
                }
            }
        }
        Ok(())
    }

    pub(super) fn finish(
        self,
        context: LoweringContext,
    ) -> (
        LoweringCompletion<HostedLoweredExecution>,
        HostFunctionTables<Profile>,
    ) {
        let Self {
            registered: _,
            value_functions,
            never_functions,
            additional,
        } = self;
        let completion = context.finish_hosted(additional);
        let tables = HostFunctionTables::new(
            value_functions.into_boxed_slice(),
            never_functions.into_boxed_slice(),
        );
        (completion, tables)
    }
}

impl LoweringContext {
    fn finish_hosted(
        self,
        additional: function::ProfiledFunctionEntries<HostedExecutionProfile>,
    ) -> LoweringCompletion<HostedLoweredExecution> {
        let Self {
            constant_templates,
            constants,
            types,
            representations,
            functions,
            erased_specializations,
            ..
        } = self;
        let outcome = functions
            .finish_hosted(additional)
            .zip_with(
                SpecializationOutcome::Complete(constants.finish_hosted()),
                |functions, constants| {
                    let (list_types, custom_types, external_types, value_shapes) =
                        types.into_tables();
                    Box::new(LoweredExecution {
                        constants,
                        functions: *functions,
                        list_types,
                        custom_types,
                        external_types,
                        value_shapes,
                    })
                },
            )
            .include_prior_erasure(erased_specializations);
        (constant_templates, representations, outcome)
    }
}

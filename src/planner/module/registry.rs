use crate::plan::{
    ConstantTemplateId, ConstantTemplateSignature, ConstantTemplates, CustomTypeDefinition,
    CustomTypeName, ModuleId, ValueShape,
};
use crate::planner::context::FunctionInfo;
use crate::planner::module::constant::ConstantSignatures;
use ecow::EcoString;
use std::collections::HashMap;

pub(in crate::planner) struct ProgramRegistry {
    modules: Vec<ModuleRegistry>,
}

pub(in crate::planner) struct ModuleRegistry {
    name: EcoString,
    custom_types: Vec<CustomTypeDefinition>,
    functions: HashMap<EcoString, FunctionInfo>,
    constants: ConstantSignatures,
}

impl ProgramRegistry {
    pub(in crate::planner) fn new(modules: Vec<ModuleRegistry>) -> Self {
        Self { modules }
    }

    pub(in crate::planner) fn module_name(&self, module: ModuleId) -> &EcoString {
        &self.modules[module.index()].name
    }

    pub(in crate::planner) fn function(
        &self,
        module: ModuleId,
        name: &EcoString,
    ) -> Option<FunctionInfo> {
        self.modules[module.index()].functions.get(name).cloned()
    }

    pub(in crate::planner) fn constant_expr(
        &self,
        module: ModuleId,
        name: &EcoString,
        shape: &ValueShape,
    ) -> Option<crate::plan::Expr> {
        self.modules[module.index()]
            .constants
            .instantiate(name, shape)
            .map(ConstantTemplates::reference)
    }

    pub(in crate::planner) fn constant_signatures(&self, module: ModuleId) -> &ConstantSignatures {
        &self.modules[module.index()].constants
    }

    pub(in crate::planner) fn constant_signature(
        &self,
        id: ConstantTemplateId,
    ) -> &ConstantTemplateSignature {
        self.modules[id.module().index()].constants.get(id)
    }

    pub(in crate::planner) fn custom_type(
        &self,
        name: &CustomTypeName,
    ) -> Option<&CustomTypeDefinition> {
        self.modules
            .iter()
            .flat_map(|module| &module.custom_types)
            .find(|definition| definition.name() == name)
    }

    pub(in crate::planner) fn custom_types(&self, module: ModuleId) -> &[CustomTypeDefinition] {
        &self.modules[module.index()].custom_types
    }
}

impl ModuleRegistry {
    pub(in crate::planner) fn new(
        name: EcoString,
        custom_types: Vec<CustomTypeDefinition>,
        functions: HashMap<EcoString, FunctionInfo>,
        constants: ConstantSignatures,
    ) -> Self {
        Self {
            name,
            custom_types,
            functions,
            constants,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ModuleRegistry, ProgramRegistry};
    use crate::plan::{
        CustomTypeDefinition, CustomTypeName, CustomTypePublicity, FunctionShape,
        FunctionTemplateId, FunctionTemplateSignature, ModuleId, TypeScheme, ValueShape,
    };
    use crate::planner::context::FunctionInfo;
    use crate::planner::module::constant::ConstantSignatures;
    use crate::planner::type_parameter::TypeParameterScope;
    use std::collections::HashMap;

    #[test]
    fn registry_qualifies_same_named_declarations_by_module() {
        let alpha = ModuleId::new(0);
        let root = ModuleId::new(1);
        let alpha_type = custom_type("alpha", "Box");
        let root_type = custom_type("root", "Box");
        let registry = ProgramRegistry::new(vec![
            ModuleRegistry::new(
                "alpha".into(),
                vec![alpha_type.clone()],
                HashMap::from([("same".into(), function_info(alpha))]),
                ConstantSignatures::default(),
            ),
            ModuleRegistry::new(
                "root".into(),
                vec![root_type.clone()],
                HashMap::from([("same".into(), function_info(root))]),
                ConstantSignatures::default(),
            ),
        ]);

        assert_eq!(registry.module_name(alpha), "alpha");
        assert_eq!(registry.module_name(root), "root");
        assert_eq!(
            registry
                .function(alpha, &"same".into())
                .map(|info| info.signature.id()),
            Some(FunctionTemplateId::in_module(alpha, 0)),
        );
        assert_eq!(
            registry
                .function(root, &"same".into())
                .map(|info| info.signature.id()),
            Some(FunctionTemplateId::in_module(root, 0)),
        );
        assert_eq!(registry.custom_type(alpha_type.name()), Some(&alpha_type),);
        assert_eq!(registry.custom_type(root_type.name()), Some(&root_type));
        assert_eq!(
            registry.constant_expr(root, &"missing".into(), &ValueShape::Int),
            None,
        );
    }

    fn function_info(module: ModuleId) -> FunctionInfo {
        FunctionInfo {
            signature: FunctionTemplateSignature::new(
                FunctionTemplateId::in_module(module, 0),
                TypeScheme::new(0),
                FunctionShape::new(Vec::new(), ValueShape::Int),
            ),
            type_parameters: TypeParameterScope::default(),
            return_shape: ValueShape::Int,
            params: Vec::new(),
        }
    }

    fn custom_type(module: &str, name: &str) -> CustomTypeDefinition {
        CustomTypeDefinition::new(
            CustomTypeName::new("geam".into(), module.into(), name.into()),
            CustomTypePublicity::Public,
            false,
            Vec::new(),
            Vec::new(),
        )
    }
}

use crate::plan::{
    ConstantTemplateId, ConstantTemplateSignature, ConstantTemplates, CustomTypeDefinition,
    CustomTypeName, ModuleId, ValueShape,
};
use crate::planner::context::FunctionInfo;
use crate::planner::module::constant::ConstantSignatures;
use ecow::EcoString;
use std::collections::HashMap;

pub(in crate::planner) struct ProgramRegistry {
    by_name: HashMap<EcoString, ModuleId>,
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
        let by_name = modules
            .iter()
            .enumerate()
            .map(|(index, module)| (module.name.clone(), ModuleId::new(index)))
            .collect();
        Self { by_name, modules }
    }

    pub(in crate::planner) fn module_name(&self, module: ModuleId) -> &EcoString {
        &self.modules[module.index()].name
    }

    pub(in crate::planner) fn module_id(&self, module: &EcoString) -> Option<ModuleId> {
        self.by_name.get(module).copied()
    }

    pub(in crate::planner) fn function(
        &self,
        module: &EcoString,
        name: &EcoString,
    ) -> Option<FunctionInfo> {
        let module = self.module_id(module)?;
        self.modules[module.index()].functions.get(name).cloned()
    }

    pub(in crate::planner) fn constant_expr(
        &self,
        module: &EcoString,
        name: &EcoString,
        shape: &ValueShape,
    ) -> Option<crate::plan::Expr> {
        self.constant_instantiation(module, name, shape)
            .map(ConstantTemplates::reference)
    }

    pub(in crate::planner) fn constant_instantiation(
        &self,
        module: &EcoString,
        name: &EcoString,
        shape: &ValueShape,
    ) -> Option<crate::plan::ConstantInstantiation> {
        let module = self.module_id(module)?;
        self.modules[module.index()]
            .constants
            .instantiate(name, shape)
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
        assert_eq!(registry.module_id(&"alpha".into()), Some(alpha));
        assert_eq!(registry.module_id(&"root".into()), Some(root));
        assert_eq!(registry.module_id(&"missing".into()), None);
        assert_eq!(
            registry
                .function(&"alpha".into(), &"same".into())
                .map(function_template_id),
            Some(FunctionTemplateId::in_module(alpha, 0)),
        );
        assert_eq!(
            registry
                .function(&"root".into(), &"same".into())
                .map(function_template_id),
            Some(FunctionTemplateId::in_module(root, 0)),
        );
        assert_eq!(
            registry
                .function(&"missing".into(), &"same".into())
                .map(function_template_id),
            None,
        );
        assert_eq!(
            registry
                .function(&"root".into(), &"missing".into())
                .map(function_template_id),
            None,
        );
        assert_eq!(registry.custom_type(alpha_type.name()), Some(&alpha_type),);
        assert_eq!(registry.custom_type(root_type.name()), Some(&root_type));
        assert_eq!(
            registry.custom_type(&CustomTypeName::new(
                "geam".into(),
                "missing".into(),
                "Box".into(),
            )),
            None,
        );
        assert_eq!(
            registry.constant_expr(&"root".into(), &"missing".into(), &ValueShape::Int),
            None,
        );
        assert_eq!(
            registry.constant_expr(&"missing".into(), &"value".into(), &ValueShape::Int),
            None,
        );
    }

    fn function_template_id(info: FunctionInfo) -> FunctionTemplateId {
        info.signature.id()
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

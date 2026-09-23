use super::{CallError, Target};
use crate::plan::execution::function::{self, FunctionTableFamily};
use crate::plan::execution::graph::{self, ParamLocal};
use crate::plan::execution::prepared::admission::catalog::{Catalog, Function};
use crate::plan::execution::prepared::admission::type_::Types;
use crate::plan::execution::type_::ValueType;
use std::convert::Infallible;

impl Target for Infallible {
    fn resolve<'data>(
        &self,
        _catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        match *self {}
    }
}

impl Target for function::NeverFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::Never, self.0)
            .map_err(CallError::Catalog)?;
        Ok(function)
    }
}

impl Target for function::IntFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::Int, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Int) {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::FloatFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::Float, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Float) {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::StringFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::String, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::String) {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::BitArrayFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::BitArray, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::BitArray) {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::UtfCodepointFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::UtfCodepoint, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::UtfCodepoint) {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::CustomFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::Custom, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::Custom(graph::CustomLocal {
                id: graph::CustomLocalId(0),
                shape: self.return_shape,
            }),
            types,
        )
    }
}

impl Target for function::ExternalFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::External, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::External(graph::ExternalLocal {
                id: graph::ExternalLocalId(0),
                type_id: self.return_type,
            }),
            types,
        )
    }
}

impl Target for function::BoolFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::Bool, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Bool) {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::NilFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::Nil, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Nil) {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::TupleFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::Tuple, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Tuple(_)) {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::ParameterListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::ParameterList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::Parameter {
                local: graph::ParameterListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::IntListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::IntList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::Int {
                local: graph::IntListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::StringListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::StringList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::String {
                local: graph::StringListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::BitArrayListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::BitArrayList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::BitArray {
                local: graph::BitArrayListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::UtfCodepointListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::UtfCodepointList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::UtfCodepoint {
                local: graph::UtfCodepointListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::CustomListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::CustomList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::Custom {
                local: graph::CustomListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::ExternalListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::ExternalList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::External {
                local: graph::ExternalListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::FloatListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::FloatList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::Float {
                local: graph::FloatListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::BoolListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::BoolList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::Bool {
                local: graph::BoolListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::NilListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::NilList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::Nil {
                local: graph::NilListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::TupleListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::TupleList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::Tuple {
                local: graph::TupleListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::ParameterListListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::ParameterListList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::ParameterList {
                local: graph::ParameterListListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::ListListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::ListList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::List {
                local: graph::ListListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::FunctionListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::FunctionList, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::List(graph::ListLocal::Function {
                local: graph::FunctionListLocalId(0),
                type_id: self.type_id,
            }),
            types,
        )
    }
}

impl Target for function::IntFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::IntFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::Int))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::FloatFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::FloatFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::Float))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::StringFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::StringFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::String))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::BitArrayFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::BitArrayFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::BitArray))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::UtfCodepointFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::UtfCodepointFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::UtfCodepoint))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::CustomFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::CustomFunction, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::CustomFunction(graph::CustomFunctionLocal {
                id: graph::CustomFunctionLocalId(0),
                type_: self.type_.clone(),
            }),
            types,
        )
    }
}

impl Target for function::ExternalFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::ExternalFunction, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::ExternalFunction(graph::ExternalFunctionLocal {
                id: graph::ExternalFunctionLocalId(0),
                type_: self.type_.clone(),
            }),
            types,
        )
    }
}

impl Target for function::BoolFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::BoolFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::Bool))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::NilFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::NilFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::Nil))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::TupleFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::TupleFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::Tuple(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::GenericFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::GenericFunction, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::GenericFunction(graph::GenericFunctionLocal {
                id: graph::GenericFunctionLocalId(0),
                type_: self.type_.clone(),
            }),
            types,
        )
    }
}

impl Target for function::NeverFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::NeverFunction, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::NeverFunction(graph::NeverFunctionLocal {
                id: graph::NeverFunctionLocalId(0),
                type_: self.type_.clone(),
            }),
            types,
        )
    }
}

impl Target for function::ParameterListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::ParameterListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::ParameterListListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::ParameterListListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::IntListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::IntListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::StringListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::StringListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::BitArrayListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::BitArrayListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::UtfCodepointListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::UtfCodepointListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::CustomListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::CustomListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::ExternalListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::ExternalListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::FloatListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::FloatListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::BoolListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::BoolListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::NilListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::NilListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::TupleListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::TupleListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::ListListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::ListListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::FunctionListFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::FunctionListFunction, self.0)
            .map_err(CallError::Catalog)?;
        if !matches!(function.return_type, ValueType::Function(type_) if matches!(type_.return_(), ValueType::List(_)))
        {
            return Err(CallError::TargetType);
        }
        Ok(function)
    }
}

impl Target for function::FunctionFunctionFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let function = catalog
            .function(FunctionTableFamily::FunctionFunction, self.index)
            .map_err(CallError::Catalog)?;
        with_return(
            function,
            &ParamLocal::FunctionFunction(graph::FunctionFunctionLocal::Core(
                graph::CoreFunctionFunctionLocal {
                    id: graph::CoreFunctionFunctionLocalId(0),
                    type_: self.type_.clone(),
                },
            )),
            types,
        )
    }
}

impl Target for function::ListFunctionId {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        match self {
            Self::Parameter(id) => id.resolve(catalog, types),
            Self::ParameterList(id) => id.resolve(catalog, types),
            Self::Int(id) => id.resolve(catalog, types),
            Self::String(id) => id.resolve(catalog, types),
            Self::BitArray(id) => id.resolve(catalog, types),
            Self::UtfCodepoint(id) => id.resolve(catalog, types),
            Self::Custom(id) => id.resolve(catalog, types),
            Self::Float(id) => id.resolve(catalog, types),
            Self::Bool(id) => id.resolve(catalog, types),
            Self::Nil(id) => id.resolve(catalog, types),
            Self::Tuple(id) => id.resolve(catalog, types),
            Self::List(id) => id.resolve(catalog, types),
            Self::Function(id) => id.resolve(catalog, types),
        }
    }
}

impl<Graph: function::ExecutionGraphProfile> Target for function::ProfiledListFunctionId<Graph>
where
    Graph::ExternalListFunctionId: Target,
{
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        match self {
            Self::Core(id) => id.resolve(catalog, types),
            Self::External(id) => id.resolve(catalog, types),
        }
    }
}

impl<Graph: function::ExecutionGraphProfile, Symbolic: Target> Target
    for function::ProfiledFunctionFunctionId<Graph, Symbolic>
where
    Graph::ExternalFunctionFunctionId: Target,
    Graph::ExternalListFunctionFunctionId: Target,
{
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        match self {
            Self::Generic(id) => id.resolve(catalog, types),
            Self::Never(id) => id.resolve(catalog, types),
            Self::Int(id) => id.resolve(catalog, types),
            Self::Float(id) => id.resolve(catalog, types),
            Self::String(id) => id.resolve(catalog, types),
            Self::BitArray(id) => id.resolve(catalog, types),
            Self::UtfCodepoint(id) => id.resolve(catalog, types),
            Self::Custom(id) => id.resolve(catalog, types),
            Self::External(id) => id.resolve(catalog, types),
            Self::Bool(id) => id.resolve(catalog, types),
            Self::Nil(id) => id.resolve(catalog, types),
            Self::Tuple(id) => id.resolve(catalog, types),
            Self::List(id) => id.resolve(catalog, types),
            Self::Function(id) => id.resolve(catalog, types),
        }
    }
}

impl<Graph: function::ExecutionGraphProfile> Target
    for function::ProfiledListFunctionFunctionId<Graph>
where
    Graph::ExternalListFunctionFunctionId: Target,
{
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        let (function, local) = match self {
            Self::Parameter {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::Parameter {
                    local: graph::ParameterListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::ParameterList {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::ParameterList {
                    local: graph::ParameterListListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::Int {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::Int {
                    local: graph::IntListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::String {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::String {
                    local: graph::StringListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::BitArray {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::BitArray {
                    local: graph::BitArrayListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::UtfCodepoint {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::UtfCodepoint {
                    local: graph::UtfCodepointListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::Custom {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::Custom {
                    local: graph::CustomListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::External {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::External {
                    local: graph::ExternalListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::Float {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::Float {
                    local: graph::FloatListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::Bool {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::Bool {
                    local: graph::BoolListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::Nil {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::Nil {
                    local: graph::NilListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::Tuple {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::Tuple {
                    local: graph::TupleListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::List {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::List {
                    local: graph::ListListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
            Self::Function {
                id,
                type_,
                list_type,
            } => (
                id.resolve(catalog, types)?,
                graph::ListFunctionLocal::Function {
                    local: graph::FunctionListFunctionLocalId(0),
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
        };
        with_return(function, &ParamLocal::ListFunction(local), types)
    }
}

fn with_return<'data>(
    function: Function<'data>,
    local: &ParamLocal,
    types: &Types<'data>,
) -> Result<Function<'data>, CallError> {
    let expected = types.local_type(local).map_err(CallError::Type)?;
    if function.return_type != &expected {
        return Err(CallError::TargetType);
    }
    Ok(function)
}

#[cfg(test)]
mod tests {
    use super::{CallError, Catalog, FunctionTableFamily, Target, Types, ValueType, function};
    use crate::plan::execution::function::FunctionCatalog;
    use crate::plan::execution::prepared::admission::catalog::CatalogError;
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::{FunctionType, ValueShapeId};

    #[test]
    fn scalar_and_tuple_targets_resolve_only_their_declared_return_families() {
        let source = r#"
fn integer() { 42 }
fn decimal() { 1.5 }
fn text() { "text" }
fn bits() { <<42>> }
fn codepoint() { let assert <<value:utf8_codepoint>> = <<65>> value }
fn flag() { True }
fn nil() { Nil }
fn pair() { #(42, True) }
pub fn main() {
  let _ = #(integer(), decimal(), text(), bits(), codepoint(), flag(), nil(), pair())
  42
}
"#;
        let plan = lower(source);
        let common = &plan.program.common;
        let types = execution_types(&plan.program);
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let cases: [(&dyn Target, &dyn Target, FunctionTableFamily, ValueType); 8] = [
            (
                &function::IntFunctionId(0),
                &function::IntFunctionId(99),
                FunctionTableFamily::Int,
                ValueType::Int,
            ),
            (
                &function::FloatFunctionId(0),
                &function::FloatFunctionId(99),
                FunctionTableFamily::Float,
                ValueType::Float,
            ),
            (
                &function::StringFunctionId(0),
                &function::StringFunctionId(99),
                FunctionTableFamily::String,
                ValueType::String,
            ),
            (
                &function::BitArrayFunctionId(0),
                &function::BitArrayFunctionId(99),
                FunctionTableFamily::BitArray,
                ValueType::BitArray,
            ),
            (
                &function::UtfCodepointFunctionId(0),
                &function::UtfCodepointFunctionId(99),
                FunctionTableFamily::UtfCodepoint,
                ValueType::UtfCodepoint,
            ),
            (
                &function::BoolFunctionId(0),
                &function::BoolFunctionId(99),
                FunctionTableFamily::Bool,
                ValueType::Bool,
            ),
            (
                &function::NilFunctionId(0),
                &function::NilFunctionId(99),
                FunctionTableFamily::Nil,
                ValueType::Nil,
            ),
            (
                &function::TupleFunctionId(0),
                &function::TupleFunctionId(99),
                FunctionTableFamily::Tuple,
                ValueType::Tuple(Table::Static(&[ValueType::Int, ValueType::Bool])),
            ),
        ];
        for (target, missing, family, expected) in cases {
            let actual = target.resolve(&catalog, &types).unwrap();
            assert_eq!(actual.return_type, &expected);
            assert!(actual.parameters.is_empty());
            assert!(actual.parameter_shapes.is_empty());
            assert!(actual.captures.is_empty());
            assert_eq!(
                missing.resolve(&catalog, &types).err(),
                Some(CallError::Catalog(CatalogError::MissingFunction {
                    family,
                    index: 99
                }))
            );
            let wrong_type = if expected == ValueType::Int {
                ValueType::Bool
            } else {
                ValueType::Int
            };
            let wrong_shape = ValueShapeId(
                common
                    .value_shapes
                    .shape_types
                    .iter()
                    .position(|type_| type_ == &wrong_type)
                    .unwrap(),
            );
            let mut raw = copy_catalog(&common.function_parameters);
            let index = raw.families[family as usize].start;
            let mut contracts = raw.functions.into_vec();
            contracts[index].return_ = wrong_shape;
            raw.functions = contracts.into();
            let wrong = Catalog::admit(&raw, &plan.program.functions, &types).unwrap();
            assert_eq!(
                target.resolve(&wrong, &types).err(),
                Some(CallError::TargetType)
            );
        }
    }

    #[test]
    fn function_targets_check_the_outer_and_inner_return_families() {
        let source = r#"
fn integer() { fn() { 42 } }
fn decimal() { fn() { 1.5 } }
fn text() { fn() { "text" } }
fn bits() { fn() { <<42>> } }
fn codepoint() { fn() { let assert <<value:utf8_codepoint>> = <<65>> value } }
fn flag() { fn() { True } }
fn nil() { fn() { Nil } }
fn pair() { fn() { #(42, True) } }
pub fn main() {
  let _ = #(integer(), decimal(), text(), bits(), codepoint(), flag(), nil(), pair())
  42
}
"#;
        let plan = lower(source);
        let common = &plan.program.common;
        let types = execution_types(&plan.program);
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let cases: [(&dyn Target, &dyn Target, FunctionTableFamily, ValueType); 8] = [
            (
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::Int(function::IntFunctionFunctionId(0)),
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::Int(function::IntFunctionFunctionId(99)),
                FunctionTableFamily::IntFunction,
                ValueType::Int,
            ),
            (
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::Float(function::FloatFunctionFunctionId(0)),
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::Float(function::FloatFunctionFunctionId(99)),
                FunctionTableFamily::FloatFunction,
                ValueType::Float,
            ),
            (
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::String(function::StringFunctionFunctionId(0)),
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::String(function::StringFunctionFunctionId(99)),
                FunctionTableFamily::StringFunction,
                ValueType::String,
            ),
            (
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::BitArray(function::BitArrayFunctionFunctionId(0)),
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::BitArray(function::BitArrayFunctionFunctionId(99)),
                FunctionTableFamily::BitArrayFunction,
                ValueType::BitArray,
            ),
            (
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::UtfCodepoint(function::UtfCodepointFunctionFunctionId(0)),
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::UtfCodepoint(function::UtfCodepointFunctionFunctionId(99)),
                FunctionTableFamily::UtfCodepointFunction,
                ValueType::UtfCodepoint,
            ),
            (
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::Bool(function::BoolFunctionFunctionId(0)),
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::Bool(function::BoolFunctionFunctionId(99)),
                FunctionTableFamily::BoolFunction,
                ValueType::Bool,
            ),
            (
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::Nil(function::NilFunctionFunctionId(0)),
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::Nil(function::NilFunctionFunctionId(99)),
                FunctionTableFamily::NilFunction,
                ValueType::Nil,
            ),
            (
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::Tuple(function::TupleFunctionFunctionId(0)),
                &function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::Tuple(function::TupleFunctionFunctionId(99)),
                FunctionTableFamily::TupleFunction,
                ValueType::Tuple(Table::Static(&[ValueType::Int, ValueType::Bool])),
            ),
        ];
        for (target, missing, family, inner) in cases {
            let expected = ValueType::Function(FunctionType {
                arguments: Table::Static(&[]),
                return_: Node::Owned(Box::new(inner.clone())),
            });
            assert_eq!(
                target.resolve(&catalog, &types).unwrap().return_type,
                &expected
            );
            assert_eq!(
                missing.resolve(&catalog, &types).err(),
                Some(CallError::Catalog(CatalogError::MissingFunction {
                    family,
                    index: 99
                }))
            );
            let wrong_inner = if inner == ValueType::Int {
                ValueType::Bool
            } else {
                ValueType::Int
            };
            let wrong_function = ValueType::Function(FunctionType {
                arguments: Table::Static(&[]),
                return_: Node::Owned(Box::new(wrong_inner)),
            });
            for wrong_type in [ValueType::Int, wrong_function] {
                let wrong_shape = ValueShapeId(
                    common
                        .value_shapes
                        .shape_types
                        .iter()
                        .position(|type_| type_ == &wrong_type)
                        .unwrap(),
                );
                let mut raw = copy_catalog(&common.function_parameters);
                let index = raw.families[family as usize].start;
                let mut contracts = raw.functions.into_vec();
                contracts[index].return_ = wrong_shape;
                raw.functions = contracts.into();
                let wrong = Catalog::admit(&raw, &plan.program.functions, &types).unwrap();
                assert_eq!(
                    target.resolve(&wrong, &types).err(),
                    Some(CallError::TargetType)
                );
            }
        }
    }

    #[test]
    fn never_targets_resolve_the_divergent_catalog_entry() {
        let plan = lower("pub fn main() { panic as \"stopped\" }");
        let types = execution_types(&plan.program);
        let catalog = Catalog::admit(
            &plan.program.common.function_parameters,
            &plan.program.functions,
            &types,
        )
        .unwrap();
        let target = function::NeverFunctionId(0)
            .resolve(&catalog, &types)
            .unwrap();
        assert!(target.parameters.is_empty());
        assert!(target.captures.is_empty());
        assert_eq!(
            function::NeverFunctionId(1).resolve(&catalog, &types).err(),
            Some(CallError::Catalog(CatalogError::MissingFunction {
                family: FunctionTableFamily::Never,
                index: 1,
            }))
        );
    }

    #[test]
    fn nominal_and_symbolic_targets_keep_metadata_without_accepting_missing_entries() {
        use crate::plan::execution::type_::GenericFunctionType;
        let source = r#"
pub type Box { Box(Int) }
fn identity(value) { value }
fn stop() -> a { panic }
fn generic() { identity }
fn never() { stop }
fn custom() { fn() { Box(42) } }
fn nested() { fn() { fn() { 42 } } }
pub fn main() {
  let _ = #(generic(), never(), custom(), nested())
  42
}
"#;
        let plan = lower(source);
        let common = &plan.program.common;
        let types = execution_types(&plan.program);
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let functions = &plan.program.functions.function_returns;
        let generic = &functions.generic_function_functions[0].body()._shape;
        let never = &functions.never_function_functions[0].body()._shape;
        for index in [0, 99] {
            let targets: [(Box<dyn Target>, FunctionTableFamily); 5] = [
                (
                    Box::new(function::CustomFunctionId {
                        index,
                        return_shape: plan.program.functions.value_returns.custom_functions[0]
                            .body()
                            ._signature_shape,
                    }),
                    FunctionTableFamily::Custom,
                ),
                (
                    Box::new(function::ProfiledFunctionFunctionId::<
                        function::HostedExecutionGraph,
                    >::Custom(
                        function::CustomFunctionFunctionId {
                            index,
                            type_: functions.custom_function_functions[0].body()._type.clone(),
                        },
                    )),
                    FunctionTableFamily::CustomFunction,
                ),
                (
                    Box::new(function::ProfiledFunctionFunctionId::<
                        function::HostedExecutionGraph,
                    >::Generic(
                        function::GenericFunctionFunctionId {
                            index,
                            type_: GenericFunctionType::from_shapes(
                                generic.type_.clone(),
                                generic.clone(),
                            ),
                        },
                    )),
                    FunctionTableFamily::GenericFunction,
                ),
                (
                    Box::new(function::ProfiledFunctionFunctionId::<
                        function::HostedExecutionGraph,
                    >::Never(
                        function::NeverFunctionFunctionId {
                            index,
                            type_: GenericFunctionType::from_shapes(
                                never.type_.clone(),
                                never.clone(),
                            ),
                        },
                    )),
                    FunctionTableFamily::NeverFunction,
                ),
                (
                    Box::new(function::ProfiledFunctionFunctionId::<
                        function::HostedExecutionGraph,
                    >::Function(
                        function::FunctionFunctionFunctionId {
                            index,
                            type_: functions.function_function_functions[0]
                                .body()
                                ._type
                                .clone(),
                        },
                    )),
                    FunctionTableFamily::FunctionFunction,
                ),
            ];
            for (target, family) in targets {
                if index == 0 {
                    assert_eq!(
                        target.resolve(&catalog, &types).unwrap().return_,
                        catalog.function(family, 0).unwrap().return_
                    );
                } else {
                    assert_eq!(
                        target.resolve(&catalog, &types).err(),
                        Some(CallError::Catalog(CatalogError::MissingFunction {
                            family,
                            index: 99
                        }))
                    );
                }
            }
        }
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(42.into())
        );
    }

    #[test]
    fn external_callable_wrappers_keep_their_nominal_result_and_table_identity() {
        use crate::plan::execution::prepared::admission::tests::lowered_native;

        let source = r#"
pub type Key
@external(erlang, "native", "key") fn key() -> Key
pub fn main() { fn() { key() } }
"#;
        let (program, _, _) = lowered_native(source);
        let types = execution_types(&program);
        let catalog = Catalog::admit(
            &program.common.function_parameters,
            &program.functions,
            &types,
        )
        .unwrap();
        let return_ = crate::plan::execution::type_::ExternalTypeId(0);
        let type_ = crate::plan::execution::type_::ExternalFunctionType::from_shapes(
            FunctionType::new(vec![], ValueType::External(return_)),
            vec![],
            return_,
        );
        for index in [0, 99] {
            let target =
                function::ProfiledFunctionFunctionId::<function::HostedExecutionGraph>::External(
                    function::ExternalFunctionFunctionId {
                        index,
                        type_: type_.clone(),
                    },
                );
            if index == 0 {
                assert_eq!(
                    target.resolve(&catalog, &types).unwrap().return_,
                    catalog
                        .function(FunctionTableFamily::ExternalFunction, 0)
                        .unwrap()
                        .return_
                );
            } else {
                assert_eq!(
                    target.resolve(&catalog, &types).err(),
                    Some(CallError::Catalog(CatalogError::MissingFunction {
                        family: FunctionTableFamily::ExternalFunction,
                        index: 99,
                    }))
                );
            }
        }
    }

    #[test]
    fn list_callable_targets_check_every_typed_signature_and_missing_index() {
        use crate::plan::execution::function::{
            ExecutionGraphProfile, HostedExecutionGraph, RuntimeFunctionFunctionTarget,
        };
        use crate::plan::execution::function::{
            ProfiledCoreRuntimeFunctionId as Core, ProfiledFunctionFunctionId as Function,
            ProfiledListFunctionFunctionId as List, ProfiledRuntimeFunctionId as Runtime,
        };
        use crate::plan::execution::graph::ExternalFunctionCallTarget;
        use crate::plan::execution::prepared::admission::catalog::CatalogError;
        use crate::plan::execution::prepared::admission::tests::lowered_native;
        use crate::plan::execution::type_::FunctionType;

        fn fields(value: &mut List<HostedExecutionGraph>) -> (&mut FunctionType, &mut usize) {
            match value {
                List::Parameter { type_, id, .. } => (type_, &mut id.0),
                List::ParameterList { type_, id, .. } => (type_, &mut id.0),
                List::Int { type_, id, .. } => (type_, &mut id.0),
                List::String { type_, id, .. } => (type_, &mut id.0),
                List::BitArray { type_, id, .. } => (type_, &mut id.0),
                List::UtfCodepoint { type_, id, .. } => (type_, &mut id.0),
                List::Custom { type_, id, .. } => (type_, &mut id.0),
                List::Float { type_, id, .. } => (type_, &mut id.0),
                List::Bool { type_, id, .. } => (type_, &mut id.0),
                List::Nil { type_, id, .. } => (type_, &mut id.0),
                List::Tuple { type_, id, .. } => (type_, &mut id.0),
                List::List { type_, id, .. } => (type_, &mut id.0),
                List::Function { type_, id, .. } => (type_, &mut id.0),
                List::External { type_, id, .. } => (type_, &mut id.0),
            }
        }
        let cases = [
            (FunctionTableFamily::ParameterListFunction, r#"[]"#),
            (FunctionTableFamily::IntFunction, r#"42"#),
            (FunctionTableFamily::ParameterListListFunction, r#"[[]]"#),
            (FunctionTableFamily::IntListFunction, r#"[42]"#),
            (FunctionTableFamily::StringListFunction, r#"["text"]"#),
            (FunctionTableFamily::BitArrayListFunction, r#"[<<42>>]"#),
            (
                FunctionTableFamily::UtfCodepointListFunction,
                r#"{ let assert <<point:utf8_codepoint>> = <<65>> [point] }"#,
            ),
            (FunctionTableFamily::CustomListFunction, r#"[Box(42)]"#),
            (FunctionTableFamily::ExternalListFunction, r#"[key()]"#),
            (FunctionTableFamily::FloatListFunction, r#"[1.5]"#),
            (FunctionTableFamily::BoolListFunction, r#"[True]"#),
            (FunctionTableFamily::NilListFunction, r#"[Nil]"#),
            (FunctionTableFamily::TupleListFunction, r#"[#(42, True)]"#),
            (FunctionTableFamily::ListListFunction, r#"[[42]]"#),
            (
                FunctionTableFamily::FunctionListFunction,
                r#"[fn(x) { x + 1 }]"#,
            ),
        ];
        for (family, expression) in cases {
            let source = format!(
                "pub type Box(a) {{ Box(a) }} pub type Key \
                @external(erlang, \"native\", \"key\") fn key() -> Key \
                pub fn main() {{ echo #(42, fn() {{ 42 }}) fn() {{ {expression} }} }}"
            );
            let (program, _, _) = lowered_native(&source);
            let types = execution_types(&program);
            let catalog = Catalog::admit(
                &program.common.function_parameters,
                &program.functions,
                &types,
            )
            .unwrap();
            let target = match &program.common.main {
                Runtime::Core(Core::Function {
                    id: RuntimeFunctionFunctionTarget::Core(Function::List(target)),
                    ..
                }) => Some(std::convert::Infallible::list_function_function(target)),
                Runtime::Core(Core::Function {
                    id:
                        RuntimeFunctionFunctionTarget::External(
                            ExternalFunctionCallTarget::ListFunction {
                                id,
                                type_,
                                list_type,
                            },
                        ),
                    ..
                }) => Some(List::External {
                    id: *id,
                    type_: type_.clone(),
                    list_type: *list_type,
                }),
                _ => None,
            };
            assert_eq!(target.is_some(), family != FunctionTableFamily::IntFunction);
            let Some(target) = target else { continue };
            let expected = catalog.function(family, 0).unwrap();
            let actual = target.resolve(&catalog, &types).unwrap();
            assert_eq!(actual.return_, expected.return_);
            assert!(std::ptr::eq(actual.return_type, expected.return_type));

            let wrapper = Function::<HostedExecutionGraph>::List(target.clone());
            assert_eq!(
                wrapper.resolve(&catalog, &types).unwrap().return_,
                expected.return_
            );

            for wrong_type in [
                ValueType::Int,
                ValueType::Function(FunctionType::new(Vec::new(), ValueType::Int)),
            ] {
                let wrong_shape = ValueShapeId(
                    program
                        .common
                        .value_shapes
                        .shape_types
                        .iter()
                        .position(|type_| type_ == &wrong_type)
                        .unwrap(),
                );
                let mut raw = copy_catalog(&program.common.function_parameters);
                let index = raw.families[family as usize].start;
                let mut contracts = raw.functions.into_vec();
                contracts[index].return_ = wrong_shape;
                raw.functions = contracts.into();
                let wrong = Catalog::admit(&raw, &program.functions, &types).unwrap();
                assert_eq!(
                    target.resolve(&wrong, &types).err(),
                    Some(CallError::TargetType)
                );
            }

            let mut wrong = target.clone();
            let (type_, _) = fields(&mut wrong);
            *type_ = FunctionType::new(vec![ValueType::Bool], type_.return_().clone());
            assert_eq!(
                wrong.resolve(&catalog, &types).err(),
                Some(CallError::TargetType)
            );

            let mut absent = target.clone();
            *fields(&mut absent).1 = 99;
            assert_eq!(
                absent.resolve(&catalog, &types).err(),
                Some(CallError::Catalog(CatalogError::MissingFunction {
                    family,
                    index: 99
                }))
            );
        }
    }

    #[test]
    fn list_targets_reject_missing_entries_and_non_list_results_in_every_family() {
        use crate::plan::execution::prepared::admission::tests::lowered_native;
        use function::{
            ListFunctionId as List, ProfiledCoreRuntimeFunctionId as Core, ProfiledListFunctionId,
            ProfiledRuntimeFunctionId as Runtime,
        };
        let cases = [
            (FunctionTableFamily::ParameterList, "[]"),
            (FunctionTableFamily::Int, "42"),
            (FunctionTableFamily::ParameterListList, "[[]]"),
            (FunctionTableFamily::IntList, "[42]"),
            (FunctionTableFamily::StringList, "[\"text\"]"),
            (FunctionTableFamily::BitArrayList, "[<<42>>]"),
            (
                FunctionTableFamily::UtfCodepointList,
                "{ let assert <<point:utf8_codepoint>> = <<65>> [point] }",
            ),
            (FunctionTableFamily::CustomList, "[Box(42)]"),
            (FunctionTableFamily::ExternalList, "[key()]"),
            (FunctionTableFamily::FloatList, "[1.5]"),
            (FunctionTableFamily::BoolList, "[True]"),
            (FunctionTableFamily::NilList, "[Nil]"),
            (FunctionTableFamily::TupleList, "[#(42, True)]"),
            (FunctionTableFamily::ListList, "[[42]]"),
            (FunctionTableFamily::FunctionList, "[fn(x: Int) { x + 1 }]"),
        ];
        for (family, expression) in cases {
            let source = format!(
                "pub type Box(a) {{ Box(a) }} pub type Key \
                @external(erlang, \"native\", \"key\") fn key() -> Key \
                pub fn main() {{ echo 42 {expression} }}"
            );
            let (program, _, _) = lowered_native(&source);
            let common = &program.common;
            let types = execution_types(&program);
            let catalog =
                Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
            let target = match &common.main {
                Runtime::Core(Core::List(target)) => Some(target),
                _ => None,
            };
            assert_eq!(target.is_some(), family != FunctionTableFamily::Int);
            let Some(target) = target else { continue };
            assert_eq!(
                target.resolve(&catalog, &types).unwrap().return_,
                catalog.function(family, 0).unwrap().return_
            );
            let mut missing = target.clone();
            let index = match &mut missing {
                ProfiledListFunctionId::External(id) => &mut id.index,
                ProfiledListFunctionId::Core(id) => match id {
                    List::Parameter(id) => &mut id.index,
                    List::ParameterList(id) => &mut id.index,
                    List::Int(id) => &mut id.index,
                    List::String(id) => &mut id.index,
                    List::BitArray(id) => &mut id.index,
                    List::UtfCodepoint(id) => &mut id.index,
                    List::Custom(id) => &mut id.index,
                    List::Float(id) => &mut id.index,
                    List::Bool(id) => &mut id.index,
                    List::Nil(id) => &mut id.index,
                    List::Tuple(id) => &mut id.index,
                    List::List(id) => &mut id.index,
                    List::Function(id) => &mut id.index,
                },
            };
            *index = 99;
            assert_eq!(
                missing.resolve(&catalog, &types).err(),
                Some(CallError::Catalog(CatalogError::MissingFunction {
                    family,
                    index: 99
                }))
            );
            let mut raw = copy_catalog(&common.function_parameters);
            let index = raw.families[family as usize].start;
            let mut contracts = raw.functions.into_vec();
            contracts[index].return_ = ValueShapeId(
                common
                    .value_shapes
                    .shape_types
                    .iter()
                    .position(|type_| type_ == &ValueType::Int)
                    .unwrap(),
            );
            raw.functions = contracts.into();
            let wrong = Catalog::admit(&raw, &program.functions, &types).unwrap();
            assert_eq!(
                target.resolve(&wrong, &types).err(),
                Some(CallError::TargetType)
            );
        }
    }

    fn lower(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap())
    }

    fn execution_types(
        program: &crate::plan::execution::ExecutionProgram<impl function::ExecutionProfile>,
    ) -> Types<'_> {
        let common = &program.common;
        Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap()
    }

    fn copy_catalog(raw: &FunctionCatalog) -> FunctionCatalog {
        FunctionCatalog {
            families: raw.families.clone(),
            functions: raw.functions.clone(),
            parameters: raw.parameters.clone(),
        }
    }
}

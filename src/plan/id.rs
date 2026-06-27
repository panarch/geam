#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalId {
    Int(IntLocalId),
    String(StringLocalId),
    Bool(BoolLocalId),
    Nil(NilLocalId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NilFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionFunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionId(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RuntimeFunctionId {
    Int(IntFunctionId),
    String(StringFunctionId),
    Bool(BoolFunctionId),
    Nil(NilFunctionId),
    Function {
        id: FunctionFunctionId,
        return_type: crate::plan::FunctionType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FunctionFunctionId {
    Int(IntFunctionFunctionId),
    String(StringFunctionFunctionId),
    Bool(BoolFunctionFunctionId),
    Nil(NilFunctionFunctionId),
    Function(FunctionFunctionFunctionId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilFunctionFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionFunctionFunctionId(pub(crate) usize);

impl FunctionId {
    pub(crate) fn new(index: usize) -> Self {
        Self(index)
    }

    #[cfg(test)]
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl FunctionFunctionId {
    pub(crate) fn int(self) -> IntFunctionFunctionId {
        match self {
            Self::Int(id) => id,
            _ => validated_function_tag_mismatch("Int"),
        }
    }

    pub(crate) fn string(self) -> StringFunctionFunctionId {
        match self {
            Self::String(id) => id,
            _ => validated_function_tag_mismatch("String"),
        }
    }

    pub(crate) fn bool(self) -> BoolFunctionFunctionId {
        match self {
            Self::Bool(id) => id,
            _ => validated_function_tag_mismatch("Bool"),
        }
    }

    pub(crate) fn nil(self) -> NilFunctionFunctionId {
        match self {
            Self::Nil(id) => id,
            _ => validated_function_tag_mismatch("Nil"),
        }
    }

    pub(crate) fn function(self) -> FunctionFunctionFunctionId {
        match self {
            Self::Function(id) => id,
            _ => validated_function_tag_mismatch("Function"),
        }
    }
}

#[cold]
fn validated_function_tag_mismatch(expected: &'static str) -> ! {
    panic!(
        "planner-validated function tag mismatch: expected {expected} function-returning function"
    )
}

#[cfg(test)]
mod tests {
    use super::{
        BoolFunctionFunctionId, BoolFunctionLocalId, FunctionFunctionFunctionId,
        FunctionFunctionId, FunctionFunctionLocalId, FunctionId, IntFunctionFunctionId,
        IntFunctionLocalId, NilFunctionFunctionId, NilFunctionLocalId, StringFunctionFunctionId,
        StringFunctionLocalId,
    };

    #[test]
    fn function_id_index() {
        assert_eq!(FunctionId::new(5).index(), 5);
    }

    #[test]
    fn function_local_id_debug_surface() {
        assert_eq!(
            format!("{:?}", IntFunctionLocalId(3)),
            "IntFunctionLocalId(3)"
        );
        assert_eq!(
            format!("{:?}", StringFunctionLocalId(3)),
            "StringFunctionLocalId(3)"
        );
        assert_eq!(
            format!("{:?}", BoolFunctionLocalId(3)),
            "BoolFunctionLocalId(3)"
        );
        assert_eq!(
            format!("{:?}", NilFunctionLocalId(3)),
            "NilFunctionLocalId(3)"
        );
        assert_eq!(
            format!("{:?}", FunctionFunctionLocalId(3)),
            "FunctionFunctionLocalId(3)"
        );
    }

    #[test]
    fn function_function_id_typed_projection() {
        assert_eq!(
            FunctionFunctionId::Int(IntFunctionFunctionId(1)).int(),
            IntFunctionFunctionId(1),
        );
        assert_eq!(
            FunctionFunctionId::String(StringFunctionFunctionId(2)).string(),
            StringFunctionFunctionId(2),
        );
        assert_eq!(
            FunctionFunctionId::Bool(BoolFunctionFunctionId(3)).bool(),
            BoolFunctionFunctionId(3),
        );
        assert_eq!(
            FunctionFunctionId::Nil(NilFunctionFunctionId(4)).nil(),
            NilFunctionFunctionId(4),
        );
        assert_eq!(
            FunctionFunctionId::Function(FunctionFunctionFunctionId(5)).function(),
            FunctionFunctionFunctionId(5),
        );
    }

    #[test]
    #[should_panic(expected = "planner-validated function tag mismatch")]
    fn function_function_id_int_projection_panics_on_mismatch() {
        FunctionFunctionId::String(StringFunctionFunctionId(1)).int();
    }

    #[test]
    #[should_panic(expected = "planner-validated function tag mismatch")]
    fn function_function_id_string_projection_panics_on_mismatch() {
        FunctionFunctionId::Int(IntFunctionFunctionId(1)).string();
    }

    #[test]
    #[should_panic(expected = "planner-validated function tag mismatch")]
    fn function_function_id_bool_projection_panics_on_mismatch() {
        FunctionFunctionId::Int(IntFunctionFunctionId(1)).bool();
    }

    #[test]
    #[should_panic(expected = "planner-validated function tag mismatch")]
    fn function_function_id_nil_projection_panics_on_mismatch() {
        FunctionFunctionId::Int(IntFunctionFunctionId(1)).nil();
    }

    #[test]
    #[should_panic(expected = "planner-validated function tag mismatch")]
    fn function_function_id_function_projection_panics_on_mismatch() {
        FunctionFunctionId::Int(IntFunctionFunctionId(1)).function();
    }
}

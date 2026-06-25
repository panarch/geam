#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalId {
    Int(IntLocalId),
    String(StringLocalId),
    Bool(BoolLocalId),
    Nil(NilLocalId),
    Function(FunctionLocalId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionLocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuntimeFunctionId {
    Int(IntFunctionId),
    String(StringFunctionId),
    Bool(BoolFunctionId),
    Nil(NilFunctionId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilFunctionId(pub(crate) usize);

impl FunctionId {
    pub(crate) fn new(index: usize) -> Self {
        Self(index)
    }

    #[cfg(test)]
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionId;

    #[test]
    fn function_id_index() {
        assert_eq!(FunctionId::new(5).index(), 5);
    }
}

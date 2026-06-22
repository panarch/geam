#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalId {
    Int(IntLocalId),
    String(StringLocalId),
    Bool(BoolLocalId),
    Nil(NilLocalId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NilLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId(pub usize);

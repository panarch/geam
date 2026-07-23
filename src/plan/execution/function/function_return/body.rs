use super::super::body::FunctionBody;
use super::{
    BitArrayFunctionFunctionId, BoolFunctionFunctionId, CustomFunctionFunctionId,
    FloatFunctionFunctionId, FunctionFunctionFunctionId, GenericFunctionFunctionId,
    IntFunctionFunctionId, ListFunctionFunctionId, NeverFunctionFunctionId, NilFunctionFunctionId,
    StringFunctionFunctionId, TupleFunctionFunctionId, UtfCodepointFunctionFunctionId,
};
use crate::plan::execution::function::FunctionBodyOwner;
use crate::plan::execution::graph::{
    BitArrayFunctionLocalId, BoolFunctionLocalId, CustomFunctionLocal, FloatFunctionLocalId,
    FunctionFunctionLocal, GenericFunctionLocal, IntFunctionLocalId, ListFunctionLocal,
    NeverFunctionLocal, NilFunctionLocalId, StringFunctionLocalId, TupleFunctionLocalId,
    UtfCodepointFunctionLocalId,
};
use crate::plan::execution::type_::{CustomFunctionType, FunctionFunctionType, FunctionShape};

pub(crate) type IntFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<IntFunctionLocalId, IntFunctionFunctionId>>;
pub(crate) type FloatFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<FloatFunctionLocalId, FloatFunctionFunctionId>>;
pub(crate) type StringFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<StringFunctionLocalId, StringFunctionFunctionId>>;
pub(crate) type BitArrayFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<BitArrayFunctionLocalId, BitArrayFunctionFunctionId>>;
pub(crate) type UtfCodepointFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<UtfCodepointFunctionLocalId, UtfCodepointFunctionFunctionId>>;
pub(crate) type GenericFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<GenericFunctionLocal, GenericFunctionFunctionId>>;
pub(crate) type NeverFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<NeverFunctionLocal, NeverFunctionFunctionId>>;
pub(crate) type BoolFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<BoolFunctionLocalId, BoolFunctionFunctionId>>;
pub(crate) type NilFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<NilFunctionLocalId, NilFunctionFunctionId>>;
pub(crate) type TupleFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<TupleFunctionLocalId, TupleFunctionFunctionId>>;
pub(crate) type ListFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<ListFunctionLocal, ListFunctionFunctionId>>;

pub(crate) struct CustomFunctionFunctionBody {
    _shape: FunctionShape,
    type_: CustomFunctionType,
    body: FunctionBody<CustomFunctionLocal, usize>,
}

pub(crate) struct FunctionFunctionFunctionBody {
    _shape: FunctionShape,
    type_: FunctionFunctionType,
    body: FunctionBody<FunctionFunctionLocal, usize>,
}

pub(crate) struct TypedFunctionBody<Body> {
    _shape: FunctionShape,
    body: Body,
}

impl CustomFunctionFunctionBody {
    pub(in crate::plan::execution) fn from_parts(
        shape: FunctionShape,
        type_: CustomFunctionType,
        body: FunctionBody<CustomFunctionLocal, usize>,
    ) -> Self {
        Self {
            _shape: shape,
            type_,
            body,
        }
    }

    pub(crate) fn function_body(&self) -> &FunctionBody<CustomFunctionLocal, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> CustomFunctionFunctionId {
        CustomFunctionFunctionId::new(index, self.type_.clone())
    }
}

impl FunctionFunctionFunctionBody {
    pub(in crate::plan::execution) fn from_parts(
        shape: FunctionShape,
        type_: FunctionFunctionType,
        body: FunctionBody<FunctionFunctionLocal, usize>,
    ) -> Self {
        Self {
            _shape: shape,
            type_,
            body,
        }
    }

    #[cfg(test)]
    pub(crate) fn type_(&self) -> &FunctionFunctionType {
        &self.type_
    }

    pub(crate) fn function_body(&self) -> &FunctionBody<FunctionFunctionLocal, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> FunctionFunctionFunctionId {
        FunctionFunctionFunctionId::new(index, self.type_.clone())
    }
}

impl<Body> TypedFunctionBody<Body> {
    pub(in crate::plan::execution) fn new(shape: FunctionShape, body: Body) -> Self {
        Self {
            _shape: shape,
            body,
        }
    }

    pub(crate) fn function_body(&self) -> &Body {
        &self.body
    }
}

impl<Body> FunctionBodyOwner for TypedFunctionBody<Body>
where
    Body: FunctionBodyOwner,
{
    type Return = Body::Return;
    type TailCall = Body::TailCall;

    fn function_body(&self) -> &FunctionBody<Self::Return, Self::TailCall> {
        self.body.function_body()
    }
}

impl FunctionBodyOwner for CustomFunctionFunctionBody {
    type Return = CustomFunctionLocal;
    type TailCall = usize;

    fn function_body(&self) -> &FunctionBody<Self::Return, Self::TailCall> {
        &self.body
    }
}

impl FunctionBodyOwner for FunctionFunctionFunctionBody {
    type Return = FunctionFunctionLocal;
    type TailCall = usize;

    fn function_body(&self) -> &FunctionBody<Self::Return, Self::TailCall> {
        &self.body
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{
        CustomFunctionFunctionBody, FunctionBodyOwner, FunctionFunctionFunctionBody,
        TypedFunctionBody,
    };
    use crate::plan::execution::explain;
    use crate::plan::execution::function::{FunctionFunctionId, RuntimeFunctionId};

    #[test]
    fn exposes_every_typed_function_return_body() {
        let sources = [
            "pub fn main() -> fn() -> Int { fn() { 1 } }",
            "pub fn main() -> fn() -> Float { fn() { 1.0 } }",
            "pub fn main() -> fn() -> String { fn() { \"one\" } }",
            "pub fn main() -> fn() -> BitArray { fn() { <<1>> } }",
            "pub fn main() -> fn() -> UtfCodepoint { fn() { panic } }",
            "pub fn main() -> fn(value) -> value { fn(value) { value } }",
            "pub fn main() -> fn() -> value { fn() { panic } }",
            "pub fn main() -> fn() -> Bool { fn() { True } }",
            "pub fn main() -> fn() -> Nil { fn() { Nil } }",
            "pub fn main() -> fn() -> #(Int) { fn() { #(1) } }",
            "pub fn main() -> fn() -> List(Int) { fn() { [] } }",
        ];

        for source in sources {
            assert_function_body_owner(source);
        }
    }

    #[test]
    fn exposes_custom_and_function_return_bodies() {
        let sources = [
            r#"
pub type Boxed { Boxed(Int) }
pub fn main() -> fn() -> Boxed { fn() { Boxed(1) } }
"#,
            "pub fn main() -> fn() -> fn() -> Int { fn() { fn() { 1 } } }",
        ];

        for source in sources {
            assert_function_body_owner(source);
        }
    }

    #[test]
    #[should_panic(expected = "source should lower a function-returning main function")]
    fn function_body_owner_shape_guard_is_visible() {
        assert_function_body_owner("pub fn main() { 1 }");
    }

    fn assert_typed_body_owner<Body>(owner: &TypedFunctionBody<Body>)
    where
        Body: FunctionBodyOwner,
    {
        let actual = <TypedFunctionBody<Body> as FunctionBodyOwner>::function_body(owner);
        let expected = FunctionBodyOwner::function_body(owner.function_body());
        assert!(std::ptr::eq(actual, expected));
    }

    fn assert_function_body_owner(source: &str) {
        explain::with_execution_plan(source, |plan| {
            let RuntimeFunctionId::Function { id, .. } = plan.main_runtime() else {
                panic!("source should lower a function-returning main function");
            };
            match id {
                FunctionFunctionId::Generic(id) => {
                    assert_typed_body_owner(plan.generic_function_function(&id).body());
                }
                FunctionFunctionId::Never(id) => {
                    assert_typed_body_owner(plan.never_function_function(&id).body());
                }
                FunctionFunctionId::Int(id) => {
                    assert_typed_body_owner(plan.int_function_function(id).body());
                }
                FunctionFunctionId::Float(id) => {
                    assert_typed_body_owner(plan.float_function_function(id).body());
                }
                FunctionFunctionId::String(id) => {
                    assert_typed_body_owner(plan.string_function_function(id).body());
                }
                FunctionFunctionId::BitArray(id) => {
                    assert_typed_body_owner(plan.bit_array_function_function(id).body());
                }
                FunctionFunctionId::UtfCodepoint(id) => {
                    assert_typed_body_owner(plan.utf_codepoint_function_function(id).body());
                }
                FunctionFunctionId::Custom(id) => {
                    let owner = plan.custom_function_function(&id).body();
                    let actual =
                        <CustomFunctionFunctionBody as FunctionBodyOwner>::function_body(owner);
                    assert!(std::ptr::eq(actual, owner.function_body()));
                }
                FunctionFunctionId::Bool(id) => {
                    assert_typed_body_owner(plan.bool_function_function(id).body());
                }
                FunctionFunctionId::Nil(id) => {
                    assert_typed_body_owner(plan.nil_function_function(id).body());
                }
                FunctionFunctionId::Tuple(id) => {
                    assert_typed_body_owner(plan.tuple_function_function(id).body());
                }
                FunctionFunctionId::List(id) => {
                    assert_typed_body_owner(plan.list_function_function(&id).body());
                }
                FunctionFunctionId::Function(id) => {
                    let owner = plan.function_function_function(&id).body();
                    let actual =
                        <FunctionFunctionFunctionBody as FunctionBodyOwner>::function_body(owner);
                    assert!(std::ptr::eq(actual, owner.function_body()));
                }
            }
        });
    }
}

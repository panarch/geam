use crate::GleamStdlibProviderProfile;
use crate::string_tree::{self, StringTree, StringTreePayload};
use geam_core::BitArrayValue;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostCustom, HostCustomConstructorAt,
    HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
    HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomIndex0,
    HostCustomIndexNext, HostCustomSchema, HostCustomType, HostListType, HostProfile, HostProvider,
    HostProviderModule, HostRegistrationError, HostType,
};
use geam_core::provider::{
    List, MissingListContext, ProviderConstruction, ProviderConstructions, ProviderExternalCodec,
    ProviderExternalPayloadAccess, ProviderInputValue, ProviderListContext, ProviderListInputCodec,
    ProviderListInputValue, ProviderListItemDecoder, ProviderListItemValue,
    ProviderNoConstructions, ProviderOutputValue, ProviderOwnedExternal, ProviderRootOutputValue,
    ProviderStaticValueForms, ProviderTypedListItemDecoder, ProviderValue, ProviderValueForms,
};
use geam_core::provider_support::bit_array_pad_to_bytes;
use std::convert::Infallible;

/// A retained, read-only input for the original `gleam/bytes_tree.BytesTree`.
///
/// Receive this value in an ordinary provider function. Its children and text
/// payloads remain retained until explicitly read; receiving it does not flatten
/// the tree. It owns its input handles and may cross a native suspension point.
/// It is an input adapter, not a constructor or output adapter for BytesTree.
/// Returning the input adapter does not grant output construction:
///
/// ```compile_fail
/// #[geam_macros::module(
///     path = "binary_sink",
///     crate_path = geam_core,
///     profile = geam_stdlib::GleamStdlibHostProfile,
///     component = geam_stdlib::Component<Profile::Io>,
/// )]
/// mod native {
///     use geam_stdlib::service;
///
///     #[geam_macros::function]
///     fn invalid(tree: service::BytesTreeInput) -> service::BytesTreeInput {
///         tree
///     }
/// }
/// # let _ = native::__geam_module::<geam_stdlib::GleamStdlibProfile>();
/// ```
pub struct BytesTreeInput {
    node: Node,
}

/// An owned binary leaf returned as the original `gleam/bytes_tree.BytesTree`.
///
/// Return this adapter from an ordinary provider function, directly or inside
/// tuples, Results, and Lists. Compose the standard-library component in the
/// host profile; the standard library owns the opaque schema and construction.
/// This is an output adapter, not a tree builder or an input adapter.
///
/// ```
/// #[geam_macros::module(
///     path = "binary_source",
///     crate_path = geam_core,
///     profile = geam_stdlib::GleamStdlibHostProfile,
///     component = geam_stdlib::Component<Profile::Io>,
/// )]
/// mod native {
///     use geam_core::BitArrayValue;
///     use geam_stdlib::service;
///
///     #[geam_macros::function]
///     fn make(bytes: BitArrayValue) -> service::BytesTreeOutput {
///         service::BytesTreeOutput::from_bit_array(bytes)
///     }
/// }
/// # let _ = native::__geam_module::<geam_stdlib::GleamStdlibProfile>();
/// ```
///
/// Receiving a tree uses [`BytesTreeInput`], rather than this output adapter:
///
/// ```compile_fail
/// #[geam_macros::module(
///     path = "binary_source",
///     crate_path = geam_core,
///     profile = geam_stdlib::GleamStdlibHostProfile,
///     component = geam_stdlib::Component<Profile::Io>,
/// )]
/// mod native {
///     use geam_stdlib::service;
///
///     #[geam_macros::function]
///     fn invalid(tree: service::BytesTreeOutput) -> service::BytesTreeOutput {
///         tree
///     }
/// }
/// # let _ = native::__geam_module::<geam_stdlib::GleamStdlibProfile>();
/// ```
pub struct BytesTreeOutput {
    bytes: BitArrayValue,
}

impl BytesTreeOutput {
    /// Moves an owned BitArray into a binary leaf without copying its payload.
    ///
    /// Like Gleam's `bytes_tree.from_bit_array`, construction pads a partial
    /// final byte with zero bits. Aligned inputs move unchanged; padding shares
    /// the canonical byte storage and extends only the logical bit length.
    /// Byte-range views keep their selected range. The returned tree retains
    /// its immutable storage independently of the call and its input aliases.
    pub fn from_bit_array(bytes: BitArrayValue) -> Self {
        let bytes = if bytes.bit_len().is_multiple_of(8) {
            bytes
        } else {
            bit_array_pad_to_bytes(&bytes)
        };
        Self { bytes }
    }
}

enum Node {
    Leaf(Leaf),
    Many(Box<Children>),
}

enum Leaf {
    Bytes(BitArrayValue),
    Text(ProviderOwnedExternal<StringTreePayload>),
}

type Host = HostCustomType<Schema>;
type Children = List<BytesTreeInput, ProviderListContext<Host, Decoder>>;

impl BytesTreeInput {
    /// Copies the tree's complete content into an independently owned BitArray.
    ///
    /// This explicit read preserves byte order and does not modify the input or
    /// its aliases. The result borrows neither a call nor an external store and
    /// remains usable after the input and execution have been dropped.
    /// Each call traverses the tree again; no flattened result is cached.
    pub fn to_bit_array(&self) -> BitArrayValue {
        let mut output = Vec::new();
        let mut pending = Vec::new();
        match &self.node {
            Node::Leaf(leaf) => leaf.append_bytes(&mut output),
            Node::Many(children) => pending.push(Frame {
                children: FrameChildren::Borrowed(children),
                next: 0,
            }),
        }
        while let Some(frame) = pending.last_mut() {
            let Some(child) = frame.next() else {
                pending.pop();
                continue;
            };
            match child.node {
                Node::Leaf(leaf) => leaf.append_bytes(&mut output),
                Node::Many(children) => pending.push(Frame {
                    children: FrameChildren::Owned(children),
                    next: 0,
                }),
            }
        }
        BitArrayValue::from_bytes(output)
    }
}

impl Leaf {
    fn append_bytes(&self, output: &mut Vec<u8>) {
        match self {
            Self::Bytes(bytes) => output.extend_from_slice(bytes.bytes()),
            Self::Text(tree) => tree.with(|payload| payload.append_bytes(output)),
        }
    }
}

struct Frame<'tree> {
    children: FrameChildren<'tree>,
    next: usize,
}

enum FrameChildren<'tree> {
    Borrowed(&'tree Children),
    Owned(Box<Children>),
}

impl Frame<'_> {
    fn next(&mut self) -> Option<BytesTreeInput> {
        let children = match &self.children {
            FrameChildren::Borrowed(children) => *children,
            FrameChildren::Owned(children) => children.as_ref(),
        };
        let child = children.get(self.next)?;
        self.next += 1;
        Some(child)
    }
}

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibProviderProfile,
{
    HostProviderModule::new("gleam_stdlib", "gleam/bytes_tree")
        .and_then(|module| module.with_shared_custom_type::<Schema>())
}

#[doc(hidden)]
pub struct Schema;

impl HostCustomSchema for Schema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/bytes_tree";
    const NAME: &'static str = "BytesTree";
    const PARAMETER_COUNT: usize = 0;
    const SHARED: bool = true;
    type Constructors = HostCustomConstructorList<
        Bytes,
        HostCustomConstructorList<
            Text,
            HostCustomConstructorList<Many, HostCustomConstructorListEnd>,
        >,
    >;
}

pub struct Bytes;
pub struct Text;
pub struct Many;
pub struct BytesField;
pub struct TextField;
pub struct ManyField;

impl HostCustomConstructorDefinition for Bytes {
    const NAME: &'static str = "Bytes";
    type Fields = HostCustomFieldList<BytesField, HostCustomFieldListEnd>;
}
impl HostCustomConstructorDefinition for Text {
    const NAME: &'static str = "Text";
    type Fields = HostCustomFieldList<TextField, HostCustomFieldListEnd>;
}
impl HostCustomConstructorDefinition for Many {
    const NAME: &'static str = "Many";
    type Fields = HostCustomFieldList<ManyField, HostCustomFieldListEnd>;
}
impl HostCustomField for BytesField {
    const LABEL: Option<&'static str> = None;
    type Type = BitArrayValue;
}
impl HostCustomField for TextField {
    const LABEL: Option<&'static str> = None;
    type Type = StringTree;
}
impl HostCustomField for ManyField {
    const LABEL: Option<&'static str> = None;
    type Type = HostListType<Host>;
}

type BytesConstructor = HostCustomConstructorAt<Host, HostCustomIndex0, Bytes>;
type TextConstructor = HostCustomConstructorAt<Host, HostCustomIndexNext<HostCustomIndex0>, Text>;
type ManyConstructor =
    HostCustomConstructorAt<Host, HostCustomIndexNext<HostCustomIndexNext<HostCustomIndex0>>, Many>;

impl ProviderValue for BytesTreeOutput {
    type Host = Host;
    type OutputRequirements = ProviderConstruction<Host>;
    type RootRequirements = ProviderNoConstructions;
}

impl ProviderValueForms for BytesTreeOutput {
    type InvocationRequirements = ();
    type Runtime<Profile: HostProfile> = ProviderStaticValueForms<Self>;
    type Output = Self;
    type ImmediateInput = Self;
    type ImmediateListInput = Self;
    type OwnedInput = Self;
    type OwnedListInput = Self;
    type ImmediateListDecoder = MissingListContext;
    type OwnedListDecoder = MissingListContext;
}

impl<Profile, Provider, Return> ProviderOutputValue<Profile, Provider, Return> for BytesTreeOutput
where
    Profile: GleamStdlibProviderProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Error = Infallible;

    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<HostCustom<'call, Host>, Self::Error> {
        Ok(call.construct_custom::<BytesConstructor>(constructions.token(), (self.bytes, ())))
    }
}

impl<Profile, Provider> ProviderRootOutputValue<Profile, Provider> for BytesTreeOutput
where
    Profile: GleamStdlibProviderProfile,
    Provider: HostProvider<Profile>,
{
    fn complete<'call>(
        self,
        call: HostCall<'call, Profile, Provider, Host>,
        _: &ProviderConstructions<'call, Self::RootRequirements>,
    ) -> Result<HostCallCompletion<'call, Host>, HostCallError> {
        Ok(call.return_custom::<BytesConstructor>((self.bytes, ())))
    }
}

impl ProviderValue for BytesTreeInput {
    type Host = Host;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl ProviderValueForms for BytesTreeInput {
    type InvocationRequirements = ();
    type Runtime<Profile: HostProfile> = ProviderStaticValueForms<Self>;
    type Output = Self;
    type ImmediateInput = Self;
    type ImmediateListInput = Self;
    type OwnedInput = Self;
    type OwnedListInput = Self;
    type ImmediateListDecoder = Decoder;
    type OwnedListDecoder = Decoder;
}

impl<Profile, Provider, Return> ProviderInputValue<Profile, Provider, Return> for BytesTreeInput
where
    Profile: GleamStdlibProviderProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Host = Host;
    type Requirements = ProviderNoConstructions;

    fn from_host_with<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: HostCustom<'call, Host>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self {
        let node = if let Some((bytes, ())) = call.custom_fields::<BytesConstructor>(value) {
            Node::Leaf(Leaf::Bytes(bytes))
        } else if let Some((text, ())) = call.custom_fields::<TextConstructor>(value) {
            Node::Leaf(Leaf::Text(<StringTreePayload as ProviderExternalCodec<
                Profile,
            >>::owned_input(call, text)))
        } else {
            let (children, ()) =
                call.provider_borrow_remaining_custom_fields::<ManyConstructor>(value);
            Node::Many(Box::new(
                call.provider_retained_list(children, Decoder::from_call(call)),
            ))
        };
        Self { node }
    }
}

/// Statically linked item decoder retained with BytesTree's lazy child Lists.
#[doc(hidden)]
#[derive(Clone)]
pub struct Decoder {
    text: ProviderExternalPayloadAccess<StringTreePayload>,
}

impl Decoder {
    fn from_call<Profile, Provider, Return>(call: &HostCall<'_, Profile, Provider, Return>) -> Self
    where
        Profile: GleamStdlibProviderProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        Self {
            text: string_tree::payload_access(call),
        }
    }
}

impl ProviderListItemDecoder<BytesTreeInput> for Decoder {
    type View = BytesTreeInput;

    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
        let mut fields = value.into_custom();
        let node = match fields.constructor() {
            0 => Node::Leaf(Leaf::Bytes(fields.take_field(0).into_scalar())),
            1 => Node::Leaf(Leaf::Text(fields.take_field(0).into_external(&self.text))),
            _ => Node::Many(Box::new(fields.take_field(0).into_typed_list(self.clone()))),
        };
        BytesTreeInput { node }
    }
}

impl ProviderTypedListItemDecoder<BytesTreeInput> for Decoder {
    type Host = Host;
}

impl ProviderListInputValue for BytesTreeInput {
    type Host = Host;
    type View = Self;
    type Decoder = Decoder;
}

impl<Profile, Provider> ProviderListInputCodec<Profile, Provider> for BytesTreeInput
where
    Profile: GleamStdlibProviderProfile,
    Provider: HostProvider<Profile>,
{
    type Requirements = ProviderNoConstructions;

    fn decoder_with<'call, Return: HostType>(
        call: &HostCall<'call, Profile, Provider, Return>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self::Decoder {
        Decoder::from_call(call)
    }
}

#[cfg(test)]
mod tests {
    use super::{BytesTreeInput, BytesTreeOutput, Leaf, Node, host_provider};
    use crate::string_tree::{STRING_TREE_DECLARATIONS, host_provider as string_provider};
    use crate::{
        GleamStdlibProfile, GleamStdlibRunState, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, compile_typed_host_program, plan_host_program,
    };
    use geam_core::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use geam_core::frontend::HostedTypedProgram;
    use geam_core::plan::{CustomType, FunctionType, ValueType};
    use geam_core::{BitArrayValue, StringValue};
    use geam_core::{CustomTypeName, HostProviderLinkReason, PlanError};

    const DECLARATIONS: &str = r#"
import gleam/string_tree.{type StringTree}
pub opaque type BytesTree {
  Bytes(BitArray)
  Text(StringTree)
  Many(List(BytesTree))
}
pub fn bytes(bits: BitArray) { Bytes(bits) }
pub fn text(value: StringTree) { Text(value) }
pub fn many(children: List(BytesTree)) { Many(children) }
"#;

    #[geam_macros::module(
        path = "bytes_tree_consumer",
        crate_path = geam_core,
        profile = crate::GleamStdlibHostProfile,
        component = crate::Component<Profile::Io>,
    )]
    mod native {
        use super::{Leaf, Node};
        use crate::service;
        use geam_core::BitArrayValue;

        #[geam_macros::function]
        fn read(tree: service::BytesTreeInput) -> BitArrayValue {
            tree.to_bit_array()
        }

        #[geam_macros::function]
        fn read_list(trees: List<service::BytesTreeInput>) -> Vec<BitArrayValue> {
            let mut bytes = Vec::new();
            let mut index = 0;
            while let Some(tree) = trees.get(index) {
                bytes.push(tree.to_bit_array());
                index += 1;
            }
            bytes
        }

        #[geam_macros::function]
        fn make(bytes: BitArrayValue) -> service::BytesTreeOutput {
            service::BytesTreeOutput::from_bit_array(bytes)
        }

        #[geam_macros::function]
        fn pair(bytes: BitArrayValue) -> (service::BytesTreeOutput, BitArrayValue) {
            (
                service::BytesTreeOutput::from_bit_array(bytes.clone()),
                bytes,
            )
        }

        #[geam_macros::function]
        fn binary_field(tree: service::BytesTreeInput) -> Result<BitArrayValue, ()> {
            match tree.node {
                Node::Leaf(Leaf::Bytes(bytes)) => Ok(bytes),
                _ => Err(()),
            }
        }
    }

    #[geam_macros::module(
        path = "bytes_tree_output_contract",
        crate_path = geam_core,
        profile = crate::GleamStdlibHostProfile,
        component = crate::Component<Profile::Io>,
    )]
    mod output_contract {
        use crate::service;
        use geam_core::BitArrayValue;

        #[geam_macros::function]
        fn make(bytes: BitArrayValue) -> service::BytesTreeOutput {
            service::BytesTreeOutput::from_bit_array(bytes)
        }
    }

    fn execution(source: &str) -> HostedExecution<GleamStdlibProfile> {
        let typed = program(source, DECLARATIONS, true, "bytes_tree.BytesTree");
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap()
    }

    fn program(
        source: &str,
        declarations: &str,
        shared: bool,
        read_argument: &str,
    ) -> HostedTypedProgram<GleamStdlibProfile> {
        let mut providers = vec![
            string_provider::<GleamStdlibProfile>().unwrap(),
            native::__geam_module::<GleamStdlibProfile>().unwrap(),
        ];
        if shared {
            providers.push(host_provider::<GleamStdlibProfile>().unwrap());
        }
        compile_typed_host_program(
            "gleam_stdlib",
            "bytes_tree_consumer",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<&str>::new(),
                [
                    ModuleSource::new(
                        "gleam/string_tree",
                        "string_tree.gleam",
                        STRING_TREE_DECLARATIONS,
                    ),
                    ModuleSource::new("gleam/bytes_tree", "bytes_tree.gleam", declarations),
                    ModuleSource::new(
                        "bytes_tree_consumer",
                        "consumer.gleam",
                        format!(
                            r#"
import gleam/bytes_tree
import gleam/string_tree
@external(erlang, "native", "read")
fn read(tree: {read_argument}) -> BitArray
@external(erlang, "native", "read_list")
fn read_list(trees: List({read_argument})) -> List(BitArray)
@external(erlang, "native", "make")
fn make(bytes: BitArray) -> bytes_tree.BytesTree
@external(erlang, "native", "pair")
fn pair(bytes: BitArray) -> #(bytes_tree.BytesTree, BitArray)
@external(erlang, "native", "binary_field")
fn binary_field(tree: bytes_tree.BytesTree) -> Result(BitArray, Nil)
{source}
"#
                        ),
                    ),
                ],
            )],
            HostProviderSet::from_providers(providers).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn owned_output_pads_partial_bits_without_copying_its_input_storage() {
        let bytes = BitArrayValue::try_from_parts(vec![255], 3).unwrap();
        let alias = bytes.clone();
        let output = BytesTreeOutput::from_bit_array(bytes);
        assert_eq!(output.bytes.bytes().as_ptr(), alias.bytes().as_ptr());
        drop(alias);
        assert_eq!(output.bytes.bytes(), [224]);
        assert_eq!(output.bytes.bit_len(), 8);
    }

    #[test]
    fn root_and_nested_codecs_preserve_storage_and_the_padded_or_selected_range() {
        for (source, expected, input_bit_len, output_bit_len) in [
            (
                r#"pub fn main() {
  let bits = <<5:size(3)>>
  let assert Ok(leaf) = binary_field(make(bits))
  #(bits, leaf)
}"#,
                &[160][..],
                3,
                8,
            ),
            (
                r#"pub fn main() {
  let assert <<_:bytes-size(1), bits:bytes-size(2), _:bytes>> = <<42, 255, 0, 99>>
  let pair = pair(bits)
  let assert Ok(leaf) = binary_field(pair.0)
  #(pair.1, leaf)
}"#,
                &[255, 0][..],
                16,
                16,
            ),
        ] {
            let typed = program(source, DECLARATIONS, true, "bytes_tree.BytesTree");
            let (bindings, function) = HostedModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(), (BitArrayValue, BitArrayValue)>::new("main"))
                .unwrap();
            let mut module = bindings.seal().unwrap();
            let mut state = GleamStdlibRunState::from_seed([0; 32]);
            let host = crate::execution_fixture::TestHost::default();
            let mut echo = Vec::new();
            let (original, leaf) = host
                .block_on(
                    module.with_execution(&host, &mut state, &mut echo, async |scope| {
                        scope.call(&function, ()).await
                    }),
                )
                .unwrap()
                .try_into_value()
                .unwrap()
                .unwrap();
            assert_eq!(leaf.bytes().as_ptr(), original.bytes().as_ptr());
            assert_eq!(original.bit_len(), input_bit_len);
            drop(module);
            drop(state);
            drop(original);
            assert_eq!(leaf.bytes(), expected);
            assert_eq!(leaf.bit_len(), output_bit_len);
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn constructed_leaves_have_the_original_nominal_schema_and_reject_non_binary_fields() {
        let mut execution = execution(
            r#"
pub fn main() {
  let leaf = make(<<5:size(3)>>)
  #(leaf, read(leaf), binary_field(bytes_tree.text(string_tree.from_string("x"))), binary_field(bytes_tree.many([])))
}
"#,
        );
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        let result = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(
            result.inspect().to_string(),
            "#(Bytes(<<160>>), <<160>>, Error(Nil), Error(Nil))"
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn ordinary_input_and_list_codecs_preserve_binary_text_order_and_aliases() {
        let mut execution = execution(
            r#"
pub fn main() {
  let text = string_tree.concat([
    string_tree.from_string("한"),
    string_tree.from_string("\u{0}e\u{301}🙂"),
  ])
  let tree = bytes_tree.many([
    bytes_tree.bytes(<<0, 255, 128>>),
    bytes_tree.many([bytes_tree.many([]), bytes_tree.text(text)]),
    bytes_tree.bytes(<<1, 2>>),
  ])
  let alias = tree
  #(read(tree), read(alias), read_list([tree, bytes_tree.many([])]), read_list([]))
}
"#,
        );
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        let result = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap()
            .try_into_value()
            .unwrap();
        drop(execution);
        drop(state);
        assert_eq!(
            result.inspect().to_string(),
            concat!(
                "#(<<0, 255, 128, 237, 149, 156, 0, 101, 204, 129, 240, 159, 153, 130, 1, 2>>, ",
                "<<0, 255, 128, 237, 149, 156, 0, 101, 204, 129, 240, 159, 153, 130, 1, 2>>, ",
                "[<<0, 255, 128, 237, 149, 156, 0, 101, 204, 129, 240, 159, 153, 130, 1, 2>>, <<>>], [])",
            )
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn raw_string_tree_leaves_reach_bytes_tree_without_text_validation() {
        let typed = program(
            r#"
pub fn main(value: String) {
  let text = string_tree.concat([
    string_tree.from_string("λ"),
    string_tree.from_string(value),
  ])
  let tree = bytes_tree.many([
    bytes_tree.text(text),
    bytes_tree.many([bytes_tree.text(string_tree.from_string(value))]),
  ])
  #(read(tree), read(tree))
}
"#,
            DECLARATIONS,
            true,
            "bytes_tree.BytesTree",
        );
        let (bindings, function) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<
                (StringValue,),
                (BitArrayValue, BitArrayValue),
            >::new("main"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        for _ in 0..2 {
            let input = StringValue::from_bytes(vec![255, 0, 195]);
            let (first, second) = host
                .block_on(
                    module.with_execution(&host, &mut state, &mut echo, async |scope| {
                        scope.call(&function, (input,)).await
                    }),
                )
                .unwrap()
                .try_into_value()
                .unwrap()
                .unwrap();
            assert_eq!(first.bytes(), [206, 187, 255, 0, 195, 255, 0, 195]);
            assert_eq!(second.bytes(), first.bytes());
            drop(first);
            assert_eq!(second.bytes(), [206, 187, 255, 0, 195, 255, 0, 195]);
        }
        assert!(echo.is_empty());
    }

    #[test]
    fn root_leaf_codecs_and_byte_list_items_read_all_original_branches() {
        let mut execution = execution(
            r#"
pub fn main() {
  let assert <<_:size(8), slice:bytes-size(2), _:bytes>> = <<42, 255, 0, 99>>
  let binary = bytes_tree.bytes(slice)
  let text = bytes_tree.text(string_tree.from_string("\u{0}한"))
  #(read(binary), read(text), read_list([binary, text, bytes_tree.many([])]))
}
"#,
        );
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        let result = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(
            result.inspect().to_string(),
            "#(<<255, 0>>, <<0, 237, 149, 156>>, [<<255, 0>>, <<0, 237, 149, 156>>, <<>>])"
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn deep_many_input_is_read_iteratively_without_reconstructing_the_tree() {
        let mut execution = execution(
            r#"
fn nest(tree: bytes_tree.BytesTree, depth: Int) -> bytes_tree.BytesTree {
  case depth {
    0 -> tree
    _ -> nest(bytes_tree.many([tree]), depth - 1)
  }
}
pub fn main() {
  let tree = nest(bytes_tree.bytes(<<255, 0, 42>>), 10000)
  #(read(tree), read(tree))
}
"#,
        );
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        let result = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(
            result.inspect().to_string(),
            "#(<<255, 0, 42>>, <<255, 0, 42>>)"
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn owned_binary_read_copies_only_on_request_and_outlives_its_input() {
        let leaf = BitArrayValue::from_bytes(vec![0, 255, 128, 42]);
        let input = BytesTreeInput {
            node: Node::Leaf(Leaf::Bytes(leaf.clone())),
        };
        let first = input.to_bit_array();
        let second = input.to_bit_array();
        assert_eq!(first.bytes(), leaf.bytes());
        assert_eq!(second.bytes(), leaf.bytes());
        assert_ne!(first.bytes().as_ptr(), leaf.bytes().as_ptr());
        assert_ne!(second.bytes().as_ptr(), first.bytes().as_ptr());
        drop(input);
        drop(leaf);
        assert_eq!(first.bytes(), [0, 255, 128, 42]);
        assert_eq!(second.bit_len(), 32);
    }

    #[test]
    fn opaque_consumption_requires_its_original_producer_grant() {
        let typed = program(
            "pub fn main() { read(bytes_tree.bytes(<<255>>)) }",
            DECLARATIONS,
            false,
            "bytes_tree.BytesTree",
        );
        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "gleam_stdlib".into(),
                module: "bytes_tree_consumer".into(),
                function: "read".into(),
                reason: Box::new(HostProviderLinkReason::MissingSharedCustomType {
                    custom_type: CustomTypeName::new(
                        "gleam_stdlib".into(),
                        "gleam/bytes_tree".into(),
                        "BytesTree".into(),
                    ),
                }),
            }),
        );
    }

    #[test]
    fn producer_grant_allows_an_output_only_provider_to_construct_the_original_leaf() {
        let typed = output_program(
            "pub fn main() { make(<<255>>) }",
            true,
            "bytes_tree.BytesTree",
        );
        let mut execution =
            HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        let result = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(result.inspect().to_string(), "Bytes(<<255>>)");
        assert!(echo.is_empty());
    }

    #[test]
    fn opaque_output_requires_its_original_producer_grant() {
        let typed = output_program(
            "pub fn main() { make(<<255>>) }",
            false,
            "bytes_tree.BytesTree",
        );
        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "gleam_stdlib".into(),
                module: "bytes_tree_output_contract".into(),
                function: "make".into(),
                reason: Box::new(HostProviderLinkReason::MissingSharedCustomType {
                    custom_type: CustomTypeName::new(
                        "gleam_stdlib".into(),
                        "gleam/bytes_tree".into(),
                        "BytesTree".into(),
                    ),
                }),
            }),
        );
    }

    fn output_program(
        source: &str,
        shared: bool,
        make_return: &str,
    ) -> HostedTypedProgram<GleamStdlibProfile> {
        let mut providers = vec![
            string_provider::<GleamStdlibProfile>().unwrap(),
            output_contract::__geam_module::<GleamStdlibProfile>().unwrap(),
        ];
        if shared {
            providers.push(host_provider::<GleamStdlibProfile>().unwrap());
        }
        compile_typed_host_program(
            "gleam_stdlib",
            "bytes_tree_output_contract",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<&str>::new(),
                [
                    ModuleSource::new(
                        "gleam/string_tree",
                        "string_tree.gleam",
                        STRING_TREE_DECLARATIONS,
                    ),
                    ModuleSource::new(
                        "gleam/bytes_tree",
                        "bytes_tree.gleam",
                        format!("{DECLARATIONS}\npub type OtherTree {{ OtherTree(BytesTree) }}"),
                    ),
                    ModuleSource::new(
                        "bytes_tree_output_contract",
                        "output.gleam",
                        format!(
                            r#"
import gleam/bytes_tree
@external(erlang, "native", "make")
fn make(bytes: BitArray) -> {make_return}
{source}
"#
                        ),
                    ),
                ],
            )],
            HostProviderSet::from_providers(providers).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn output_linkage_rejects_scalar_substitution_and_a_different_nominal_type() {
        for (return_type, expected_return) in [
            ("BitArray", ValueType::BitArray),
            (
                "bytes_tree.OtherTree",
                ValueType::Custom(CustomType::new(
                    CustomTypeName::new(
                        "gleam_stdlib".into(),
                        "gleam/bytes_tree".into(),
                        "OtherTree".into(),
                    ),
                    Vec::new(),
                )),
            ),
        ] {
            let typed = output_program("pub fn main() { make(<<255>>) }", true, return_type);
            let expected_type = FunctionType::new(vec![ValueType::BitArray], expected_return);
            let actual_type = FunctionType::new(
                vec![ValueType::BitArray],
                ValueType::Custom(CustomType::new(
                    CustomTypeName::new(
                        "gleam_stdlib".into(),
                        "gleam/bytes_tree".into(),
                        "BytesTree".into(),
                    ),
                    Vec::new(),
                )),
            );
            assert_eq!(
                plan_host_program(typed).err().unwrap().to_string(),
                format!(
                    "host provider gleam_stdlib::bytes_tree_output_contract.make: function scheme mismatch: expected TypeScheme {{ parameters: [] }} {expected_type:?}, got TypeScheme {{ parameters: [] }} {actual_type:?}"
                )
            );
        }
    }

    #[test]
    fn producer_grant_rejects_an_incompatible_recursive_source_schema() {
        let declarations = DECLARATIONS.replace("List(BytesTree)", "List(BitArray)");
        let typed = program(
            "pub fn main() { read(bytes_tree.bytes(<<255>>)) }",
            &declarations,
            true,
            "bytes_tree.BytesTree",
        );
        let error = plan_host_program(typed).err().unwrap();
        assert!(error.to_string().starts_with(concat!(
            "shared custom type provider gleam_stdlib::gleam/bytes_tree.BytesTree: ",
            "shared custom schema mismatch: ",
        )));
    }

    #[test]
    fn input_linkage_rejects_scalar_substitution_and_a_different_nominal_type() {
        let declarations = format!(
            "{DECLARATIONS}\npub type OtherTree {{ OtherTree(BytesTree) }}\npub fn other() {{ OtherTree(bytes(<<255>>)) }}"
        );
        for (argument, source) in [
            ("BitArray", "pub fn main() { read(<<255>>) }"),
            (
                "bytes_tree.OtherTree",
                "pub fn main() { read(bytes_tree.other()) }",
            ),
        ] {
            let typed = program(source, &declarations, true, argument);
            let error = plan_host_program(typed).err().unwrap();
            assert!(error.to_string().starts_with(concat!(
                "host provider gleam_stdlib::bytes_tree_consumer.read: ",
                "function scheme mismatch: ",
            )));
        }
    }
}

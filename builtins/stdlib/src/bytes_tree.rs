use crate::GleamStdlibProviderProfile;
use crate::string_tree::{self, StringTree, StringTreePayload};
use geam_core::BitArrayValue;
use geam_core::host::{
    HostCall, HostCustom, HostCustomConstructorAt, HostCustomConstructorDefinition,
    HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList,
    HostCustomFieldListEnd, HostCustomIndex0, HostCustomIndexNext, HostCustomSchema,
    HostCustomType, HostListType, HostProfile, HostProvider, HostProviderModule,
    HostRegistrationError, HostType,
};
use geam_core::provider::{
    List, ProviderConstructions, ProviderExternalCodec, ProviderExternalPayloadAccess,
    ProviderInputValue, ProviderListContext, ProviderListInputCodec, ProviderListInputValue,
    ProviderListItemDecoder, ProviderListItemValue, ProviderNoConstructions, ProviderOwnedExternal,
    ProviderStaticValueForms, ProviderTypedListItemDecoder, ProviderValue, ProviderValueForms,
};

/// A retained, read-only input for the original `gleam/bytes_tree.BytesTree`.
///
/// Receive this value in an ordinary provider function. Its children and text
/// payloads remain retained until explicitly read; receiving it does not flatten
/// the tree. It owns its input handles and may cross a native suspension point.
/// It is an input adapter, not a constructor or output adapter for BytesTree.
pub struct BytesTreeInput {
    node: Node,
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
    use super::{BytesTreeInput, Leaf, Node, host_provider};
    use crate::string_tree::{STRING_TREE_DECLARATIONS, host_provider as string_provider};
    use crate::{
        GleamStdlibProfile, GleamStdlibRunState, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, compile_typed_host_program, plan_host_program,
    };
    use geam_core::BitArrayValue;
    use geam_core::frontend::HostedTypedProgram;
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

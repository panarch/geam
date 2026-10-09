use super::local::values;
use super::shape::CallLocal;
use super::{CallGroupCodegen, CallTarget, Code, ExecutionGraphProfile, Rust, target_id};
use crate::plan::HostCallSite;

impl<Graph: ExecutionGraphProfile> CallGroupCodegen<'_, '_, Graph> {
    pub(super) fn is_native(&self, target: CallTarget, arguments: &[CallLocal]) -> bool {
        matches!(target, CallTarget::String(id) if self.native_strings.contains(&id.0))
            && arguments.iter().all(CallLocal::native_argument)
    }

    pub(super) fn has_native(&self) -> bool {
        self.functions.iter().any(|function| {
            function.shape.calls.iter().any(|call| {
                matches!(call.target,
                super::CallContractTarget::Static(target) if self.is_native(target, &call.args))
            }) || function
                .shape
                .tails
                .iter()
                .any(|tail| self.is_native(tail.target, &tail.args))
        })
    }

    pub(super) fn write_native_request(
        &self,
        source: &mut Code,
        target: CallTarget,
        site: &HostCallSite,
        arguments: &[CallLocal],
        caller: &str,
    ) {
        source.push_str(&format!(
            "return FunctionStep::StringNative {{ function: {}, site: {}, arguments: {}, caller: {caller} }};\n",
            target_id(target), Rust::expression(site), values(arguments, true),
        ));
    }

    pub(super) fn write_native_delivery(&self, source: &mut Code, native: bool) {
        source.open("if let Some(result) = self.native_result.take() {\n");
        source.open("if let Some(caller) = self.native_caller.take().or_else(|| self.string_returns.pop()) {\n");
        source.push_str(&format!(
            "self.active = Some({});\n",
            self.active("caller.small(result)")
        ));
        source.alternative("} else {\n");
        for family in self.return_families() {
            source.push_str(&format!("self.{}.clear();\n", family.return_stack()));
        }
        let result =
            "CallProgress::Complete { output: CallOutput::String(result), execution: self }";
        source.push_str(&format!(
            "return {};\n",
            if native {
                format!("Ok(Some({result}))")
            } else {
                result.to_owned()
            }
        ));
        source.close("}\n");
        source.close("}\n");
    }
}

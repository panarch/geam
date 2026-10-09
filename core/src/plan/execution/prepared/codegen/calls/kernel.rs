use super::shape::{CallFunction, KernelReturn};
use super::{CallFamily, CallGroupCodegen, Code, function_name, state_name, target_family};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::prepared::codegen::{CompiledShape, FunctionCodegen, KernelKind};
use crate::plan::execution::prepared::rust::Rust;

impl<Graph: ExecutionGraphProfile> CallGroupCodegen<'_, '_, Graph> {
    pub(super) fn write_kernel_state(&self, source: &mut Code, function: &CallFunction<'_, Graph>) {
        if let Some(kernel) = &function.kernel
            && kernel.kind != KernelKind::Numeric
        {
            source.push_str(&format!(
                "{}Kernel {{ point: usize, values: Box<{}> }},\n",
                state_name(function.target, 0).trim_end_matches("Point0"),
                kernel.kind.values()
            ));
        }
    }

    pub(super) fn write_kernel_resume(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
    ) {
        if let Some(kernel) = &function.kernel
            && kernel.kind != KernelKind::Numeric
        {
            source.open(&format!(
                "FunctionState::{}Kernel {{ point, mut values }} => {{\n",
                state_name(function.target, 0).trim_end_matches("Point0")
            ));
            source.push_str(&format!(
                "let progress = {}(point, &mut values, budget);\n{}_kernel(progress, values, ops)\n",
                kernel_name(function, kernel),
                function_name(function.target)
            ));
            source.close("},\n");
        }
    }

    pub(super) fn write_kernel_bodies(&self, source: &mut Code) {
        let local = self
            .functions
            .iter()
            .filter_map(|function| {
                function
                    .kernel
                    .as_ref()
                    .filter(|_| {
                        matches!(
                            target_family(function.target),
                            CallFamily::String | CallFamily::BitArray
                        )
                    })
                    .map(|kernel| (function, kernel))
            })
            .collect::<Vec<_>>();
        if !local.is_empty() {
            source.open("enum CompiledResume {\n");
            if local.iter().any(|(_, kernel)| kernel.resumes_forward()) {
                source.push_str("Next(usize),\n");
            }
            source.push_str("Exit(data::compiled::CompiledProgress),\n");
            source.close("}\n");
            let resumes_next = local.iter().any(|(_, kernel)| kernel.resumes_forward());
            for (function, kernel) in local {
                let name = kernel_name(function, kernel);
                FunctionCodegen {
                    name: &name,
                    shape: kernel,
                }
                .write_code(source, resumes_next);
            }
        }
    }

    pub(super) fn write_kernel_step(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
        kernel: &CompiledShape<'_, Graph>,
        point: usize,
    ) {
        if kernel.kind == KernelKind::Numeric {
            self.write_numeric_step(source, function, kernel, point);
            return;
        }
        let checkpoint = function.shape.checkpoints[point];
        let kind = if kernel.kind == KernelKind::String {
            "strings"
        } else {
            "bit_arrays"
        };
        let count = if kernel.kind == KernelKind::String {
            checkpoint.strings
        } else {
            checkpoint.bit_arrays
        };
        let local = if kernel.kind == KernelKind::String {
            "string"
        } else {
            "bit_array"
        };
        let ints = (0..checkpoint.ints)
            .map(|local| format!("int{local}"))
            .collect::<Vec<_>>()
            .join(", ");
        let bools = (0..checkpoint.bools)
            .map(|local| format!("bool{local}"))
            .collect::<Vec<_>>()
            .join(", ");
        let inputs = (0..count)
            .map(|index| format!("{local}{index}"))
            .collect::<Vec<_>>()
            .join(", ");
        source.push_str(&format!(
            "let mut values = ops.{kind}(&[{ints}], &[{bools}], [{inputs}]);\n"
        ));
        let name = kernel_name(function, kernel);
        let run = format!("{name}({point}, &mut values, budget)");
        source.push_str(&format!(
            "let progress = {run};\n{}_kernel(progress, values, ops)\n",
            function_name(function.target)
        ));
    }

    pub(super) fn write_kernel(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
        kernel: &CompiledShape<'_, Graph>,
    ) {
        if kernel.kind == KernelKind::Numeric {
            self.write_numeric(source, function, kernel);
            return;
        }
        let values_type = kernel.kind.values();
        let strings = kernel.kind == KernelKind::String;
        let column = if strings { "strings" } else { "bit_arrays" };
        source.open(&format!("fn {}_kernel(progress: data::compiled::CompiledProgress, mut values: Box<{values_type}>, ops: &mut CallOps<'_>) -> FunctionStep {{\n", function_name(function.target)));
        source.open(&format!(
            "const RETURNS: [fn(&mut {values_type}) -> FunctionStep; {}] = [\n",
            function.kernel_returns.len()
        ));
        for local in &function.kernel_returns {
            let value = local.value_expression();
            let value = match local {
                KernelReturn::String(_) => format!("values.finish({value})"),
                KernelReturn::BitArray(_) => format!("CallBitArray::from(values.finish({value}))"),
                KernelReturn::Int(_) | KernelReturn::Bool(_) => value,
            };
            source.open("|values| {\n");
            source.push_str(&format!("let value = {value};\n"));
            if matches!(local, KernelReturn::Int(_) | KernelReturn::Bool(_)) {
                source.push_str("values.release_inputs();\n");
            }
            source.push_str(&format!(
                "FunctionStep::{} {{ value }}\n",
                target_family(function.target)
            ));
            source.close("},\n");
        }
        source.close("];\n");
        source.open("match progress {\n");
        source.push_str(&format!(
            "data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(FunctionState::{}Kernel {{ point, values }}),\n",
            state_name(function.target, 0).trim_end_matches("Point0")
        ));
        source.open("data::compiled::CompiledProgress::Interpreted(point) => {\n");
        source.open(&format!(
            "const POINTS: [data::compiled::CompiledCheckpoint; {}] = [\n",
            kernel.checkpoints.len()
        ));
        for checkpoint in &kernel.checkpoints {
            source.push_str(&format!("{},\n", Rust::expression(checkpoint)));
        }
        source.close("];\n");
        source.push_str(&format!("let {column} = values.take_{column}();\n"));
        let owned = if strings {
            "strings"
        } else {
            "bit_arrays: bit_arrays.into_iter().map(CallBitArray::from).collect()"
        };
        source.push_str(&format!("let step = FunctionStep::Canonical {{ target: {}, point: POINTS[point], values: Box::new(CallValues {{ ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), {owned}, ..CallValues::default() }}) }};\nops.recycle_{column}(values);\nstep\n", Rust::expression(&function.target)));
        source.close("},\n");
        source.open("data::compiled::CompiledProgress::Complete(exit) => {\n");
        source.push_str(&format!(
            "let step = RETURNS[exit.0](&mut values);\nops.recycle_{column}(values);\nstep\n"
        ));
        source.close("},\n");
        source.close("}\n");
        source.close("}\n");
    }
}

impl KernelReturn {
    pub(super) fn value_expression(&self) -> String {
        match self {
            Self::Int(local) => format!("values.ints[{}]", local.0),
            Self::Bool(local) => format!("values.bools[{}]", local.0),
            Self::String(local) => format!("values.strings[{}]", local.0),
            Self::BitArray(local) => format!("values.bit_arrays[{}]", local.0),
        }
    }
}

fn kernel_name<Graph: ExecutionGraphProfile>(
    function: &CallFunction<'_, Graph>,
    kernel: &CompiledShape<'_, Graph>,
) -> String {
    format!(
        "{}_{}_{}",
        kernel.kind.name(),
        target_family(function.target).name().to_lowercase(),
        function.target.index()
    )
}

#[cfg(test)]
mod tests {
    use super::KernelReturn;
    use crate::plan::execution::graph::{BitArrayLocalId, BoolLocalId, IntLocalId, StringLocalId};

    #[test]
    fn completed_kernel_values_read_the_exact_typed_storage_column() {
        for (returned, expected) in [
            (KernelReturn::Int(IntLocalId(2)), "values.ints[2]"),
            (KernelReturn::Bool(BoolLocalId(3)), "values.bools[3]"),
            (KernelReturn::String(StringLocalId(4)), "values.strings[4]"),
            (
                KernelReturn::BitArray(BitArrayLocalId(5)),
                "values.bit_arrays[5]",
            ),
        ] {
            assert_eq!(returned.value_expression(), expected);
        }
    }
}

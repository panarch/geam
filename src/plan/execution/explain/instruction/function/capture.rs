use super::super::super::value::ExplainLocal;
use crate::plan::execution::graph::FunctionCapture;

pub(super) fn write_capture(output: &mut String, capture: &FunctionCapture) {
    match capture {
        FunctionCapture::Int { target, source } => write_pair(output, target, source),
        FunctionCapture::Float { target, source } => write_pair(output, target, source),
        FunctionCapture::String { target, source } => write_pair(output, target, source),
        FunctionCapture::BitArray { target, source } => write_pair(output, target, source),
        FunctionCapture::UtfCodepoint { target, source } => write_pair(output, target, source),
        FunctionCapture::Custom { target, source } => write_pair(output, target, source),
        FunctionCapture::Bool { target, source } => write_pair(output, target, source),
        FunctionCapture::Nil { target, source } => write_pair(output, target, source),
        FunctionCapture::Tuple { target, source } => write_pair(output, target, source),
        FunctionCapture::ParameterList { target, source } => write_pair(output, target, source),
        FunctionCapture::ParameterListList { target, source } => write_pair(output, target, source),
        FunctionCapture::IntList { target, source } => write_pair(output, target, source),
        FunctionCapture::StringList { target, source } => write_pair(output, target, source),
        FunctionCapture::BitArrayList { target, source } => write_pair(output, target, source),
        FunctionCapture::UtfCodepointList { target, source } => write_pair(output, target, source),
        FunctionCapture::CustomList { target, source } => write_pair(output, target, source),
        FunctionCapture::FloatList { target, source } => write_pair(output, target, source),
        FunctionCapture::BoolList { target, source } => write_pair(output, target, source),
        FunctionCapture::NilList { target, source } => write_pair(output, target, source),
        FunctionCapture::TupleList { target, source } => write_pair(output, target, source),
        FunctionCapture::ListList { target, source } => write_pair(output, target, source),
        FunctionCapture::FunctionList { target, source } => write_pair(output, target, source),
        FunctionCapture::IntFunction { target, source } => write_pair(output, target, source),
        FunctionCapture::FloatFunction { target, source } => write_pair(output, target, source),
        FunctionCapture::StringFunction { target, source } => write_pair(output, target, source),
        FunctionCapture::BitArrayFunction { target, source } => write_pair(output, target, source),
        FunctionCapture::UtfCodepointFunction { target, source } => {
            write_pair(output, target, source);
        }
        FunctionCapture::GenericFunction { target, source } => write_pair(output, target, source),
        FunctionCapture::NeverFunction { target, source } => write_pair(output, target, source),
        FunctionCapture::CustomFunction { target, source } => write_pair(output, target, source),
        FunctionCapture::BoolFunction { target, source } => write_pair(output, target, source),
        FunctionCapture::NilFunction { target, source } => write_pair(output, target, source),
        FunctionCapture::TupleFunction { target, source } => write_pair(output, target, source),
        FunctionCapture::ListFunction { target, source } => write_pair(output, target, source),
        FunctionCapture::FunctionFunction { target, source } => write_pair(output, target, source),
    }
}

fn write_pair<Target, Source>(output: &mut String, target: &Target, source: &Source)
where
    Target: ExplainLocal,
    Source: ExplainLocal,
{
    target.write_local(output);
    output.push_str("<-");
    source.write_local(output);
}

use crate::codegen::dumper::internal_config::ConfigDumpContent;
use crate::codegen::dumper::Dumper;
use crate::codegen::generator::misc::path_texts::PathTexts;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::ir::mir::pack::MirPackComputedCache;
use crate::utils::basic_code::general_code::GeneralCode;

pub(crate) mod internal_config;
pub(crate) mod spec_generator;
mod text_generator;

pub(crate) struct GeneratorWireRustOutput {
    pub output_texts: PathTexts,
    pub extern_funcs: Vec<ExternFunc>,
    pub content_hash: i32,
    pub extern_struct_names: Vec<String>,
}

pub(crate) fn generate(
    context: WireRustGeneratorContext,
    dumper: &Dumper,
) -> anyhow::Result<GeneratorWireRustOutput> {
    let spec = spec_generator::generate(context, dumper)?;
    (dumper.with_content(ConfigDumpContent::GeneratorSpec)).dump("wire_rust.json", &spec)?;

    let text = text_generator::generate(&spec, context.config)?;
    (dumper.with_content(ConfigDumpContent::GeneratorText)).dump_acc(
        "wire_rust",
        "rs",
        &text.text,
    )?;

    Ok(GeneratorWireRustOutput {
        output_texts: PathTexts::new_from_targets(
            &context.config.rust_output_path,
            &(text.text.clone()).map(|x, _| x.map(GeneralCode::new_rust)),
        ),
        extern_funcs: text.extern_funcs,
        content_hash: spec.misc.content_hash,
        extern_struct_names: spec.extern_struct_names,
    })
}

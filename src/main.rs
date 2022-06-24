use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use tera::Tera;
use tree_sitter::Query;

mod parse;

#[derive(thiserror::Error, Debug)]
pub enum CppDeriveError {
    #[error("No input file provided")]
    NoInputFile,
}

const CLASSES_QUERY: &str = include_str!("classes.scm");

/// Generates C++ sources from source code annotations
#[derive(clap::Parser, Debug)]
#[clap(version, author)]
struct Args {
    /// Folder path for C++ templates
    #[clap(short, long)]
    template_folder: PathBuf,
    /// Output file name (will be extended by .h/.cpp to generate header and source file)
    output_file: PathBuf,
    /// Input C++ files to parse for annotations
    input_files: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.input_files.is_empty() {
        return Err(CppDeriveError::NoInputFile.into());
    }

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(tree_sitter_cuda::language())
        .expect("Error loading cuda grammar");

    let class_query = Query::new(tree_sitter_cuda::language(), CLASSES_QUERY)
        .with_context(|| "Query compilation failed")?;
    let tera = Tera::new(format!("{}/**/*", args.template_folder.to_string_lossy()).as_str())?;

    let mut cpp_file = File::create(format!("{}.cpp", args.output_file.to_string_lossy()))?;
    let mut header_file = File::create(format!("{}.hpp", args.output_file.to_string_lossy()))?;

    let _ = header_file.write(b"#pragma once\n")?;
    let _ = cpp_file.write(
        format!(
            "#include \"{}.hpp\"\n",
            args.output_file.file_name().unwrap().to_string_lossy()
        )
        .as_bytes(),
    )?;

    for path in args.input_files.iter() {
        let source_code = std::fs::read(path)?;
        let classes = parse::parse_classes(&mut parser, &source_code, &class_query)?;

        let mut per_attribute = HashMap::new();
        for c in classes.values() {
            for a in c.attributes.iter() {
                per_attribute.entry(a).or_insert(Vec::new()).push(c);
            }
        }
        for (attribute, classes) in per_attribute {
            let mut context = tera::Context::new();
            context.insert("classes", &classes);
            dbg!(&context);

            let _ = cpp_file.write(
                tera.render(format!("{attribute}/source.cpp").as_str(), &context)?
                    .as_bytes(),
            )?;
            let _ = header_file.write(
                tera.render(format!("{attribute}/header.hpp").as_str(), &context)?
                    .as_bytes(),
            )?;
        }
    }

    Ok(())
}

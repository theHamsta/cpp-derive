use std::{collections::HashMap, path::PathBuf, fs::File, io::Write};

use anyhow::Context;
use clap::Parser;
use tera::Tera;
use tree_sitter::Query;

mod parse_file;

#[derive(thiserror::Error, Debug)]
pub enum CppDeriveError {
    #[error("No input file provided")]
    NoInputFile,
}

const QUERY_SOURCE: &str = include_str!("query.scm");

/// Generates C++ sources from source code annotations
#[derive(clap::Parser, Debug)]
#[clap(version, author)]
struct Args {
    /// Folder path for C++ templates
    #[clap(short, long)]
    template_folder: PathBuf,
    /// Input C++ files to parse for annotations
    output_file: PathBuf,
    /// Output file name (will be extended by .h/.cpp to generate header and source file)
    input_files: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.input_files.len() < 1 {
        return Err(CppDeriveError::NoInputFile.into());
    }

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(tree_sitter_cuda::language())
        .expect("Error loading cuda grammar");

    let query = Query::new(tree_sitter_cuda::language(), &QUERY_SOURCE)
        .with_context(|| "Query compilation failed")?;
    let tera = Tera::new(format!("{}/**/*", args.template_folder.to_string_lossy()).as_str())?;
    for path in args.input_files.iter() {
        let source_code = std::fs::read(path)?;
        let classes = parse_file::parse_classes(&mut parser, &source_code, &query)?;

        let mut per_attribute = HashMap::new();
        for c in classes.values() {
            for a in c.attributes.iter() {
                per_attribute.entry(a).or_insert(Vec::new()).push(c);
            }
        }
        let mut cpp_file = File::create(format!("{}.cpp", args.output_file.to_string_lossy()))?;
        let mut header_file = File::create(format!("{}.hpp", args.output_file.to_string_lossy()))?;

        header_file.write(b"#pragma once")?;
        for (attribute, classes) in per_attribute {
            let mut context = tera::Context::new();
            context.insert("classes", &classes);
            context.insert("header_name", &format!("{}.hpp", args.output_file.file_name().unwrap().to_string_lossy()));

            cpp_file.write(tera.render(format!("{attribute}/source.cpp").as_str(), &context)?.as_bytes())?;
            header_file.write(tera.render(format!("{attribute}/header.hpp").as_str(), &context)?.as_bytes())?;
        }
    }

    Ok(())
}

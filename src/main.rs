use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use tree_sitter::Query;

mod parse_file;

#[derive(thiserror::Error, Debug)]
pub enum CppDeriveError {
    #[error("No input file provided")]
    NoInputFile,
}

const QUERY_SOURCE: &str = include_str!("query.scm");

/// Reads the unit test format for highlighting of tree-sitter
/// https://tree-sitter.github.io/tree-sitter/syntax-highlighting#unit-testing to make it available for
/// unit test for https://github.com/nvim-treesitter/nvim-treesitter.
/// Output will be printed to stdout.
#[derive(clap::Parser, Debug)]
#[clap(version, author)]
struct Args {
    /// Source file with highlight assertions following https://tree-sitter.github.io/tree-sitter/syntax-highlighting#unit-testing
    output_file: PathBuf,
    /// Parser library to load (e.g. cpp.so from nvim-treesitter/parser)
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

    for path in args.input_files.iter() {
        let source_code = std::fs::read(path)?;
        let classes = parse_file::parse_classes(&mut parser, &source_code, &query);
        dbg!(&classes);
    }

    Ok(())
}

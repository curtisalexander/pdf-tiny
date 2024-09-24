use anyhow::{Context, Result};
use clap::Parser;
use glob::glob;
use owo_colors::OwoColorize;
use std::path::PathBuf;
use xshell::{cmd, Shell};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input PDF file
    #[arg(short, long)]
    path: std::path::PathBuf
}

fn main() -> anyhow::Result<()> {

    let args = Args::parse();

    let gs_pattern = "C:\\Program Files\\gs\\**\\bin";
    let gs_path = get_first_path(&gs_pattern)?;
    let gs_executable = gs_path.join("gswin64c.exe");

    let input_file = args.path;
    if input_file.extension() != Some(std::ffi::OsStr::new("pdf")) { anyhow::bail!("Input file provided is not a PDF (i.e. does not have a pdf extension): {:?}", input_file)};

    let input_file_stem = input_file.file_stem().unwrap().to_str().unwrap();
    let output_file = input_file.parent().unwrap().join(format!("{input_file_stem}_tiny.pdf"));

    println!("Shrinking: {}", input_file.display().blue());
    println!("Tiny version will be: {}", output_file.display().green());
    println!("\n{} the command...\n", String::from("Running").purple());

    let sh = Shell::new()?;
    cmd!(sh, "{gs_executable} -sDEVICE=pdfwrite -dCompatibilityLevel=1.4 -dPDFSETTINGS=/ebook -dNOPAUSE -dQUIET -dBATCH -sOutputFile={output_file} {input_file}").run()?;

    Ok(())
}

fn get_first_path(pattern: &str) -> Result<PathBuf, anyhow::Error> {
    let paths = glob(pattern).with_context(|| format!("Pattern error: {:?}", pattern));

    for entry in paths? {
        match entry {
            Ok(path) => { return Ok(path.to_path_buf()) },
            Err(e) => { anyhow::bail!("Match error: {}", e); }
        }
    }
    anyhow::bail!("No path found based on pattern: {:?}", pattern);
}

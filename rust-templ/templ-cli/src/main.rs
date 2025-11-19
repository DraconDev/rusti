mod generator;
mod parser;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use anyhow::Context;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Generate {
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Generate { input } => {
            let content = fs::read_to_string(&input).context("Failed to read input file")?;
            let (_, templ) = parser::parse_templ(&content).map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
            let code = generator::generate(&templ).context("Failed to generate code")?;
            
            let output_path = input.with_extension("rs");
            fs::write(&output_path, code).context("Failed to write output file")?;
            
            println!("Generated {}", output_path.display());
        }
    }
    Ok(())
}

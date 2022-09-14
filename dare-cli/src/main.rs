use std::{
    io::{self, Read},
    path::PathBuf,
    str::FromStr,
};

use clap::{Parser, Subcommand};
use clipboard::ClipboardProvider;

#[cfg(not(target_os = "linux"))]
use clipboard::ClipboardContext;
#[cfg(target_os = "linux")]
use clipboard_ext::x11_fork::ClipboardContext;

use dare::{Solutions, TableauWriter};

#[derive(Debug)]
enum OutputFormat {
    Latex,
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "latex" => Ok(OutputFormat::Latex),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

#[derive(Parser, Debug)]
struct Tableau {
    /// The output format.
    ///
    /// Currently only latex is supported.
    format: OutputFormat,

    /// Solve the given tableau when it evaluates to false.
    #[clap(short = 'f', long = "false")]
    expect_false: bool,

    /// Copy the output to the clipboard.
    #[clap(short, long)]
    clip_board: bool,

    /// If this is used and source isn't supplied, the expression will be read path.
    #[clap(short, long)]
    path: Option<PathBuf>,

    /// The logical expression to evaluate.
    ///
    /// If not provided, the expression will be read from stdin.
    source: Option<String>,
}

#[derive(Parser, Debug)]
struct Solve {
    /// Solve the given tableau when it evaluates to false.
    #[clap(short = 'f', long = "false")]
    expect_false: bool,

    /// If this is used and source isn't supplied, the expression will be read path.
    #[clap(short, long)]
    path: Option<PathBuf>,

    /// The logical expression to evaluate.
    ///
    /// If not provided, the expression will be read from stdin.
    source: Option<String>,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Print the tableau for a given logical expression.
    Tableau(Tableau),
    /// Print the solutions for a given logical expression.
    Solve(Solve),
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

fn get_source(source: Option<String>, path: Option<PathBuf>) -> String {
    match source {
        Some(source) => source,
        None => {
            if let Some(path) = path {
                std::fs::read_to_string(path).expect("Failed to read file")
            } else {
                let mut buffer = String::new();
                io::stdin()
                    .read_to_string(&mut buffer)
                    .expect("Failed to read from stdin");
                buffer
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.subcommand {
        SubCommand::Tableau(command) => {
            let source = get_source(command.source, command.path);

            // TODO: proper error reporting
            let tableau = dare::Tableau::parse(&source, !command.expect_false).unwrap();

            let output = match command.format {
                OutputFormat::Latex => {
                    let mut latex = dare::LatexTableauWriter::default();
                    latex.write_tableau(&tableau).unwrap();
                    latex.finalize()
                }
            };

            println!("{}", output);
            if command.clip_board {
                ClipboardContext::new()
                    .unwrap()
                    .set_contents(output)
                    .unwrap();
            }
        }
        SubCommand::Solve(command) => {
            let source = get_source(command.source, command.path);

            let tableau = dare::Tableau::parse(&source, !command.expect_false).unwrap();
            let mut solutions = Solutions::from(&tableau);
            solutions.clean();

            if solutions.is_empty() {
                println!("No solutions found.");
            }

            for (i, solution) in solutions.iter().enumerate() {
                println!("Solution #{}", i);

                for (variable, value) in solution.iter() {
                    let value = if value { "T" } else { "F" };
                    println!("\t{}: {}", variable, value);
                }

                if i < solutions.len() - 1 {
                    println!();
                }
            }
        }
    }
}

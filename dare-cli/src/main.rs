use std::{io, str::FromStr};

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

    /// The logical expression to evaluate.
    ///
    /// If not provided, the expression will be read from stdin.
    source: Option<String>,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Print the tableau for a given logical expression.
    Tableau(Tableau),
    Solve(Solve),
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

fn get_source(source: Option<String>) -> String {
    match source {
        Some(source) => source,
        None => {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            buffer
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.subcommand {
        SubCommand::Tableau(print) => {
            let source = get_source(print.source);

            // TODO: proper error reporting
            let tableau = dare::Tableau::parse(&source, !print.expect_false).unwrap();

            let output = match print.format {
                OutputFormat::Latex => {
                    let mut latex = dare::LatexTableauWriter::default();
                    latex.write_tableau(&tableau).unwrap();
                    latex.finalize()
                }
            };

            println!("{}", output);
            if print.clip_board {
                ClipboardContext::new()
                    .unwrap()
                    .set_contents(output)
                    .unwrap();
            }
        }
        SubCommand::Solve(solve) => {
            let source = get_source(solve.source);

            let tableau = dare::Tableau::parse(&source, !solve.expect_false).unwrap();
            let mut solutions = Solutions::from(&tableau);
            solutions.clean();

            print!("Solutions:");

            for solution in solutions.iter() {
                println!();

                for (variable, value) in solution.iter() {
                    let value = if value { "T" } else { "F" };
                    println!("\t{}: {}", variable, value);
                }
            }
        }
    }
}

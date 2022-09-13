use std::{io, str::FromStr};

use clap::{Parser, Subcommand};
use clipboard::ClipboardProvider;

#[cfg(not(target_os = "linux"))]
use clipboard::ClipboardContext;
#[cfg(target_os = "linux")]
use clipboard_ext::x11_fork::ClipboardContext;

use dare::{TableauBuilder, TableauWriter};

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

    /// Copy the output to the clipboard.
    #[clap(short, long)]
    clip_board: bool,

    /// The logical expression to evaluate.
    ///
    /// If not provided, the expression will be read from stdin.
    source: Option<String>,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Print the tableau for a given logical expression.
    Tableau(Tableau),
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

fn main() {
    let args = Args::parse();

    match args.subcommand {
        SubCommand::Tableau(print) => {
            let parser = dare::Parser::new();

            let source = if let Some(source) = print.source {
                source
            } else {
                let mut source = String::new();
                io::stdin().read_line(&mut source).unwrap();
                source
            };

            // TODO: proper error reporting
            let expr = parser.parse(&source).unwrap();
            let builder = TableauBuilder::default();
            let tableau = builder.build_expression(&expr, true);

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
    }
}

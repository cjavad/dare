use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Latex,
}

#[derive(Parser, Debug)]
pub struct Tableau {
    /// The output format.
    #[clap(value_enum)]
    pub format: OutputFormat,

    /// Solve the given tableau when it evaluates to false.
    #[clap(short = 'f', long = "false")]
    pub expect_false: bool,

    /// Show the id of each branch and expression.
    #[clap(short, long)]
    pub show_all_ids: bool,

    /// Copy the output to the clipboard.
    #[clap(short, long)]
    pub clip_board: bool,

    /// If this is used and source isn't supplied, the expression will be read path.
    #[clap(short, long)]
    pub path: Option<PathBuf>,

    /// The logical expression to evaluate.
    ///
    /// If not provided, the expression will be read from stdin.
    pub source: Option<String>,
}

#[derive(Parser, Debug)]
pub struct Solve {
    /// Solve the given tableau when it evaluates to false.
    #[clap(short = 'f', long = "false")]
    pub expect_false: bool,

    /// If this is used and source isn't supplied, the expression will be read path.
    #[clap(short, long)]
    pub path: Option<PathBuf>,

    /// The logical expression to evaluate.
    ///
    /// If not provided, the expression will be read from stdin.
    pub source: Option<String>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum CompleteCommand {
    /// Installs completions for the given shell.
    Install {
        /// The shell to generate completions for.
        ///
        /// Options: bash, elvish, fish, powershell, zsh
        #[clap(value_enum)]
        shell: Shell,
    },
    /// Uninstalls completions for the given shell.
    Uninstall {
        /// The shell to generate completions for.
        #[clap(value_enum)]
        shell: Shell,
    },
}

#[derive(ValueEnum, Clone, Debug)]
#[cfg(target_os = "linux")]
pub enum Shell {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
}

#[derive(ValueEnum, Clone, Debug)]
#[cfg(target_os = "windows")]
pub enum Shell {
    PowerShell,
}

#[derive(ValueEnum, Clone, Debug)]
#[cfg(target_os = "macos")]
pub enum Shell {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
}

#[cfg(target_os = "linux")]
impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shell::Bash => write!(f, "bash"),
            Shell::Elvish => write!(f, "elvish"),
            Shell::Fish => write!(f, "fish"),
            Shell::PowerShell => write!(f, "powershell"),
            Shell::Zsh => write!(f, "zsh"),
        }
    }
}

#[cfg(target_os = "windows")]
impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shell::PowerShell => write!(f, "powershell"),
        }
    }
}

#[cfg(target_os = "macos")]
impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shell::Bash => write!(f, "bash"),
            Shell::Elvish => write!(f, "elvish"),
            Shell::Fish => write!(f, "fish"),
            Shell::PowerShell => write!(f, "powershell"),
            Shell::Zsh => write!(f, "zsh"),
        }
    }
}

#[derive(Parser, Debug)]
pub struct Complete {
    #[clap(subcommand)]
    pub subcommand: CompleteCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Print the tableau for a given logical expression.
    Tableau(Tableau),
    /// Print the solutions for a given logical expression.
    Solve(Solve),
    /// Installs the completion script for the given shell.
    Complete(Complete),
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

use clap::{Parser, Subcommand};
use anyhow::Result;

mod git;
mod commands;

use commands::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show added but not staged files
    #[command(alias = "a")]
    Added,
    
    /// Show modified but not staged files
    #[command(alias = "m")]
    Modified,
    
    /// Show all unstaged files
    #[command(alias = "u")]
    Unstaged,
    
    /// Show files different from branch
    #[command(alias = "bf")]
    BranchFiles {
        /// Target branch to compare against
        #[arg(default_value = "main")]
        branch: String,
    },
    
    /// Group files by type
    #[command(alias = "g")]
    Group {
        /// Show condensed oneline format
        #[arg(long)]
        oneline: bool,
    },
    
    /// Show diff for file set
    #[command(alias = "d")]
    Diff {
        /// File command to diff (added, modified, etc.)
        file_command: Option<String>,
    },
    
    /// Show diff of staged files
    #[command(alias = "ds")]
    DiffStaged,
    
    /// Show diff for specific file
    #[command(alias = "df")]
    DiffFile {
        /// File path to diff
        file: String,
    },
    
    /// Run tool on file set
    #[command(alias = "r")]
    Run {
        /// Tool to run
        tool: String,
        /// File command (added, modified, etc.)
        file_command: Option<String>,
    },
    
    /// Run prettier on file set
    #[command(alias = "p")]
    Prettier {
        /// File command (added, modified, etc.)
        file_command: Option<String>,
    },
    
    /// Run formatter on file set
    #[command(alias = "f")]
    Format {
        /// File command (added, modified, etc.)
        file_command: Option<String>,
    },
    
    /// Run linter on file set
    #[command(alias = "l")]
    Lint {
        /// File command (added, modified, etc.)
        file_command: Option<String>,
    },
    
    /// Run rubocop on file set
    #[command(alias = "rb")]
    Rubocop {
        /// File command (added, modified, etc.)
        file_command: Option<String>,
    },
    
    /// Run eslint on file set
    #[command(alias = "es")]
    Eslint {
        /// File command (added, modified, etc.)
        file_command: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Added => list_added_files(),
        Commands::Modified => list_modified_files(),
        Commands::Unstaged => list_unstaged_files(),
        Commands::BranchFiles { branch } => list_branch_files(&branch),
        Commands::Group { oneline } => group_files(oneline),
        Commands::Diff { file_command } => diff_files(file_command.as_deref()),
        Commands::DiffStaged => diff_staged(),
        Commands::DiffFile { file } => diff_file(&file),
        Commands::Run { tool, file_command } => run_tool(&tool, file_command.as_deref()),
        Commands::Prettier { file_command } => run_tool("prettier", file_command.as_deref()),
        Commands::Format { file_command } => run_tool("prettier", file_command.as_deref()),
        Commands::Lint { file_command } => run_tool("eslint", file_command.as_deref()),
        Commands::Rubocop { file_command } => run_tool("rubocop", file_command.as_deref()),
        Commands::Eslint { file_command } => run_tool("eslint", file_command.as_deref()),
    }
}

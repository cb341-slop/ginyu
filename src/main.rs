use clap::{Parser, Subcommand};
use anyhow::Result;

mod git;
mod commands;

use commands::*;

#[derive(Debug, Clone)]
pub struct OutputModifiers {
    pub group: bool,
    pub oneline: bool,
    pub count: bool,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Group output by file type
    #[arg(short = 'g', long = "group", global = true)]
    group: bool,
    
    /// Show oneline/condensed output
    #[arg(short = '1', long = "oneline", global = true)]
    oneline: bool,
    
    /// Show only count of files
    #[arg(short = 'c', long = "count", global = true)]
    count: bool,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show added but not staged files
    #[command(visible_alias = "a")]
    Added,
    
    /// Show modified but not staged files
    #[command(visible_alias = "m")]
    Modified,
    
    /// Show all unstaged files
    #[command(visible_alias = "u")]
    Unstaged,
    
    /// Show files different from branch
    #[command(visible_alias = "bf")]
    BranchFiles {
        /// Target branch to compare against
        #[arg(default_value = "main")]
        branch: String,
    },
    
    /// Group files by type
    #[command(visible_alias = "g")]
    Group {
        /// Show condensed oneline format
        #[arg(long)]
        oneline: bool,
    },
    
    /// Show diff for file set
    #[command(visible_alias = "d")]
    Diff {
        /// File command to diff (added, modified, etc.)
        file_command: Option<String>,
    },
    
    /// Show diff of staged files
    #[command(visible_alias = "ds")]
    DiffStaged,
    
    /// Show diff for specific file
    #[command(visible_alias = "df")]
    DiffFile {
        /// File path to diff
        file: String,
    },
    
    /// Run tool on file set
    #[command(visible_alias = "r")]
    Run {
        /// Tool to run
        tool: String,
        /// File command (added, modified, etc.)
        file_command: Option<String>,
    },
    
    /// Run prettier on file set
    #[command(visible_alias = "p")]
    Prettier {
        /// File command (added, modified, etc.)
        file_command: Option<String>,
    },
    
    /// Run formatter on file set
    #[command(visible_alias = "f")]
    Format {
        /// File command (added, modified, etc.)
        file_command: Option<String>,
    },
    
    /// Run linter on file set
    #[command(visible_alias = "l")]
    Lint {
        /// File command (added, modified, etc.)
        file_command: Option<String>,
    },
    
    /// Run rubocop on file set
    #[command(visible_alias = "rb")]
    Rubocop {
        /// File command (added, modified, etc.)
        file_command: Option<String>,
    },
    
    /// Run eslint on file set
    #[command(visible_alias = "es")]
    Eslint {
        /// File command (added, modified, etc.)
        file_command: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let modifiers = OutputModifiers {
        group: cli.group,
        oneline: cli.oneline,
        count: cli.count,
    };
    
    match cli.command {
        Commands::Added => list_files("added", &modifiers),
        Commands::Modified => list_files("modified", &modifiers),
        Commands::Unstaged => list_files("unstaged", &modifiers),
        Commands::BranchFiles { branch } => list_files(&format!("branch-files {}", branch), &modifiers),
        Commands::Group { oneline } => {
            // For backward compatibility, merge oneline flag with global modifiers
            let mut group_modifiers = modifiers;
            if oneline {
                group_modifiers.oneline = true;
            }
            group_modifiers.group = true; // Group command always groups
            list_files("unstaged", &group_modifiers)
        },
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

use anyhow::Result;
use colored::Colorize;
use std::process::Command;

use crate::git::{GitRepo, group_files_by_type};

pub fn list_added_files() -> Result<()> {
    let repo = GitRepo::open()?;
    let files = repo.get_added_files()?;
    
    for file in files {
        println!("{}", file);
    }
    
    Ok(())
}

pub fn list_modified_files() -> Result<()> {
    let repo = GitRepo::open()?;
    let files = repo.get_modified_files()?;
    
    for file in files {
        println!("{}", file);
    }
    
    Ok(())
}

pub fn list_unstaged_files() -> Result<()> {
    let repo = GitRepo::open()?;
    let files = repo.get_unstaged_files()?;
    
    for file in files {
        println!("{}", file);
    }
    
    Ok(())
}

pub fn list_branch_files(branch: &str) -> Result<()> {
    let repo = GitRepo::open()?;
    let files = repo.get_branch_files(branch)?;
    
    for file in files {
        println!("{}", file);
    }
    
    Ok(())
}

pub fn group_files(oneline: bool) -> Result<()> {
    let repo = GitRepo::open()?;
    let files = repo.get_unstaged_files()?;
    let groups = group_files_by_type(&files);
    
    if oneline {
        for (file_type, files) in groups {
            let file_names: Vec<String> = files.iter()
                .map(|f| std::path::Path::new(f).file_name().unwrap().to_string_lossy().to_string())
                .collect();
            println!("{} ({}): {}", 
                file_type.bright_blue(), 
                files.len(), 
                file_names.join(", ")
            );
        }
    } else {
        for (file_type, files) in groups {
            println!("{}:", file_type.bright_blue());
            for file in files {
                println!("  {}", file);
            }
            println!();
        }
    }
    
    Ok(())
}

pub fn diff_files(file_command: Option<&str>) -> Result<()> {
    let repo = GitRepo::open()?;
    
    let files = match file_command {
        Some("added") | Some("a") => repo.get_added_files()?,
        Some("modified") | Some("m") => repo.get_modified_files()?,
        Some("unstaged") | Some("u") => repo.get_unstaged_files()?,
        Some(cmd) if cmd.starts_with("branch-files") || cmd.starts_with("bf") => {
            let branch = cmd.split_whitespace().nth(1).unwrap_or("main");
            repo.get_branch_files(branch)?
        },
        None => repo.get_unstaged_files()?,
        _ => {
            eprintln!("Unknown file command: {:?}", file_command);
            return Ok(());
        }
    };
    
    let diff = repo.diff_files(&files)?;
    print!("{}", diff);
    
    Ok(())
}

pub fn diff_staged() -> Result<()> {
    let repo = GitRepo::open()?;
    let diff = repo.diff_staged()?;
    print!("{}", diff);
    Ok(())
}

pub fn diff_file(file: &str) -> Result<()> {
    let repo = GitRepo::open()?;
    let diff = repo.diff_file(file)?;
    print!("{}", diff);
    Ok(())
}

pub fn run_tool(tool: &str, file_command: Option<&str>) -> Result<()> {
    let repo = GitRepo::open()?;
    
    let files = match file_command {
        Some("added") | Some("a") => repo.get_added_files()?,
        Some("modified") | Some("m") => repo.get_modified_files()?,
        Some("unstaged") | Some("u") => repo.get_unstaged_files()?,
        Some(cmd) if cmd.starts_with("branch-files") || cmd.starts_with("bf") => {
            let branch = cmd.split_whitespace().nth(1).unwrap_or("main");
            repo.get_branch_files(branch)?
        },
        None => repo.get_unstaged_files()?,
        _ => {
            eprintln!("Unknown file command: {:?}", file_command);
            return Ok(());
        }
    };
    
    if files.is_empty() {
        println!("No files to process");
        return Ok(());
    }
    
    println!("Running {} on {} files...", tool.bright_green(), files.len());
    for file in &files {
        println!("  {}", file);
    }
    println!();
    
    let mut cmd = Command::new(tool);
    cmd.args(&files);
    
    let status = cmd.status()?;
    
    if !status.success() {
        eprintln!("{} failed with exit code: {:?}", tool.red(), status.code());
        std::process::exit(status.code().unwrap_or(1));
    }
    
    Ok(())
}

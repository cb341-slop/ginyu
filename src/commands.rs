use anyhow::Result;
use colored::Colorize;
use std::process::Command;

use crate::git::{GitRepo, group_files_by_type};
use crate::OutputModifiers;

pub fn list_files(file_command: &str, modifiers: &OutputModifiers) -> Result<()> {
    let repo = GitRepo::open()?;
    
    let files = get_files_for_command(&repo, file_command)?;
    
    if modifiers.count {
        if modifiers.group {
            let groups = group_files_by_type(&files);
            let mut total = 0;
            for (file_type, files) in &groups {
                println!("{}: {}", file_type, files.len());
                total += files.len();
            }
            println!("Total: {}", total);
        } else {
            println!("{}", files.len());
        }
    } else if modifiers.group {
        let groups = group_files_by_type(&files);
        
        if modifiers.oneline {
            let group_strs: Vec<String> = groups.iter()
                .map(|(file_type, files)| {
                    let file_names: Vec<String> = files.iter()
                        .map(|f| std::path::Path::new(f).file_name().unwrap().to_string_lossy().to_string())
                        .collect();
                    format!("{} ({}): {}", 
                        file_type.bright_blue(), 
                        files.len(), 
                        file_names.join(", ")
                    )
                })
                .collect();
            println!("{}", group_strs.join(" | "));
        } else {
            for (file_type, files) in groups {
                println!("{}:", file_type.bright_blue());
                for file in files {
                    println!("  {}", file);
                }
                println!();
            }
        }
    } else if modifiers.oneline {
        let file_names: Vec<String> = files.iter()
            .map(|f| std::path::Path::new(f).file_name().unwrap().to_string_lossy().to_string())
            .collect();
        println!("{} ({} files)", file_names.join(" "), files.len());
    } else {
        for file in files {
            println!("{}", file);
        }
    }
    
    Ok(())
}

fn get_files_for_command(repo: &GitRepo, file_command: &str) -> Result<Vec<String>> {
    let parts: Vec<&str> = file_command.split_whitespace().collect();
    
    match parts.get(0) {
        Some(&"added") => repo.get_added_files(),
        Some(&"modified") => repo.get_modified_files(),
        Some(&"unstaged") => repo.get_unstaged_files(),
        Some(&"branch-files") => {
            let branch = parts.get(1).unwrap_or(&"main");
            repo.get_branch_files(branch)
        },
        _ => {
            eprintln!("Unknown file command: {}", file_command);
            Ok(vec![])
        }
    }
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

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
            // Sort groups by file type for deterministic output
            let mut sorted_groups: Vec<(String, Vec<String>)> = groups.into_iter().collect();
            sorted_groups.sort_by(|a, b| a.0.cmp(&b.0));
            
            for (file_type, files) in &sorted_groups {
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
            // Sort groups by file type for deterministic output
            let mut sorted_groups: Vec<(String, Vec<String>)> = groups.into_iter().collect();
            sorted_groups.sort_by(|a, b| a.0.cmp(&b.0));
            
            for (file_type, files) in sorted_groups {
                let file_names: Vec<String> = files.iter()
                    .map(|f| std::path::Path::new(f).file_name().unwrap().to_string_lossy().to_string())
                    .collect();
                println!("{} ({}): {}", 
                    file_type.bright_blue(), 
                    files.len(),
                    file_names.join(" ")
                );
            }
        } else {
            // Sort groups by file type for deterministic output
            let mut sorted_groups: Vec<(String, Vec<String>)> = groups.into_iter().collect();
            sorted_groups.sort_by(|a, b| a.0.cmp(&b.0));
            
            for (file_type, files) in sorted_groups {
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
    
    let mut files = match file_command {
        Some("added") | Some("a") => repo.get_added_files()?,
        Some("modified") | Some("m") => repo.get_modified_files()?,
        Some("unstaged") | Some("u") => repo.get_unstaged_files()?,
        Some(cmd) if cmd.starts_with("branch-files") || cmd.starts_with("bf") => {
            let branch = cmd.split_whitespace().nth(1).unwrap_or("main");
            repo.get_branch_files(branch)?
        },
        None => repo.get_unstaged_files()?,
        _ => {
            // Check if it's a file type filter
            if let Some(filtered) = filter_files_by_type(&repo.get_unstaged_files()?, file_command.unwrap()) {
                filtered
            } else {
                eprintln!("Unknown file command: {:?}", file_command);
                return Ok(());
            }
        }
    };
    
    // Auto-filter for specific tools
    files = auto_filter_for_tool(tool, files);
    
    if files.is_empty() {
        println!("No {} files to process", 
            if file_command.is_some() { file_command.unwrap() } else { "applicable" });
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

fn filter_files_by_type(files: &[String], file_type: &str) -> Option<Vec<String>> {
    let groups = group_files_by_type(files);
    
    // Try exact match first
    if let Some(files) = groups.get(file_type) {
        return Some(files.clone());
    }
    
    // Try case-insensitive match
    for (group_type, group_files) in groups {
        if group_type.to_lowercase() == file_type.to_lowercase() {
            return Some(group_files);
        }
    }
    
    None
}

fn auto_filter_for_tool(tool: &str, files: Vec<String>) -> Vec<String> {
    match tool {
        "erb_lint" => {
            files.into_iter()
                .filter(|f| f.ends_with(".erb"))
                .collect()
        },
        "haml_lint" => {
            files.into_iter()
                .filter(|f| f.ends_with(".haml"))
                .collect()
        },
        "rubocop" => {
            files.into_iter()
                .filter(|f| f.ends_with(".rb") || f.ends_with(".rake") || f.ends_with("Rakefile") || f.ends_with("Gemfile"))
                .collect()
        },
        "eslint" => {
            files.into_iter()
                .filter(|f| f.ends_with(".js") || f.ends_with(".jsx") || f.ends_with(".ts") || f.ends_with(".tsx"))
                .collect()
        },
        "prettier" => {
            files.into_iter()
                .filter(|f| {
                    f.ends_with(".js") || f.ends_with(".jsx") || f.ends_with(".ts") || f.ends_with(".tsx") ||
                    f.ends_with(".json") || f.ends_with(".css") || f.ends_with(".scss") || f.ends_with(".md")
                })
                .collect()
        },
        _ => files, // No auto-filtering for unknown tools
    }
}

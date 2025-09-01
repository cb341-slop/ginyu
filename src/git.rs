use git2::{Repository, Status, StatusOptions};
use anyhow::{Result, Context};
use std::path::Path;

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    pub fn open() -> Result<Self> {
        let repo = Repository::discover(".")
            .context("Not in a git repository")?;
        Ok(GitRepo { repo })
    }
    
    pub fn get_added_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        let statuses = self.repo.statuses(Some(
            StatusOptions::new()
                .include_untracked(true)
                .recurse_untracked_dirs(true)
        ))?;
        
        for entry in statuses.iter() {
            if entry.status().contains(Status::WT_NEW) {
                if let Some(path) = entry.path() {
                    // Skip directories, only include files
                    let full_path = self.repo.workdir().unwrap().join(path);
                    if full_path.is_file() {
                        files.push(path.to_string());
                    }
                }
            }
        }
        
        Ok(files)
    }
    
    pub fn get_modified_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        let statuses = self.repo.statuses(None)?;
        
        for entry in statuses.iter() {
            if entry.status().contains(Status::WT_MODIFIED) {
                if let Some(path) = entry.path() {
                    // Skip directories, only include files
                    let full_path = self.repo.workdir().unwrap().join(path);
                    if full_path.is_file() {
                        files.push(path.to_string());
                    }
                }
            }
        }
        
        Ok(files)
    }
    
    pub fn get_unstaged_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        let statuses = self.repo.statuses(Some(
            StatusOptions::new()
                .include_untracked(true)
                .recurse_untracked_dirs(true)
        ))?;
        
        for entry in statuses.iter() {
            let status = entry.status();
            if status.contains(Status::WT_NEW) || status.contains(Status::WT_MODIFIED) {
                if let Some(path) = entry.path() {
                    // Skip directories, only include files
                    let full_path = self.repo.workdir().unwrap().join(path);
                    if full_path.is_file() {
                        files.push(path.to_string());
                    }
                }
            }
        }
        
        Ok(files)
    }
    
    pub fn get_branch_files(&self, branch: &str) -> Result<Vec<String>> {
        use std::process::Command;
        
        let output = Command::new("git")
            .args(["diff", "--name-only", &format!("{}...HEAD", branch)])
            .output()
            .context("Failed to run git diff")?;
            
        if !output.status.success() {
            anyhow::bail!("Git diff failed: {}", String::from_utf8_lossy(&output.stderr));
        }
        
        let files = String::from_utf8(output.stdout)?
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect();
            
        Ok(files)
    }
    
    pub fn diff_files(&self, files: &[String]) -> Result<String> {
        use std::process::Command;
        
        if files.is_empty() {
            let output = Command::new("git")
                .args(["diff"])
                .output()
                .context("Failed to run git diff")?;
            return Ok(String::from_utf8(output.stdout)?);
        }
        
        let mut args = vec!["diff"];
        args.extend(files.iter().map(|s| s.as_str()));
        
        let output = Command::new("git")
            .args(&args)
            .output()
            .context("Failed to run git diff")?;
            
        Ok(String::from_utf8(output.stdout)?)
    }
    
    pub fn diff_staged(&self) -> Result<String> {
        use std::process::Command;
        
        let output = Command::new("git")
            .args(["diff", "--cached"])
            .output()
            .context("Failed to run git diff --cached")?;
            
        Ok(String::from_utf8(output.stdout)?)
    }
    
    pub fn diff_file(&self, file: &str) -> Result<String> {
        use std::process::Command;
        
        let output = Command::new("git")
            .args(["diff", file])
            .output()
            .context("Failed to run git diff")?;
            
        Ok(String::from_utf8(output.stdout)?)
    }
}

pub fn group_files_by_type(files: &[String]) -> std::collections::HashMap<String, Vec<String>> {
    let mut groups = std::collections::HashMap::new();
    
    for file in files {
        let extension = Path::new(file)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("no extension");
            
        let file_type = match extension {
            "rs" => "Rust",
            "js" | "jsx" => "JavaScript", 
            "ts" | "tsx" => "TypeScript",
            "py" => "Python",
            "rb" => "Ruby",
            "erb" => "ERB",
            "haml" => "Haml",
            "slim" => "Slim",
            "go" => "Go",
            "md" => "Markdown",
            "json" => "JSON",
            "toml" => "TOML",
            "yaml" | "yml" => "YAML",
            "html" => "HTML",
            "css" => "CSS",
            "scss" | "sass" => "Sass/SCSS",
            "less" => "Less",
            "vue" => "Vue",
            "svelte" => "Svelte",
            "php" => "PHP",
            "java" => "Java",
            "kt" => "Kotlin",
            "swift" => "Swift",
            "c" => "C",
            "cpp" | "cc" | "cxx" => "C++",
            "h" | "hpp" => "C/C++ Header",
            "cs" => "C#",
            "sh" | "bash" | "zsh" => "Shell",
            "dockerfile" => "Dockerfile",
            "sql" => "SQL",
            "env" => "Environment",
            "lock" => "Lock File",
            "gitignore" => "Git Ignore",
            "no extension" => "No Extension",
            _ => extension,
        };
        
        groups.entry(file_type.to_string())
            .or_insert_with(Vec::new)
            .push(file.clone());
    }
    
    groups
}

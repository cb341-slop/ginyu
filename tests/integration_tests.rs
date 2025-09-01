use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::process::Command as StdCommand;
use tempfile::TempDir;

struct TestRepo {
    _temp_dir: TempDir,
    repo_path: std::path::PathBuf,
}

impl TestRepo {
    fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().to_path_buf();
        
        // Initialize git repo
        StdCommand::new("git")
            .arg("init")
            .current_dir(&repo_path)
            .output()
            .unwrap();
            
        // Configure git user
        StdCommand::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo_path)
            .output()
            .unwrap();
            
        StdCommand::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo_path)
            .output()
            .unwrap();
        
        Self {
            _temp_dir: temp_dir,
            repo_path,
        }
    }
    
    fn create_file(&self, path: &str, content: &str) {
        let file_path = self.repo_path.join(path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(file_path, content).unwrap();
    }
    
    fn modify_file(&self, path: &str, content: &str) {
        let file_path = self.repo_path.join(path);
        fs::write(file_path, content).unwrap();
    }
    
    fn add_file(&self, path: &str) {
        StdCommand::new("git")
            .args(["add", path])
            .current_dir(&self.repo_path)
            .output()
            .unwrap();
    }
    
    fn commit(&self, message: &str) {
        StdCommand::new("git")
            .args(["commit", "-m", message])
            .current_dir(&self.repo_path)
            .output()
            .unwrap();
    }
    
    fn create_branch(&self, name: &str) {
        StdCommand::new("git")
            .args(["checkout", "-b", name])
            .current_dir(&self.repo_path)
            .output()
            .unwrap();
    }
    
    fn checkout(&self, branch: &str) {
        StdCommand::new("git")
            .args(["checkout", branch])
            .current_dir(&self.repo_path)
            .output()
            .unwrap();
    }
}

#[test]
fn test_added_files() {
    let repo = TestRepo::new();
    
    // Create some new files
    repo.create_file("src/main.rs", "fn main() {}");
    repo.create_file("README.md", "# Test");
    
    let mut cmd = Command::cargo_bin("ginyu").unwrap();
    cmd.arg("added")
        .current_dir(&repo.repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("src/main.rs"))
        .stdout(predicate::str::contains("README.md"));
}

#[test]
fn test_modified_files() {
    let repo = TestRepo::new();
    
    // Create and commit a file
    repo.create_file("src/lib.rs", "// original content");
    repo.add_file("src/lib.rs");
    repo.commit("Initial commit");
    
    // Modify the file
    repo.modify_file("src/lib.rs", "// modified content");
    
    let mut cmd = Command::cargo_bin("ginyu").unwrap();
    cmd.arg("modified")
        .current_dir(&repo.repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("src/lib.rs"));
}

#[test]
fn test_unstaged_files() {
    let repo = TestRepo::new();
    
    // Create a new file and modify an existing one
    repo.create_file("new_file.txt", "new");
    repo.create_file("existing.txt", "original");
    repo.add_file("existing.txt");
    repo.commit("Add existing file");
    repo.modify_file("existing.txt", "modified");
    
    let mut cmd = Command::cargo_bin("ginyu").unwrap();
    cmd.arg("unstaged")
        .current_dir(&repo.repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("new_file.txt"))
        .stdout(predicate::str::contains("existing.txt"));
}

#[test]
fn test_branch_files() {
    let repo = TestRepo::new();
    
    // Create initial commit on main
    repo.create_file("main_file.txt", "main content");
    repo.add_file("main_file.txt");
    repo.commit("Initial commit");
    
    // Create feature branch and add files
    repo.create_branch("feature");
    repo.create_file("feature_file.rs", "fn feature() {}");
    repo.create_file("another.js", "console.log('hi')");
    repo.add_file("feature_file.rs");
    repo.add_file("another.js");
    repo.commit("Add feature files");
    
    let mut cmd = Command::cargo_bin("ginyu").unwrap();
    cmd.args(["branch-files", "main"])
        .current_dir(&repo.repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("feature_file.rs"))
        .stdout(predicate::str::contains("another.js"));
}

#[test]
fn test_group_files() {
    let repo = TestRepo::new();
    
    // Create files of different types
    repo.create_file("src/main.rs", "fn main() {}");
    repo.create_file("app.js", "console.log('hi')");
    repo.create_file("README.md", "# Test");
    repo.create_file("package.json", "{}");
    
    let mut cmd = Command::cargo_bin("ginyu").unwrap();
    cmd.arg("group")
        .current_dir(&repo.repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust:"))
        .stdout(predicate::str::contains("JavaScript:"))
        .stdout(predicate::str::contains("Markdown:"))
        .stdout(predicate::str::contains("JSON:"));
}

#[test]
fn test_group_files_oneline() {
    let repo = TestRepo::new();
    
    repo.create_file("src/main.rs", "fn main() {}");
    repo.create_file("src/lib.rs", "// lib");
    repo.create_file("app.js", "console.log('hi')");
    
    let mut cmd = Command::cargo_bin("ginyu").unwrap();
    cmd.args(["group", "--oneline"])
        .current_dir(&repo.repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust (2):"))
        .stdout(predicate::str::contains("JavaScript (1):"));
}

#[test]
fn test_diff_modified() {
    let repo = TestRepo::new();
    
    // Create and commit a file
    repo.create_file("test.txt", "original content\n");
    repo.add_file("test.txt");
    repo.commit("Initial commit");
    
    // Modify the file
    repo.modify_file("test.txt", "modified content\n");
    
    let mut cmd = Command::cargo_bin("ginyu").unwrap();
    cmd.args(["diff", "modified"])
        .current_dir(&repo.repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("diff --git"))
        .stdout(predicate::str::contains("test.txt"));
}

#[test]
fn test_aliases() {
    let repo = TestRepo::new();
    repo.create_file("test.rs", "fn test() {}");
    
    // Test that aliases work
    let mut cmd = Command::cargo_bin("ginyu").unwrap();
    cmd.arg("a")  // alias for "added"
        .current_dir(&repo.repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("test.rs"));
        
    let mut cmd = Command::cargo_bin("ginyu").unwrap();
    cmd.arg("g")  // alias for "group"
        .current_dir(&repo.repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust:"));
}

#[test]
fn test_run_tool_dry_run() {
    let repo = TestRepo::new();
    repo.create_file("test.js", "console.log('test')");
    
    // Test that the tool runner shows what it would do
    // Using 'echo' as a safe tool that exists on all systems
    let mut cmd = Command::cargo_bin("ginyu").unwrap();
    cmd.args(["run", "echo", "added"])
        .current_dir(&repo.repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Running echo on 1 files"))
        .stdout(predicate::str::contains("test.js"));
}

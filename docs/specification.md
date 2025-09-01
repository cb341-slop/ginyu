# Ginyu - Git CLI Wrapper Specification

A lightweight git CLI wrapper that provides intuitive aliases for common git operations with enhanced file listing capabilities.

## Why Ginyu?

### Vanilla Git Pain Points vs Ginyu Solutions

**Problem: Complex commands for simple tasks**
```bash
# Vanilla git - multiple commands to format only modified files
git status --porcelain | grep "^ M" | cut -c4- | xargs prettier --write

# Ginyu - one command
ginyu prettier modified
```

**Problem: No easy way to run tools on specific file sets**
```bash
# Vanilla git - manual file selection for linting
git diff --name-only | grep "\.rb$" | xargs rubocop

# Ginyu - semantic and simple
ginyu rubocop modified
```

**Problem: Verbose diffs when you only care about specific files**
```bash
# Vanilla git - shows everything, hard to focus
git diff

# Ginyu - targeted and clean
ginyu diff modified
```

**Problem: Branch comparisons are cumbersome**
```bash
# Vanilla git - multiple steps to see and format feature branch files
git diff --name-only main...HEAD | xargs prettier --write

# Ginyu - intuitive workflow
ginyu prettier branch-files main
```

**Problem: File status requires parsing git output**
```bash
# Vanilla git - cryptic status codes
git status --porcelain
# Output: ?? src/new.ts
#          M src/old.ts

# Ginyu - human readable
ginyu added     # just shows: src/new.ts
ginyu modified  # just shows: src/old.ts
```

### Key Benefits

1. **Workflow Efficiency**: Combines file discovery + tool execution in one command
2. **Cognitive Load Reduction**: Semantic commands instead of git plumbing
3. **Error Prevention**: No more accidentally running tools on wrong files
4. **Team Consistency**: Standardized commands across projects and developers
5. **Development Speed**: 5-10x faster common operations

## Core Features

### File Status Operations

#### Added but not staged files
```bash
ginyu added
# or
ginyu a
```

Lists all files that have been added to the repository but are not yet staged for commit.

**Example:**
```bash
$ ginyu added
src/components/Button.tsx
src/utils/validation.ts
docs/setup.md
```

#### Modified but not staged files
```bash
ginyu modified
# or
ginyu m
```

Lists all files that have been modified but are not yet staged for commit.

**Example:**
```bash
$ ginyu modified
src/App.tsx
package.json
README.md
```

#### All unstaged files
```bash
ginyu unstaged
# or
ginyu u
```

Lists all files that are either added or modified but not staged.

**Example:**
```bash
$ ginyu unstaged
src/components/Button.tsx  (added)
src/utils/validation.ts    (added)
src/App.tsx               (modified)
package.json              (modified)
```

### Branch Operations

#### List all files in feature branch
```bash
ginyu branch-files [branch-name]
# or
ginyu bf [branch-name]
```

Lists all files that differ between the current branch and the specified branch (defaults to main/master).

**Example:**
```bash
$ ginyu branch-files main
src/components/NewFeature.tsx
src/hooks/useFeature.ts
tests/feature.test.ts
docs/feature.md
```

### File Grouping

#### Group files by type
```bash
ginyu group [--oneline]
# or
ginyu g [--oneline]
```

Groups files by their extension/type. The `--oneline` flag shows a condensed view.

**Example (default view):**
```bash
$ ginyu group
TypeScript:
  src/App.tsx
  src/components/Button.tsx
  src/utils/validation.ts

Markdown:
  README.md
  docs/setup.md

JSON:
  package.json
  tsconfig.json
```

**Example (oneline view):**
```bash
$ ginyu group --oneline
TypeScript (3): App.tsx, Button.tsx, validation.ts
Markdown (2): README.md, setup.md
JSON (2): package.json, tsconfig.json
```

### Git Diff Operations

#### Diff specific file sets
```bash
ginyu diff [file-command]
# or
ginyu d [file-command]
```

Show git diff for specific file sets. Defaults to all unstaged files if no file-command specified.

**Examples:**
```bash
# Diff all modified files
$ ginyu diff modified
$ ginyu d m                 # same as above

# Diff files added but not staged
$ ginyu diff added
$ ginyu d a                 # same as above

# Diff all files different from main branch
$ ginyu diff branch-files main
$ ginyu d bf main           # same as above

# Diff all unstaged files (default)
$ ginyu diff
```

#### Staged diff
```bash
ginyu diff-staged
# or
ginyu ds
```

Show diff of staged files (equivalent to `git diff --cached`).

**Example:**
```bash
$ ginyu diff-staged
diff --git a/src/App.tsx b/src/App.tsx
index 1234567..abcdefg 100644
--- a/src/App.tsx
+++ b/src/App.tsx
@@ -10,6 +10,7 @@ function App() {
   return (
     <div className="App">
       <header className="App-header">
+        <h1>Welcome to Ginyu</h1>
         <img src={logo} className="App-logo" alt="logo" />
```

#### Quick file-specific diffs
```bash
ginyu diff-file <filename>
# or
ginyu df <filename>
```

Show diff for a specific file.

**Example:**
```bash
$ ginyu diff-file src/App.tsx
$ ginyu df src/App.tsx      # same as above
```

### Tool Execution

#### Run tools on file sets
```bash
ginyu run <tool> [file-command]
# or
ginyu r <tool> [file-command]
```

Execute linters, formatters, or other tools on specific file sets. If no file-command is specified, runs on all unstaged files.

**Examples:**
```bash
# Run prettier on all modified files
$ ginyu run prettier modified
Running prettier on modified files...
src/App.tsx
package.json

# Run rubocop on added files
$ ginyu run rubocop added
Running rubocop on added files...
lib/user.rb
spec/user_spec.rb

# Run eslint on all files in feature branch
$ ginyu run eslint branch-files main
Running eslint on files different from main...
src/components/NewFeature.tsx
src/hooks/useFeature.ts

# Run prettier on all unstaged files (default)
$ ginyu run prettier
Running prettier on unstaged files...
src/App.tsx
package.json
lib/user.rb
```

#### Tool shortcuts
Common tools can be executed with dedicated shortcuts:

```bash
# Prettier shortcuts
ginyu prettier [file-command]    # or ginyu p [file-command]
ginyu format [file-command]      # or ginyu f [file-command]

# Linter shortcuts  
ginyu lint [file-command]        # or ginyu l [file-command]
ginyu rubocop [file-command]     # or ginyu rb [file-command]
ginyu eslint [file-command]      # or ginyu es [file-command]
```

**Examples:**
```bash
$ ginyu prettier modified
$ ginyu p m                 # same as above
$ ginyu lint added
$ ginyu l a                 # same as above
$ ginyu rubocop branch-files
$ ginyu rb bf               # same as above
```

## Command Reference

| Command | Alias | Description |
|---------|-------|-------------|
| `ginyu added` | `ginyu a` | Show added but not staged files |
| `ginyu modified` | `ginyu m` | Show modified but not staged files |
| `ginyu unstaged` | `ginyu u` | Show all unstaged files |
| `ginyu branch-files [branch]` | `ginyu bf [branch]` | Show files different from branch |
| `ginyu group [--oneline]` | `ginyu g [--oneline]` | Group files by type |
| `ginyu diff [files]` | `ginyu d [files]` | Show diff for file set |
| `ginyu diff-staged` | `ginyu ds` | Show diff of staged files |
| `ginyu diff-file <file>` | `ginyu df <file>` | Show diff for specific file |
| `ginyu run <tool> [files]` | `ginyu r <tool> [files]` | Run tool on specified file set |
| `ginyu prettier [files]` | `ginyu p [files]` | Run prettier on file set |
| `ginyu format [files]` | `ginyu f [files]` | Run formatter on file set |
| `ginyu lint [files]` | `ginyu l [files]` | Run linter on file set |
| `ginyu rubocop [files]` | `ginyu rb [files]` | Run rubocop on file set |
| `ginyu eslint [files]` | `ginyu es [files]` | Run eslint on file set |

## Real-World Workflow Examples

### Daily Development Scenarios

**Scenario: Code review prep**
```bash
# Vanilla git workflow
git status                           # see what's changed
git diff src/components/Button.tsx   # review specific file
prettier --write src/components/Button.tsx src/utils/validation.ts  # format manually
git add .                           # stage everything

# Ginyu workflow  
ginyu modified                      # clean list of changed files
ginyu diff modified                 # review all changes at once
ginyu prettier modified             # format only what changed
git add .                          # stage everything
```

**Scenario: Feature branch cleanup**
```bash
# Vanilla git workflow
git diff --name-only main...HEAD                    # see feature files
git diff --name-only main...HEAD | xargs eslint     # lint manually
git diff --name-only main...HEAD | grep "\.tsx$" | xargs prettier --write

# Ginyu workflow
ginyu branch-files main            # see feature files
ginyu lint branch-files main       # lint feature branch
ginyu prettier branch-files main   # format feature branch
```

**Scenario: Debugging failing tests**
```bash
# Vanilla git workflow
git status --porcelain | grep "^ M" | cut -c4- | grep "\.test\." | xargs jest

# Ginyu workflow
ginyu modified | grep test         # find modified test files
ginyu run jest modified            # run tests on modified files only
```

## Implementation Notes

- All commands respect `.gitignore` patterns
- File paths are shown relative to repository root
- Exit codes follow git conventions (0 for success, non-zero for errors)
- Supports both long and short command aliases for efficiency
- Colors and formatting follow git's default styling when possible
- Tool execution preserves original tool exit codes
- Tools are executed with their default configurations (respects local config files)
- File arguments can be chained with any file listing command
- Performance: File operations are cached during command execution
- Compatibility: Works with any git repository, no additional setup required

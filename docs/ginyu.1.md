# GINYU(1)

## NAME

ginyu - A lightweight git CLI wrapper with intuitive aliases

## SYNOPSIS

**ginyu** \[*COMMAND*\] \[*OPTIONS*\] \[*ARGS*\]

## DESCRIPTION

Ginyu is a lightweight git CLI wrapper that provides intuitive aliases for common git operations with enhanced file listing capabilities. It simplifies workflows by combining file discovery and tool execution in single commands.

## COMMANDS

### File Operations

**ginyu added**, **ginyu a**
: Show files that are added but not yet staged

**ginyu modified**, **ginyu m**
: Show files that are modified but not yet staged

**ginyu unstaged**, **ginyu u**
: Show all files that are either added or modified but not staged

**ginyu branch-files** \[*branch*\], **ginyu bf** \[*branch*\]
: Show files that differ between current branch and target branch (default: main)

**ginyu group** \[**--oneline**\], **ginyu g** \[**--oneline**\]
: Group files by type. Use --oneline for condensed view

### Diff Operations

**ginyu diff** \[*file-command*\], **ginyu d** \[*file-command*\]
: Show git diff for specific file sets (added, modified, etc.)

**ginyu diff-staged**, **ginyu ds**
: Show diff of staged files (equivalent to git diff --cached)

**ginyu diff-file** *file*, **ginyu df** *file*
: Show diff for a specific file

### Tool Execution

**ginyu run** *tool* \[*file-command*\], **ginyu r** *tool* \[*file-command*\]
: Run specified tool on file sets

**ginyu prettier** \[*file-command*\], **ginyu p** \[*file-command*\]
: Run prettier on file sets

**ginyu format** \[*file-command*\], **ginyu f** \[*file-command*\]
: Run formatter on file sets

**ginyu lint** \[*file-command*\], **ginyu l** \[*file-command*\]
: Run linter on file sets

**ginyu rubocop** \[*file-command*\], **ginyu rb** \[*file-command*\]
: Run rubocop on file sets

**ginyu eslint** \[*file-command*\], **ginyu es** \[*file-command*\]
: Run eslint on file sets

## EXAMPLES

List all modified files:
```
ginyu modified
```

Format only modified files:
```
ginyu prettier modified
```

Show diff of files different from main branch:
```
ginyu diff branch-files main
```

Group unstaged files by type:
```
ginyu group --oneline
```

Run rubocop on added Ruby files:
```
ginyu rubocop added
```

## EXIT STATUS

**0**
: Success

**1**
: General error

**2**
: Git repository not found

## ENVIRONMENT

Ginyu respects all git environment variables and configuration files.

## FILES

All commands respect *.gitignore* patterns and operate relative to the git repository root.

## AUTHOR

Written by the Ginyu contributors.

## REPORTING BUGS

Report bugs at: <https://github.com/CuddlyBunion341/ginyu/issues>

## SEE ALSO

**git**(1), **prettier**(1), **eslint**(1), **rubocop**(1)

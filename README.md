# RGCH: Rust implementation of Git Commit Handler
[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)][license]

[license]: https://github.com/Scstechr/rgch/blob/master/LICENSE

A tool to handle git related commands such as: `git init`, `git commit`, `git diff`, `git add`, `git push`.  
This is a Rust implementation of [gch](https://github.com/Scstechr/gch), which we aim to replace it.

## Overview
This tool makes it easier to execute certain `git` commands from terminal.  
Also, this `rgch` aims for beginners of `git` by showing actual commands executed in specific color.

## How to Use

### Show help

```bash
$ rgch --help
RGCH: v0.1.15 (a9d52ebe 2020-01-03)

Usage:

  rgch [FLAGS] [OPTIONS] <INPUT>

Options:

           Name            Save Type   Explainations                 
  CREATE │ --clone              STRING Clone remote repository.
         │ -i, --init           FLAG   Initialize repository.
  BRANCH │ -b, --branch      ⎘  STRING Specify branch name.
  CHANGE │ -l, --log            FLAG   Display log.
         │ -a, --add         ⎘  PATH   Specify path to add.
         │ -c, --commit         FLAG   Commit.
  REMOTE │ -r, --remote         STRING Specify remote repository.
         │ --pull               FLAG   Pull (fetch and rebase).
         │ -p, --push           FLAG   Push.
  EXTRAS │ -g, --gitdir      ⎘  PATH   Specify path of `.git`.
         │ -f, --force          FLAG   `-f/--force` option to `add`.
         │ -v, --verbose     ⎘  FLAG   Verbose option.
         │ -s, --save           FLAG   Save current setting to TOML file.
         │ --version            FLAG   Display version and compiler info.
         │ -h, --help           FLAG   Show this message and exit.
```

### Simple command

```bash
$ rgch -c
```

or equivalently,

```bash
$ rgch --commit
```

This command executes `git status --short`, `git diff --stat`, `git add .` etc.  
(shown as a blue line while executed)  
Also, adds everything except configured in `.gitignore` or `gch -f` command.  

### Linked commands

Commands can be executed together in the manner below:

```bash
$ rgch -cp
```

This executes `git commit` and `git push`.

#### Further example

##### `gch -cp -r localhost`
`commit`, then `push` to the remote called `localhost`.
##### `gch -cp -b test -d`
Shows `diff` first, then `commit` and `push`.
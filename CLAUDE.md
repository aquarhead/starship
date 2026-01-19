# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Starship is a personalized fork of an ancient version of the original starship prompt. It's a minimalist, zsh-only shell prompt written in Rust with all dynamic config features removed. Designed to work with Fira Code or MonoLisa fonts.

## Build Commands

```bash
cargo build --release    # Build optimized binary
cargo install --path .   # Install locally
cargo test               # Run all tests
cargo test test_name     # Run specific test
RUST_LOG=debug cargo run -- prompt --status 0 --cmd-duration 1000  # Debug prompt generation
```

## Architecture

**Entry point:** `src/main.rs` - CLI dispatcher with two subcommands:
- `init` - Outputs shell initialization script
- `prompt` - Generates and prints the prompt

**Prompt generation:** `src/print.rs` - Orchestrates modules in fixed order:
1. directory, jj, git_state, git_status, git_track
2. rust, aws, plaio, plaio_db, kube
3. cmd_duration, line_break, prompt, jobs

**Context:** `src/context.rs` - Runtime context with VCS information, directory scanning with 30ms timeout, and path utilities.

**Shell integration:** `src/init/starship.zsh` - ZSH hooks (`precmd`/`preexec`) that capture exit status, command duration, and invoke the Rust binary.

## Key Data Structures

- **Segment** (`src/segment.rs`): Single styled text element
- **Context** (`src/context.rs`): Working directory, cmd_duration, jobs, status_code, cached git repo info

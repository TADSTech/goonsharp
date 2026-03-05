# GoonHub: Project Management

## Creating a Project

```bash
# New binary project
goonhub new my_app

# New library project
goonhub new my_lib --lib

# Initialize in current directory
goonhub init
```

## Project Structure

```
my_app/
├── Goon.toml
└── src/
    └── main.goons
```

## Goon.toml

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"
authors = ["Your Name"]
description = "A goon application"

[dependencies]
```

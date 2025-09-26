# RCIP Validator - Rust

High-performance RCIP format validator written in Rust.

## Installation

```bash
cargo install rcip-validator
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
rcip-validator = "1.0"
```

## Usage

### As a Library

```rust
use rcip_validator::RCIPValidator;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut validator = RCIPValidator::new("0.1");
    validator.init(None)?;
    
    // Validate a recipe
    let recipe = serde_json::from_str(recipe_json)?;
    let result = validator.validate_recipe(&recipe);
    
    if result.valid {
        println!("Recipe is valid!");
    } else {
        println!("Validation errors: {:?}", result.errors);
    }
    
    // Validate a file
    let file_result = validator.validate_file(Path::new("recipe.rcip"))?;
    
    // Validate a directory
    let results = validator.validate_directory(Path::new("./recipes"))?;
    
    Ok(())
}
```

### Command Line

```bash
# Validate a single file
rcip-validator recipe.rcip

# Validate a directory
rcip-validator ./recipes/

# Use custom schema version
rcip-validator --version 0.2 recipe.json

# Use custom schema file
rcip-validator --schema custom-schema.json recipe.rcip
```

## Performance

The Rust validator is optimized for performance and can validate thousands of recipes per second.

## Building from Source

```bash
git clone https://github.com/AlexeyKoz/rcip-format.git
cd rcip-format/validators/rust
cargo build --release
```

## API Reference

See main documentation for detailed API reference.
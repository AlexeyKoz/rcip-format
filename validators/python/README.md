# RCIP Validator - Python

## Installation

```bash
pip install rcip-validator
```

Or from source:

```bash
git clone https://github.com/AlexeyKoz/rcip-format.git
cd rcip-format/validators/python
pip install -e .
```

## Usage

### As a Library

```python
from rcip_validator import RCIPValidator

# Initialize validator
validator = RCIPValidator(schema_version="0.1")
validator.init()

# Validate a recipe dictionary
result = validator.validate_recipe(recipe_dict)
print("Valid!" if result.valid else "Invalid!")

# Validate a file
from pathlib import Path
result = validator.validate_file(Path("recipe.rcip"))

# Validate a directory
results = validator.validate_directory(Path("./recipes"))
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

## API Reference

See main documentation for detailed API reference.

---
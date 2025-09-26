# RCIP Validator - JavaScript

## Installation

```bash
npm install @rcip/validator
```

## Usage

### As a Library

```javascript
const RCIPValidator = require('@rcip/validator');

const validator = new RCIPValidator('0.1');
await validator.init();

// Validate a recipe object
const result = validator.validateRecipe(recipeObject);
console.log(result.valid ? 'Valid!' : 'Invalid!');

// Validate a file
const fileResult = await validator.validateFile('recipe.rcip');

// Validate a directory
const results = await validator.validateDirectory('./recipes');
```

### Command Line

```bash
# Validate a single file
node rcip-validator.js recipe.rcip

# Validate all recipes in a directory
node rcip-validator.js ./recipes/

# Use a specific schema version
node rcip-validator.js --version 0.2 recipe.json
```

## API

See main documentation for detailed API reference.

---
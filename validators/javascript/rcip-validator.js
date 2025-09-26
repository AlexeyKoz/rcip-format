/**
 * RCIP Format Validator for JavaScript/Node.js
 * Created by Alexey Kozlov
 * Version: 1.0.0
 * License: MIT
 */

const Ajv = require('ajv');
const addFormats = require('ajv-formats');
const fs = require('fs').promises;
const path = require('path');

/**
 * RCIP Validator Class
 * Validates recipes against the RCIP schema specification
 */
class RCIPValidator {
  constructor(schemaVersion = '0.1') {
    this.schemaVersion = schemaVersion;
    this.ajv = new Ajv({ allErrors: true, verbose: true });
    addFormats(this.ajv);
    this.schema = null;
    this.validate = null;
    this.stats = {
      validated: 0,
      passed: 0,
      failed: 0
    };
  }

  /**
   * Initialize validator with schema
   * @param {string} schemaPath - Path to JSON schema file
   */
  async init(schemaPath = null) {
    try {
      if (!schemaPath) {
        schemaPath = path.join(__dirname, '../../schemas', `rcip-v${this.schemaVersion}.json`);
      }

      const schemaContent = await fs.readFile(schemaPath, 'utf8');
      this.schema = JSON.parse(schemaContent);
      this.validate = this.ajv.compile(this.schema);

      console.log(`✅ RCIP Validator initialized with schema v${this.schemaVersion}`);
      return true;
    } catch (error) {
      console.error(`❌ Failed to initialize validator: ${error.message}`);
      throw error;
    }
  }

  /**
   * Validate a recipe object
   * @param {Object} recipe - Recipe object to validate
   * @returns {Object} Validation result
   */
  validateRecipe(recipe) {
    if (!this.validate) {
      throw new Error('Validator not initialized. Call init() first.');
    }

    const valid = this.validate(recipe);
    this.stats.validated++;

    if (valid) {
      this.stats.passed++;
    } else {
      this.stats.failed++;
    }

    return {
      valid,
      errors: valid ? null : this.validate.errors,
      warnings: this.checkWarnings(recipe),
      info: this.getRecipeInfo(recipe)
    };
  }

  /**
   * Validate a recipe file
   * @param {string} filePath - Path to recipe file
   * @returns {Object} Validation result
   */
  async validateFile(filePath) {
    try {
      const content = await fs.readFile(filePath, 'utf8');
      const recipe = JSON.parse(content);

      console.log(`\n📄 Validating: ${path.basename(filePath)}`);
      const result = this.validateRecipe(recipe);

      this.printResult(result, recipe.meta?.name || 'Unknown Recipe');
      return result;
    } catch (error) {
      console.error(`❌ Error reading file: ${error.message}`);
      return {
        valid: false,
        errors: [{ message: error.message }]
      };
    }
  }

  /**
   * Validate multiple recipe files
   * @param {string} directory - Directory containing recipe files
   * @returns {Object} Batch validation results
   */
  async validateDirectory(directory) {
    try {
      const files = await fs.readdir(directory);
      const recipeFiles = files.filter(f => f.endsWith('.rcip') || f.endsWith('.json'));

      console.log(`\n🔍 Found ${recipeFiles.length} recipe files to validate\n`);

      const results = [];
      for (const file of recipeFiles) {
        const filePath = path.join(directory, file);
        const result = await this.validateFile(filePath);
        results.push({ file, ...result });
      }

      this.printSummary();
      return results;
    } catch (error) {
      console.error(`❌ Error reading directory: ${error.message}`);
      throw error;
    }
  }

  /**
   * Check for warnings (non-critical issues)
   * @param {Object} recipe - Recipe object
   * @returns {Array} Warning messages
   */
  checkWarnings(recipe) {
    const warnings = [];

    // Check for missing optional but recommended fields
    if (!recipe.meta?.description) {
      warnings.push('Missing recommended field: meta.description');
    }

    if (!recipe.meta?.servings) {
      warnings.push('Missing recommended field: meta.servings');
    }

    if (!recipe.meta?.difficulty) {
      warnings.push('Missing recommended field: meta.difficulty');
    }

    // Check for missing nutritional data
    const hasNutritionalData = recipe.ingredients?.some(ing => ing.nutritional);
    if (!hasNutritionalData) {
      warnings.push('No nutritional data provided for any ingredient');
    }

    // Check for missing external IDs
    const hasExternalIds = recipe.ingredients?.some(ing => ing.external_ids && Object.keys(ing.external_ids).length > 0);
    if (!hasExternalIds) {
      warnings.push('No external IDs (USDA, GTIN, etc.) provided for ingredients');
    }

    // Check for device profiles without sensors
    if (recipe.device_profiles?.length > 0 && !recipe.sensors?.length) {
      warnings.push('Device profiles defined but no sensors specified');
    }

    // Check for very long cooking times
    if (recipe.meta?.total_time_minutes > 1440) { // 24 hours
      warnings.push('Very long cooking time (>24 hours) - ensure this is intentional');
    }

    // Check for missing images
    if (!recipe.images || recipe.images.length === 0) {
      warnings.push('No images provided for recipe');
    }

    return warnings;
  }

  /**
   * Get recipe information summary
   * @param {Object} recipe - Recipe object
   * @returns {Object} Recipe info
   */
  getRecipeInfo(recipe) {
    return {
      name: recipe.meta?.name,
      version: recipe.rcip_version,
      recipeVersion: recipe.meta?.version,
      ingredientCount: recipe.ingredients?.length || 0,
      stepCount: recipe.steps?.length || 0,
      hasDeviceProfiles: !!(recipe.device_profiles?.length > 0),
      hasSensors: !!(recipe.sensors?.length > 0),
      allergens: this.getAllAllergens(recipe),
      dietLabels: recipe.meta?.diet_labels || [],
      difficulty: recipe.meta?.difficulty || 'not specified',
      totalTime: recipe.meta?.total_time_minutes || 'not specified'
    };
  }

  /**
   * Get all allergens in recipe
   * @param {Object} recipe - Recipe object
   * @returns {Array} Unique allergens
   */
  getAllAllergens(recipe) {
    const allergens = new Set();

    recipe.ingredients?.forEach(ingredient => {
      ingredient.allergens?.forEach(allergen => {
        allergens.add(allergen);
      });
    });

    return Array.from(allergens);
  }

  /**
   * Validate specific components
   */

  validateIngredient(ingredient) {
    const errors = [];

    // Check required fields
    if (!ingredient.id) errors.push('Missing required field: id');
    if (!ingredient.name) errors.push('Missing required field: name');
    if (!ingredient.human_amount) errors.push('Missing required field: human_amount');
    if (!ingredient.machine_amount) errors.push('Missing required field: machine_amount');
    if (!Array.isArray(ingredient.allergens)) {
      errors.push('Missing required field: allergens (must be array, can be empty)');
    }

    // Validate ID format
    if (ingredient.id && !ingredient.id.match(/^ing-[0-9a-zA-Z]+$/)) {
      errors.push(`Invalid ingredient ID format: ${ingredient.id}`);
    }

    // Validate machine amount
    if (ingredient.machine_amount) {
      if (typeof ingredient.machine_amount.value !== 'number' || ingredient.machine_amount.value < 0) {
        errors.push('machine_amount.value must be a non-negative number');
      }
      if (!ingredient.machine_amount.unit) {
        errors.push('machine_amount.unit is required');
      }
    }

    // Validate allergens
    const validAllergens = [
      'milk', 'eggs', 'fish', 'shellfish', 'tree-nuts', 'peanuts',
      'wheat', 'gluten', 'soybeans', 'sesame', 'celery', 'mustard',
      'molluscs', 'lupins', 'sulphites', 'lactose'
    ];

    ingredient.allergens?.forEach(allergen => {
      if (!validAllergens.includes(allergen)) {
        errors.push(`Invalid allergen: ${allergen}`);
      }
    });

    return { valid: errors.length === 0, errors };
  }

  validateStep(step) {
    const errors = [];

    // Check required fields
    if (!step.step_id) errors.push('Missing required field: step_id');
    if (!step.human_text) errors.push('Missing required field: human_text');
    if (!step.action) errors.push('Missing required field: action');

    // Validate ID format
    if (step.step_id && !step.step_id.match(/^s-[0-9a-zA-Z]+$/)) {
      errors.push(`Invalid step ID format: ${step.step_id}`);
    }

    // Validate action
    const validActions = [
      'add', 'mix', 'combine', 'blend', 'cut', 'slice', 'dice', 'chop', 'mince',
      'heat', 'boil', 'simmer', 'steam', 'fry', 'saute', 'bake', 'roast', 'grill',
      'cool', 'chill', 'freeze', 'knead', 'fold', 'roll', 'shape', 'ferment',
      'proof', 'rest', 'strain', 'filter', 'separate', 'measure', 'weigh', 'wait',
      'dissolve', 'prepare', 'spread', 'garnish', 'divide'
    ];

    if (step.action && !validActions.includes(step.action)) {
      errors.push(`Invalid action: ${step.action}`);
    }

    // Validate hazards if present
    const validHazards = [
      'hot-surface', 'sharp-tool', 'electrical', 'chemical', 'pressure', 'allergen-cross-contact'
    ];

    step.hazards?.forEach(hazard => {
      if (!validHazards.includes(hazard)) {
        errors.push(`Invalid hazard: ${hazard}`);
      }
    });

    return { valid: errors.length === 0, errors };
  }

  /**
   * Print validation result
   * @param {Object} result - Validation result
   * @param {string} recipeName - Recipe name
   */
  printResult(result, recipeName) {
    console.log(`\n${'='.repeat(60)}`);
    console.log(`Recipe: ${recipeName}`);
    console.log(`Status: ${result.valid ? '✅ VALID' : '❌ INVALID'}`);

    if (result.info) {
      console.log(`\n📊 Recipe Info:`);
      console.log(`  - RCIP Version: ${result.info.version}`);
      console.log(`  - Ingredients: ${result.info.ingredientCount}`);
      console.log(`  - Steps: ${result.info.stepCount}`);
      console.log(`  - Difficulty: ${result.info.difficulty}`);
      console.log(`  - Total Time: ${result.info.totalTime} minutes`);
      if (result.info.allergens.length > 0) {
        console.log(`  - Allergens: ${result.info.allergens.join(', ')}`);
      }
      if (result.info.dietLabels.length > 0) {
        console.log(`  - Diet Labels: ${result.info.dietLabels.join(', ')}`);
      }
    }

    if (result.errors && result.errors.length > 0) {
      console.log(`\n❌ Errors (${result.errors.length}):`);
      result.errors.slice(0, 10).forEach((error, i) => {
        console.log(`  ${i + 1}. ${error.schemaPath || error.instancePath}: ${error.message}`);
      });
      if (result.errors.length > 10) {
        console.log(`  ... and ${result.errors.length - 10} more errors`);
      }
    }

    if (result.warnings && result.warnings.length > 0) {
      console.log(`\n⚠️  Warnings (${result.warnings.length}):`);
      result.warnings.forEach(warning => {
        console.log(`  - ${warning}`);
      });
    }

    console.log(`${'='.repeat(60)}`);
  }

  /**
   * Print validation summary
   */
  printSummary() {
    console.log(`\n${'='.repeat(60)}`);
    console.log('📈 VALIDATION SUMMARY');
    console.log(`${'='.repeat(60)}`);
    console.log(`Total Validated: ${this.stats.validated}`);
    console.log(`✅ Passed: ${this.stats.passed} (${Math.round(this.stats.passed/this.stats.validated*100)}%)`);
    console.log(`❌ Failed: ${this.stats.failed} (${Math.round(this.stats.failed/this.stats.validated*100)}%)`);
    console.log(`${'='.repeat(60)}\n`);
  }

  /**
   * Reset statistics
   */
  resetStats() {
    this.stats = {
      validated: 0,
      passed: 0,
      failed: 0
    };
  }
}

// Export for use in Node.js
module.exports = RCIPValidator;

// CLI Interface
if (require.main === module) {
  const validator = new RCIPValidator();

  async function main() {
    const args = process.argv.slice(2);

    if (args.length === 0) {
      console.log('Usage: node rcip-validator.js <file.rcip|directory>');
      process.exit(1);
    }

    try {
      await validator.init();

      const target = args[0];
      const stats = await fs.stat(target);

      if (stats.isDirectory()) {
        await validator.validateDirectory(target);
      } else {
        await validator.validateFile(target);
      }
    } catch (error) {
      console.error(`Error: ${error.message}`);
      process.exit(1);
    }
  }

  main();
}
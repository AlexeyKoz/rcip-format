# RCIP Implementation Guide

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Basic Implementation](#basic-implementation)
4. [Advanced Features](#advanced-features)
5. [Best Practices](#best-practices)
6. [Common Patterns](#common-patterns)
7. [Troubleshooting](#troubleshooting)
8. [Migration Guide](#migration-guide)

## Introduction

The Recipe Interchange Protocol (RCIP) is a comprehensive format for storing and exchanging culinary recipes. This guide will help you implement RCIP in your applications, whether you're building a recipe website, smart kitchen appliance, or food service system.

### Who This Guide Is For

- **Web Developers** building recipe platforms
- **IoT Engineers** creating smart kitchen devices
- **Data Scientists** working with food data
- **Mobile App Developers** building cooking applications
- **Restaurant Systems** implementing digital recipes

### Prerequisites

- Basic understanding of JSON
- Familiarity with data validation concepts
- Knowledge of your implementation language (JavaScript, Python, etc.)

## Getting Started

### Installation

#### JavaScript/Node.js
```bash
npm install @rcip/validator @rcip/converter
```

#### Python
```bash
pip install rcip-validator rcip-converter
```

#### Rust
```toml
[dependencies]
rcip = "1.0"
```

### Your First RCIP Recipe

Here's the minimal valid RCIP recipe:

```json
{
  "rcip_version": "0.1",
  "id": "rcip-550e8400-e29b-41d4-a716-446655440000",
  "meta": {
    "name": "Simple Boiled Egg",
    "author": "Home Cook",
    "created_date": "2025-01-15T10:00:00Z"
  },
  "ingredients": [
    {
      "id": "ing-0001",
      "name": "egg",
      "human_amount": "1 large",
      "machine_amount": {
        "value": 1,
        "unit": "pcs"
      },
      "allergens": ["eggs"]
    }
  ],
  "steps": [
    {
      "step_id": "s-01",
      "human_text": "Boil egg in water for 7 minutes",
      "action": "boil"
    }
  ]
}
```

### Validation

Always validate recipes before processing:

```javascript
// JavaScript
const RCIPValidator = require('@rcip/validator');
const validator = new RCIPValidator();
await validator.init();

const result = validator.validateRecipe(recipe);
if (!result.valid) {
  console.error('Validation errors:', result.errors);
}
```

```python
# Python
from rcip_validator import RCIPValidator

validator = RCIPValidator()
validator.init()

result = validator.validate_recipe(recipe)
if not result.valid:
    print("Validation errors:", result.errors)
```

## Basic Implementation

### 1. Recipe Creation

```javascript
class RecipeBuilder {
  constructor() {
    this.recipe = {
      rcip_version: "0.1",
      id: this.generateId(),
      meta: {},
      ingredients: [],
      steps: []
    };
  }
  
  generateId() {
    return `rcip-${uuidv4()}`;
  }
  
  setMeta(name, author, description) {
    this.recipe.meta = {
      name,
      author,
      description,
      created_date: new Date().toISOString()
    };
    return this;
  }
  
  addIngredient(name, amount, unit, allergens = []) {
    const id = `ing-${String(this.recipe.ingredients.length + 1).padStart(4, '0')}`;
    
    this.recipe.ingredients.push({
      id,
      name,
      human_amount: `${amount} ${unit}`,
      machine_amount: {
        value: parseFloat(amount),
        unit: this.normalizeUnit(unit)
      },
      allergens,
      since_version: "0.1"
    });
    return this;
  }
  
  addStep(instruction, action = 'prepare', params = {}) {
    const id = `s-${String(this.recipe.steps.length + 1).padStart(2, '0')}`;
    
    this.recipe.steps.push({
      step_id: id,
      human_text: instruction,
      action,
      params,
      since_version: "0.1"
    });
    return this;
  }
  
  normalizeUnit(unit) {
    const unitMap = {
      'cups': 'cup',
      'tablespoons': 'tbsp',
      'teaspoons': 'tsp',
      'pounds': 'lb',
      'ounces': 'oz',
      'grams': 'g'
    };
    return unitMap[unit.toLowerCase()] || unit.toLowerCase();
  }
  
  build() {
    return this.recipe;
  }
}

// Usage
const builder = new RecipeBuilder();
const recipe = builder
  .setMeta("Chocolate Cake", "Jane Doe", "Rich chocolate cake")
  .addIngredient("flour", "2", "cups", ["gluten", "wheat"])
  .addIngredient("cocoa powder", "0.75", "cup", [])
  .addIngredient("eggs", "3", "pieces", ["eggs"])
  .addStep("Mix dry ingredients", "mix")
  .addStep("Add wet ingredients and combine", "combine")
  .addStep("Bake at 350°F for 30 minutes", "bake", {
    temperature_f: 350,
    time_minutes: 30
  })
  .build();
```

### 2. Recipe Parsing

```python
class RecipeParser:
    """Parse recipes from various sources"""
    
    def parse_ingredient_string(self, text: str) -> dict:
        """
        Parse "2 cups flour" into structured format
        """
        import re
        
        # Pattern: amount unit ingredient
        pattern = r'^([\d/.]+)\s*(\w+)?\s+(.+)$'
        match = re.match(pattern, text)
        
        if match:
            amount = self._parse_amount(match.group(1))
            unit = match.group(2) or 'unit'
            name = match.group(3)
            
            return {
                'name': name,
                'human_amount': text,
                'machine_amount': {
                    'value': amount,
                    'unit': self._normalize_unit(unit)
                },
                'allergens': self._detect_allergens(name)
            }
        
        return {
            'name': text,
            'human_amount': text,
            'machine_amount': {'value': 1, 'unit': 'unit'},
            'allergens': []
        }
    
    def _parse_amount(self, amount_str: str) -> float:
        """Parse amounts like '1/2' or '2.5'"""
        if '/' in amount_str:
            parts = amount_str.split('/')
            return float(parts[0]) / float(parts[1])
        return float(amount_str)
    
    def _detect_allergens(self, ingredient: str) -> list:
        """Auto-detect common allergens"""
        allergen_map = {
            'milk': ['milk', 'cream', 'butter', 'cheese'],
            'eggs': ['egg'],
            'wheat': ['flour', 'bread'],
            'nuts': ['almond', 'walnut', 'pecan'],
            'soy': ['soy', 'tofu']
        }
        
        ingredient_lower = ingredient.lower()
        detected = []
        
        for allergen, keywords in allergen_map.items():
            if any(kw in ingredient_lower for kw in keywords):
                detected.append(allergen)
        
        return detected
```

### 3. Recipe Rendering

```javascript
class RecipeRenderer {
  renderHTML(recipe) {
    return `
      <article class="rcip-recipe">
        <header>
          <h1>${recipe.meta.name}</h1>
          <p>${recipe.meta.description || ''}</p>
          <div class="meta">
            <span>By ${this.renderAuthor(recipe.meta.author)}</span>
            <span>Serves ${recipe.meta.servings?.amount || '?'}</span>
            <span>${recipe.meta.total_time_minutes || '?'} minutes</span>
          </div>
        </header>
        
        <section class="allergens">
          ${this.renderAllergens(recipe)}
        </section>
        
        <section class="ingredients">
          <h2>Ingredients</h2>
          <ul>
            ${recipe.ingredients.map(ing => `
              <li>${ing.human_amount} ${ing.name}</li>
            `).join('')}
          </ul>
        </section>
        
        <section class="instructions">
          <h2>Instructions</h2>
          <ol>
            ${recipe.steps.map(step => `
              <li>${step.human_text}</li>
            `).join('')}
          </ol>
        </section>
      </article>
    `;
  }
  
  renderAuthor(author) {
    if (typeof author === 'string') return author;
    return author.name || 'Unknown';
  }
  
  renderAllergens(recipe) {
    const allergens = new Set();
    recipe.ingredients.forEach(ing => {
      ing.allergens?.forEach(a => allergens.add(a));
    });
    
    if (allergens.size === 0) return '';
    
    return `
      <div class="allergen-warning">
        ⚠️ Contains: ${Array.from(allergens).join(', ')}
      </div>
    `;
  }
}
```

## Advanced Features

### 1. Device Integration

```javascript
class SmartOvenIntegration {
  constructor(ovenAPI) {
    this.oven = ovenAPI;
  }
  
  async executeStep(step, deviceProfile) {
    if (step.action === 'bake' && deviceProfile.type === 'oven') {
      // Extract parameters
      const temp = step.params?.temperature_c || deviceProfile.params?.temperature_c;
      const time = step.params?.time_minutes;
      
      // Send to oven
      await this.oven.preheat(temp);
      await this.oven.startTimer(time * 60); // Convert to seconds
      
      // Monitor with sensors
      return this.monitorBaking(step.done_when);
    }
  }
  
  async monitorBaking(doneConditions) {
    const interval = setInterval(async () => {
      const sensorData = await this.oven.getSensors();
      
      if (doneConditions?.color) {
        if (this.checkColor(sensorData.camera, doneConditions.color)) {
          clearInterval(interval);
          await this.oven.stop();
          return 'complete';
        }
      }
      
      if (doneConditions?.temperature_c) {
        if (sensorData.probe >= doneConditions.temperature_c) {
          clearInterval(interval);
          await this.oven.stop();
          return 'complete';
        }
      }
    }, 5000); // Check every 5 seconds
  }
  
  checkColor(image, targetColor) {
    // Image processing to check color
    // Return true if color matches target
  }
}
```

### 2. Nutritional Calculation

```python
class NutritionCalculator:
    """Calculate recipe nutrition from ingredients"""
    
    def __init__(self, usda_api_key: str = None):
        self.usda_api_key = usda_api_key
        self.cache = {}
    
    def calculate_recipe_nutrition(self, recipe: dict) -> dict:
        """Calculate total nutrition for recipe"""
        totals = {
            'calories': 0,
            'protein': 0,
            'carbs': 0,
            'fat': 0,
            'fiber': 0,
            'sodium': 0
        }
        
        for ingredient in recipe['ingredients']:
            nutrition = self._get_ingredient_nutrition(ingredient)
            if nutrition:
                amount_factor = self._calculate_amount_factor(ingredient)
                for nutrient, value in nutrition.items():
                    if nutrient in totals:
                        totals[nutrient] += value * amount_factor
        
        # Calculate per serving
        servings = recipe['meta'].get('servings', {}).get('amount', 1)
        per_serving = {k: round(v / servings, 1) for k, v in totals.items()}
        
        return {
            'total': totals,
            'per_serving': per_serving
        }
    
    def _get_ingredient_nutrition(self, ingredient: dict) -> dict:
        """Get nutrition data for ingredient"""
        # Check if ingredient has nutrition data
        if 'nutritional' in ingredient:
            return ingredient['nutritional'].get('per_100g', {})
        
        # Check cache
        if ingredient['name'] in self.cache:
            return self.cache[ingredient['name']]
        
        # Fetch from USDA if ID provided
        if 'external_ids' in ingredient and 'USDA' in ingredient['external_ids']:
            nutrition = self._fetch_usda_nutrition(ingredient['external_ids']['USDA'])
            self.cache[ingredient['name']] = nutrition
            return nutrition
        
        return None
    
    def _fetch_usda_nutrition(self, usda_id: str) -> dict:
        """Fetch nutrition from USDA database"""
        # Implementation would call USDA API
        pass
    
    def _calculate_amount_factor(self, ingredient: dict) -> float:
        """Calculate multiplication factor based on amount"""
        machine_amount = ingredient.get('machine_amount', {})
        value = machine_amount.get('value', 0)
        unit = machine_amount.get('unit', '')
        
        # Convert to grams (base unit for per_100g)
        grams = self._convert_to_grams(value, unit)
        return grams / 100 if grams else 0
    
    def _convert_to_grams(self, value: float, unit: str) -> float:
        """Convert various units to grams"""
        conversions = {
            'g': 1,
            'kg': 1000,
            'mg': 0.001,
            'oz': 28.35,
            'lb': 453.6,
            'cup': 240,  # Approximate for water
            'tbsp': 15,
            'tsp': 5
        }
        return value * conversions.get(unit, 0)
```

### 3. Recipe Scaling

```javascript
class RecipeScaler {
  scale(recipe, targetServings) {
    const currentServings = recipe.meta.servings?.amount || 1;
    const factor = targetServings / currentServings;
    
    // Deep clone recipe
    const scaledRecipe = JSON.parse(JSON.stringify(recipe));
    
    // Update servings
    scaledRecipe.meta.servings = {
      amount: targetServings,
      unit: recipe.meta.servings?.unit || "servings"
    };
    
    // Scale ingredients
    scaledRecipe.ingredients = scaledRecipe.ingredients.map(ing => {
      const scaled = this.scaleIngredient(ing, factor);
      return scaled;
    });
    
    // Adjust cooking times (some may need adjustment)
    scaledRecipe.steps = scaledRecipe.steps.map(step => {
      return this.adjustStep(step, factor);
    });
    
    return scaledRecipe;
  }
  
  scaleIngredient(ingredient, factor) {
    const scaled = {...ingredient};
    
    // Scale machine amount
    if (scaled.machine_amount) {
      scaled.machine_amount.value *= factor;
      
      // Round to reasonable precision
      scaled.machine_amount.value = this.roundAmount(
        scaled.machine_amount.value,
        scaled.machine_amount.unit
      );
    }
    
    // Regenerate human amount
    scaled.human_amount = this.formatHumanAmount(
      scaled.machine_amount.value,
      scaled.machine_amount.unit,
      scaled.name
    );
    
    return scaled;
  }
  
  roundAmount(value, unit) {
    // Different rounding for different units
    const precision = {
      'tsp': 0.25,
      'tbsp': 0.5,
      'cup': 0.25,
      'g': 1,
      'kg': 0.01,
      'pcs': 1
    };
    
    const p = precision[unit] || 0.1;
    return Math.round(value / p) * p;
  }
  
  formatHumanAmount(value, unit, name) {
    // Convert to mixed numbers for readability
    if (unit === 'cup' || unit === 'tsp' || unit === 'tbsp') {
      return this.toMixedNumber(value) + ' ' + unit + ' ' + name;
    }
    return value + ' ' + unit + ' ' + name;
  }
  
  toMixedNumber(decimal) {
    const whole = Math.floor(decimal);
    const fraction = decimal - whole;
    
    if (fraction === 0) return String(whole);
    if (whole === 0) return this.toFraction(fraction);
    return whole + ' ' + this.toFraction(fraction);
  }
  
  toFraction(decimal) {
    const fractions = [
      {decimal: 0.25, fraction: '1/4'},
      {decimal: 0.33, fraction: '1/3'},
      {decimal: 0.5, fraction: '1/2'},
      {decimal: 0.67, fraction: '2/3'},
      {decimal: 0.75, fraction: '3/4'}
    ];
    
    const closest = fractions.reduce((prev, curr) => 
      Math.abs(curr.decimal - decimal) < Math.abs(prev.decimal - decimal) ? curr : prev
    );
    
    return closest.fraction;
  }
  
  adjustStep(step, factor) {
    const adjusted = {...step};
    
    // Some cooking times don't scale linearly
    if (adjusted.params?.time_minutes) {
      // Baking times scale less than linearly
      if (step.action === 'bake') {
        // Rough approximation: time scales with square root for baking
        adjusted.params.time_minutes *= Math.sqrt(factor);
        adjusted.params.time_minutes = Math.round(adjusted.params.time_minutes);
      }
    }
    
    return adjusted;
  }
}
```

## Best Practices

### 1. Always Include Allergens

```javascript
// ❌ Bad - missing allergens
{
  "id": "ing-0001",
  "name": "peanut butter",
  "human_amount": "2 tbsp",
  "machine_amount": {"value": 2, "unit": "tbsp"}
}

// ✅ Good - allergens declared
{
  "id": "ing-0001",
  "name": "peanut butter",
  "human_amount": "2 tbsp",
  "machine_amount": {"value": 2, "unit": "tbsp"},
  "allergens": ["peanuts"]
}
```

### 2. Use Semantic Actions

```javascript
// ❌ Bad - generic action
{
  "step_id": "s-01",
  "human_text": "Put in oven at 180°C for 25 minutes",
  "action": "heat"
}

// ✅ Good - specific action with parameters
{
  "step_id": "s-01",
  "human_text": "Bake at 180°C for 25 minutes",
  "action": "bake",
  "params": {
    "temperature_c": 180,
    "time_minutes": 25
  }
}
```

### 3. Provide Machine-Readable Data

```javascript
// ❌ Bad - only human readable
{
  "human_amount": "a handful"
}

// ✅ Good - both human and machine readable
{
  "human_amount": "a handful",
  "machine_amount": {
    "value": 30,
    "unit": "g",
    "approximate": true
  }
}
```

### 4. Include Completion Criteria

```javascript
// ✅ Good - clear done conditions
{
  "step_id": "s-05",
  "human_text": "Bake until golden brown",
  "action": "bake",
  "done_when": {
    "color": {
      "description": "golden-brown",
      "rgb": {"r": [180, 200], "g": [140, 160], "b": [100, 120]}
    },
    "internal_temp_c": 98,
    "texture": "firm-bounce-back"
  }
}
```

## Common Patterns

### 1. Multi-Stage Recipes

```javascript
// Recipe with sub-components (e.g., cake with frosting)
{
  "meta": {
    "name": "Chocolate Cake with Buttercream",
    "components": ["cake", "frosting"]
  },
  "ingredients": [
    // Cake ingredients
    {"id": "ing-0001", "name": "flour", "component": "cake", ...},
    {"id": "ing-0002", "name": "cocoa", "component": "cake", ...},
    // Frosting ingredients
    {"id": "ing-0010", "name": "butter", "component": "frosting", ...},
    {"id": "ing-0011", "name": "powdered sugar", "component": "frosting", ...}
  ],
  "steps": [
    // Cake steps
    {"step_id": "s-01", "component": "cake", ...},
    // Frosting steps
    {"step_id": "s-10", "component": "frosting", ...}
  ]
}
```

### 2. Alternative Methods

```javascript
{
  "device_profiles": [
    {
      "id": "oven-01",
      "type": "oven",
      "name": "Conventional Oven",
      "params": {"temperature_c": 180, "time_minutes": 30}
    },
    {
      "id": "airfryer-01",
      "type": "airfryer",
      "name": "Air Fryer Alternative",
      "params": {"temperature_c": 160, "time_minutes": 20}
    }
  ]
}
```

### 3. Conditional Steps

```javascript
{
  "step_id": "s-05",
  "human_text": "If using fresh herbs, add now. If dried, wait until step 8",
  "action": "add",
  "conditions": {
    "if": {"ingredient_state": "ing-0008", "equals": "fresh"},
    "then": {"add": "ing-0008"},
    "else": {"wait_until": "s-08"}
  }
}
```

## Troubleshooting

### Common Validation Errors

#### Missing Allergens
**Error**: `Missing required field: allergens`
**Solution**: Every ingredient must have an `allergens` array (can be empty)

```javascript
// Fix
ingredients.forEach(ing => {
  if (!ing.allergens) {
    ing.allergens = [];
  }
});
```

#### Invalid ID Format
**Error**: `Invalid recipe ID format`
**Solution**: Use UUID v4 format

```javascript
const { v4: uuidv4 } = require('uuid');
recipe.id = `rcip-${uuidv4()}`;
```

#### Invalid Action
**Error**: `Invalid action 'stir'`
**Solution**: Use canonical actions

```javascript
const actionMap = {
  'stir': 'mix',
  'stir-fry': 'fry',
  'sauté': 'saute'
};
step.action = actionMap[step.action] || step.action;
```

### Performance Issues

#### Large Recipe Collections
```javascript
// Use streaming for large datasets
const stream = fs.createReadStream('recipes.ndjson');
const rl = readline.createInterface({input: stream});

rl.on('line', (line) => {
  const recipe = JSON.parse(line);
  processRecipe(recipe);
});
```

#### Validation Performance
```javascript
// Cache schema compilation
let validator;
function getValidator() {
  if (!validator) {
    validator = new RCIPValidator();
    validator.init();
  }
  return validator;
}
```

## Migration Guide

### From Schema.org

```javascript
const SchemaOrgToRCIP = require('@rcip/converter').SchemaOrgToRCIP;
const converter = new SchemaOrgToRCIP();

const schemaRecipe = {
  "@type": "Recipe",
  "name": "Pasta Carbonara",
  // ... Schema.org format
};

const rcipRecipe = converter.convert(schemaRecipe);
```

### From Custom Format

```javascript
function migrateFromCustom(customRecipe) {
  return {
    rcip_version: "0.1",
    id: `rcip-${customRecipe.id || uuidv4()}`,
    meta: {
      name: customRecipe.title,
      author: customRecipe.chef || "Unknown",
      created_date: customRecipe.date || new Date().toISOString(),
      servings: {
        amount: customRecipe.yields || 1,
        unit: "servings"
      }
    },
    ingredients: customRecipe.ingredients.map((ing, i) => ({
      id: `ing-${String(i + 1).padStart(4, '0')}`,
      name: ing.item,
      human_amount: `${ing.amount} ${ing.unit}`,
      machine_amount: {
        value: parseFloat(ing.amount),
        unit: normalizeUnit(ing.unit)
      },
      allergens: detectAllergens(ing.item)
    })),
    steps: customRecipe.instructions.map((inst, i) => ({
      step_id: `s-${String(i + 1).padStart(2, '0')}`,
      human_text: inst,
      action: detectAction(inst)
    }))
  };
}
```

### Batch Migration

```python
import json
from pathlib import Path
from rcip_converter import CustomToRCIPConverter

def batch_migrate(input_dir: Path, output_dir: Path):
    """Migrate all recipes in directory"""
    converter = CustomToRCIPConverter()
    
    for input_file in input_dir.glob("*.json"):
        try:
            # Read custom format
            with open(input_file) as f:
                custom_recipe = json.load(f)
            
            # Convert to RCIP
            rcip_recipe = converter.convert(custom_recipe)
            
            # Validate
            validator = RCIPValidator()
            result = validator.validate(rcip_recipe)
            
            if result.valid:
                # Save RCIP format
                output_file = output_dir / f"{input_file.stem}.rcip"
                with open(output_file, 'w') as f:
                    json.dump(rcip_recipe, f, indent=2)
                print(f"✅ Migrated: {input_file.name}")
            else:
                print(f"❌ Validation failed: {input_file.name}")
                print(f"   Errors: {result.errors}")
                
        except Exception as e:
            print(f"❌ Error migrating {input_file.name}: {e}")
    
    print(f"\nMigration complete!")
```

## Next Steps

- Read the [API Reference](api-reference.md) for detailed field descriptions
- Explore [Device Profiles](device-profiles.md) for IoT integration
- Study [Sensor Specifications](sensor-specifications.md) for quality control
- Join our [Discord Community](https://discord.gg/rcip-format) for support

---

*Created by Alexey Kozlov - [al7koz.com](https://al7koz.com)*
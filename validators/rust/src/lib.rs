// RCIP Format Validator for Rust
// Created by Alexey Kozlov
// Version: 1.0.0
// License: MIT

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use jsonschema::{JSONSchema, ValidationError};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt;
use lazy_static::lazy_static;

// Regex patterns for validation
lazy_static! {
    static ref RECIPE_ID_REGEX: Regex =
        Regex::new(r"^rcip-[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
    static ref INGREDIENT_ID_REGEX: Regex =
        Regex::new(r"^ing-[0-9a-zA-Z]+$").unwrap();
    static ref STEP_ID_REGEX: Regex =
        Regex::new(r"^s-[0-9a-zA-Z]+$").unwrap();
    static ref VERSION_REGEX: Regex =
        Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    static ref COUNTRY_CODE_REGEX: Regex =
        Regex::new(r"^[A-Z]{2}$").unwrap();
}

/// Valid diet labels
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DietLabel {
    Vegetarian,
    Vegan,
    GlutenFree,
    DairyFree,
    NutFree,
    EggFree,
    SoyFree,
    FishFree,
    ShellfishFree,
    Kosher,
    Halal,
    LowSodium,
    LowCarb,
    Keto,
    Paleo,
}

/// Valid allergens
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Allergen {
    Milk,
    Eggs,
    Fish,
    Shellfish,
    TreeNuts,
    Peanuts,
    Wheat,
    Gluten,
    Soybeans,
    Sesame,
    Celery,
    Mustard,
    Molluscs,
    Lupins,
    Sulphites,
    Lactose,
}

/// Valid cooking actions
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CookingAction {
    Add, Mix, Combine, Blend,
    Cut, Slice, Dice, Chop, Mince,
    Heat, Boil, Simmer, Steam, Fry, Saute, Bake, Roast, Grill,
    Cool, Chill, Freeze,
    Knead, Fold, Roll, Shape,
    Ferment, Proof, Rest,
    Strain, Filter, Separate,
    Measure, Weigh, Wait,
    Dissolve, Prepare, Spread, Garnish, Divide,
}

/// Valid measurement units
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Unit {
    // Mass
    Mg, G, Kg, Oz, Lb,
    // Volume
    Ml, L, Tsp, Tbsp, Cup, FlOz, Pt, Qt, Gal,
    // Count
    Pcs, Dozen,
    // Special
    Pinch, Dash, Handful, ToTaste,
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub info: RecipeInfo,
}

/// Recipe information
#[derive(Debug, Clone, Default)]
pub struct RecipeInfo {
    pub name: String,
    pub version: String,
    pub recipe_version: Option<String>,
    pub ingredient_count: usize,
    pub step_count: usize,
    pub has_device_profiles: bool,
    pub has_sensors: bool,
    pub allergens: Vec<String>,
    pub diet_labels: Vec<String>,
    pub difficulty: Option<String>,
    pub total_time: Option<f64>,
}

/// Validation statistics
#[derive(Debug, Default)]
pub struct ValidationStats {
    pub validated: u32,
    pub passed: u32,
    pub failed: u32,
}

/// Custom error type
#[derive(Debug)]
pub enum RCIPError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    ValidationError(String),
    SchemaError(String),
}

impl fmt::Display for RCIPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RCIPError::IoError(e) => write!(f, "IO error: {}", e),
            RCIPError::JsonError(e) => write!(f, "JSON error: {}", e),
            RCIPError::ValidationError(e) => write!(f, "Validation error: {}", e),
            RCIPError::SchemaError(e) => write!(f, "Schema error: {}", e),
        }
    }
}

impl Error for RCIPError {}

impl From<std::io::Error> for RCIPError {
    fn from(err: std::io::Error) -> Self {
        RCIPError::IoError(err)
    }
}

impl From<serde_json::Error> for RCIPError {
    fn from(err: serde_json::Error) -> Self {
        RCIPError::JsonError(err)
    }
}

/// RCIP Validator
pub struct RCIPValidator {
    schema_version: String,
    schema: Option<Value>,
    compiled_schema: Option<JSONSchema>,
    stats: ValidationStats,
}

impl RCIPValidator {
    /// Create a new validator
    pub fn new(schema_version: &str) -> Self {
        RCIPValidator {
            schema_version: schema_version.to_string(),
            schema: None,
            compiled_schema: None,
            stats: ValidationStats::default(),
        }
    }

    /// Initialize validator with schema
    pub fn init(&mut self, schema_path: Option<&Path>) -> Result<(), RCIPError> {
        let path = if let Some(p) = schema_path {
            p.to_path_buf()
        } else {
            PathBuf::from(format!("../../schemas/rcip-v{}.json", self.schema_version))
        };

        let schema_content = fs::read_to_string(&path)?;
        let schema: Value = serde_json::from_str(&schema_content)?;

        match JSONSchema::compile(&schema) {
            Ok(compiled) => {
                self.compiled_schema = Some(compiled);
                self.schema = Some(schema);
                println!("✅ RCIP Validator initialized with schema v{}", self.schema_version);
                Ok(())
            }
            Err(e) => Err(RCIPError::SchemaError(format!("Failed to compile schema: {}", e)))
        }
    }

    /// Validate a recipe
    pub fn validate_recipe(&mut self, recipe: &Value) -> ValidationResult {
        let mut result = ValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            info: RecipeInfo::default(),
        };

        // Check if validator is initialized
        let compiled_schema = match &self.compiled_schema {
            Some(s) => s,
            None => {
                result.valid = false;
                result.errors.push("Validator not initialized. Call init() first.".to_string());
                return result;
            }
        };

        // JSON Schema validation
        if let Err(errors) = compiled_schema.validate(recipe) {
            result.valid = false;
            for error in errors {
                result.errors.push(format!("{}: {}", error.instance_path, error));
            }
        }

        // Custom validations
        self.validate_custom_rules(recipe, &mut result);

        // Check warnings
        result.warnings = self.check_warnings(recipe);

        // Get recipe info
        result.info = self.get_recipe_info(recipe);

        // Update stats
        self.stats.validated += 1;
        if result.valid {
            self.stats.passed += 1;
        } else {
            self.stats.failed += 1;
        }

        result
    }

    /// Validate a recipe file
    pub fn validate_file(&mut self, file_path: &Path) -> Result<ValidationResult, RCIPError> {
        let content = fs::read_to_string(file_path)?;
        let recipe: Value = serde_json::from_str(&content)?;

        println!("\n📄 Validating: {}", file_path.file_name().unwrap().to_str().unwrap());

        let result = self.validate_recipe(&recipe);
        let recipe_name = recipe.get("meta")
            .and_then(|m| m.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("Unknown Recipe");

        self.print_result(&result, recipe_name);
        Ok(result)
    }

    /// Validate all recipes in a directory
    pub fn validate_directory(&mut self, dir_path: &Path) -> Result<Vec<(String, ValidationResult)>, RCIPError> {
        let mut results = Vec::new();

        let entries = fs::read_dir(dir_path)?;
        let mut recipe_files = Vec::new();

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "rcip" || ext == "json" {
                    recipe_files.push(path);
                }
            }
        }

        println!("\n🔍 Found {} recipe files to validate\n", recipe_files.len());

        for file_path in recipe_files {
            let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
            match self.validate_file(&file_path) {
                Ok(result) => results.push((file_name, result)),
                Err(e) => {
                    let mut result = ValidationResult {
                        valid: false,
                        errors: vec![format!("Error reading file: {}", e)],
                        warnings: Vec::new(),
                        info: RecipeInfo::default(),
                    };
                    results.push((file_name, result));
                }
            }
        }

        self.print_summary();
        Ok(results)
    }

    /// Apply custom validation rules
    fn validate_custom_rules(&self, recipe: &Value, result: &mut ValidationResult) {
        // Validate recipe ID
        if let Some(id) = recipe.get("id").and_then(|v| v.as_str()) {
            if !RECIPE_ID_REGEX.is_match(id) {
                result.valid = false;
                result.errors.push(format!("Invalid recipe ID format: {}", id));
            }
        }

        // Validate ingredients
        if let Some(ingredients) = recipe.get("ingredients").and_then(|v| v.as_array()) {
            for (i, ingredient) in ingredients.iter().enumerate() {
                self.validate_ingredient(ingredient, i, result);
            }
        }

        // Validate steps
        if let Some(steps) = recipe.get("steps").and_then(|v| v.as_array()) {
            for (i, step) in steps.iter().enumerate() {
                self.validate_step(step, i, result);
            }
        }

        // Validate cross-references
        self.validate_references(recipe, result);

        // Check version compatibility
        if let Some(version) = recipe.get("rcip_version").and_then(|v| v.as_str()) {
            if version != self.schema_version {
                result.warnings.push(format!(
                    "Recipe version {} may not be fully compatible with validator version {}",
                    version, self.schema_version
                ));
            }
        }
    }

    /// Validate an ingredient
    fn validate_ingredient(&self, ingredient: &Value, index: usize, result: &mut ValidationResult) {
        // Check ID format
        if let Some(id) = ingredient.get("id").and_then(|v| v.as_str()) {
            if !INGREDIENT_ID_REGEX.is_match(id) {
                result.valid = false;
                result.errors.push(format!("Ingredient {}: Invalid ID format: {}", index, id));
            }
        }

        // Check allergens (must be present, can be empty)
        match ingredient.get("allergens") {
            None => {
                result.valid = false;
                result.errors.push(format!("Ingredient {}: Missing required allergens field", index));
            }
            Some(allergens) if !allergens.is_array() => {
                result.valid = false;
                result.errors.push(format!("Ingredient {}: allergens must be an array", index));
            }
            Some(allergens) => {
                // Validate allergen values
                let valid_allergens = vec![
                    "milk", "eggs", "fish", "shellfish", "tree-nuts", "peanuts",
                    "wheat", "gluten", "soybeans", "sesame", "celery", "mustard",
                    "molluscs", "lupins", "sulphites", "lactose"
                ];

                if let Some(allergen_array) = allergens.as_array() {
                    for allergen in allergen_array {
                        if let Some(allergen_str) = allergen.as_str() {
                            if !valid_allergens.contains(&allergen_str) {
                                result.errors.push(format!(
                                    "Ingredient {}: Invalid allergen '{}'",
                                    index, allergen_str
                                ));
                            }
                        }
                    }
                }
            }
        }

        // Check machine amount
        if let Some(ma) = ingredient.get("machine_amount") {
            if let Some(value) = ma.get("value") {
                if !value.is_number() || value.as_f64().unwrap_or(-1.0) < 0.0 {
                    result.errors.push(format!(
                        "Ingredient {}: machine_amount.value must be non-negative number",
                        index
                    ));
                }
            }
            if ma.get("unit").is_none() {
                result.errors.push(format!("Ingredient {}: machine_amount.unit is required", index));
            }
        }
    }

    /// Validate a step
    fn validate_step(&self, step: &Value, index: usize, result: &mut ValidationResult) {
        // Check ID format
        if let Some(id) = step.get("step_id").and_then(|v| v.as_str()) {
            if !STEP_ID_REGEX.is_match(id) {
                result.valid = false;
                result.errors.push(format!("Step {}: Invalid ID format: {}", index, id));
            }
        }

        // Check action
        if let Some(action) = step.get("action").and_then(|v| v.as_str()) {
            let valid_actions = vec![
                "add", "mix", "combine", "blend", "cut", "slice", "dice", "chop", "mince",
                "heat", "boil", "simmer", "steam", "fry", "saute", "bake", "roast", "grill",
                "cool", "chill", "freeze", "knead", "fold", "roll", "shape", "ferment",
                "proof", "rest", "strain", "filter", "separate", "measure", "weigh", "wait",
                "dissolve", "prepare", "spread", "garnish", "divide"
            ];

            if !valid_actions.contains(&action) {
                result.errors.push(format!("Step {}: Invalid action '{}'", index, action));
            }
        }

        // Check hazards
        if let Some(hazards) = step.get("hazards").and_then(|v| v.as_array()) {
            let valid_hazards = vec![
                "hot-surface", "sharp-tool", "electrical", "chemical", "pressure", "allergen-cross-contact"
            ];

            for hazard in hazards {
                if let Some(hazard_str) = hazard.as_str() {
                    if !valid_hazards.contains(&hazard_str) {
                        result.warnings.push(format!("Step {}: Non-standard hazard '{}'", index, hazard_str));
                    }
                }
            }
        }
    }

    /// Validate cross-references
    fn validate_references(&self, recipe: &Value, result: &mut ValidationResult) {
        // Collect all IDs
        let mut ingredient_ids = HashSet::new();
        let mut step_ids = HashSet::new();

        if let Some(ingredients) = recipe.get("ingredients").and_then(|v| v.as_array()) {
            for ing in ingredients {
                if let Some(id) = ing.get("id").and_then(|v| v.as_str()) {
                    ingredient_ids.insert(id.to_string());
                }
            }
        }

        if let Some(steps) = recipe.get("steps").and_then(|v| v.as_array()) {
            for step in steps {
                if let Some(id) = step.get("step_id").and_then(|v| v.as_str()) {
                    step_ids.insert(id.to_string());
                }
            }
        }

        // Check step targets
        if let Some(steps) = recipe.get("steps").and_then(|v| v.as_array()) {
            for step in steps {
                if let Some(targets) = step.get("target").and_then(|v| v.as_array()) {
                    for target in targets {
                        if let Some(target_str) = target.as_str() {
                            if target_str.starts_with("ing-") && !ingredient_ids.contains(target_str) {
                                result.errors.push(format!(
                                    "Step {}: Invalid ingredient reference '{}'",
                                    step.get("step_id").and_then(|v| v.as_str()).unwrap_or("?"),
                                    target_str
                                ));
                            } else if target_str.contains(":result") {
                                let step_ref = target_str.split(':').next().unwrap();
                                if !step_ids.contains(step_ref) {
                                    result.errors.push(format!(
                                        "Step {}: Invalid step reference '{}'",
                                        step.get("step_id").and_then(|v| v.as_str()).unwrap_or("?"),
                                        target_str
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Check for warnings
    fn check_warnings(&self, recipe: &Value) -> Vec<String> {
        let mut warnings = Vec::new();

        let meta = recipe.get("meta");

        // Check for missing recommended fields
        if meta.and_then(|m| m.get("description")).is_none() {
            warnings.push("Missing recommended field: meta.description".to_string());
        }

        if meta.and_then(|m| m.get("servings")).is_none() {
            warnings.push("Missing recommended field: meta.servings".to_string());
        }

        if meta.and_then(|m| m.get("difficulty")).is_none() {
            warnings.push("Missing recommended field: meta.difficulty".to_string());
        }

        // Check for missing nutritional data
        let has_nutritional = recipe.get("ingredients")
            .and_then(|v| v.as_array())
            .map(|ingredients| {
                ingredients.iter().any(|ing| ing.get("nutritional").is_some())
            })
            .unwrap_or(false);

        if !has_nutritional {
            warnings.push("No nutritional data provided for any ingredient".to_string());
        }

        // Check for missing external IDs
        let has_external_ids = recipe.get("ingredients")
            .and_then(|v| v.as_array())
            .map(|ingredients| {
                ingredients.iter().any(|ing| {
                    ing.get("external_ids")
                        .and_then(|e| e.as_object())
                        .map(|o| !o.is_empty())
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false);

        if !has_external_ids {
            warnings.push("No external IDs (USDA, GTIN, etc.) provided".to_string());
        }

        // Check for very long cooking times
        if let Some(total_time) = meta.and_then(|m| m.get("total_time_minutes")).and_then(|v| v.as_f64()) {
            if total_time > 1440.0 {
                warnings.push(format!(
                    "Very long cooking time ({} min / {:.1} hours)",
                    total_time, total_time / 60.0
                ));
            }
        }

        // Check for missing images
        if recipe.get("images").and_then(|v| v.as_array()).map(|a| a.is_empty()).unwrap_or(true) {
            warnings.push("No images provided for recipe".to_string());
        }

        warnings
    }

    /// Get recipe information
    fn get_recipe_info(&self, recipe: &Value) -> RecipeInfo {
        let meta = recipe.get("meta");

        // Get all allergens
        let mut allergens = HashSet::new();
        if let Some(ingredients) = recipe.get("ingredients").and_then(|v| v.as_array()) {
            for ingredient in ingredients {
                if let Some(allergen_array) = ingredient.get("allergens").and_then(|v| v.as_array()) {
                    for allergen in allergen_array {
                        if let Some(allergen_str) = allergen.as_str() {
                            allergens.insert(allergen_str.to_string());
                        }
                    }
                }
            }
        }

        RecipeInfo {
            name: meta.and_then(|m| m.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string(),
            version: recipe.get("rcip_version")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            recipe_version: meta.and_then(|m| m.get("version"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            ingredient_count: recipe.get("ingredients")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0),
            step_count: recipe.get("steps")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0),
            has_device_profiles: recipe.get("device_profiles")
                .and_then(|v| v.as_array())
                .map(|a| !a.is_empty())
                .unwrap_or(false),
            has_sensors: recipe.get("sensors")
                .and_then(|v| v.as_array())
                .map(|a| !a.is_empty())
                .unwrap_or(false),
            allergens: allergens.into_iter().collect(),
            diet_labels: meta.and_then(|m| m.get("diet_labels"))
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_else(Vec::new),
            difficulty: meta.and_then(|m| m.get("difficulty"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            total_time: meta.and_then(|m| m.get("total_time_minutes"))
                .and_then(|v| v.as_f64()),
        }
    }

    /// Print validation result
    fn print_result(&self, result: &ValidationResult, recipe_name: &str) {
        println!("\n{}", "=".repeat(60));
        println!("Recipe: {}", recipe_name);
        println!("Status: {}", if result.valid { "✅ VALID" } else { "❌ INVALID" });

        println!("\n📊 Recipe Info:");
        println!("  - RCIP Version: {}", result.info.version);
        println!("  - Ingredients: {}", result.info.ingredient_count);
        println!("  - Steps: {}", result.info.step_count);
        println!("  - Difficulty: {}", result.info.difficulty.as_ref().unwrap_or(&"not specified".to_string()));

        if let Some(time) = result.info.total_time {
            println!("  - Total Time: {} minutes", time);
        }

        if !result.info.allergens.is_empty() {
            println!("  - Allergens: {}", result.info.allergens.join(", "));
        }

        if !result.info.diet_labels.is_empty() {
            println!("  - Diet Labels: {}", result.info.diet_labels.join(", "));
        }

        if !result.errors.is_empty() {
            println!("\n❌ Errors ({}):", result.errors.len());
            for (i, error) in result.errors.iter().take(10).enumerate() {
                println!("  {}. {}", i + 1, error);
            }
            if result.errors.len() > 10 {
                println!("  ... and {} more errors", result.errors.len() - 10);
            }
        }

        if !result.warnings.is_empty() {
            println!("\n⚠️  Warnings ({}):", result.warnings.len());
            for warning in &result.warnings {
                println!("  - {}", warning);
            }
        }

        println!("{}", "=".repeat(60));
    }

    /// Print validation summary
    fn print_summary(&self) {
        println!("\n{}", "=".repeat(60));
        println!("📈 VALIDATION SUMMARY");
        println!("{}", "=".repeat(60));
        println!("Total Validated: {}", self.stats.validated);

        if self.stats.validated > 0 {
            let pass_rate = (self.stats.passed as f64 / self.stats.validated as f64) * 100.0;
            let fail_rate = (self.stats.failed as f64 / self.stats.validated as f64) * 100.0;
            println!("✅ Passed: {} ({:.0}%)", self.stats.passed, pass_rate);
            println!("❌ Failed: {} ({:.0}%)", self.stats.failed, fail_rate);
        }

        println!("{}\n", "=".repeat(60));
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = ValidationStats::default();
    }

    /// Get statistics
    pub fn get_stats(&self) -> &ValidationStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_minimal_recipe() {
        let mut validator = RCIPValidator::new("0.1");

        let recipe = json!({
            "rcip_version": "0.1",
            "id": "rcip-123e4567-e89b-12d3-a456-426614174000",
            "meta": {
                "name": "Test Recipe",
                "author": "Test Author",
                "created_date": "2025-01-15T10:00:00Z"
            },
            "ingredients": [
                {
                    "id": "ing-0001",
                    "name": "test ingredient",
                    "human_amount": "1 piece",
                    "machine_amount": {
                        "value": 1,
                        "unit": "pcs"
                    },
                    "allergens": []
                }
            ],
            "steps": [
                {
                    "step_id": "s-01",
                    "human_text": "Test step",
                    "action": "mix"
                }
            ]
        });

        let result = validator.validate_recipe(&recipe);
        assert!(result.valid, "Minimal recipe should be valid");
    }

    #[test]
    fn test_invalid_recipe_id() {
        let mut validator = RCIPValidator::new("0.1");

        let recipe = json!({
            "rcip_version": "0.1",
            "id": "invalid-id",
            "meta": {
                "name": "Test Recipe",
                "author": "Test Author",
                "created_date": "2025-01-15T10:00:00Z"
            },
            "ingredients": [],
            "steps": []
        });

        let result = validator.validate_recipe(&recipe);
        assert!(!result.valid, "Recipe with invalid ID should not be valid");
        assert!(result.errors.iter().any(|e| e.contains("Invalid recipe ID format")));
    }

    #[test]
    fn test_missing_allergens() {
        let mut validator = RCIPValidator::new("0.1");

        let recipe = json!({
            "rcip_version": "0.1",
            "id": "rcip-123e4567-e89b-12d3-a456-426614174000",
            "meta": {
                "name": "Test Recipe",
                "author": "Test Author",
                "created_date": "2025-01-15T10:00:00Z"
            },
            "ingredients": [
                {
                    "id": "ing-0001",
                    "name": "test ingredient",
                    "human_amount": "1 piece",
                    "machine_amount": {
                        "value": 1,
                        "unit": "pcs"
                    }
                    // Missing allergens field
                }
            ],
            "steps": []
        });

        let result = validator.validate_recipe(&recipe);
        assert!(!result.valid, "Recipe with missing allergens should not be valid");
        assert!(result.errors.iter().any(|e| e.contains("Missing required allergens field")));
    }

    #[test]
    fn test_warnings() {
        let mut validator = RCIPValidator::new("0.1");

        let recipe = json!({
            "rcip_version": "0.1",
            "id": "rcip-123e4567-e89b-12d3-a456-426614174000",
            "meta": {
                "name": "Test Recipe",
                "author": "Test Author",
                "created_date": "2025-01-15T10:00:00Z"
                // Missing recommended fields
            },
            "ingredients": [
                {
                    "id": "ing-0001",
                    "name": "test ingredient",
                    "human_amount": "1 piece",
                    "machine_amount": {
                        "value": 1,
                        "unit": "pcs"
                    },
                    "allergens": []
                }
            ],
            "steps": [
                {
                    "step_id": "s-01",
                    "human_text": "Test step",
                    "action": "mix"
                }
            ]
        });

        let result = validator.validate_recipe(&recipe);
        assert!(!result.warnings.is_empty(), "Should have warnings for missing recommended fields");
        assert!(result.warnings.iter().any(|w| w.contains("meta.description")));
        assert!(result.warnings.iter().any(|w| w.contains("meta.servings")));
    }
}

// CLI binary implementation (src/main.rs)
pub mod cli {
    use super::*;
    use clap::{Arg, Command};
    use std::process;

    pub fn run() {
        let matches = Command::new("RCIP Validator")
            .version("1.0.0")
            .author("Alexey Kozlov")
            .about("Validates RCIP format recipes")
            .arg(
                Arg::new("target")
                    .help("Recipe file or directory to validate")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("version")
                    .short('v')
                    .long("version")
                    .value_name("VERSION")
                    .help("RCIP schema version")
                    .default_value("0.1"),
            )
            .arg(
                Arg::new("schema")
                    .short('s')
                    .long("schema")
                    .value_name("PATH")
                    .help("Path to custom schema file"),
            )
            .get_matches();

        let target = matches.get_one::<String>("target").unwrap();
        let version = matches.get_one::<String>("version").unwrap();
        let schema_path = matches.get_one::<String>("schema").map(|s| Path::new(s));

        let mut validator = RCIPValidator::new(version);

        if let Err(e) = validator.init(schema_path) {
            eprintln!("Error initializing validator: {}", e);
            process::exit(1);
        }

        let target_path = Path::new(target);

        if target_path.is_dir() {
            match validator.validate_directory(target_path) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error validating directory: {}", e);
                    process::exit(1);
                }
            }
        } else if target_path.is_file() {
            match validator.validate_file(target_path) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error validating file: {}", e);
                    process::exit(1);
                }
            }
        } else {
            eprintln!("Error: {} is not a valid file or directory", target);
            process::exit(1);
        }
    }
}
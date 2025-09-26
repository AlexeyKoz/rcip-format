#!/usr/bin/env python3
"""
RCIP Format Validator for Python
Created by Alexey Kozlov
Version: 1.0.0
License: MIT
"""

import json
import re
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
import jsonschema
from jsonschema import validate, ValidationError, Draft7Validator


class DietLabel(Enum):
    """Valid diet labels"""
    VEGETARIAN = "vegetarian"
    VEGAN = "vegan"
    GLUTEN_FREE = "gluten-free"
    DAIRY_FREE = "dairy-free"
    NUT_FREE = "nut-free"
    EGG_FREE = "egg-free"
    SOY_FREE = "soy-free"
    FISH_FREE = "fish-free"
    SHELLFISH_FREE = "shellfish-free"
    KOSHER = "kosher"
    HALAL = "halal"
    LOW_SODIUM = "low-sodium"
    LOW_CARB = "low-carb"
    KETO = "keto"
    PALEO = "paleo"


class Allergen(Enum):
    """Valid allergens"""
    MILK = "milk"
    EGGS = "eggs"
    FISH = "fish"
    SHELLFISH = "shellfish"
    TREE_NUTS = "tree-nuts"
    PEANUTS = "peanuts"
    WHEAT = "wheat"
    GLUTEN = "gluten"
    SOYBEANS = "soybeans"
    SESAME = "sesame"
    CELERY = "celery"
    MUSTARD = "mustard"
    MOLLUSCS = "molluscs"
    LUPINS = "lupins"
    SULPHITES = "sulphites"
    LACTOSE = "lactose"


class CookingAction(Enum):
    """Valid cooking actions"""
    ADD = "add"
    MIX = "mix"
    COMBINE = "combine"
    BLEND = "blend"
    CUT = "cut"
    SLICE = "slice"
    DICE = "dice"
    CHOP = "chop"
    MINCE = "mince"
    HEAT = "heat"
    BOIL = "boil"
    SIMMER = "simmer"
    STEAM = "steam"
    FRY = "fry"
    SAUTE = "saute"
    BAKE = "bake"
    ROAST = "roast"
    GRILL = "grill"
    COOL = "cool"
    CHILL = "chill"
    FREEZE = "freeze"
    KNEAD = "knead"
    FOLD = "fold"
    ROLL = "roll"
    SHAPE = "shape"
    FERMENT = "ferment"
    PROOF = "proof"
    REST = "rest"
    STRAIN = "strain"
    FILTER = "filter"
    SEPARATE = "separate"
    MEASURE = "measure"
    WEIGH = "weigh"
    WAIT = "wait"
    DISSOLVE = "dissolve"
    PREPARE = "prepare"
    SPREAD = "spread"
    GARNISH = "garnish"
    DIVIDE = "divide"


class Unit(Enum):
    """Valid measurement units"""
    # Mass
    MG = "mg"
    G = "g"
    KG = "kg"
    OZ = "oz"
    LB = "lb"
    # Volume
    ML = "ml"
    L = "l"
    TSP = "tsp"
    TBSP = "tbsp"
    CUP = "cup"
    FL_OZ = "fl-oz"
    PT = "pt"
    QT = "qt"
    GAL = "gal"
    # Count
    PCS = "pcs"
    DOZEN = "dozen"
    # Special
    PINCH = "pinch"
    DASH = "dash"
    HANDFUL = "handful"
    TO_TASTE = "to-taste"


@dataclass
class ValidationResult:
    """Validation result structure"""
    valid: bool
    errors: List[str] = field(default_factory=list)
    warnings: List[str] = field(default_factory=list)
    info: Dict[str, Any] = field(default_factory=dict)


@dataclass
class ValidationStats:
    """Validation statistics"""
    validated: int = 0
    passed: int = 0
    failed: int = 0


class RCIPValidator:
    """RCIP Format Validator"""

    RECIPE_ID_PATTERN = re.compile(r'^rcip-[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$')
    INGREDIENT_ID_PATTERN = re.compile(r'^ing-[0-9a-zA-Z]+$')
    STEP_ID_PATTERN = re.compile(r'^s-[0-9a-zA-Z]+$')
    VERSION_PATTERN = re.compile(r'^\d+\.\d+\.\d+$')
    COUNTRY_CODE_PATTERN = re.compile(r'^[A-Z]{2}$')

    def __init__(self, schema_version: str = "0.1"):
        """Initialize validator with schema version"""
        self.schema_version = schema_version
        self.schema = None
        self.validator = None
        self.stats = ValidationStats()

    def init(self, schema_path: Optional[Path] = None) -> None:
        """
        Initialize validator with schema

        Args:
            schema_path: Path to JSON schema file
        """
        if schema_path is None:
            schema_path = Path(__file__).parent.parent.parent / "schemas" / f"rcip-v{self.schema_version}.json"

        try:
            with open(schema_path, 'r', encoding='utf-8') as f:
                self.schema = json.load(f)

            self.validator = Draft7Validator(self.schema)
            print(f"✅ RCIP Validator initialized with schema v{self.schema_version}")
        except FileNotFoundError:
            raise FileNotFoundError(f"Schema file not found: {schema_path}")
        except json.JSONDecodeError as e:
            raise ValueError(f"Invalid JSON in schema file: {e}")

    def validate_recipe(self, recipe: Dict[str, Any]) -> ValidationResult:
        """
        Validate a recipe object

        Args:
            recipe: Recipe dictionary to validate

        Returns:
            ValidationResult object
        """
        if self.validator is None:
            raise RuntimeError("Validator not initialized. Call init() first.")

        result = ValidationResult(valid=True)

        # JSON Schema validation
        try:
            self.validator.validate(recipe)
        except ValidationError as e:
            result.valid = False
            result.errors.append(f"Schema validation failed: {e.message}")
            # Collect all errors
            for error in self.validator.iter_errors(recipe):
                error_path = " > ".join(str(p) for p in error.path)
                result.errors.append(f"{error_path}: {error.message}")

        # Custom validations
        self._validate_custom_rules(recipe, result)

        # Check warnings
        result.warnings = self._check_warnings(recipe)

        # Get recipe info
        result.info = self._get_recipe_info(recipe)

        # Update stats
        self.stats.validated += 1
        if result.valid:
            self.stats.passed += 1
        else:
            self.stats.failed += 1

        return result

    def validate_file(self, file_path: Path) -> ValidationResult:
        """
        Validate a recipe file

        Args:
            file_path: Path to recipe file

        Returns:
            ValidationResult object
        """
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                recipe = json.load(f)

            print(f"\n📄 Validating: {file_path.name}")
            result = self.validate_recipe(recipe)

            self._print_result(result, recipe.get('meta', {}).get('name', 'Unknown Recipe'))
            return result

        except FileNotFoundError:
            return ValidationResult(
                valid=False,
                errors=[f"File not found: {file_path}"]
            )
        except json.JSONDecodeError as e:
            return ValidationResult(
                valid=False,
                errors=[f"Invalid JSON: {e}"]
            )

    def validate_directory(self, directory: Path) -> List[Dict[str, Any]]:
        """
        Validate all recipe files in a directory

        Args:
            directory: Directory path

        Returns:
            List of validation results
        """
        recipe_files = list(directory.glob("*.rcip")) + list(directory.glob("*.json"))
        print(f"\n🔍 Found {len(recipe_files)} recipe files to validate\n")

        results = []
        for file_path in recipe_files:
            result = self.validate_file(file_path)
            results.append({
                "file": file_path.name,
                "result": result
            })

        self._print_summary()
        return results

    def _validate_custom_rules(self, recipe: Dict[str, Any], result: ValidationResult) -> None:
        """Apply custom validation rules beyond JSON schema"""

        # Validate recipe ID format
        if 'id' in recipe and not self.RECIPE_ID_PATTERN.match(recipe['id']):
            result.valid = False
            result.errors.append(f"Invalid recipe ID format: {recipe['id']}")

        # Validate ingredients
        for i, ingredient in enumerate(recipe.get('ingredients', [])):
            self._validate_ingredient(ingredient, i, result)

        # Validate steps
        for i, step in enumerate(recipe.get('steps', [])):
            self._validate_step(step, i, result)

        # Validate cross-references
        self._validate_references(recipe, result)

        # Validate version compatibility
        if 'rcip_version' in recipe and recipe['rcip_version'] != self.schema_version:
            result.warnings.append(
                f"Recipe version {recipe['rcip_version']} may not be fully compatible with validator version {self.schema_version}")

    def _validate_ingredient(self, ingredient: Dict[str, Any], index: int, result: ValidationResult) -> None:
        """Validate an ingredient"""

        # Check ID format
        if 'id' in ingredient and not self.INGREDIENT_ID_PATTERN.match(ingredient['id']):
            result.valid = False
            result.errors.append(f"Ingredient {index}: Invalid ID format: {ingredient['id']}")

        # Check allergens (must be present, can be empty)
        if 'allergens' not in ingredient:
            result.valid = False
            result.errors.append(f"Ingredient {index}: Missing required allergens field")
        elif not isinstance(ingredient['allergens'], list):
            result.valid = False
            result.errors.append(f"Ingredient {index}: allergens must be an array")
        else:
            # Validate allergen values
            valid_allergens = [a.value for a in Allergen]
            for allergen in ingredient['allergens']:
                if allergen not in valid_allergens:
                    result.errors.append(f"Ingredient {index}: Invalid allergen '{allergen}'")

        # Check machine amount
        if 'machine_amount' in ingredient:
            ma = ingredient['machine_amount']
            if 'value' not in ma or not isinstance(ma['value'], (int, float)) or ma['value'] < 0:
                result.errors.append(f"Ingredient {index}: machine_amount.value must be non-negative number")
            if 'unit' not in ma:
                result.errors.append(f"Ingredient {index}: machine_amount.unit is required")
            elif ma['unit'] not in [u.value for u in Unit]:
                result.warnings.append(f"Ingredient {index}: Non-standard unit '{ma['unit']}'")

    def _validate_step(self, step: Dict[str, Any], index: int, result: ValidationResult) -> None:
        """Validate a step"""

        # Check ID format
        if 'step_id' in step and not self.STEP_ID_PATTERN.match(step['step_id']):
            result.valid = False
            result.errors.append(f"Step {index}: Invalid ID format: {step['step_id']}")

        # Check action
        if 'action' in step:
            valid_actions = [a.value for a in CookingAction]
            if step['action'] not in valid_actions:
                result.errors.append(f"Step {index}: Invalid action '{step['action']}'")

        # Check hazards
        if 'hazards' in step and isinstance(step['hazards'], list):
            valid_hazards = ['hot-surface', 'sharp-tool', 'electrical', 'chemical', 'pressure',
                             'allergen-cross-contact']
            for hazard in step['hazards']:
                if hazard not in valid_hazards:
                    result.warnings.append(f"Step {index}: Non-standard hazard '{hazard}'")

    def _validate_references(self, recipe: Dict[str, Any], result: ValidationResult) -> None:
        """Validate cross-references between steps and ingredients"""

        ingredient_ids = {ing['id'] for ing in recipe.get('ingredients', []) if 'id' in ing}
        step_ids = {step['step_id'] for step in recipe.get('steps', []) if 'step_id' in step}

        # Check step targets
        for step in recipe.get('steps', []):
            if 'target' in step and isinstance(step['target'], list):
                for target in step['target']:
                    if isinstance(target, str):
                        if target.startswith('ing-') and target not in ingredient_ids:
                            result.errors.append(
                                f"Step {step.get('step_id', '?')}: Invalid ingredient reference '{target}'")
                        elif ':result' in target:
                            step_ref = target.split(':')[0]
                            if step_ref not in step_ids:
                                result.errors.append(
                                    f"Step {step.get('step_id', '?')}: Invalid step reference '{target}'")

        # Check device profile references
        device_ids = {dp['id'] for dp in recipe.get('device_profiles', []) if 'id' in dp}
        for step in recipe.get('steps', []):
            if 'device_profile_ref' in step and step['device_profile_ref'] not in device_ids:
                result.warnings.append(
                    f"Step {step.get('step_id', '?')}: Unknown device profile '{step['device_profile_ref']}'")

    def _check_warnings(self, recipe: Dict[str, Any]) -> List[str]:
        """Check for warnings (non-critical issues)"""
        warnings = []

        meta = recipe.get('meta', {})

        # Check for missing optional but recommended fields
        if 'description' not in meta:
            warnings.append("Missing recommended field: meta.description")

        if 'servings' not in meta:
            warnings.append("Missing recommended field: meta.servings")

        if 'difficulty' not in meta:
            warnings.append("Missing recommended field: meta.difficulty")

        # Check for missing nutritional data
        has_nutritional = any(
            'nutritional' in ing
            for ing in recipe.get('ingredients', [])
        )
        if not has_nutritional:
            warnings.append("No nutritional data provided for any ingredient")

        # Check for missing external IDs
        has_external_ids = any(
            ing.get('external_ids', {})
            for ing in recipe.get('ingredients', [])
        )
        if not has_external_ids:
            warnings.append("No external IDs (USDA, GTIN, etc.) provided")

        # Check for device profiles without sensors
        if recipe.get('device_profiles') and not recipe.get('sensors'):
            warnings.append("Device profiles defined but no sensors specified")

        # Check for very long cooking times
        total_time = meta.get('total_time_minutes', 0)
        if total_time > 1440:  # 24 hours
            warnings.append(f"Very long cooking time ({total_time} min / {total_time / 60:.1f} hours)")

        # Check for missing images
        if not recipe.get('images'):
            warnings.append("No images provided for recipe")

        return warnings

    def _get_recipe_info(self, recipe: Dict[str, Any]) -> Dict[str, Any]:
        """Get recipe information summary"""
        meta = recipe.get('meta', {})

        # Get all allergens
        allergens = set()
        for ingredient in recipe.get('ingredients', []):
            for allergen in ingredient.get('allergens', []):
                allergens.add(allergen)

        return {
            'name': meta.get('name', 'Unknown'),
            'version': recipe.get('rcip_version'),
            'recipe_version': meta.get('version'),
            'ingredient_count': len(recipe.get('ingredients', [])),
            'step_count': len(recipe.get('steps', [])),
            'has_device_profiles': bool(recipe.get('device_profiles')),
            'has_sensors': bool(recipe.get('sensors')),
            'allergens': sorted(list(allergens)),
            'diet_labels': meta.get('diet_labels', []),
            'difficulty': meta.get('difficulty', 'not specified'),
            'total_time': meta.get('total_time_minutes', 'not specified')
        }

    def _print_result(self, result: ValidationResult, recipe_name: str) -> None:
        """Print validation result"""
        print("\n" + "=" * 60)
        print(f"Recipe: {recipe_name}")
        print(f"Status: {'✅ VALID' if result.valid else '❌ INVALID'}")

        if result.info:
            print("\n📊 Recipe Info:")
            print(f"  - RCIP Version: {result.info['version']}")
            print(f"  - Ingredients: {result.info['ingredient_count']}")
            print(f"  - Steps: {result.info['step_count']}")
            print(f"  - Difficulty: {result.info['difficulty']}")
            print(f"  - Total Time: {result.info['total_time']} minutes")

            if result.info['allergens']:
                print(f"  - Allergens: {', '.join(result.info['allergens'])}")

            if result.info['diet_labels']:
                print(f"  - Diet Labels: {', '.join(result.info['diet_labels'])}")

        if result.errors:
            print(f"\n❌ Errors ({len(result.errors)}):")
            for i, error in enumerate(result.errors[:10], 1):
                print(f"  {i}. {error}")
            if len(result.errors) > 10:
                print(f"  ... and {len(result.errors) - 10} more errors")

        if result.warnings:
            print(f"\n⚠️  Warnings ({len(result.warnings)}):")
            for warning in result.warnings:
                print(f"  - {warning}")

        print("=" * 60)

    def _print_summary(self) -> None:
        """Print validation summary"""
        print("\n" + "=" * 60)
        print("📈 VALIDATION SUMMARY")
        print("=" * 60)
        print(f"Total Validated: {self.stats.validated}")

        if self.stats.validated > 0:
            pass_rate = (self.stats.passed / self.stats.validated) * 100
            fail_rate = (self.stats.failed / self.stats.validated) * 100
            print(f"✅ Passed: {self.stats.passed} ({pass_rate:.0f}%)")
            print(f"❌ Failed: {self.stats.failed} ({fail_rate:.0f}%)")

        print("=" * 60 + "\n")

    def reset_stats(self) -> None:
        """Reset validation statistics"""
        self.stats = ValidationStats()


def main():
    """CLI interface"""
    import argparse

    parser = argparse.ArgumentParser(
        description='RCIP Format Validator',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python rcip_validator.py recipe.rcip
  python rcip_validator.py recipes/
  python rcip_validator.py --version 0.2 recipe.json
        """
    )

    parser.add_argument(
        'target',
        type=Path,
        help='Recipe file or directory to validate'
    )

    parser.add_argument(
        '--version',
        default='0.1',
        help='RCIP schema version (default: 0.1)'
    )

    parser.add_argument(
        '--schema',
        type=Path,
        help='Path to custom schema file'
    )

    args = parser.parse_args()

    # Initialize validator
    validator = RCIPValidator(schema_version=args.version)

    try:
        validator.init(schema_path=args.schema)

        if args.target.is_dir():
            validator.validate_directory(args.target)
        elif args.target.is_file():
            validator.validate_file(args.target)
        else:
            print(f"Error: {args.target} is not a valid file or directory")
            sys.exit(1)

    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
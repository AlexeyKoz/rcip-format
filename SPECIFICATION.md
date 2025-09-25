# RCIP (Recipe Interchange Protocol) Format Specification v0.1

**Created by:** Alexey Kozlov  
**Date:** January 2025  
**Status:** Draft Standard  
**License:** MIT License  

## Table of Contents

1. [Introduction](#introduction)
2. [Design Principles](#design-principles)
3. [Format Structure](#format-structure)
4. [Data Types](#data-types)
5. [Core Fields](#core-fields)
6. [Ingredient Specification](#ingredient-specification)
7. [Step Specification](#step-specification)
8. [Device Profiles](#device-profiles)
9. [Sensor Specification](#sensor-specification)
10. [Versioning](#versioning)
11. [File Extensions](#file-extensions)
12. [MIME Types](#mime-types)
13. [Examples](#examples)

## Introduction

The Recipe Interchange Protocol (RCIP) is a comprehensive, machine-readable, and human-friendly format for storing and exchanging culinary recipes. RCIP is designed to bridge traditional cooking with modern smart kitchen technologies, IoT devices, and automated food preparation systems.

### Goals

- **Interoperability**: Work across different platforms, devices, and applications
- **Precision**: Support both human-friendly and machine-precise measurements
- **Safety**: Mandatory allergen and dietary information
- **Evolution**: Built-in versioning and backward compatibility
- **Automation**: Native support for robotic and IoT kitchen devices

## Design Principles

1. **Dual Readability**: Every measurement MUST be both human-readable and machine-parseable
2. **Safety First**: Allergen information is MANDATORY for all ingredients
3. **Version Control**: All fields support `since_version` and `deprecated_in` attributes
4. **Extensibility**: Unknown fields MUST be preserved for forward compatibility
5. **Validation**: Recipes MUST be validatable against JSON Schema
6. **Internationalization**: UTF-8 encoding, ISO standards for units and dates

## Format Structure

### Top-Level Structure

```json
{
  "rcip_version": "0.1",
  "id": "rcip-{UUID}",
  "meta": { },
  "ingredients": [ ],
  "steps": [ ],
  "device_profiles": [ ],
  "sensors": [ ],
  "images": [ ],
  "compatibility": { },
  "extensions": { }
}
```

### Required Fields

- `rcip_version` (string): Format version
- `id` (string): Unique identifier following pattern `rcip-{UUID-v4}`
- `meta` (object): Recipe metadata
- `ingredients` (array): List of ingredients
- `steps` (array): Preparation instructions

### Optional Fields

- `device_profiles` (array): Device-specific parameters
- `sensors` (array): Sensor-based quality control
- `images` (array): Visual content
- `compatibility` (object): Format compatibility information
- `extensions` (object): Custom extensions

## Data Types

### Measurement Object

```json
{
  "value": 250,
  "unit": "g",
  "approximate": false,
  "tolerance": {
    "min": 245,
    "max": 255
  }
}
```

### Temperature Object

```json
{
  "value": 180,
  "unit": "C",
  "surface": "top",
  "method": "convection"
}
```

### Time Duration

Time is expressed in ISO 8601 duration format or simplified units:
- `"PT30M"` - 30 minutes (ISO 8601)
- `{ "minutes": 30 }` - Object notation
- `1800` - Seconds (deprecated in v0.2)

## Core Fields

### Meta Object

```json
{
  "meta": {
    "name": "Classic Margherita Pizza",
    "description": "Traditional Neapolitan pizza",
    "author": {
      "name": "Chef Name",
      "email": "email@example.com",
      "organization": "Organization Name"
    },
    "origin": {
      "country": "IT",
      "region": "Campania",
      "cuisine_type": "Italian",
      "traditional": true
    },
    "servings": {
      "amount": 4,
      "unit": "portions",
      "adjustable": true
    },
    "diet_labels": ["vegetarian", "nut-free"],
    "keywords": ["pizza", "italian", "traditional"],
    "difficulty": "intermediate",
    "active_time_minutes": 45,
    "total_time_minutes": 1500,
    "created_date": "2025-01-15T10:30:00Z",
    "updated_date": "2025-01-15T10:30:00Z",
    "version": "1.0.0",
    "license": "CC-BY-SA-4.0"
  }
}
```

#### Required Meta Fields
- `name` (string): Recipe name
- `author` (object|string): Author information
- `created_date` (ISO 8601): Creation timestamp

#### Optional Meta Fields
- `description` (string): Brief description
- `origin` (object): Cultural/geographic origin
- `servings` (object): Serving information
- `diet_labels` (array): Dietary classifications
- `keywords` (array): Searchable tags
- `difficulty` (enum): `beginner`, `intermediate`, `advanced`, `professional`
- Times: `prep_time_minutes`, `cook_time_minutes`, `total_time_minutes`

### Diet Labels (Standardized)

Allowed values:
- `vegetarian`
- `vegan`
- `gluten-free`
- `dairy-free`
- `nut-free`
- `egg-free`
- `soy-free`
- `fish-free`
- `shellfish-free`
- `kosher`
- `halal`
- `low-sodium`
- `low-carb`
- `keto`
- `paleo`

## Ingredient Specification

### Ingredient Entry Structure

```json
{
  "id": "ing-0001",
  "name": "00 flour",
  "human_amount": "500g",
  "machine_amount": {
    "value": 500,
    "unit": "g",
    "approximate": false
  },
  "state": "fine-milled",
  "temperature_c": {
    "min": 20,
    "max": 22
  },
  "external_ids": {
    "GTIN": "8001810000123",
    "USDA": "20081",
    "OpenFoodFacts": "3017620422003"
  },
  "nutritional": {
    "per_100g": {
      "calories": 361,
      "protein": 11.0,
      "carbs": 72.5,
      "fat": 1.5,
      "fiber": 2.7
    }
  },
  "substitutes": [
    {
      "id": "sub-0001",
      "name": "bread flour",
      "ratio": 1.0,
      "notes": "Higher protein content"
    }
  ],
  "allergens": ["gluten", "wheat"],
  "notes": "Caputo or similar Italian 00 flour recommended",
  "since_version": "0.1"
}
```

### Required Ingredient Fields

- `id` (string): Unique identifier (pattern: `ing-XXXX`)
- `name` (string): Ingredient name
- `human_amount` (string): Human-readable quantity
- `machine_amount` (object): Machine-readable measurement
- `allergens` (array): MUST be present, can be empty array

### Optional Ingredient Fields

- `state` (string): Physical state or preparation
- `brand` (string): Preferred brand
- `external_ids` (object): Database references
- `nutritional` (object): Nutritional information
- `substitutes` (array): Alternative ingredients
- `temperature_c` (object): Temperature requirements
- `notes` (string): Additional information

### Standardized Allergens

Required allergen identifiers (based on FDA/EU regulations):
- `milk` (includes lactose)
- `eggs`
- `fish`
- `shellfish`
- `tree-nuts`
- `peanuts`
- `wheat`
- `gluten`
- `soybeans`
- `sesame`
- `celery`
- `mustard`
- `molluscs`
- `lupins`
- `sulphites`

### Standardized Units

#### Mass
- `mg`, `g`, `kg` (metric)
- `oz`, `lb` (imperial)

#### Volume
- `ml`, `l` (metric)
- `tsp`, `tbsp`, `cup`, `fl-oz`, `pt`, `qt`, `gal` (imperial/US)

#### Count
- `pcs`, `dozen`

#### Special
- `pinch`, `dash`, `handful`, `to-taste`

## Step Specification

### Step Entry Structure

```json
{
  "step_id": "s-01",
  "human_text": "Knead dough for 10 minutes until smooth",
  "action": "knead",
  "target": ["ing-0001", "s-01:result"],
  "params": {
    "time_minutes": 10,
    "method": "hand-kneading",
    "surface": "floured",
    "speed_rpm": 60,
    "force_n": 20
  },
  "device_profile_ref": "mixer-01",
  "done_when": {
    "texture": "smooth-elastic",
    "windowpane_test": "pass",
    "temperature_c": 24
  },
  "tolerance": {
    "time_percent": 20,
    "temperature_c": 2
  },
  "hazards": ["hot-surface"],
  "since_version": "0.1"
}
```

### Required Step Fields

- `step_id` (string): Unique identifier (pattern: `s-XX`)
- `human_text` (string): Human-readable instruction
- `action` (string): Canonical action verb

### Optional Step Fields

- `target` (array): Ingredient IDs or previous step results
- `params` (object): Structured parameters
- `device_profile_ref` (string): Reference to device profile
- `done_when` (object): Completion criteria
- `tolerance` (object): Acceptable parameter variations
- `hazards` (array): Safety warnings

### Canonical Action Verbs

Primary actions (MUST be supported):
- `add`, `mix`, `combine`, `blend`
- `cut`, `slice`, `dice`, `chop`, `mince`
- `heat`, `boil`, `simmer`, `steam`, `fry`, `saute`, `bake`, `roast`, `grill`
- `cool`, `chill`, `freeze`
- `knead`, `fold`, `roll`, `shape`
- `ferment`, `proof`, `rest`
- `strain`, `filter`, `separate`
- `measure`, `weigh`
- `wait`, `rest`

## Device Profiles

### Device Profile Structure

```json
{
  "id": "oven-01",
  "type": "oven",
  "name": "Convection Oven",
  "manufacturer": "BoschKitchen",
  "model": "HBG6764S6B",
  "params": {
    "temperature_c": 180,
    "mode": "convection",
    "fan_speed": "medium",
    "preheat_time_minutes": 15,
    "rack_position": "middle"
  },
  "capabilities": {
    "temperature_range_c": { "min": 30, "max": 300 },
    "modes": ["convection", "conventional", "grill", "steam"],
    "capacity_liters": 71
  },
  "api_endpoint": "https://api.device.com/control",
  "since_version": "0.1"
}
```

### Standard Device Types

- `oven` - Conventional/convection ovens
- `stovetop` - Cooktops and ranges
- `mixer` - Stand mixers, hand mixers
- `blender` - Blenders, food processors
- `scale` - Digital scales
- `thermometer` - Temperature sensors
- `timer` - Timing devices
- `cutter` - Automated cutting devices
- `printer_3d_food` - 3D food printers
- `dispenser` - Ingredient dispensers

## Sensor Specification

### Sensor Entry Structure

```json
{
  "id": "sensor-01",
  "type": "color",
  "target": "crust",
  "spec": {
    "rgb": {
      "r": [180, 220],
      "g": [130, 170],
      "b": [80, 120]
    },
    "hsv": {
      "h": [25, 35],
      "s": [60, 80],
      "v": [40, 60]
    },
    "calibration": "D65-illuminant"
  },
  "priority": "required",
  "since_version": "0.1"
}
```

### Sensor Types

- `color` - RGB/HSV color detection
- `temperature` - Temperature monitoring
- `moisture` - Humidity/moisture content
- `texture` - Hardness, elasticity, crispness
- `aroma` - VOC/chemical composition
- `weight` - Mass measurement
- `volume` - Volume measurement
- `ph` - Acidity/alkalinity
- `conductivity` - Electrical conductivity
- `viscosity` - Fluid thickness

## Versioning

### Version Format

Semantic versioning: `MAJOR.MINOR.PATCH`

- MAJOR: Breaking changes
- MINOR: New features, backward compatible
- PATCH: Bug fixes

### Field Versioning

```json
{
  "field_name": "value",
  "since_version": "0.1",
  "deprecated_in": "0.3",
  "removed_in": "1.0",
  "migration_note": "Use 'new_field_name' instead"
}
```

### Backward Compatibility Rules

1. Fields marked `deprecated_in` MUST be supported for 2 major versions
2. Unknown fields MUST be preserved when processing
3. Version downgrade MUST warn about data loss
4. Format version MUST be checked before processing

## File Extensions

### Primary Extensions

- `.rcip` - Standard JSON format (recommended)
- `.rcipz` - Compressed with Brotli
- `.rcipx` - XML representation
- `.rcipy` - YAML representation

### Archive Formats

- `.rcipa` - Archive with media files
- `.rcipk` - Encrypted recipe format

## MIME Types

### Registered MIME Types

- `application/vnd.rcip+json` - JSON format
- `application/vnd.rcip+xml` - XML format
- `application/vnd.rcip+yaml` - YAML format
- `application/vnd.rcip+cbor` - CBOR binary format

### Content-Type Headers

```http
Content-Type: application/vnd.rcip+json; charset=utf-8; version=0.1
```

## Examples

### Minimal Valid Recipe

```json
{
  "rcip_version": "0.1",
  "id": "rcip-123e4567-e89b-12d3-a456-426614174000",
  "meta": {
    "name": "Boiled Egg",
    "author": "Anonymous",
    "created_date": "2025-01-15T10:00:00Z"
  },
  "ingredients": [
    {
      "id": "ing-0001",
      "name": "egg",
      "human_amount": "1 piece",
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
      "human_text": "Boil egg for 10 minutes",
      "action": "boil"
    }
  ]
}
```

### Complex Recipe Fragment

```json
{
  "rcip_version": "0.1",
  "id": "rcip-advanced-001",
  "meta": {
    "name": "Sous Vide Steak with Sensor Control",
    "difficulty": "advanced"
  },
  "ingredients": [
    {
      "id": "ing-0001",
      "name": "ribeye steak",
      "human_amount": "250g",
      "machine_amount": {
        "value": 250,
        "unit": "g",
        "tolerance": {
          "min": 240,
          "max": 260
        }
      },
      "allergens": [],
      "temperature_c": {
        "min": 2,
        "max": 4
      }
    }
  ],
  "steps": [
    {
      "step_id": "s-01",
      "human_text": "Vacuum seal and cook sous vide",
      "action": "heat",
      "params": {
        "temperature_c": 55,
        "time_minutes": 120,
        "method": "sous-vide"
      },
      "done_when": {
        "internal_temp_c": 54,
        "color": {
          "rgb": {
            "r": [140, 160],
            "g": [40, 60],
            "b": [50, 70]
          }
        }
      },
      "device_profile_ref": "sous-vide-01"
    }
  ],
  "device_profiles": [
    {
      "id": "sous-vide-01",
      "type": "immersion_circulator",
      "params": {
        "temperature_c": 55,
        "precision_c": 0.1,
        "circulation_lpm": 8
      }
    }
  ],
  "sensors": [
    {
      "id": "sensor-01",
      "type": "temperature",
      "target": "core",
      "spec": {
        "range_c": [54, 56],
        "precision": 0.1
      }
    }
  ]
}
```

## Validation

### JSON Schema

Recipes MUST validate against the official JSON Schema available at:
`https://github.com/alexey-kozlov/rcip-format/schemas/rcip-v0.1.json`

### Validation Rules

1. All required fields MUST be present
2. IDs MUST be unique within their scope
3. References MUST point to existing elements
4. Allergen arrays MUST be present (can be empty)
5. Measurements MUST use standardized units
6. Dates MUST be valid ISO 8601
7. Version strings MUST follow semantic versioning

## Security Considerations

### Data Integrity

- Recipes MAY include digital signatures
- Hash verification using SHA-256
- Optional encryption for proprietary recipes

### Privacy

- Personal information in `author` field is optional
- Email addresses SHOULD be validated if included
- Location data SHOULD be generalized (country/region only)

### Safety

- Allergen information MUST NOT be removed
- Temperature values MUST include units
- Hazard warnings MUST be preserved

## Implementation Notes

### Parser Requirements

1. MUST support UTF-8 encoding
2. MUST preserve field order for digital signatures
3. MUST validate against schema
4. SHOULD support streaming for large files
5. SHOULD implement progress callbacks

### Best Practices

1. Always validate before processing
2. Preserve unknown fields for forward compatibility
3. Log version mismatches
4. Implement graceful degradation for missing features
5. Cache external ID lookups
6. Support offline mode

## Future Considerations (v0.2+)

- Blockchain integration for authenticity
- AI-generated instructions
- Nutritional optimization algorithms
- Multi-recipe meal planning
- Supply chain integration
- Real-time collaboration features

## References

- ISO 8601: Date and time format
- ISO 3166: Country codes
- FDA Food Code: Allergen specifications
- EU Regulation 1169/2011: Food information
- JSON Schema Draft 2020-12
- Semantic Versioning 2.0.0

## Changelog

### Version 0.1 (January 2025)
- Initial specification release
- Core recipe structure
- Basic device profiles
- Sensor specifications
- Allergen requirements

---

**Copyright © 2025 Alexey Kozlov. Released under MIT License.**

For questions and contributions: https://github.com/AlexeyKoz/rcip-format

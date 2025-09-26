# RCIP API Reference Documentation

## Table of Contents
1. [Overview](#overview)
2. [Data Types](#data-types)
3. [Core Objects](#core-objects)
4. [Validators](#validators)
5. [Converters](#converters)
6. [Device Control](#device-control)
7. [Sensor Interface](#sensor-interface)
8. [REST API](#rest-api)
9. [WebSocket API](#websocket-api)
10. [Error Codes](#error-codes)

## Overview

The RCIP API provides programmatic access to recipe data, validation, conversion, and device control functionality. All APIs support the current RCIP version 0.1 specification.

### Authentication

```http
Authorization: Bearer <api_token>
X-RCIP-Version: 0.1
```

### Base URLs

- Production: `https://api.rcip-format.org/v1`
- Sandbox: `https://sandbox-api.rcip-format.org/v1`

### Rate Limiting

- Standard: 1000 requests/hour
- Premium: 10000 requests/hour
- Enterprise: Unlimited

## Data Types

### Primitive Types

| Type | Description | Example |
|------|-------------|---------|
| `RecipeId` | UUID v4 with rcip prefix | `"rcip-550e8400-e29b-41d4-a716-446655440000"` |
| `IngredientId` | Ingredient identifier | `"ing-0001"` |
| `StepId` | Step identifier | `"s-01"` |
| `Temperature` | Temperature with unit | `{"value": 180, "unit": "C"}` |
| `Measurement` | Quantity with unit | `{"value": 500, "unit": "g"}` |
| `Duration` | ISO 8601 duration | `"PT30M"` |
| `DateTime` | ISO 8601 timestamp | `"2025-01-15T10:30:00Z"` |

### Enumerations

```typescript
enum DietLabel {
  VEGETARIAN = "vegetarian",
  VEGAN = "vegan",
  GLUTEN_FREE = "gluten-free",
  DAIRY_FREE = "dairy-free",
  NUT_FREE = "nut-free",
  EGG_FREE = "egg-free",
  SOY_FREE = "soy-free",
  FISH_FREE = "fish-free",
  SHELLFISH_FREE = "shellfish-free",
  KOSHER = "kosher",
  HALAL = "halal",
  LOW_SODIUM = "low-sodium",
  LOW_CARB = "low-carb",
  KETO = "keto",
  PALEO = "paleo"
}

enum Allergen {
  MILK = "milk",
  EGGS = "eggs",
  FISH = "fish",
  SHELLFISH = "shellfish",
  TREE_NUTS = "tree-nuts",
  PEANUTS = "peanuts",
  WHEAT = "wheat",
  GLUTEN = "gluten",
  SOYBEANS = "soybeans",
  SESAME = "sesame",
  CELERY = "celery",
  MUSTARD = "mustard",
  MOLLUSCS = "molluscs",
  LUPINS = "lupins",
  SULPHITES = "sulphites",
  LACTOSE = "lactose"
}

enum CookingAction {
  ADD = "add",
  MIX = "mix",
  COMBINE = "combine",
  BLEND = "blend",
  CUT = "cut",
  SLICE = "slice",
  DICE = "dice",
  CHOP = "chop",
  MINCE = "mince",
  HEAT = "heat",
  BOIL = "boil",
  SIMMER = "simmer",
  STEAM = "steam",
  FRY = "fry",
  SAUTE = "saute",
  BAKE = "bake",
  ROAST = "roast",
  GRILL = "grill",
  COOL = "cool",
  CHILL = "chill",
  FREEZE = "freeze",
  KNEAD = "knead",
  FOLD = "fold",
  ROLL = "roll",
  SHAPE = "shape",
  FERMENT = "ferment",
  PROOF = "proof",
  REST = "rest",
  STRAIN = "strain",
  FILTER = "filter",
  SEPARATE = "separate",
  MEASURE = "measure",
  WEIGH = "weigh",
  WAIT = "wait",
  DISSOLVE = "dissolve",
  PREPARE = "prepare",
  SPREAD = "spread",
  GARNISH = "garnish",
  DIVIDE = "divide"
}

enum Unit {
  // Mass
  MG = "mg", G = "g", KG = "kg",
  // Volume
  ML = "ml", L = "l",
  // Imperial
  TSP = "tsp", TBSP = "tbsp", CUP = "cup",
  FL_OZ = "fl-oz", PT = "pt", QT = "qt", GAL = "gal",
  // Weight
  OZ = "oz", LB = "lb",
  // Count
  PCS = "pcs", DOZEN = "dozen",
  // Special
  PINCH = "pinch", DASH = "dash",
  HANDFUL = "handful", TO_TASTE = "to-taste"
}
```

## Core Objects

### Recipe Object

```typescript
interface Recipe {
  rcip_version: "0.1";
  id: RecipeId;
  meta: RecipeMeta;
  ingredients: Ingredient[];
  steps: Step[];
  device_profiles?: DeviceProfile[];
  sensors?: Sensor[];
  images?: Image[];
  compatibility?: Compatibility;
  extensions?: Extensions;
}
```

### RecipeMeta Object

```typescript
interface RecipeMeta {
  name: string;
  description?: string;
  author: string | Author;
  origin?: Origin;
  servings?: Servings;
  diet_labels?: DietLabel[];
  keywords?: string[];
  difficulty?: "beginner" | "intermediate" | "advanced" | "professional";
  prep_time_minutes?: number;
  cook_time_minutes?: number;
  active_time_minutes?: number;
  total_time_minutes?: number;
  created_date: DateTime;
  updated_date?: DateTime;
  version?: string;
  license?: string;
}

interface Author {
  name: string;
  email?: string;
  organization?: string;
}

interface Origin {
  country: string;  // ISO 3166-1 alpha-2
  region?: string;
  city?: string;
  cuisine_type?: string;
  traditional?: boolean;
}

interface Servings {
  amount: number;
  unit?: string;
  adjustable?: boolean;
}
```

### Ingredient Object

```typescript
interface Ingredient {
  id: IngredientId;
  name: string;
  human_amount: string;
  machine_amount: MachineAmount;
  state?: string;
  brand?: string;
  temperature_c?: TemperatureRange;
  external_ids?: ExternalIds;
  nutritional?: Nutritional;
  substitutes?: Substitute[];
  allergens: Allergen[];  // REQUIRED, can be empty
  notes?: string;
  since_version?: string;
  deprecated_in?: string;
}

interface MachineAmount {
  value: number;
  unit: Unit;
  approximate?: boolean;
  tolerance?: {
    min: number;
    max: number;
  };
}

interface TemperatureRange {
  min?: number;
  max?: number;
  unit?: "C" | "F" | "K";
}

interface ExternalIds {
  GTIN?: string;
  USDA?: string;
  OpenFoodFacts?: string;
  [key: string]: string | undefined;
}

interface Nutritional {
  per_100g?: NutrientValues;
  per_serving?: NutrientValues;
}

interface NutrientValues {
  calories?: number;
  protein?: number;
  carbs?: number;
  fat?: number;
  saturated_fat?: number;
  fiber?: number;
  sugar?: number;
  sodium?: number;
  calcium?: number;
  [key: string]: number | undefined;
}

interface Substitute {
  id?: string;
  name: string;
  ratio?: number;
  notes?: string;
}
```

### Step Object

```typescript
interface Step {
  step_id: StepId;
  human_text: string;
  action: CookingAction;
  target?: string[];  // Ingredient IDs or step results
  params?: StepParameters;
  device_profile_ref?: string;
  done_when?: DoneConditions;
  tolerance?: Tolerance;
  hazards?: Hazard[];
  notes?: string;
  since_version?: string;
  deprecated_in?: string;
}

interface StepParameters {
  time_minutes?: number;
  time_seconds?: number;
  time_hours?: number;
  temperature_c?: number;
  temperature_f?: number;
  speed_rpm?: number;
  pressure_bar?: number;
  method?: string;
  vessel?: string;
  surface?: string;
  [key: string]: any;
}

interface DoneConditions {
  temperature_c?: number;
  color?: ColorSpec;
  texture?: TextureSpec;
  visual?: string;
  timer?: string;
  volume_increase?: number;
  [key: string]: any;
}

interface Tolerance {
  time_percent?: number;
  time_seconds?: number;
  temperature_c?: number;
  [key: string]: any;
}

type Hazard = "hot-surface" | "sharp-tool" | "electrical" | 
              "chemical" | "pressure" | "allergen-cross-contact";
```

## Validators

### JavaScript/TypeScript

```typescript
import { RCIPValidator } from '@rcip/validator';

class RCIPValidator {
  constructor(schemaVersion?: string);
  
  async init(schemaPath?: string): Promise<void>;
  
  validateRecipe(recipe: Recipe): ValidationResult;
  
  async validateFile(filePath: string): Promise<ValidationResult>;
  
  async validateDirectory(directory: string): Promise<ValidationResult[]>;
  
  validateIngredient(ingredient: Ingredient): ComponentValidation;
  
  validateStep(step: Step): ComponentValidation;
  
  resetStats(): void;
  
  getStats(): ValidationStats;
}

interface ValidationResult {
  valid: boolean;
  errors?: ValidationError[];
  warnings?: string[];
  info?: RecipeInfo;
}

interface ValidationError {
  path: string;
  message: string;
  code?: string;
}

interface ValidationStats {
  validated: number;
  passed: number;
  failed: number;
}

interface RecipeInfo {
  name: string;
  version: string;
  ingredientCount: number;
  stepCount: number;
  hasDeviceProfiles: boolean;
  hasSensors: boolean;
  allergens: string[];
  dietLabels: string[];
  difficulty?: string;
  totalTime?: number;
}
```

### Python

```python
from rcip_validator import RCIPValidator
from typing import Dict, List, Optional
from pathlib import Path

class RCIPValidator:
    def __init__(self, schema_version: str = "0.1"):
        """Initialize validator with schema version"""
        
    def init(self, schema_path: Optional[Path] = None) -> None:
        """Initialize validator with schema"""
        
    def validate_recipe(self, recipe: Dict) -> ValidationResult:
        """Validate a recipe object"""
        
    def validate_file(self, file_path: Path) -> ValidationResult:
        """Validate a recipe file"""
        
    def validate_directory(self, directory: Path) -> List[ValidationResult]:
        """Validate all recipes in directory"""
        
    def reset_stats(self) -> None:
        """Reset validation statistics"""
        
@dataclass
class ValidationResult:
    valid: bool
    errors: List[str]
    warnings: List[str]
    info: Dict[str, Any]
```

### Rust

```rust
use rcip_validator::{RCIPValidator, ValidationResult};
use std::path::Path;

pub struct RCIPValidator {
    schema_version: String,
}

impl RCIPValidator {
    pub fn new(schema_version: &str) -> Self;
    
    pub fn init(&mut self, schema_path: Option<&Path>) 
        -> Result<(), RCIPError>;
    
    pub fn validate_recipe(&mut self, recipe: &Value) 
        -> ValidationResult;
    
    pub fn validate_file(&mut self, file_path: &Path) 
        -> Result<ValidationResult, RCIPError>;
    
    pub fn validate_directory(&mut self, dir_path: &Path) 
        -> Result<Vec<(String, ValidationResult)>, RCIPError>;
    
    pub fn reset_stats(&mut self);
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub info: RecipeInfo,
}
```

## Converters

### Format Conversion

```typescript
import { RCIPConverter } from '@rcip/converter';

class RCIPConverter {
  // Import converters
  fromSchemaOrg(schemaOrgRecipe: any): Recipe;
  fromMealMaster(mealMasterText: string): Recipe;
  fromPaprikaJSON(paprikaRecipe: any): Recipe;
  fromChefkoch(chefkochData: any): Recipe;
  
  // Export converters
  toSchemaOrg(recipe: Recipe): any;
  toMarkdown(recipe: Recipe): string;
  toHTML(recipe: Recipe): string;
  toPDF(recipe: Recipe): Buffer;
  toDocx(recipe: Recipe): Buffer;
  
  // Batch operations
  async convertDirectory(
    inputDir: string,
    outputDir: string,
    inputFormat: string,
    outputFormat: string
  ): Promise<ConversionResult[]>;
}

interface ConversionResult {
  source: string;
  destination: string;
  success: boolean;
  errors?: string[];
}
```

### Scale Converter

```typescript
class RecipeScaler {
  scale(recipe: Recipe, targetServings: number): Recipe;
  
  scaleIngredient(
    ingredient: Ingredient, 
    factor: number
  ): Ingredient;
  
  adjustCookingTime(
    step: Step, 
    factor: number
  ): Step;
  
  convertUnits(
    recipe: Recipe, 
    targetSystem: "metric" | "imperial"
  ): Recipe;
}
```

## Device Control

### Device Manager

```typescript
interface DeviceManager {
  // Discovery
  scanForDevices(): Promise<Device[]>;
  
  // Registration
  registerDevice(profile: DeviceProfile): Promise<string>;
  unregisterDevice(deviceId: string): Promise<void>;
  
  // Control
  sendCommand(
    deviceId: string, 
    command: DeviceCommand
  ): Promise<CommandResult>;
  
  // Monitoring
  getStatus(deviceId: string): Promise<DeviceStatus>;
  subscribeToUpdates(
    deviceId: string, 
    callback: (status: DeviceStatus) => void
  ): Subscription;
}

interface DeviceCommand {
  action: string;
  parameters: Record<string, any>;
  timeout?: number;
}

interface DeviceStatus {
  deviceId: string;
  online: boolean;
  state: Record<string, any>;
  lastUpdate: DateTime;
}
```

### Device Profiles

```typescript
interface DeviceProfile {
  id: string;
  type: DeviceType;
  name?: string;
  manufacturer?: string;
  model?: string;
  params?: Record<string, any>;
  capabilities?: DeviceCapabilities;
  api_endpoint?: string;
  since_version?: string;
}

enum DeviceType {
  OVEN = "oven",
  STOVETOP = "stovetop",
  MIXER = "mixer",
  BLENDER = "blender",
  SCALE = "scale",
  THERMOMETER = "thermometer",
  TIMER = "timer",
  CUTTER = "cutter",
  PRINTER_3D_FOOD = "printer_3d_food",
  DISPENSER = "dispenser",
  SOUS_VIDE = "sous_vide",
  PRESSURE_COOKER = "pressure_cooker",
  DOUGH_PRESS = "dough_press",
  IMMERSION_CIRCULATOR = "immersion_circulator"
}

interface DeviceCapabilities {
  temperature_range_c?: { min: number; max: number };
  power_watts?: number;
  capacity_liters?: number;
  speed_range_rpm?: { min: number; max: number };
  programs?: string[];
  [key: string]: any;
}
```

## Sensor Interface

### Sensor Manager

```typescript
interface SensorManager {
  // Registration
  registerSensor(sensor: Sensor): Promise<string>;
  unregisterSensor(sensorId: string): Promise<void>;
  
  // Reading
  readSensor(sensorId: string): Promise<SensorReading>;
  readAllSensors(): Promise<Map<string, SensorReading>>;
  
  // Streaming
  streamSensorData(
    sensorId: string,
    callback: (reading: SensorReading) => void,
    samplingRate?: number
  ): StreamSubscription;
  
  // Calibration
  calibrateSensor(
    sensorId: string,
    referencePoints: CalibrationPoint[]
  ): Promise<CalibrationResult>;
}

interface SensorReading {
  sensorId: string;
  type: SensorType;
  value: any;
  unit?: string;
  timestamp: number;
  confidence?: number;
}

interface CalibrationPoint {
  reference: number;
  measured: number;
  temperature?: number;
}

enum SensorType {
  COLOR = "color",
  TEMPERATURE = "temperature",
  MOISTURE = "moisture",
  TEXTURE = "texture",
  AROMA = "aroma",
  WEIGHT = "weight",
  VOLUME = "volume",
  PH = "ph",
  CONDUCTIVITY = "conductivity",
  VISCOSITY = "viscosity"
}
```

## REST API

### Recipes

#### Get Recipe

```http
GET /recipes/{recipeId}
Accept: application/vnd.rcip+json
```

**Response:**
```json
{
  "rcip_version": "0.1",
  "id": "rcip-550e8400-e29b-41d4-a716-446655440000",
  "meta": { ... },
  "ingredients": [ ... ],
  "steps": [ ... ]
}
```

#### Create Recipe

```http
POST /recipes
Content-Type: application/vnd.rcip+json

{
  "rcip_version": "0.1",
  "meta": { ... },
  "ingredients": [ ... ],
  "steps": [ ... ]
}
```

**Response:**
```json
{
  "id": "rcip-550e8400-e29b-41d4-a716-446655440001",
  "created": "2025-01-15T10:30:00Z",
  "url": "https://api.rcip-format.org/v1/recipes/rcip-550e8400..."
}
```

#### Update Recipe

```http
PUT /recipes/{recipeId}
Content-Type: application/vnd.rcip+json

{
  "rcip_version": "0.1",
  "id": "rcip-550e8400-e29b-41d4-a716-446655440001",
  "meta": { ... },
  "ingredients": [ ... ],
  "steps": [ ... ]
}
```

#### Delete Recipe

```http
DELETE /recipes/{recipeId}
```

**Response:**
```http
204 No Content
```

#### Search Recipes

```http
GET /recipes/search?q=pizza&diet=vegetarian&difficulty=intermediate
```

**Query Parameters:**
- `q` - Search query
- `diet` - Diet labels (comma-separated)
- `allergens` - Exclude allergens
- `difficulty` - Difficulty level
- `time_max` - Maximum cooking time
- `page` - Page number
- `limit` - Results per page

**Response:**
```json
{
  "results": [ ... ],
  "total": 42,
  "page": 1,
  "per_page": 20
}
```

### Validation

#### Validate Recipe

```http
POST /validate
Content-Type: application/vnd.rcip+json

{
  "rcip_version": "0.1",
  ...
}
```

**Response:**
```json
{
  "valid": true,
  "errors": [],
  "warnings": [
    "Missing recommended field: meta.description"
  ],
  "info": {
    "ingredient_count": 10,
    "step_count": 12,
    "allergens": ["gluten", "milk"]
  }
}
```

### Conversion

#### Convert Recipe

```http
POST /convert
Content-Type: application/json

{
  "source_format": "schema_org",
  "target_format": "rcip",
  "recipe": { ... }
}
```

#### Scale Recipe

```http
POST /recipes/{recipeId}/scale
Content-Type: application/json

{
  "target_servings": 8,
  "maintain_proportions": true
}
```

### Devices

#### List Devices

```http
GET /devices
```

**Response:**
```json
{
  "devices": [
    {
      "id": "oven-01",
      "type": "oven",
      "name": "Smart Oven",
      "online": true,
      "status": { ... }
    }
  ]
}
```

#### Register Device

```http
POST /devices
Content-Type: application/json

{
  "type": "oven",
  "name": "Smart Oven",
  "manufacturer": "BoschKitchen",
  "model": "HBG675BS1",
  "capabilities": { ... }
}
```

#### Send Command

```http
POST /devices/{deviceId}/command
Content-Type: application/json

{
  "action": "set_temperature",
  "parameters": {
    "temperature_c": 180,
    "mode": "convection"
  }
}
```

#### Get Device Status

```http
GET /devices/{deviceId}/status
```

**Response:**
```json
{
  "deviceId": "oven-01",
  "online": true,
  "state": {
    "temperature_c": 180,
    "mode": "convection",
    "timer_remaining": 1200,
    "door_open": false
  },
  "lastUpdate": "2025-01-15T10:30:00Z"
}
```

### Sensors

#### Read Sensor

```http
GET /sensors/{sensorId}/reading
```

**Response:**
```json
{
  "sensorId": "sensor-temp-01",
  "type": "temperature",
  "value": 165.5,
  "unit": "C",
  "timestamp": 1705315800000,
  "confidence": 0.98
}
```

#### Stream Sensor Data

```http
GET /sensors/{sensorId}/stream
Accept: text/event-stream
```

**Response (SSE):**
```
data: {"value": 165.5, "timestamp": 1705315800000}
data: {"value": 165.6, "timestamp": 1705315801000}
data: {"value": 165.7, "timestamp": 1705315802000}
```

## WebSocket API

### Connection

```javascript
const ws = new WebSocket('wss://api.rcip-format.org/v1/ws');

ws.on('open', () => {
  ws.send(JSON.stringify({
    type: 'auth',
    token: 'Bearer <api_token>'
  }));
});
```

### Subscribe to Device

```javascript
ws.send(JSON.stringify({
  type: 'subscribe',
  channel: 'device',
  deviceId: 'oven-01'
}));

ws.on('message', (data) => {
  const message = JSON.parse(data);
  if (message.channel === 'device') {
    console.log('Device update:', message.data);
  }
});
```

### Real-time Cooking Session

```javascript
// Start cooking session
ws.send(JSON.stringify({
  type: 'start_session',
  recipeId: 'rcip-550e8400-e29b-41d4-a716-446655440000',
  devices: ['oven-01', 'mixer-01'],
  sensors: ['temp-01', 'color-01']
}));

// Receive updates
ws.on('message', (data) => {
  const message = JSON.parse(data);
  
  switch(message.type) {
    case 'step_started':
      console.log(`Step ${message.stepId} started`);
      break;
      
    case 'sensor_update':
      console.log(`Sensor ${message.sensorId}: ${message.value}`);
      break;
      
    case 'step_complete':
      console.log(`Step ${message.stepId} complete`);
      break;
      
    case 'session_complete':
      console.log('Recipe complete!');
      break;
  }
});
```

## Error Codes

### HTTP Status Codes

| Code | Description |
|------|-------------|
| 200 | Success |
| 201 | Created |
| 204 | No Content |
| 400 | Bad Request - Invalid input |
| 401 | Unauthorized - Invalid API key |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found |
| 409 | Conflict - Resource already exists |
| 422 | Unprocessable Entity - Validation failed |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error |
| 503 | Service Unavailable |

### Application Error Codes

```json
{
  "error": {
    "code": "RCIP_VALIDATION_FAILED",
    "message": "Recipe validation failed",
    "details": [
      {
        "path": "ingredients[0].allergens",
        "message": "Required field missing"
      }
    ]
  }
}
```

| Code | Description |
|------|-------------|
| `RCIP_VALIDATION_FAILED` | Recipe validation failed |
| `RCIP_INVALID_FORMAT` | Invalid RCIP format |
| `RCIP_VERSION_MISMATCH` | Incompatible version |
| `RCIP_CONVERSION_FAILED` | Format conversion failed |
| `DEVICE_NOT_FOUND` | Device not registered |
| `DEVICE_OFFLINE` | Device is offline |
| `DEVICE_COMMAND_FAILED` | Command execution failed |
| `SENSOR_NOT_FOUND` | Sensor not registered |
| `SENSOR_READ_FAILED` | Sensor reading failed |
| `CALIBRATION_REQUIRED` | Sensor needs calibration |

## SDK Examples

### JavaScript/Node.js

```javascript
import { RCIPClient } from '@rcip/client';

const client = new RCIPClient({
  apiKey: 'your-api-key',
  version: '0.1'
});

// Get recipe
const recipe = await client.recipes.get('rcip-550e8400...');

// Validate recipe
const validation = await client.validate(recipe);

// Scale recipe
const scaled = await client.recipes.scale(recipe.id, {
  targetServings: 8
});

// Control device
await client.devices.command('oven-01', {
  action: 'preheat',
  parameters: {
    temperature_c: 180
  }
});

// Read sensor
const reading = await client.sensors.read('temp-01');

// Stream sensor data
const stream = client.sensors.stream('temp-01', (reading) => {
  console.log(`Temperature: ${reading.value}°C`);
});

// Stop streaming
stream.unsubscribe();
```

### Python

```python
from rcip_client import RCIPClient

client = RCIPClient(
    api_key='your-api-key',
    version='0.1'
)

# Get recipe
recipe = client.recipes.get('rcip-550e8400...')

# Validate recipe
validation = client.validate(recipe)

# Scale recipe
scaled = client.recipes.scale(
    recipe['id'],
    target_servings=8
)

# Control device
client.devices.command('oven-01', {
    'action': 'preheat',
    'parameters': {
        'temperature_c': 180
    }
})

# Stream sensor data
for reading in client.sensors.stream('temp-01'):
    print(f"Temperature: {reading['value']}°C")
```

### cURL

```bash
# Get recipe
curl -H "Authorization: Bearer <token>" \
     -H "Accept: application/vnd.rcip+json" \
     https://api.rcip-format.org/v1/recipes/rcip-550e8400...

# Validate recipe
curl -X POST \
     -H "Authorization: Bearer <token>" \
     -H "Content-Type: application/vnd.rcip+json" \
     -d @recipe.json \
     https://api.rcip-format.org/v1/validate

# Send device command
curl -X POST \
     -H "Authorization: Bearer <token>" \
     -H "Content-Type: application/json" \
     -d '{"action":"preheat","parameters":{"temperature_c":180}}' \
     https://api.rcip-format.org/v1/devices/oven-01/command
```

## Rate Limiting

### Headers

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1705320000
```

### Retry After

```http
HTTP/1.1 429 Too Many Requests
Retry-After: 3600
```

## Webhooks

### Configuration

```http
POST /webhooks
Content-Type: application/json

{
  "url": "https://your-app.com/webhook",
  "events": [
    "recipe.created",
    "recipe.updated",
    "device.status_changed",
    "sensor.threshold_exceeded"
  ],
  "secret": "webhook-secret"
}
```

### Event Payload

```json
{
  "id": "evt-123456",
  "type": "recipe.created",
  "timestamp": "2025-01-15T10:30:00Z",
  "data": {
    "recipe_id": "rcip-550e8400...",
    "author": "user@example.com"
  }
}
```

### Signature Verification

```javascript
const crypto = require('crypto');

function verifyWebhook(payload, signature, secret) {
  const hash = crypto
    .createHmac('sha256', secret)
    .update(payload)
    .digest('hex');
  
  return `sha256=${hash}` === signature;
}
```

## Pagination

### Offset Pagination

```http
GET /recipes?page=2&limit=20
```

**Response:**
```json
{
  "data": [ ... ],
  "pagination": {
    "page": 2,
    "limit": 20,
    "total": 150,
    "pages": 8
  },
  "links": {
    "first": "/recipes?page=1&limit=20",
    "prev": "/recipes?page=1&limit=20",
    "next": "/recipes?page=3&limit=20",
    "last": "/recipes?page=8&limit=20"
  }
}
```

### Cursor Pagination

```http
GET /recipes?cursor=eyJpZCI6MTAwfQ&limit=20
```

**Response:**
```json
{
  "data": [ ... ],
  "pagination": {
    "cursor": "eyJpZCI6MTIwfQ",
    "has_more": true,
    "limit": 20
  }
}
```

## Batch Operations

### Batch Validation

```http
POST /validate/batch
Content-Type: application/json

{
  "recipes": [
    { "rcip_version": "0.1", ... },
    { "rcip_version": "0.1", ... }
  ]
}
```

**Response:**
```json
{
  "results": [
    {
      "index": 0,
      "valid": true,
      "errors": [],
      "warnings": []
    },
    {
      "index": 1,
      "valid": false,
      "errors": ["Missing required field: ingredients"],
      "warnings": []
    }
  ],
  "summary": {
    "total": 2,
    "valid": 1,
    "invalid": 1
  }
}
```

### Batch Device Commands

```http
POST /devices/batch/command
Content-Type: application/json

{
  "commands": [
    {
      "deviceId": "oven-01",
      "action": "preheat",
      "parameters": {"temperature_c": 180}
    },
    {
      "deviceId": "mixer-01",
      "action": "start",
      "parameters": {"speed": 3}
    }
  ]
}
```

## GraphQL API (Alternative)

```graphql
# Schema
type Query {
  recipe(id: ID!): Recipe
  recipes(filter: RecipeFilter, page: Int, limit: Int): RecipeConnection
  device(id: ID!): Device
  sensor(id: ID!): Sensor
}

type Mutation {
  createRecipe(input: RecipeInput!): Recipe
  updateRecipe(id: ID!, input: RecipeInput!): Recipe
  deleteRecipe(id: ID!): Boolean
  sendDeviceCommand(deviceId: ID!, command: CommandInput!): CommandResult
  startCookingSession(recipeId: ID!, devices: [ID!]): Session
}

type Subscription {
  deviceStatus(deviceId: ID!): DeviceStatus
  sensorReading(sensorId: ID!): SensorReading
  cookingSession(sessionId: ID!): SessionUpdate
}

# Example Query
query GetRecipe($id: ID!) {
  recipe(id: $id) {
    id
    meta {
      name
      author {
        name
        email
      }
      servings {
        amount
        unit
      }
    }
    ingredients {
      id
      name
      allergens
      nutritional {
        per_100g {
          calories
          protein
        }
      }
    }
    steps {
      step_id
      human_text
      action
      params
    }
  }
}

# Example Mutation
mutation ScaleRecipe($id: ID!, $servings: Int!) {
  scaleRecipe(id: $id, targetServings: $servings) {
    id
    meta {
      servings {
        amount
      }
    }
    ingredients {
      human_amount
      machine_amount {
        value
        unit
      }
    }
  }
}

# Example Subscription
subscription MonitorCooking($sessionId: ID!) {
  cookingSession(sessionId: $sessionId) {
    type
    stepId
    progress
    sensorReadings {
      sensorId
      value
      unit
    }
  }
}
```

## Testing

### Test Endpoints

```http
POST /test/validate
Content-Type: application/json

{
  "recipe": { ... },
  "strict_mode": true
}
```

### Mock Data

```http
GET /test/recipes/sample?type=simple
GET /test/recipes/sample?type=complex
GET /test/recipes/sample?type=invalid
```

### Health Check

```http
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "rcip_version": "0.1",
  "timestamp": "2025-01-15T10:30:00Z",
  "services": {
    "database": "ok",
    "cache": "ok",
    "device_manager": "ok",
    "sensor_manager": "ok"
  }
}
```

## Versioning

### API Versioning

```http
GET /v1/recipes
GET /v2/recipes  # Future version
```

### Recipe Format Versioning

```http
X-RCIP-Version: 0.1
Accept: application/vnd.rcip.v0.1+json
```

### Backward Compatibility

```javascript
// Client with version negotiation
const client = new RCIPClient({
  apiKey: 'your-api-key',
  preferredVersion: '0.2',
  fallbackVersions: ['0.1'],
  strictMode: false  // Allow minor version differences
});
```

## Security

### Authentication Methods

#### API Key

```http
Authorization: Bearer <api_key>
```

#### OAuth 2.0

```http
Authorization: Bearer <oauth_token>
```

#### JWT

```javascript
const jwt = require('jsonwebtoken');

const token = jwt.sign(
  {
    userId: 'user123',
    permissions: ['read', 'write'],
    exp: Math.floor(Date.now() / 1000) + (60 * 60)
  },
  process.env.JWT_SECRET
);
```

### Permissions

| Scope | Description |
|-------|-------------|
| `recipes:read` | Read recipes |
| `recipes:write` | Create/update recipes |
| `recipes:delete` | Delete recipes |
| `devices:read` | View device status |
| `devices:control` | Control devices |
| `sensors:read` | Read sensor data |
| `admin` | Full access |

### CORS Configuration

```javascript
// Server configuration
app.use(cors({
  origin: ['https://app.rcip-format.org', 'http://localhost:3000'],
  credentials: true,
  methods: ['GET', 'POST', 'PUT', 'DELETE'],
  allowedHeaders: ['Content-Type', 'Authorization', 'X-RCIP-Version']
}));
```

## Client Libraries

### Installation

```bash
# JavaScript/Node.js
npm install @rcip/client

# Python
pip install rcip-client

# Ruby
gem install rcip-client

# Go
go get github.com/rcip/go-client

# Rust
cargo add rcip-client

# Java
<dependency>
    <groupId>org.rcip</groupId>
    <artifactId>rcip-client</artifactId>
    <version>1.0.0</version>
</dependency>
```

### Quick Start Examples

#### JavaScript
```javascript
import { RCIP } from '@rcip/client';

const rcip = new RCIP('your-api-key');
const recipes = await rcip.recipes.list({ diet: 'vegan' });
```

#### Python
```python
from rcip import Client

client = Client('your-api-key')
recipes = client.recipes.list(diet='vegan')
```

#### Go
```go
import "github.com/rcip/go-client"

client := rcip.NewClient("your-api-key")
recipes, err := client.Recipes.List(&rcip.RecipeFilter{
    Diet: "vegan",
})
```

---

*Created by Alexey Kozlov - [al7koz.com](https://al7koz.com)*
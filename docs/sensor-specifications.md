# RCIP Sensor Specifications Documentation

## Table of Contents
1. [Overview](#overview)
2. [Sensor Architecture](#sensor-architecture)
3. [Sensor Types](#sensor-types)
4. [Implementation Guidelines](#implementation-guidelines)
5. [Calibration Standards](#calibration-standards)
6. [Integration Examples](#integration-examples)
7. [Best Practices](#best-practices)

## Overview

The RCIP sensor system enables precise quality control and automation in cooking processes. Sensors provide real-time feedback to ensure consistent results across different kitchens and equipment.

### Key Principles

- **Precision**: High-accuracy measurements for reproducible results
- **Standardization**: Consistent calibration across devices
- **Flexibility**: Optional sensors with priority levels
- **Safety**: Continuous monitoring for hazard prevention
- **Automation**: Enable fully autonomous cooking

## Sensor Architecture

### Sensor Entry Structure

```json
{
  "id": "sensor-01",
  "type": "temperature",
  "target": "core",
  "spec": {
    "range_c": [54, 56],
    "precision": 0.1,
    "sampling_rate_hz": 1
  },
  "priority": "required",
  "calibration": {
    "reference": "NIST",
    "last_calibrated": "2025-01-15",
    "drift_correction": 0.05
  },
  "since_version": "0.1"
}
```

### Required Fields

- `id`: Unique sensor identifier
- `type`: Sensor category from standard types
- `target`: What the sensor monitors

### Optional Fields

- `spec`: Technical specifications
- `priority`: Importance level (required/recommended/optional)
- `calibration`: Calibration details
- `notes`: Additional information

## Sensor Types

### 1. Color Sensor (`color`)

Monitors visual appearance for doneness and quality.

```json
{
  "id": "sensor-color-01",
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
    "lab": {
      "l": [45, 65],
      "a": [15, 25],
      "b": [20, 35]
    },
    "calibration": "D65-illuminant",
    "color_space": "sRGB",
    "resolution_bits": 12
  }
}
```

**Applications:**
- Bread crust browning
- Meat searing level
- Caramelization stage
- Vegetable blanching

**Specifications:**
- **Color Spaces**: RGB, HSV, LAB, XYZ
- **Illuminants**: D65 (daylight), A (incandescent), F2 (fluorescent)
- **Resolution**: 8-16 bit per channel
- **Sampling Rate**: 10-60 Hz

### 2. Temperature Sensor (`temperature`)

Core sensor for thermal monitoring.

```json
{
  "id": "sensor-temp-01",
  "type": "temperature",
  "target": "internal",
  "spec": {
    "range_c": [-50, 500],
    "precision": 0.1,
    "accuracy": 0.5,
    "response_time_ms": 500,
    "probe_type": "thermocouple-k",
    "placement": {
      "depth_mm": 25,
      "location": "geometric-center"
    }
  }
}
```

**Probe Types:**
- **Thermocouple**: K, J, T types (-200°C to 1350°C)
- **RTD**: PT100, PT1000 (-200°C to 850°C)
- **Thermistor**: NTC, PTC (-50°C to 300°C)
- **Infrared**: Non-contact (-50°C to 1000°C)

**Applications:**
- Internal meat temperature
- Oil temperature for frying
- Oven ambient temperature
- Surface temperature

### 3. Moisture Sensor (`moisture`)

Measures water content and humidity.

```json
{
  "id": "sensor-moisture-01",
  "type": "moisture",
  "target": "dough",
  "spec": {
    "type": "capacitive",
    "range_percent": [0, 100],
    "precision": 0.5,
    "measurement_depth_mm": 10,
    "frequency_mhz": 100,
    "temperature_compensation": true
  }
}
```

**Measurement Methods:**
- **Capacitive**: Dielectric constant measurement
- **Resistive**: Electrical resistance
- **Microwave**: Penetrative measurement
- **NIR**: Near-infrared spectroscopy

**Applications:**
- Dough hydration level
- Meat moisture loss
- Dehydration progress
- Steam oven humidity

### 4. Texture Sensor (`texture`)

Analyzes mechanical properties.

```json
{
  "id": "sensor-texture-01",
  "type": "texture",
  "target": "crust",
  "spec": {
    "hardness_n": 15,
    "crispness_hz": 2000,
    "elasticity_percent": 20,
    "measurement_method": "penetration",
    "probe_diameter_mm": 2,
    "penetration_speed_mm_s": 1,
    "penetration_depth_mm": 5
  }
}
```

**Measurement Parameters:**
- **Hardness**: Force required to compress (N)
- **Elasticity**: Recovery after deformation (%)
- **Crispness**: Acoustic frequency on breaking (Hz)
- **Chewiness**: Work required to masticate (N·mm)
- **Adhesiveness**: Work to overcome attraction (N·mm)

**Test Methods:**
- **TPA**: Texture Profile Analysis
- **Penetration**: Single probe test
- **Compression**: Parallel plate test
- **Shear**: Cutting force test

### 5. Aroma Sensor (`aroma`)

Detects volatile organic compounds (VOCs).

```json
{
  "id": "sensor-aroma-01",
  "type": "aroma",
  "target": "baking-bread",
  "spec": {
    "voc_profile": {
      "2-acetyl-1-pyrroline": "high",
      "2-acetylthiazole": "medium",
      "furaneol": "medium",
      "vanillin": "low"
    },
    "sensor_array": [
      {"type": "MOS", "target": "alcohols"},
      {"type": "MOS", "target": "aldehydes"},
      {"type": "QCM", "target": "esters"}
    ],
    "intensity_scale": 10,
    "threshold_ppb": 0.1
  }
}
```

**Sensor Technologies:**
- **MOS**: Metal Oxide Semiconductor
- **QCM**: Quartz Crystal Microbalance
- **SAW**: Surface Acoustic Wave
- **CP**: Conducting Polymer

**Key Compounds:**
- **Maillard Products**: Pyrazines, furans, thiophenes
- **Caramelization**: Maltol, isomaltol
- **Fermentation**: Esters, alcohols, acids
- **Lipid Oxidation**: Aldehydes, ketones

### 6. Weight Sensor (`weight`)

Precision mass measurement.

```json
{
  "id": "sensor-weight-01",
  "type": "weight",
  "target": "container",
  "spec": {
    "range_g": [0, 5000],
    "precision_g": 0.1,
    "tare_capability": true,
    "overload_protection_g": 6000,
    "drift_g_per_hour": 0.05,
    "temperature_compensation": true
  }
}
```

**Applications:**
- Ingredient dosing
- Reduction monitoring
- Yield measurement
- Portion control

### 7. Volume Sensor (`volume`)

Liquid level and volume measurement.

```json
{
  "id": "sensor-volume-01",
  "type": "volume",
  "target": "pot",
  "spec": {
    "method": "ultrasonic",
    "range_ml": [0, 5000],
    "precision_ml": 5,
    "temperature_correction": true,
    "foam_detection": true,
    "frequency_khz": 40
  }
}
```

**Measurement Methods:**
- **Ultrasonic**: Non-contact level sensing
- **Pressure**: Hydrostatic pressure
- **Optical**: Laser or LED level detection
- **Capacitive**: Dielectric measurement
- **Float**: Mechanical float switch

### 8. pH Sensor (`ph`)

Acidity/alkalinity measurement.

```json
{
  "id": "sensor-ph-01",
  "type": "ph",
  "target": "fermentation",
  "spec": {
    "range": [0, 14],
    "precision": 0.01,
    "temperature_compensation": "automatic",
    "electrode_type": "glass",
    "response_time_s": 5,
    "calibration_points": [4.01, 7.00, 10.01]
  }
}
```

**Applications:**
- Fermentation monitoring
- Sauce acidification
- Cheese making
- Pickling process
- Dough pH for gluten development

### 9. Conductivity Sensor (`conductivity`)

Measures ionic content and salinity.

```json
{
  "id": "sensor-conductivity-01",
  "type": "conductivity",
  "target": "brine",
  "spec": {
    "range_ms_cm": [0, 200],
    "precision": 0.5,
    "cell_constant": 1.0,
    "temperature_compensation": "linear",
    "tds_conversion_factor": 0.5
  }
}
```

**Applications:**
- Salt concentration
- Brine strength
- Water quality
- Mineral content

### 10. Viscosity Sensor (`viscosity`)

Fluid thickness and flow properties.

```json
{
  "id": "sensor-viscosity-01",
  "type": "viscosity",
  "target": "sauce",
  "spec": {
    "method": "rotational",
    "range_cp": [1, 100000],
    "shear_rate_s": 10,
    "temperature_c": 25,
    "spindle_type": "LV-2",
    "torque_range_percent": [10, 90]
  }
}
```

**Measurement Methods:**
- **Rotational**: Spindle viscometer
- **Vibrational**: Tuning fork sensor
- **Falling Ball**: Gravity-based
- **Flow Cup**: Efflux time

## Implementation Guidelines

### 1. Sensor Selection

```javascript
class SensorSelector {
  selectSensors(recipe, availableSensors) {
    const required = [];
    const recommended = [];
    const optional = [];
    
    recipe.sensors?.forEach(sensor => {
      const available = availableSensors.find(s => 
        s.type === sensor.type && 
        this.meetsSpecifications(s, sensor.spec)
      );
      
      if (available) {
        switch(sensor.priority) {
          case 'required':
            required.push({sensor, device: available});
            break;
          case 'recommended':
            recommended.push({sensor, device: available});
            break;
          case 'optional':
            optional.push({sensor, device: available});
            break;
        }
      }
    });
    
    return {required, recommended, optional};
  }
  
  meetsSpecifications(device, spec) {
    // Check if device meets recipe requirements
    if (spec.range_c) {
      const deviceRange = device.specifications.range_c;
      if (spec.range_c[0] < deviceRange[0] || 
          spec.range_c[1] > deviceRange[1]) {
        return false;
      }
    }
    
    if (spec.precision && device.specifications.precision > spec.precision) {
      return false;
    }
    
    return true;
  }
}
```

### 2. Sensor Fusion

```python
class SensorFusion:
    """Combine multiple sensors for better accuracy"""
    
    def __init__(self):
        self.sensors = {}
        self.weights = {}
        self.history = []
    
    def add_sensor(self, sensor_id: str, weight: float = 1.0):
        """Add sensor to fusion system"""
        self.sensors[sensor_id] = None
        self.weights[sensor_id] = weight
    
    def update(self, sensor_id: str, value: float, timestamp: float):
        """Update sensor reading"""
        self.sensors[sensor_id] = {
            'value': value,
            'timestamp': timestamp,
            'confidence': self.calculate_confidence(sensor_id, value)
        }
    
    def get_fused_value(self) -> dict:
        """Calculate weighted average of sensors"""
        total_weight = 0
        weighted_sum = 0
        
        for sensor_id, data in self.sensors.items():
            if data and time.time() - data['timestamp'] < 5:  # 5 second timeout
                weight = self.weights[sensor_id] * data['confidence']
                weighted_sum += data['value'] * weight
                total_weight += weight
        
        if total_weight > 0:
            fused_value = weighted_sum / total_weight
            confidence = min(total_weight, 1.0)
            
            return {
                'value': fused_value,
                'confidence': confidence,
                'sensor_count': len([s for s in self.sensors.values() if s]),
                'timestamp': time.time()
            }
        
        return None
    
    def calculate_confidence(self, sensor_id: str, value: float) -> float:
        """Calculate sensor confidence based on history"""
        # Check for drift
        if self.history:
            recent_values = [h['value'] for h in self.history[-10:]]
            mean = statistics.mean(recent_values)
            stdev = statistics.stdev(recent_values) if len(recent_values) > 1 else 0
            
            if stdev > 0:
                z_score = abs(value - mean) / stdev
                confidence = max(0, 1 - (z_score / 3))  # 3-sigma rule
            else:
                confidence = 1.0
        else:
            confidence = 1.0
        
        return confidence
```

### 3. Real-Time Monitoring

```javascript
class SensorMonitor {
  constructor() {
    this.sensors = new Map();
    this.alerts = [];
    this.dataLogger = new DataLogger();
  }
  
  async startMonitoring(sensorConfig) {
    const sensor = await this.connectSensor(sensorConfig);
    
    const monitor = setInterval(async () => {
      try {
        const reading = await sensor.read();
        
        // Log data
        this.dataLogger.log(sensorConfig.id, reading);
        
        // Check thresholds
        this.checkThresholds(sensorConfig, reading);
        
        // Check rate of change
        this.checkRateOfChange(sensorConfig, reading);
        
        // Update fusion system if applicable
        if (sensorConfig.fusion_group) {
          this.updateFusion(sensorConfig.fusion_group, reading);
        }
        
      } catch (error) {
        this.handleSensorError(sensorConfig, error);
      }
    }, sensorConfig.sampling_interval_ms || 1000);
    
    this.sensors.set(sensorConfig.id, {
      config: sensorConfig,
      sensor: sensor,
      monitor: monitor
    });
  }
  
  checkThresholds(config, reading) {
    if (config.spec.range_c) {
      const [min, max] = config.spec.range_c;
      
      if (reading.value < min) {
        this.alert('LOW_THRESHOLD', config, reading);
      } else if (reading.value > max) {
        this.alert('HIGH_THRESHOLD', config, reading);
      }
    }
    
    // Check critical thresholds
    if (config.critical_thresholds) {
      for (const threshold of config.critical_thresholds) {
        if (this.evaluateThreshold(reading, threshold)) {
          this.criticalAlert(config, reading, threshold);
        }
      }
    }
  }
  
  checkRateOfChange(config, reading) {
    const history = this.dataLogger.getRecent(config.id, 10);
    
    if (history.length >= 2) {
      const rate = (reading.value - history[0].value) / 
                   (reading.timestamp - history[0].timestamp);
      
      if (config.max_rate && Math.abs(rate) > config.max_rate) {
        this.alert('RAPID_CHANGE', config, {
          current: reading,
          rate: rate
        });
      }
    }
  }
}
```

## Calibration Standards

### Calibration Framework

```python
class CalibrationManager:
    """Manage sensor calibration"""
    
    def __init__(self):
        self.calibrations = {}
        self.standards = {
            'temperature': {
                'ice_point': 0.0,
                'boiling_point': 100.0,
                'references': ['NIST', 'DIN', 'JIS']
            },
            'ph': {
                'buffer_4': 4.01,
                'buffer_7': 7.00,
                'buffer_10': 10.01,
                'temperature_compensation': True
            },
            'weight': {
                'gravity_correction': 9.80665,
                'air_buoyancy': 1.2
            }
        }
    
    def calibrate_sensor(self, sensor_type: str, sensor_id: str, 
                         reference_points: list) -> dict:
        """Perform sensor calibration"""
        
        calibration = {
            'sensor_id': sensor_id,
            'type': sensor_type,
            'date': datetime.now().isoformat(),
            'points': [],
            'coefficients': None
        }
        
        # Collect calibration points
        for ref_point in reference_points:
            measured = self.measure_reference(sensor_id, ref_point)
            calibration['points'].append({
                'reference': ref_point['value'],
                'measured': measured,
                'temperature': ref_point.get('temperature', 25)
            })
        
        # Calculate calibration coefficients
        if sensor_type == 'temperature':
            calibration['coefficients'] = self.linear_calibration(
                calibration['points']
            )
        elif sensor_type == 'ph':
            calibration['coefficients'] = self.ph_calibration(
                calibration['points']
            )
        
        # Validate calibration
        calibration['validation'] = self.validate_calibration(calibration)
        
        # Store calibration
        self.calibrations[sensor_id] = calibration
        
        return calibration
    
    def linear_calibration(self, points: list) -> dict:
        """Linear regression calibration"""
        x = [p['measured'] for p in points]
        y = [p['reference'] for p in points]
        
        # Calculate slope and offset
        n = len(points)
        sum_x = sum(x)
        sum_y = sum(y)
        sum_xy = sum(x[i] * y[i] for i in range(n))
        sum_x2 = sum(x[i]**2 for i in range(n))
        
        slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x**2)
        offset = (sum_y - slope * sum_x) / n
        
        # Calculate R-squared
        y_mean = sum_y / n
        ss_tot = sum((y[i] - y_mean)**2 for i in range(n))
        y_pred = [slope * x[i] + offset for i in range(n)]
        ss_res = sum((y[i] - y_pred[i])**2 for i in range(n))
        r_squared = 1 - (ss_res / ss_tot) if ss_tot > 0 else 0
        
        return {
            'type': 'linear',
            'slope': slope,
            'offset': offset,
            'r_squared': r_squared
        }
    
    def apply_calibration(self, sensor_id: str, raw_value: float) -> float:
        """Apply calibration to raw sensor value"""
        if sensor_id not in self.calibrations:
            return raw_value
        
        cal = self.calibrations[sensor_id]
        coef = cal['coefficients']
        
        if coef['type'] == 'linear':
            return coef['slope'] * raw_value + coef['offset']
        
        return raw_value
```

### Standard References

| Sensor Type | Standard | Reference | Uncertainty |
|------------|----------|-----------|------------|
| Temperature | NIST SRM 1747 | Triple point of water | ±0.001°C |
| pH | NIST SRM 185j | pH buffer solutions | ±0.01 pH |
| Weight | OIML R111 | Standard masses | ±0.1% |
| Color | CIE Standard Illuminants | D65, A, F2 | ΔE < 1.0 |
| Moisture | AOAC 925.10 | Oven drying method | ±0.5% |

## Integration Examples

### 1. Multi-Sensor Cooking Control

```javascript
class SmartCookingController {
  constructor() {
    this.sensors = {};
    this.profiles = {};
    this.currentStep = null;
  }
  
  async executeStepWithSensors(step, sensors) {
    this.currentStep = step;
    
    // Initialize sensors
    for (const sensor of sensors) {
      await this.initializeSensor(sensor);
    }
    
    // Start monitoring
    const monitoring = this.startMonitoring(step.done_when);
    
    // Execute cooking action
    await this.device.execute(step.action, step.params);
    
    // Wait for completion conditions
    const result = await monitoring;
    
    return result;
  }
  
  startMonitoring(doneConditions) {
    return new Promise((resolve) => {
      const checkInterval = setInterval(async () => {
        const sensorData = await this.readAllSensors();
        
        if (this.checkDoneConditions(sensorData, doneConditions)) {
          clearInterval(checkInterval);
          resolve({
            success: true,
            finalReadings: sensorData,
            duration: Date.now() - this.startTime
          });
        }
        
        // Check for critical conditions
        if (this.checkCriticalConditions(sensorData)) {
          clearInterval(checkInterval);
          await this.emergencyStop();
          resolve({
            success: false,
            error: 'Critical condition detected',
            readings: sensorData
          });
        }
      }, 100); // 10Hz monitoring
    });
  }
  
  checkDoneConditions(sensorData, conditions) {
    for (const [sensorType, requirement] of Object.entries(conditions)) {
      const reading = sensorData[sensorType];
      
      if (!reading) continue;
      
      switch(sensorType) {
        case 'temperature_c':
          if (reading.value < requirement) return false;
          break;
          
        case 'color':
          if (!this.colorInRange(reading, requirement)) return false;
          break;
          
        case 'texture':
          if (!this.textureMatches(reading, requirement)) return false;
          break;
          
        case 'moisture':
          if (Math.abs(reading.value - requirement.value) > requirement.tolerance) {
            return false;
          }
          break;
      }
    }
    
    return true;
  }
  
  colorInRange(reading, spec) {
    if (spec.rgb) {
      const inRange = 
        reading.r >= spec.rgb.r[0] && reading.r <= spec.rgb.r[1] &&
        reading.g >= spec.rgb.g[0] && reading.g <= spec.rgb.g[1] &&
        reading.b >= spec.rgb.b[0] && reading.b <= spec.rgb.b[1];
      return inRange;
    }
    
    if (spec.hsv) {
      // Convert and check HSV
      const hsv = this.rgbToHsv(reading);
      return hsv.h >= spec.hsv.h[0] && hsv.h <= spec.hsv.h[1] &&
             hsv.s >= spec.hsv.s[0] && hsv.s <= spec.hsv.s[1] &&
             hsv.v >= spec.hsv.v[0] && hsv.v <= spec.hsv.v[1];
    }
    
    return false;
  }
}
```

### 2. Adaptive Cooking

```python
class AdaptiveCooking:
    """Adjust cooking parameters based on sensor feedback"""
    
    def __init__(self):
        self.pid_controllers = {}
        self.learning_system = CookingML()
    
    def create_pid_controller(self, sensor_type: str, setpoint: float):
        """Create PID controller for sensor"""
        return PIDController(
            kp=self.get_tuning(sensor_type, 'kp'),
            ki=self.get_tuning(sensor_type, 'ki'),
            kd=self.get_tuning(sensor_type, 'kd'),
            setpoint=setpoint
        )
    
    def adapt_temperature(self, current_temp: float, target_temp: float, 
                         rate_of_rise: float) -> dict:
        """Adapt temperature based on sensor feedback"""
        
        # PID control
        if 'temperature' not in self.pid_controllers:
            self.pid_controllers['temperature'] = self.create_pid_controller(
                'temperature', target_temp
            )
        
        pid = self.pid_controllers['temperature']
        adjustment = pid.update(current_temp)
        
        # Predictive adjustment based on rate of rise
        predicted_temp = current_temp + (rate_of_rise * 30)  # 30 second prediction
        
        if predicted_temp > target_temp + 5:
            # Overshoot predicted, reduce heat
            adjustment *= 0.5
        
        # Learn from history
        self.learning_system.record(
            current_temp, target_temp, adjustment, rate_of_rise
        )
        
        return {
            'power_adjustment': adjustment,
            'predicted_overshoot': max(0, predicted_temp - target_temp),
            'time_to_target': (target_temp - current_temp) / rate_of_rise if rate_of_rise > 0 else None
        }
    
    def adapt_moisture(self, current_moisture: float, target_moisture: float,
                      cooking_method: str) -> dict:
        """Adapt cooking to maintain moisture"""
        
        moisture_diff = target_moisture - current_moisture
        
        adjustments = {
            'steam_injection': 0,
            'temperature_adjustment': 0,
            'fan_speed': 'maintain'
        }
        
        if cooking_method in ['bake', 'roast']:
            if moisture_diff > 5:  # Too dry
                adjustments['steam_injection'] = moisture_diff * 2  # ml/min
                adjustments['temperature_adjustment'] = -5  # Reduce temp
                adjustments['fan_speed'] = 'reduce'
            elif moisture_diff < -5:  # Too moist
                adjustments['temperature_adjustment'] = 5
                adjustments['fan_speed'] = 'increase'
        
        return adjustments
```

### 3. Quality Prediction

```javascript
class QualityPredictor {
  constructor() {
    this.model = new NeuralNetwork();
    this.history = [];
  }
  
  async predictFinalQuality(currentSensors, timeElapsed, recipeProfile) {
    // Feature extraction
    const features = this.extractFeatures(currentSensors, timeElapsed);
    
    // Predict final state
    const prediction = await this.model.predict([
      features.temperature_profile,
      features.moisture_trend,
      features.color_development,
      features.texture_formation,
      recipeProfile.target_characteristics
    ]);
    
    return {
      predicted_quality_score: prediction.quality,
      predicted_completion_time: prediction.time_remaining,
      confidence: prediction.confidence,
      recommendations: this.generateRecommendations(prediction)
    };
  }
  
  extractFeatures(sensors, time) {
    return {
      temperature_profile: {
        current: sensors.temperature?.value,
        rate: this.calculateRate(sensors.temperature, time),
        stability: this.calculateStability(sensors.temperature)
      },
      moisture_trend: {
        current: sensors.moisture?.value,
        loss_rate: this.calculateMoistureLoss(sensors.moisture, time)
      },
      color_development: {
        current_rgb: sensors.color,
        browning_rate: this.calculateBrowningRate(sensors.color, time)
      },
      texture_formation: {
        hardness: sensors.texture?.hardness,
        crispness: sensors.texture?.crispness
      }
    };
  }
  
  generateRecommendations(prediction) {
    const recommendations = [];
    
    if (prediction.quality < 0.8) {
      if (prediction.issues.includes('uneven_browning')) {
        recommendations.push({
          action: 'rotate',
          timing: 'immediately',
          reason: 'Ensure even browning'
        });
      }
      
      if (prediction.issues.includes('moisture_loss')) {
        recommendations.push({
          action: 'cover',
          timing: 'next_5_minutes',
          reason: 'Prevent excessive drying'
        });
      }
    }
    
    return recommendations;
  }
}
```

## Best Practices

### 1. Sensor Placement

```yaml
sensor_placement_guidelines:
  temperature:
    internal:
      - position: "geometric_center"
      - depth: "minimum_25mm"
      - avoid: "bones, fat_pockets"
    surface:
      - position: "center_of_mass"
      - contact: "full_thermal_contact"
    ambient:
      - position: "away_from_heating_elements"
      - shielding: "radiation_shield"
  
  color:
    optimal_distance: "10-30cm"
    angle: "45_degrees"
    lighting: "consistent_diffuse"
    background: "neutral_non_reflective"
  
  moisture:
    surface:
      - depth: "2-5mm"
      - avoid: "crusts_and_edges"
    internal:
      - position: "center_mass"
      - seal: "prevent_steam_escape"
```

### 2. Data Quality

```python
class DataQualityManager:
    """Ensure sensor data quality"""
    
    def validate_reading(self, sensor_type: str, reading: dict) -> bool:
        """Validate sensor reading"""
        
        # Check range
        if not self.in_valid_range(sensor_type, reading['value']):
            return False
        
        # Check noise level
        if self.is_noisy(sensor_type, reading):
            reading = self.apply_filter(sensor_type, reading)
        
        # Check for sensor drift
        if self.detect_drift(sensor_type, reading):
            self.schedule_recalibration(sensor_type)
        
        # Outlier detection
        if self.is_outlier(sensor_type, reading):
            return False
        
        return True
    
    def apply_filter(self, sensor_type: str, reading: dict) -> dict:
        """Apply appropriate filter"""
        
        filters = {
            'temperature': 'exponential_moving_average',
            'color': 'median_filter',
            'moisture': 'kalman_filter',
            'texture': 'low_pass_filter'
        }
        
        filter_type = filters.get(sensor_type, 'moving_average')
        return self.filter_reading(reading, filter_type)
```

### 3. Redundancy and Failover

```javascript
class SensorRedundancy {
  constructor() {
    this.primarySensors = {};
    this.backupSensors = {};
    this.virtualSensors = {};
  }
  
  async getReading(sensorType) {
    // Try primary sensor
    try {
      const primary = await this.primarySensors[sensorType].read();
      if (this.validateReading(primary)) {
        return primary;
      }
    } catch (error) {
      console.warn(`Primary ${sensorType} sensor failed:`, error);
    }
    
    // Try backup sensor
    if (this.backupSensors[sensorType]) {
      try {
        const backup = await this.backupSensors[sensorType].read();
        if (this.validateReading(backup)) {
          this.notifyPrimarySensorFailure(sensorType);
          return backup;
        }
      } catch (error) {
        console.warn(`Backup ${sensorType} sensor failed:`, error);
      }
    }
    
    // Use virtual sensor (estimation)
    if (this.virtualSensors[sensorType]) {
      const estimate = this.virtualSensors[sensorType].estimate();
      estimate.estimated = true;
      estimate.confidence = 0.7;
      return estimate;
    }
    
    throw new Error(`No ${sensorType} sensor available`);
  }
  
  createVirtualSensor(type) {
    // Create estimation model based on other sensors
    switch(type) {
      case 'temperature':
        return new TemperatureEstimator(this.getAllSensors());
      case 'moisture':
        return new MoistureEstimator(this.getAllSensors());
      default:
        return null;
    }
  }
}
```

### 4. Energy Efficiency

```python
class EfficientSensorManager:
    """Manage sensors for energy efficiency"""
    
    def __init__(self):
        self.sensor_states = {}
        self.sampling_rates = {}
    
    def optimize_sampling(self, recipe_phase: str, sensor_priorities: dict):
        """Adjust sampling rates based on cooking phase"""
        
        sampling_profiles = {
            'preheat': {
                'temperature': 1,  # Hz
                'color': 0,  # Disabled
                'moisture': 0.1
            },
            'active_cooking': {
                'temperature': 10,
                'color': 2,
                'moisture': 1
            },
            'critical_phase': {
                'temperature': 20,
                'color': 10,
                'moisture': 5
            },
            'resting': {
                'temperature': 0.1,
                'color': 0,
                'moisture': 0
            }
        }
        
        profile = sampling_profiles.get(recipe_phase, {})
        
        for sensor_type, rate in profile.items():
            if sensor_priorities.get(sensor_type) == 'required':
                self.set_sampling_rate(sensor_type, rate)
            elif sensor_priorities.get(sensor_type) == 'optional' and rate > 1:
                # Reduce optional sensor rates
                self.set_sampling_rate(sensor_type, rate * 0.5)
            elif rate == 0:
                self.disable_sensor(sensor_type)
```

## Troubleshooting

### Common Sensor Issues

| Issue | Symptoms | Solution |
|-------|----------|----------|
| Drift | Gradual change in readings | Recalibrate sensor |
| Noise | Erratic fluctuations | Add filtering, check connections |
| Lag | Slow response time | Increase sampling rate, check sensor type |
| Offset | Consistent error | Apply calibration offset |
| Failure | No readings | Check power, connections, replace sensor |

### Diagnostic Tools

```javascript
class SensorDiagnostics {
  async runDiagnostics(sensorId) {
    const tests = [
      this.testConnectivity,
      this.testRange,
      this.testNoise,
      this.testResponseTime,
      this.testAccuracy
    ];
    
    const results = [];
    for (const test of tests) {
      const result = await test.call(this, sensorId);
      results.push(result);
    }
    
    return {
      sensorId,
      timestamp: new Date().toISOString(),
      tests: results,
      overall: results.every(r => r.passed) ? 'PASS' : 'FAIL'
    };
  }
  
  async testResponseTime(sensorId) {
    const sensor = this.getSensor(sensorId);
    const startTime = performance.now();
    
    // Apply step change
    await sensor.applyStimulus({type: 'step', magnitude: 10});
    
    // Measure time to 63.2% of final value (1 time constant)
    let reading;
    do {
      reading = await sensor.read();
    } while (reading.value < 6.32 && performance.now() - startTime < 10000);
    
    const responseTime = performance.now() - startTime;
    
    return {
      test: 'response_time',
      passed: responseTime < sensor.spec.max_response_time_ms,
      value: responseTime,
      unit: 'ms'
    };
  }
}
```

---

*Created by Alexey Kozlov - [al7koz.com](https://al7koz.com)*
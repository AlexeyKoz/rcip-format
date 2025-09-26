# RCIP Device Profiles Documentation

## Table of Contents
1. [Overview](#overview)
2. [Device Profile Structure](#device-profile-structure)
3. [Standard Device Types](#standard-device-types)
4. [Implementation Examples](#implementation-examples)
5. [Communication Protocols](#communication-protocols)
6. [Best Practices](#best-practices)

## Overview

Device profiles in RCIP enable recipes to communicate with smart kitchen appliances, providing precise control and automation capabilities. Each profile defines the capabilities, parameters, and control interfaces for specific devices.

### Key Benefits

- **Precision**: Exact temperature, time, and speed control
- **Reproducibility**: Consistent results across different kitchens
- **Automation**: Hands-free cooking with smart appliances
- **Adaptation**: Alternative device suggestions
- **Safety**: Built-in limits and warnings

## Device Profile Structure

### Basic Structure

```json
{
  "id": "device-unique-id",
  "type": "device-type",
  "name": "Human-readable device name",
  "manufacturer": "Device manufacturer",
  "model": "Model number",
  "params": {
    // Device-specific parameters
  },
  "capabilities": {
    // Device capabilities and limits
  },
  "api_endpoint": "https://api.device.com/control",
  "connection": {
    "type": "wifi|bluetooth|zigbee|local",
    "requirements": {}
  },
  "since_version": "0.1"
}
```

### Required Fields

- `id`: Unique identifier for the device profile
- `type`: Standard device type category
- `params`: Operating parameters for the recipe step

### Optional Fields

- `name`: Human-readable device name
- `manufacturer`: Device manufacturer
- `model`: Specific model number
- `capabilities`: Device capabilities and constraints
- `api_endpoint`: API endpoint for control
- `connection`: Connection requirements

## Standard Device Types

### 1. Oven (`oven`)

Standard oven including conventional, convection, and combination ovens.

```json
{
  "id": "oven-01",
  "type": "oven",
  "name": "Convection Oven",
  "manufacturer": "BoschKitchen",
  "model": "HBG675BS1",
  "params": {
    "temperature_c": 180,
    "mode": "convection",
    "fan_speed": "medium",
    "rack_position": "middle",
    "preheat": true,
    "steam_level": 0
  },
  "capabilities": {
    "temperature_range_c": {"min": 30, "max": 300},
    "modes": ["conventional", "convection", "grill", "steam", "combination"],
    "rack_positions": ["top", "middle", "bottom"],
    "capacity_liters": 71,
    "steam_injection": true,
    "probe_temp": true
  },
  "api_endpoint": "https://api.bosch-home.com/oven/control"
}
```

**Common Parameters:**
- `temperature_c/f/k`: Target temperature
- `mode`: Heating mode
- `time_minutes`: Cooking duration
- `preheat`: Whether to preheat
- `rack_position`: Rack placement
- `steam_level`: Steam injection level (0-100)

### 2. Stovetop (`stovetop`)

Gas, electric, or induction cooktops.

```json
{
  "id": "stovetop-01",
  "type": "stovetop",
  "name": "Induction Cooktop",
  "manufacturer": "Samsung",
  "model": "NZ64T3707AK",
  "params": {
    "burner": "front-left",
    "power_level": 7,
    "temperature_c": 120,
    "mode": "maintain-temp"
  },
  "capabilities": {
    "burner_count": 4,
    "power_levels": 9,
    "temperature_control": true,
    "boost_function": true,
    "bridge_function": true,
    "power_watts": {"min": 100, "max": 3700}
  }
}
```

**Common Parameters:**
- `burner`: Burner identifier
- `power_level`: Heat level (1-10 or percentage)
- `temperature_c`: Target temperature (if supported)
- `mode`: Control mode (power/temperature)

### 3. Mixer (`mixer`)

Stand mixers and hand mixers.

```json
{
  "id": "mixer-01",
  "type": "mixer",
  "name": "Stand Mixer",
  "manufacturer": "KitchenAid",
  "model": "Professional 600",
  "params": {
    "attachment": "dough-hook",
    "speed": 4,
    "time_minutes": 8,
    "pulse_pattern": null
  },
  "capabilities": {
    "speed_levels": 10,
    "attachments": ["paddle", "whisk", "dough-hook"],
    "bowl_capacity_liters": 5.7,
    "motor_watts": 575,
    "planetary_action": true,
    "timer": true
  }
}
```

**Common Parameters:**
- `attachment`: Active attachment type
- `speed`: Mixing speed (1-10 or RPM)
- `time_minutes`: Mix duration
- `pulse_pattern`: Pulse mixing pattern

### 4. Blender (`blender`)

Blenders and food processors.

```json
{
  "id": "blender-01",
  "type": "blender",
  "name": "High-Speed Blender",
  "manufacturer": "Vitamix",
  "model": "A3500",
  "params": {
    "speed_percent": 50,
    "time_seconds": 60,
    "program": "smoothie",
    "pulse": false
  },
  "capabilities": {
    "speed_range_rpm": {"min": 0, "max": 22000},
    "programs": ["smoothie", "soup", "frozen", "puree", "clean"],
    "capacity_liters": 2.0,
    "variable_speed": true,
    "pulse_function": true,
    "heating_capable": true
  }
}
```

### 5. Sous Vide (`sous_vide`)

Immersion circulators and water baths.

```json
{
  "id": "sousvide-01",
  "type": "sous_vide",
  "name": "Immersion Circulator",
  "manufacturer": "Anova",
  "model": "Precision Cooker Pro",
  "params": {
    "temperature_c": 56.5,
    "time_minutes": 120,
    "circulation_speed": "normal"
  },
  "capabilities": {
    "temperature_range_c": {"min": 5, "max": 95},
    "precision_c": 0.1,
    "circulation_lpm": 8,
    "capacity_liters": {"min": 5, "max": 20},
    "wifi_enabled": true
  }
}
```

### 6. Pressure Cooker (`pressure_cooker`)

Electric and stovetop pressure cookers.

```json
{
  "id": "pressure-01",
  "type": "pressure_cooker",
  "name": "Electric Pressure Cooker",
  "manufacturer": "Instant Pot",
  "model": "Duo Plus",
  "params": {
    "pressure_level": "high",
    "time_minutes": 12,
    "natural_release_minutes": 10,
    "keep_warm": true
  },
  "capabilities": {
    "pressure_levels": ["low", "high"],
    "capacity_liters": 6,
    "programs": ["rice", "soup", "meat", "bean", "saute", "steam"],
    "delay_start": true,
    "keep_warm_hours": 10
  }
}
```

### 7. Scale (`scale`)

Digital kitchen scales.

```json
{
  "id": "scale-01",
  "type": "scale",
  "name": "Smart Scale",
  "manufacturer": "OXO",
  "model": "11214800",
  "params": {
    "unit": "g",
    "tare": true,
    "target_weight": 500
  },
  "capabilities": {
    "max_weight_g": 5000,
    "precision_g": 0.1,
    "units": ["g", "kg", "oz", "lb"],
    "auto_tare": true,
    "bluetooth": true
  }
}
```

### 8. Thermometer (`thermometer`)

Digital temperature probes.

```json
{
  "id": "thermometer-01",
  "type": "thermometer",
  "name": "Wireless Meat Thermometer",
  "manufacturer": "MEATER",
  "model": "Plus",
  "params": {
    "target_temp_c": 63,
    "alarm_enabled": true,
    "placement": "center"
  },
  "capabilities": {
    "temperature_range_c": {"min": -30, "max": 275},
    "precision_c": 0.5,
    "wireless": true,
    "dual_sensor": true,
    "ambient_monitoring": true
  }
}
```

### 9. Food Printer (`printer_3d_food`)

3D food printers for precision decoration.

```json
{
  "id": "printer-01",
  "type": "printer_3d_food",
  "name": "3D Food Printer",
  "manufacturer": "Foodini",
  "model": "Commercial",
  "params": {
    "material": "chocolate",
    "temperature_c": 32,
    "layer_height_mm": 0.5,
    "print_speed_mm_s": 20,
    "pattern_file": "decoration_01.stl"
  },
  "capabilities": {
    "materials": ["chocolate", "sugar", "paste", "puree"],
    "build_volume_mm": {"x": 200, "y": 200, "z": 50},
    "nozzle_sizes_mm": [0.5, 1.0, 1.5, 2.0],
    "heated_bed": true,
    "multi_material": true
  }
}
```

### 10. Robot Arm (`robot_arm`)

Robotic kitchen assistants.

```json
{
  "id": "robot-01",
  "type": "robot_arm",
  "name": "Kitchen Robot",
  "manufacturer": "Moley",
  "model": "MK1",
  "params": {
    "action": "stir",
    "speed_rpm": 60,
    "pattern": "circular",
    "force_n": 5
  },
  "capabilities": {
    "degrees_of_freedom": 6,
    "reach_mm": 1000,
    "payload_kg": 3,
    "tools": ["spoon", "spatula", "whisk", "tongs"],
    "vision_system": true,
    "force_feedback": true
  }
}
```

## Implementation Examples

### 1. JavaScript Device Controller

```javascript
class DeviceController {
  constructor() {
    this.devices = new Map();
  }
  
  async registerDevice(profile) {
    const device = await this.connectDevice(profile);
    this.devices.set(profile.id, {
      profile,
      connection: device,
      status: 'ready'
    });
  }
  
  async executeStep(step, deviceProfileId) {
    const device = this.devices.get(deviceProfileId);
    if (!device) {
      throw new Error(`Device ${deviceProfileId} not found`);
    }
    
    // Map step action to device command
    const command = this.mapStepToCommand(step, device.profile);
    
    // Send command to device
    const result = await this.sendCommand(device, command);
    
    // Monitor execution
    return this.monitorExecution(device, step.done_when);
  }
  
  mapStepToCommand(step, profile) {
    const commands = {
      'oven': {
        'bake': {
          action: 'start_baking',
          temperature: step.params?.temperature_c || profile.params.temperature_c,
          time: step.params?.time_minutes || profile.params.time_minutes,
          mode: profile.params.mode || 'convection'
        },
        'roast': {
          action: 'start_roasting',
          temperature: step.params?.temperature_c || profile.params.temperature_c,
          time: step.params?.time_minutes
        }
      },
      'mixer': {
        'mix': {
          action: 'start_mixing',
          speed: step.params?.speed || profile.params.speed,
          time: step.params?.time_minutes
        },
        'knead': {
          action: 'start_kneading',
          attachment: 'dough-hook',
          speed: 2,
          time: step.params?.time_minutes || 10
        }
      }
    };
    
    return commands[profile.type]?.[step.action] || {
      action: step.action,
      params: step.params
    };
  }
  
  async sendCommand(device, command) {
    if (device.profile.api_endpoint) {
      // Send via API
      const response = await fetch(device.profile.api_endpoint, {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify(command)
      });
      return response.json();
    } else {
      // Send via local connection
      return device.connection.send(command);
    }
  }
  
  async monitorExecution(device, doneConditions) {
    return new Promise((resolve) => {
      const interval = setInterval(async () => {
        const status = await this.getDeviceStatus(device);
        
        if (this.checkDoneConditions(status, doneConditions)) {
          clearInterval(interval);
          resolve({success: true, finalStatus: status});
        }
        
        if (status.error) {
          clearInterval(interval);
          resolve({success: false, error: status.error});
        }
      }, 1000); // Check every second
    });
  }
  
  checkDoneConditions(status, conditions) {
    if (!conditions) return status.complete;
    
    for (const [key, value] of Object.entries(conditions)) {
      switch (key) {
        case 'temperature_c':
          if (status.temperature < value) return false;
          break;
        case 'time_elapsed_minutes':
          if (status.elapsed < value * 60) return false;
          break;
        case 'color':
          if (!this.colorMatches(status.color, value)) return false;
          break;
      }
    }
    
    return true;
  }
}
```

### 2. Python Device Manager

```python
from typing import Dict, Any, Optional
import asyncio
from abc import ABC, abstractmethod

class BaseDevice(ABC):
    """Base class for all smart devices"""
    
    def __init__(self, profile: Dict[str, Any]):
        self.profile = profile
        self.id = profile['id']
        self.type = profile['type']
        self.connected = False
    
    @abstractmethod
    async def connect(self) -> bool:
        """Establish connection to device"""
        pass
    
    @abstractmethod
    async def execute(self, command: Dict[str, Any]) -> Any:
        """Execute command on device"""
        pass
    
    @abstractmethod
    async def get_status(self) -> Dict[str, Any]:
        """Get current device status"""
        pass
    
    @abstractmethod
    async def stop(self) -> bool:
        """Stop current operation"""
        pass

class SmartOven(BaseDevice):
    """Smart oven implementation"""
    
    async def connect(self) -> bool:
        # Connect via API or local network
        if 'api_endpoint' in self.profile:
            self.api = OvenAPI(self.profile['api_endpoint'])
            self.connected = await self.api.connect()
        return self.connected
    
    async def execute(self, command: Dict[str, Any]) -> Any:
        if not self.connected:
            await self.connect()
        
        # Map command to oven-specific API
        if command['action'] == 'bake':
            return await self.bake(
                temperature=command['temperature'],
                time=command['time'],
                mode=command.get('mode', 'convection')
            )
    
    async def bake(self, temperature: float, time: int, mode: str) -> Dict:
        """Execute baking program"""
        # Preheat
        await self.api.set_temperature(temperature)
        await self.api.set_mode(mode)
        await self.wait_for_preheat()
        
        # Start baking
        await self.api.start_timer(time)
        
        # Monitor
        return await self.monitor_baking()
    
    async def wait_for_preheat(self):
        """Wait for oven to reach temperature"""
        target = self.profile['params']['temperature_c']
        while True:
            status = await self.get_status()
            if status['temperature'] >= target - 5:
                break
            await asyncio.sleep(5)
    
    async def monitor_baking(self) -> Dict:
        """Monitor baking progress"""
        while True:
            status = await self.get_status()
            
            if status['timer_remaining'] == 0:
                return {'complete': True, 'status': status}
            
            if status.get('error'):
                return {'complete': False, 'error': status['error']}
            
            await asyncio.sleep(10)
    
    async def get_status(self) -> Dict[str, Any]:
        """Get oven status"""
        return await self.api.get_status()
    
    async def stop(self) -> bool:
        """Stop oven"""
        return await self.api.stop()

class DeviceManager:
    """Manage multiple devices"""
    
    def __init__(self):
        self.devices: Dict[str, BaseDevice] = {}
        self.device_classes = {
            'oven': SmartOven,
            'mixer': SmartMixer,
            'sous_vide': SousVideDevice,
            'scale': SmartScale
        }
    
    async def register_device(self, profile: Dict[str, Any]) -> bool:
        """Register a new device"""
        device_type = profile['type']
        
        if device_type not in self.device_classes:
            raise ValueError(f"Unknown device type: {device_type}")
        
        device_class = self.device_classes[device_type]
        device = device_class(profile)
        
        if await device.connect():
            self.devices[profile['id']] = device
            return True
        return False
    
    async def execute_recipe_step(
        self, 
        step: Dict[str, Any], 
        device_id: str
    ) -> Any:
        """Execute a recipe step on specified device"""
        
        if device_id not in self.devices:
            raise ValueError(f"Device {device_id} not registered")
        
        device = self.devices[device_id]
        
        # Create command from step
        command = self.create_command(step, device.profile)
        
        # Execute and monitor
        result = await device.execute(command)
        
        # Check done conditions
        if 'done_when' in step:
            result = await self.monitor_done_conditions(
                device, 
                step['done_when']
            )
        
        return result
    
    def create_command(
        self, 
        step: Dict[str, Any], 
        profile: Dict[str, Any]
    ) -> Dict:
        """Create device command from step"""
        
        command = {
            'action': step['action'],
            'params': {}
        }
        
        # Merge step params with device profile params
        if 'params' in profile:
            command['params'].update(profile['params'])
        
        if 'params' in step:
            command['params'].update(step['params'])
        
        return command
    
    async def monitor_done_conditions(
        self, 
        device: BaseDevice, 
        conditions: Dict[str, Any]
    ) -> Dict:
        """Monitor device until done conditions are met"""
        
        while True:
            status = await device.get_status()
            
            if self.check_conditions(status, conditions):
                return {'complete': True, 'status': status}
            
            if status.get('error'):
                return {'complete': False, 'error': status['error']}
            
            await asyncio.sleep(1)
    
    def check_conditions(
        self, 
        status: Dict[str, Any], 
        conditions: Dict[str, Any]
    ) -> bool:
        """Check if status meets conditions"""
        
        for key, expected in conditions.items():
            if key not in status:
                continue
            
            if key == 'temperature_c':
                if status[key] < expected:
                    return False
            
            elif key == 'color':
                if not self.color_matches(status.get('color'), expected):
                    return False
            
            elif key == 'texture':
                if status.get('texture') != expected:
                    return False
        
        return True
```

### 3. Device Discovery

```javascript
class DeviceDiscovery {
  constructor() {
    this.discoveredDevices = [];
  }
  
  async scanNetwork() {
    const devices = [];
    
    // Scan for WiFi devices
    devices.push(...await this.scanWiFi());
    
    // Scan for Bluetooth devices
    devices.push(...await this.scanBluetooth());
    
    // Scan for Zigbee devices
    devices.push(...await this.scanZigbee());
    
    // Identify and profile devices
    this.discoveredDevices = await Promise.all(
      devices.map(d => this.profileDevice(d))
    );
    
    return this.discoveredDevices;
  }
  
  async profileDevice(device) {
    // Query device capabilities
    const capabilities = await device.getCapabilities();
    
    // Map to RCIP profile
    return {
      id: `auto-${device.id}`,
      type: this.mapDeviceType(device.type),
      name: device.name,
      manufacturer: device.manufacturer,
      model: device.model,
      capabilities: capabilities,
      connection: {
        type: device.connectionType,
        address: device.address
      },
      autoDiscovered: true
    };
  }
  
  matchRecipeRequirements(recipe) {
    const required = new Set();
    const alternatives = {};
    
    // Scan recipe for device requirements
    recipe.steps.forEach(step => {
      if (step.device_profile_ref) {
        required.add(step.device_profile_ref);
      }
    });
    
    recipe.device_profiles?.forEach(profile => {
      if (required.has(profile.id)) {
        // Find matching discovered device
        const matches = this.discoveredDevices.filter(d => 
          this.isCompatible(d, profile)
        );
        
        if (matches.length > 0) {
          alternatives[profile.id] = matches;
        }
      }
    });
    
    return {
      required: Array.from(required),
      available: alternatives,
      missing: Array.from(required).filter(id => !alternatives[id])
    };
  }
  
  isCompatible(device, profile) {
    // Check type match
    if (device.type !== profile.type) return false;
    
    // Check capabilities
    if (profile.capabilities) {
      for (const [key, value] of Object.entries(profile.capabilities)) {
        if (!this.meetsCapability(device.capabilities?.[key], value)) {
          return false;
        }
      }
    }
    
    // Check specific parameters
    if (profile.params?.temperature_c) {
      const range = device.capabilities?.temperature_range_c;
      if (!range || 
          profile.params.temperature_c < range.min || 
          profile.params.temperature_c > range.max) {
        return false;
      }
    }
    
    return true;
  }
}
```

## Communication Protocols

### REST API Integration

```javascript
class RESTDeviceAPI {
  constructor(baseURL, authToken) {
    this.baseURL = baseURL;
    this.headers = {
      'Authorization': `Bearer ${authToken}`,
      'Content-Type': 'application/json'
    };
  }
  
  async sendCommand(deviceId, command) {
    const response = await fetch(`${this.baseURL}/devices/${deviceId}/command`, {
      method: 'POST',
      headers: this.headers,
      body: JSON.stringify(command)
    });
    
    if (!response.ok) {
      throw new Error(`Device command failed: ${response.statusText}`);
    }
    
    return response.json();
  }
  
  async getStatus(deviceId) {
    const response = await fetch(`${this.baseURL}/devices/${deviceId}/status`, {
      headers: this.headers
    });
    
    return response.json();
  }
  
  async startProgram(deviceId, program) {
    return this.sendCommand(deviceId, {
      action: 'start_program',
      program: program
    });
  }
}
```

### MQTT Integration

```python
import paho.mqtt.client as mqtt
import json

class MQTTDeviceClient:
    """MQTT client for device communication"""
    
    def __init__(self, broker_url: str, port: int = 1883):
        self.broker_url = broker_url
        self.port = port
        self.client = mqtt.Client()
        self.client.on_connect = self.on_connect
        self.client.on_message = self.on_message
        self.device_status = {}
    
    def connect(self):
        """Connect to MQTT broker"""
        self.client.connect(self.broker_url, self.port, 60)
        self.client.loop_start()
    
    def on_connect(self, client, userdata, flags, rc):
        """Callback for connection"""
        if rc == 0:
            # Subscribe to device status topics
            client.subscribe("devices/+/status")
    
    def on_message(self, client, userdata, msg):
        """Handle incoming messages"""
        topic_parts = msg.topic.split('/')
        if len(topic_parts) >= 3:
            device_id = topic_parts[1]
            message_type = topic_parts[2]
            
            if message_type == 'status':
                self.device_status[device_id] = json.loads(msg.payload)
    
    def send_command(self, device_id: str, command: dict):
        """Send command to device"""
        topic = f"devices/{device_id}/command"
        payload = json.dumps(command)
        self.client.publish(topic, payload)
    
    def get_status(self, device_id: str) -> dict:
        """Get device status"""
        return self.device_status.get(device_id, {})
```

### WebSocket Real-time Control

```javascript
class WebSocketDeviceControl {
  constructor(url) {
    this.url = url;
    this.ws = null;
    this.handlers = new Map();
  }
  
  connect() {
    return new Promise((resolve, reject) => {
      this.ws = new WebSocket(this.url);
      
      this.ws.onopen = () => {
        console.log('WebSocket connected');
        resolve();
      };
      
      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        reject(error);
      };
      
      this.ws.onmessage = (event) => {
        this.handleMessage(JSON.parse(event.data));
      };
    });
  }
  
  handleMessage(message) {
    const { type, deviceId, data } = message;
    
    switch (type) {
      case 'status':
        this.handlers.get('status')?.(deviceId, data);
        break;
      
      case 'complete':
        this.handlers.get('complete')?.(deviceId, data);
        break;
      
      case 'error':
        this.handlers.get('error')?.(deviceId, data);
        break;
    }
  }
  
  sendCommand(deviceId, command) {
    this.ws.send(JSON.stringify({
      type: 'command',
      deviceId,
      command
    }));
  }
  
  onStatus(handler) {
    this.handlers.set('status', handler);
  }
  
  onComplete(handler) {
    this.handlers.set('complete', handler);
  }
  
  onError(handler) {
    this.handlers.set('error', handler);
  }
}
```

## Best Practices

### 1. Fallback Profiles

Always provide alternative device profiles:

```json
{
  "device_profiles": [
    {
      "id": "primary-oven",
      "type": "oven",
      "name": "Smart Oven (Preferred)",
      "params": {
        "temperature_c": 180,
        "mode": "convection",
        "time_minutes": 25
      }
    },
    {
      "id": "alt-oven",
      "type": "oven", 
      "name": "Conventional Oven",
      "params": {
        "temperature_c": 190,
        "mode": "conventional",
        "time_minutes": 30
      },
      "notes": "Add 10°C for conventional oven"
    },
    {
      "id": "alt-toaster",
      "type": "toaster_oven",
      "name": "Toaster Oven Alternative",
      "params": {
        "temperature_c": 175,
        "time_minutes": 20
      },
      "notes": "Check every 5 minutes"
    }
  ]
}
```

### 2. Safety Limits

Include safety constraints in capabilities:

```json
{
  "capabilities": {
    "temperature_range_c": {"min": 30, "max": 250},
    "max_runtime_minutes": 180,
    "safety_shutoff": true,
    "child_lock": true,
    "overheat_protection": true,
    "max_pressure_bar": 2.5
  }
}
```

### 3. Error Handling

```javascript
class SafeDeviceController {
  async executeWithSafety(step, device) {
    try {
      // Validate parameters against capabilities
      this.validateParameters(step.params, device.capabilities);
      
      // Set safety timeout
      const timeout = setTimeout(() => {
        this.emergencyStop(device);
      }, (device.capabilities.max_runtime_minutes || 180) * 60000);
      
      // Execute
      const result = await this.execute(step, device);
      
      clearTimeout(timeout);
      return result;
      
    } catch (error) {
      await this.emergencyStop(device);
      throw error;
    }
  }
  
  validateParameters(params, capabilities) {
    if (params.temperature_c) {
      const range = capabilities.temperature_range_c;
      if (params.temperature_c < range.min || params.temperature_c > range.max) {
        throw new Error(`Temperature ${params.temperature_c}°C out of range`);
      }
    }
    
    if (params.pressure_bar) {
      if (params.pressure_bar > capabilities.max_pressure_bar) {
        throw new Error(`Pressure ${params.pressure_bar} exceeds maximum`);
      }
    }
  }
  
  async emergencyStop(device) {
    console.error('Emergency stop activated');
    await device.stop();
    await device.disconnect();
  }
}
```

### 4. Status Monitoring

```python
class DeviceMonitor:
    """Monitor device status and health"""
    
    def __init__(self, check_interval: int = 5):
        self.check_interval = check_interval
        self.devices = {}
        self.alerts = []
    
    async def monitor_device(self, device: BaseDevice):
        """Monitor a device continuously"""
        device_id = device.profile['id']
        
        while device_id in self.devices:
            try:
                status = await device.get_status()
                
                # Check for issues
                if status.get('error'):
                    await self.handle_error(device, status['error'])
                
                if status.get('temperature_c'):
                    await self.check_temperature(device, status['temperature_c'])
                
                if status.get('runtime_minutes'):
                    await self.check_runtime(device, status['runtime_minutes'])
                
                # Store status
                self.devices[device_id]['last_status'] = status
                self.devices[device_id]['last_update'] = datetime.now()
                
            except Exception as e:
                await self.handle_error(device, str(e))
            
            await asyncio.sleep(self.check_interval)
    
    async def check_temperature(self, device: BaseDevice, temp: float):
        """Check temperature limits"""
        capabilities = device.profile.get('capabilities', {})
        temp_range = capabilities.get('temperature_range_c', {})
        
        if temp_range:
            if temp > temp_range.get('max', 999):
                await self.alert('OVERHEAT', device, f"Temperature {temp}°C exceeds maximum")
            elif temp < temp_range.get('min', -999):
                await self.alert('UNDERHEAT', device, f"Temperature {temp}°C below minimum")
```

### 5. Multi-Device Coordination

```javascript
class RecipeOrchestrator {
  async executeRecipe(recipe) {
    // Analyze device dependencies
    const timeline = this.createTimeline(recipe);
    
    // Start devices in parallel where possible
    const tasks = timeline.map(async (phase) => {
      await Promise.all(
        phase.steps.map(step => 
          this.executeStep(step, step.device_profile_ref)
        )
      );
    });
    
    // Execute phases in sequence
    for (const task of tasks) {
      await task;
    }
  }
  
  createTimeline(recipe) {
    const phases = [];
    const deviceBusy = new Set();
    
    recipe.steps.forEach(step => {
      const device = step.device_profile_ref;
      
      // Check if device is available
      if (device && !device
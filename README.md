# RCIP - Recipe Interchange Protocol (Universal Recipe Format)

[![Version](https://img.shields.io/badge/RCIP-v0.1-blue.svg)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Created by](https://img.shields.io/badge/Created%20by-Alexey%20Kozlov-green.svg)]()

RCIP (Recipe Interchange Protocol) is a comprehensive, machine-readable, and human-friendly format for storing and exchanging culinary recipes. Designed for the modern smart kitchen era.

## 🎯 Key Features

- **Machine & Human Readable** - Dual measurement system
- **Version Controlled** - Built-in backward compatibility
- **IoT Ready** - Device profiles and sensor integration
- **Internationally Standardized** - External IDs (GTIN, USDA, OpenFoodFacts)
- **Safety First** - Mandatory allergen declarations
- **Automation Compatible** - Robot-ready instructions

## 🚀 Quick Start
```json
{
  "rcip_version": "0.1",
  "id": "rcip-550e8400-e29b-41d4-a716-446655440001",
  "meta": {
    "name": "Classic Margherita Pizza",
    "author": "Chef Antonio Rossi"
  }
  // ... full recipe structure
}

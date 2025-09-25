# Changelog

All notable changes to the RCIP (Recipe Interchange Protocol) Format will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned Features
- Blockchain integration for recipe authenticity verification
- AI-powered instruction generation and optimization
- Multi-recipe meal planning with dependency graphs
- Real-time collaboration for recipe development
- Supply chain integration with automatic ordering
- Augmented reality (AR) cooking guidance markers
- Voice assistant integration protocols
- Nutritional optimization algorithms
- Carbon footprint automatic calculation
- Wine/beverage pairing specifications

---

## [0.1.0] - 2025-01-15

**Initial Release - Created by Alexey Kozlov**

### Added

#### Core Structure
- **Recipe Format Foundation** - Complete JSON-based recipe interchange format
- **Unique Identification** - UUID-based recipe ID system (`rcip-{UUID}`)
- **Metadata System** - Comprehensive recipe metadata including author, origin, and licensing
- **Dual Measurement System** - Human-readable and machine-precise measurements
- **Version Control** - Built-in versioning with `since_version` and `deprecated_in` fields

#### Safety Features
- **Mandatory Allergen Declarations** - Required allergen information for all ingredients
- **Standardized Allergen List** - 15 internationally recognized allergens (FDA/EU compliant)
- **Hazard Warnings** - Safety hazards in cooking steps (hot surfaces, sharp tools)
- **Temperature Requirements** - Specific temperature ranges for ingredients and cooking

#### Ingredients
- **Ingredient Identification** - Unique IDs for each ingredient (`ing-XXXX` pattern)
- **External Database Links** - Support for GTIN, USDA, OpenFoodFacts IDs
- **Nutritional Information** - Detailed per-100g nutritional data structure
- **Substitution System** - Alternative ingredients with conversion ratios
- **State Specifications** - Physical state descriptions (fresh, frozen, dried, etc.)
- **Brand Preferences** - Optional brand recommendations

#### Cooking Instructions
- **Step-by-Step Structure** - Unique step IDs with dependencies
- **Canonical Action Verbs** - 30+ standardized cooking actions
- **Parameter Precision** - Structured parameters for time, temperature, speed
- **Completion Criteria** - "Done when" conditions for quality control
- **Tolerance Ranges** - Acceptable variations in parameters
- **Result References** - Ability to reference previous step outputs

#### Device Integration
- **Device Profiles** - Specifications for ovens, mixers, scales, etc.
- **10 Device Types** - Standard categories for kitchen equipment
- **Manufacturer Support** - Model-specific parameters and capabilities
- **API Endpoints** - Integration with smart device APIs
- **Alternative Devices** - Fallback options for different equipment

#### Sensor Specifications
- **10 Sensor Types** - Color, temperature, moisture, texture, aroma, etc.
- **RGB/HSV Color** - Precise color matching for doneness
- **Moisture Detection** - Humidity and moisture content monitoring
- **Texture Analysis** - Hardness, elasticity, crispness measurements
- **Aroma Profiles** - VOC compound detection for smell
- **Calibration Standards** - Reference standards for sensor accuracy

#### Data Formats
- **JSON Primary** - Native JSON format with schema validation
- **CBOR Binary** - Efficient binary representation
- **XML Support** - XML transformation capability
- **YAML Support** - Human-friendly YAML format
- **Compression** - Brotli compression for `.rcipz` files

#### Standards Compliance
- **ISO 8601 Dates** - Standard date/time formatting
- **ISO 3166 Countries** - Standard country codes
- **UTF-8 Encoding** - Full Unicode support
- **Semantic Versioning** - Version 2.0.0 compliance
- **JSON Schema** - Draft 2020-12 validation

#### Units System
- **Metric Units** - mg, g, kg, ml, l
- **Imperial Units** - oz, lb, tsp, tbsp, cup, fl-oz, pt, qt, gal
- **Count Units** - pieces, dozen
- **Special Units** - pinch, dash, handful, to-taste
- **Temperature** - Celsius, Fahrenheit, Kelvin support

#### Diet Classifications
- **14 Diet Labels** - Vegetarian, vegan, gluten-free, kosher, halal, etc.
- **Standardized Terms** - Consistent dietary restriction labeling
- **Multiple Labels** - Support for multiple dietary classifications

#### Visual Content
- **Image Support** - Base64 encoded images with captions
- **WebP Format** - Efficient image compression
- **Multiple Images** - Stage-specific photography
- **QR Codes** - Recipe ID encoding for scanning

#### Compatibility Features
- **Schema.org** - JSON-LD compatibility flag
- **Forward Compatible** - Unknown field preservation
- **Archive Formats** - `.rcipa` for bundled media
- **Digital Signatures** - Optional cryptographic signing
- **License Field** - Creative Commons and other licenses

### Technical Specifications
- **MIME Type**: `application/vnd.rcip+json`
- **File Extensions**: `.rcip`, `.rcipz`, `.rcipx`, `.rcipy`
- **Schema Location**: `/schemas/rcip-v0.1.json`
- **Minimum Valid Recipe**: 7 required fields
- **Maximum Nesting**: No limit specified

### Documentation
- Complete specification document (SPECIFICATION.md)
- JSON Schema for validation
- Reference examples (Margherita Pizza, Ukrainian Borscht)
- Visual HTML representation
- Implementation guidelines

### Security
- Optional encryption for proprietary recipes
- SHA-256 hash verification support
- Privacy considerations for author information
- Data integrity through digital signatures

### Known Limitations (v0.1)
- No multi-recipe dependencies
- No cost calculation fields
- No inventory management
- No scaling algorithms
- No video content support
- Limited to single-language content
- No accessibility metadata
- No environmental impact metrics (planned for v0.2)

---

## [0.0.1-alpha] - 2024-12-01 (Internal)

### Added
- Initial concept development
- Basic JSON structure
- Preliminary field definitions

### Notes
- Internal prototype version
- Not released publicly
- Used for concept validation

---

## Upgrade Guide

### From 0.0.1-alpha to 0.1.0
This is a complete rewrite. No migration path available.

### Future Version Compatibility
Starting from v0.1.0, all minor versions will be backward compatible within the same major version.

#### Breaking Changes Policy
- Breaking changes only in major versions (1.0.0, 2.0.0)
- Deprecated features supported for 2 major versions
- Migration tools provided for major upgrades

---

## Version History Summary

| Version | Date | Status | Key Features |
|---------|------|--------|--------------|
| 0.1.0 | 2025-01-15 | **Current** | Initial public release |
| 0.0.1-alpha | 2024-12-01 | Deprecated | Internal prototype |

---

## Roadmap

### Version 0.2.0 (Q2 2025)
- [ ] Environmental impact calculations
- [ ] Multi-language support
- [ ] Recipe scaling algorithms
- [ ] Cost calculation fields
- [ ] Video content embedding

### Version 0.3.0 (Q3 2025)
- [ ] Inventory management integration
- [ ] Shopping list generation
- [ ] Multi-recipe dependencies
- [ ] Batch cooking specifications
- [ ] Restaurant-scale adaptations

### Version 1.0.0 (Q4 2025)
- [ ] Full API specification
- [ ] Certification program
- [ ] Industry partnerships
- [ ] Reference implementation suite
- [ ] Compliance testing tools

---

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on:
- How to report bugs
- How to suggest enhancements
- How to submit changes
- Code of conduct

## Support

For questions and support:
- GitHub Issues: https://github.com/alexey-kozlov/rcip-format/issues
- Email: rcip-format@example.com
- Discord: [Join our community](https://discord.gg/rcip-format)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**RCIP Format created by Alexey Kozlov**  
*Making recipes universal, precise, and future-ready since 2025*

[Unreleased]: https://github.com/alexey-kozlov/rcip-format/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/alexey-kozlov/rcip-format/releases/tag/v0.1.0
[0.0.1-alpha]: Internal version, not publicly released

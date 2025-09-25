# Contributing to RCIP Format

First off, thank you for considering contributing to RCIP (Recipe Interchange Protocol)! 🎉

RCIP is an open standard created by Alexey Kozlov to make recipes universal, precise, and future-ready. This document provides guidelines for contributing to the RCIP format specification and its ecosystem.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [Style Guidelines](#style-guidelines)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [Community](#community)

## Code of Conduct

### Our Pledge

We are committed to making participation in this project a harassment-free experience for everyone, regardless of:
- Level of experience
- Gender identity and expression
- Sexual orientation
- Disability
- Personal appearance
- Body size
- Race, ethnicity, or religion
- Nationality
- Age

### Our Standards

**Examples of positive behavior:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

**Examples of unacceptable behavior:**
- Harassment of any kind
- Discriminatory jokes and language
- Publishing others' private information
- Other conduct which could reasonably be considered inappropriate

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported by contacting the project team at rcip-conduct@example.com. All complaints will be reviewed and investigated promptly and fairly.

## How Can I Contribute?

### 🐛 Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates.

**When filing a bug report, include:**

```markdown
### Bug Description
A clear and concise description of the bug.

### RCIP Version
Version: 0.1.0

### Sample Recipe
```json
{
  "rcip_version": "0.1",
  "id": "rcip-example",
  // Minimal reproducible example
}
```

### Expected Behavior
What you expected to happen.

### Actual Behavior
What actually happened.

### Additional Context
Any other relevant information.
```

### 💡 Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues.

**When creating an enhancement suggestion, include:**

```markdown
### Feature Description
Clear description of the proposed feature.

### Use Case
Explain the problem this feature would solve.

### Proposed Solution
Your idea for implementation.

### Examples
```json
{
  // Example of how the feature would work
}
```

### Alternatives Considered
Other solutions you've thought about.
```

### 📝 Contributing Recipes

We welcome example recipes that demonstrate RCIP features!

**Recipe Requirements:**
- Must validate against the current schema
- Include all mandatory fields
- Demonstrate at least one advanced feature
- Be culturally respectful and authentic
- Include proper attribution

**Recipe Template:**
```json
{
  "rcip_version": "0.1",
  "id": "rcip-{UUID}",
  "meta": {
    "name": "Recipe Name",
    "author": {
      "name": "Your Name",
      "email": "optional@email.com"
    },
    "created_date": "2025-01-15T10:00:00Z",
    "license": "CC-BY-SA-4.0"
  },
  "ingredients": [],
  "steps": []
}
```

### 🔧 Contributing Code

Areas where we need help:
- **Validators** - JavaScript, Python, Rust, Go implementations
- **Converters** - From/to other recipe formats
- **Tools** - CLIs, libraries, plugins
- **Documentation** - Guides, tutorials, translations
- **Examples** - Recipes showcasing features

## Getting Started

### Prerequisites

1. **Git** - [Download Git](https://git-scm.com/)
2. **Node.js** (for validators) - [Download Node.js](https://nodejs.org/)
3. **Python 3.8+** (for Python tools) - [Download Python](https://python.org/)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
```bash
git clone https://github.com/YOUR-USERNAME/rcip-format.git
cd rcip-format
```

3. Add upstream remote:
```bash
git remote add upstream https://github.com/AlexeyKoz/rcip-format.git
```

4. Create a branch:
```bash
git checkout -b feature/your-feature-name
```

## Development Process

### 1. Specification Changes

Changes to the core specification require:
- Discussion in an issue first
- Clear use case demonstration
- Backward compatibility consideration
- Update to version number if breaking
- Schema update
- Example implementation

### 2. Schema Updates

When modifying `/schemas/rcip-v*.json`:
1. Update the schema file
2. Increment version if needed
3. Add migration notes
4. Update examples
5. Run validation tests
6. Update CHANGELOG.md

### 3. Validator Development

```javascript
// JavaScript Validator Structure
class RCIPValidator {
  constructor(schemaVersion = '0.1') {
    this.schema = loadSchema(schemaVersion);
  }
  
  validate(recipe) {
    // Implementation
  }
  
  validateIngredient(ingredient) {
    // Check required fields
    // Validate allergens
    // Verify measurements
  }
  
  validateStep(step) {
    // Validate action verb
    // Check parameters
    // Verify references
  }
}
```

```python
# Python Validator Structure
class RCIPValidator:
    def __init__(self, schema_version='0.1'):
        self.schema = load_schema(schema_version)
    
    def validate(self, recipe: dict) -> ValidationResult:
        # Implementation
        pass
    
    def validate_ingredient(self, ingredient: dict) -> bool:
        # Check required fields
        # Validate allergens
        # Verify measurements
        pass
```

### 4. Testing Requirements

All contributions must include tests:

```bash
# Run JavaScript tests
npm test

# Run Python tests
python -m pytest

# Validate examples
npm run validate-examples
```

## Style Guidelines

### JSON/Recipe Files

- Use 2 spaces for indentation
- Always include trailing commas in arrays
- Order: required fields first, optional after
- Use lowercase for field names with underscores
- Comments should explain "why" not "what"

### Code Style

**JavaScript:**
- ES6+ syntax
- Semicolons required
- Single quotes for strings
- JSDoc comments for functions

**Python:**
- PEP 8 compliance
- Type hints required (3.8+)
- Docstrings for all public methods
- Black formatter recommended

### Documentation

- Use Markdown for all docs
- Include code examples
- Keep line length under 100 characters
- Use present tense
- Be concise but thorough

## Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation only
- `style:` Formatting, missing semicolons, etc.
- `refactor:` Code change that neither fixes a bug nor adds a feature
- `test:` Adding missing tests
- `chore:` Changes to build process or auxiliary tools

### Examples

```bash
feat(validator): add nutritional data validation

- Validate calorie ranges
- Check macronutrient ratios
- Add vitamin/mineral validation

Closes #123
```

```bash
fix(schema): correct temperature unit validation

Temperature can now accept K (Kelvin) in addition to C and F

Fixes #456
```

```bash
docs(examples): add sous vide steak recipe

Demonstrates:
- Precision temperature control
- Sensor integration
- Device profiles
```

## Pull Request Process

### Before Submitting

1. **Update documentation** for any changed functionality
2. **Add tests** for new features
3. **Update CHANGELOG.md** with your changes
4. **Ensure all tests pass**
5. **Update schema version** if needed
6. **Validate all examples** against new changes

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix (non-breaking change)
- [ ] New feature (non-breaking change)
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] All tests pass
- [ ] Examples validate
- [ ] Documentation updated

## Checklist
- [ ] My code follows the style guidelines
- [ ] I have performed a self-review
- [ ] I have commented complex code
- [ ] I have updated the documentation
- [ ] My changes generate no warnings
- [ ] I have added tests that prove my fix/feature works
- [ ] New and existing unit tests pass locally
- [ ] Any dependent changes have been merged

## Related Issues
Closes #(issue number)

## Screenshots (if applicable)
Add screenshots for UI changes
```

### Review Process

1. **Automated checks** run on all PRs (tests, linting, validation)
2. **Code review** by at least one maintainer
3. **Discussion** and improvements if needed
4. **Approval** and merge

### After Merge

- Delete your branch
- Pull upstream changes
- Celebrate your contribution! 🎉

## Project Structure

```
rcip-format/
├── schemas/               # JSON schemas for each version
│   ├── rcip-v0.1.json
│   └── rcip-v0.2.json
├── examples/             # Example recipes
│   ├── basic/           # Simple examples
│   ├── advanced/        # Complex features
│   └── cuisines/        # Cultural varieties
├── validators/           # Validation implementations
│   ├── javascript/
│   ├── python/
│   └── rust/
├── converters/          # Format converters
│   ├── from/           # Import from other formats
│   └── to/             # Export to other formats
├── tools/               # Utility tools
│   ├── cli/            # Command-line tools
│   └── web/            # Web-based tools
├── docs/                # Documentation
│   ├── guides/         # How-to guides
│   ├── api/            # API documentation
│   └── spec/           # Specification details
└── tests/               # Test suites
    ├── fixtures/       # Test data
    └── integration/    # Integration tests
```

## Version Management

### Adding New Fields

For backward-compatible additions:

```json
{
  "new_field": "value",
  "since_version": "0.2"
}
```

### Deprecating Fields

For fields being phased out:

```json
{
  "old_field": "value",
  "deprecated_in": "0.2",
  "migration_note": "Use 'new_field' instead"
}
```

### Breaking Changes

Only allowed in major versions (1.0, 2.0):
1. Create migration guide
2. Provide conversion tool
3. Support old version for 6 months
4. Clear deprecation warnings

## Community

### Communication Channels

- **GitHub Issues** - Bug reports, feature requests
- **GitHub Discussions** - General questions, ideas
- **Discord** - Real-time chat and support
- **Twitter** - [@rcipformat](https://twitter.com/rcipformat) for updates
- **Blog** - [rcip-format.org/blog](https://rcip-format.org/blog)

### Weekly Meetings

- **When**: Thursdays at 16:00 UTC
- **Where**: Discord voice channel
- **Agenda**: Posted in #meeting-agenda
- **Notes**: Published in /docs/meetings/

### Decision Making

1. **Proposal** - Create RFC (Request for Comments) issue
2. **Discussion** - Community feedback for 2 weeks
3. **Revision** - Incorporate feedback
4. **Vote** - Core team approval
5. **Implementation** - PR with changes

## Recognition

### Contributors

All contributors are recognized in:
- AUTHORS.md file
- GitHub contributors page
- Annual contributor report

### Contribution Levels

- 🥉 **Bronze** - First merged PR
- 🥈 **Silver** - 5+ merged PRs
- 🥇 **Gold** - 10+ merged PRs
- 💎 **Diamond** - Core team member
- 🌟 **Founder** - Alexey Kozlov

## Getting Help

### Resources

- **Specification**: [SPECIFICATION.md](SPECIFICATION.md)
- **Examples**: [/examples](./examples)
- **FAQ**: [docs/FAQ.md](docs/FAQ.md)
- **Tutorials**: [docs/tutorials](docs/tutorials)

### Asking Questions

1. Check documentation first
2. Search existing issues
3. Ask in Discord #help channel
4. Create a "Question" issue

### Mentorship

New contributors can request a mentor:
- Add "mentor-wanted" label to your first issue
- Join #mentorship Discord channel
- Attend "New Contributor" monthly calls

## License Agreement

By contributing to RCIP Format, you agree that your contributions will be licensed under the MIT License.

You retain copyright to your contributions while granting the project a perpetual, worldwide, non-exclusive, no-charge, royalty-free license to use, copy, and distribute your contributions.

## Special Thanks

### Core Contributors

- **Alexey Kozlov** (@AlexeyKoz) - Creator and Lead Maintainer
- Your name could be here!

### Sponsors

RCIP Format development is supported by:
- Individual contributors
- Corporate sponsors (contact for sponsorship)

### Technology Partners

- Organizations implementing RCIP
- Academic institutions researching with RCIP
- Companies building on RCIP

---

## Quick Contribution Checklist

Before submitting a PR, ensure:

```markdown
- [ ] Fork is up to date with main branch
- [ ] Changes are in a feature branch
- [ ] Commits follow conventional commits format
- [ ] All tests pass locally
- [ ] Documentation is updated
- [ ] CHANGELOG.md includes your changes
- [ ] PR description is complete
- [ ] Related issues are linked
- [ ] Code follows project style guide
- [ ] Examples validate against schema
```

---

**Thank you for contributing to making recipes universal, precise, and future-ready!** 🚀

For any questions not covered here, please reach out to the maintainers or community.

---

*This document is licensed under [CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/)*

*Last updated: January 2025*

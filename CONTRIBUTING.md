# Contributing to RIM

[中文](CONTRIBUTING_zh.md) | **English**

Thank you for your interest in contributing to RIM! We welcome contributions of all kinds.

## 🤝 Ways to Contribute

- 🐛 **Bug Reports**: Report bugs and issues
- 💡 **Feature Requests**: Suggest new features or improvements
- 📝 **Documentation**: Improve documentation and examples
- 🔧 **Code**: Submit bug fixes and new features
- 🎨 **Design**: UI/UX improvements and visual assets
- 🌍 **Translation**: Help translate the project

## 🚀 Getting Started

### Prerequisites

- Rust 1.85 or later
- Git
- Basic knowledge of Bevy game engine (helpful but not required)

### Setting up Development Environment

1. **Fork and Clone**
   ```bash
   git clone https://github.com/your-username/rim.git
   cd rim
   ```

2. **Install Dependencies**
   ```bash
   cargo build
   ```

3. **Run the Project**
   ```bash
   cargo run
   ```

4. **Run Tests**
   ```bash
   cargo test
   ```

## 📝 Development Guidelines

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy for linting: `cargo clippy`
- Write clear, self-documenting code
- Add comments for complex logic
- Use meaningful variable and function names

### Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(render): add circle rendering support
fix(axes): correct arrow positioning bug
docs: update installation instructions
```

### Pull Request Process

1. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Changes**
   - Write clean, well-documented code
   - Add tests for new functionality
   - Update documentation as needed

3. **Test Your Changes**
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. **Commit Your Changes**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a pull request on GitHub.

### PR Requirements

- [ ] Code follows project style guidelines
- [ ] Tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation is updated if needed
- [ ] PR description clearly explains the changes

## 🐛 Bug Reports

When reporting bugs, please include:

- **Environment**: OS, Rust version, Bevy version
- **Steps to Reproduce**: Clear steps to reproduce the issue
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Screenshots**: If applicable
- **Additional Context**: Any other relevant information

Use our bug report template:

```markdown
**Environment:**
- OS: [e.g., Windows 11, macOS 13, Ubuntu 22.04]
- Rust version: [e.g., 1.85.0]
- RIM version: [e.g., 0.1.0]

**Describe the bug:**
A clear description of what the bug is.

**To Reproduce:**
1. Go to '...'
2. Click on '...'
3. See error

**Expected behavior:**
What you expected to happen.

**Screenshots:**
If applicable, add screenshots.

**Additional context:**
Any other context about the problem.
```

## 💡 Feature Requests

For feature requests, please:

- Check if the feature already exists or is planned
- Describe the problem you're trying to solve
- Explain your proposed solution
- Consider alternative solutions
- Provide examples or mockups if helpful

## 🏗️ Project Structure

```
rim/
├── src/
│   ├── main.rs              # Application entry point
│   ├── math_objects/        # Mathematical objects (axes, shapes, etc.)
│   ├── render/              # Rendering systems
│   ├── animation/           # Animation systems (planned)
│   ├── interaction/         # User interaction handling
│   ├── scene/               # Scene management
│   └── export/              # Export functionality (planned)
├── assets/                  # Game assets (textures, fonts, etc.)
├── docs/                    # Documentation
└── examples/                # Example code
```

## 🧪 Testing

- Write unit tests for new functionality
- Add integration tests for complex features
- Test on multiple platforms when possible
- Include edge cases in your tests

## 📚 Documentation

- Update README.md for user-facing changes
- Add inline documentation for public APIs
- Update DEVELOPMENT.md for development changes
- Include examples in documentation

## 🌍 Internationalization

We support multiple languages:
- English (default)
- Chinese (中文)

When adding UI text:
- Use localization keys instead of hardcoded strings
- Add translations for both English and Chinese
- Test with different languages

## 🎯 Areas Needing Help

We especially welcome contributions in these areas:

- **Mathematical Functions**: Implementing more mathematical objects
- **Animation System**: Building the animation framework
- **Performance**: Optimizing rendering and calculations
- **Documentation**: Improving guides and examples
- **Testing**: Adding comprehensive test coverage
- **UI/UX**: Improving the user interface
- **Cross-platform**: Ensuring compatibility across platforms

## 📞 Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Discord**: [Join our Discord server](https://discord.gg/your-server) (if available)

## 🏆 Recognition

Contributors will be:
- Listed in the project's contributors section
- Mentioned in release notes for significant contributions
- Invited to join the core team for outstanding contributions

## 📄 License

By contributing to RIM, you agree that your contributions will be licensed under the same license as the project (MIT or Apache 2.0).

---

Thank you for contributing to RIM! Together, we can make mathematics more beautiful and accessible. ✨ 
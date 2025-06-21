# Contributing to Nerd Font Installer CLI

Thank you for your interest in contributing! Here's how you can help make this project better.

## 🚀 Ways to Contribute

- 🐛 **Bug Reports**: Found a bug? Please open an issue
- 💡 **Feature Requests**: Have an idea? We'd love to hear it
- 📖 **Documentation**: Help improve our docs
- 🔧 **Code**: Submit pull requests for fixes or features

## 🛠️ Development Setup

1. **Prerequisites**
   - Rust 1.70+ (install via [rustup](https://rustup.rs/))
   - Git

2. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/nerd-font-installer.git
   cd nerd-font-installer
   ```

3. **Build and test**
   ```bash
   cargo build
   cargo test
   cargo run -- list
   ```

## 📝 Code Style

- Use `cargo fmt` to format code
- Run `cargo clippy` for linting
- Follow Rust conventions and idioms
- Add tests for new functionality

## 🔄 Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Add tests if applicable
5. Run tests: `cargo test`
6. Format code: `cargo fmt`
7. Check linting: `cargo clippy`
8. Commit with clear messages
9. Push and create a pull request

## 📋 Adding New Fonts

To add a new font to the installer:

1. Update the `get_nerd_fonts()` function in `src/main.rs`
2. Add a new `FontInfo` struct with:
   - `name`: Display name
   - `asset_name`: GitHub release asset name (e.g., "FontName.zip")
   - `description`: Brief description
   - `variants`: Available font variants
   - `size_mb`: Approximate download size

3. Test the new font:
   ```bash
   cargo run -- list --details
   cargo run -- install "new font name"
   ```

## 🐛 Bug Report Template

When reporting bugs, please include:

- Operating system and version
- Rust version (`rustc --version`)
- Command that caused the issue
- Expected vs actual behavior
- Error messages (if any)

## 💡 Feature Request Template

For feature requests, please describe:

- The problem you're trying to solve
- Proposed solution
- Alternative solutions considered
- Additional context

## 📜 Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## 🙏 Recognition

Contributors will be acknowledged in the README and release notes.

Happy coding! 🦀

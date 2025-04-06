# Contributing to RustWine

Thank you for your interest in contributing to RustWine! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

By participating in this project, you agree to abide by the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/rustwine.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Run benchmarks: `cargo bench`
7. Commit your changes: `git commit -am 'Add some feature'`
8. Push to your fork: `git push origin feature/your-feature-name`
9. Create a Pull Request

## Development Guidelines

### Code Style

- Follow the [Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md)
- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes

### Documentation

- Document all public APIs
- Include examples where appropriate
- Update the README.md for significant changes
- Add comments for complex algorithms

### Testing

- Write unit tests for new features
- Ensure all tests pass before submitting PR
- Add benchmarks for performance-critical code
- Document test coverage improvements

### Security

- Follow security best practices
- Document security considerations
- Report security vulnerabilities responsibly

## Project Structure

- `rustwine-core/`: Core API translation
- `rustwine-dxvk/`: Graphics translation
- `rustwine-seccomp/`: Security & sandboxing
- `rustwine-ai/`: AI optimization
- `benchmarks/`: Performance testing

## Pull Request Process

1. Ensure your code follows the style guide
2. Update documentation as needed
3. Add tests for new features
4. Run all tests and benchmarks
5. Update the CHANGELOG.md
6. Submit PR with a clear description

## Review Process

- PRs will be reviewed by maintainers
- Feedback will be provided within a week
- Address review comments promptly
- Keep PRs focused and manageable

## Release Process

1. Update version numbers
2. Update CHANGELOG.md
3. Create release tag
4. Build and test release artifacts
5. Publish to crates.io

## Questions?

Feel free to open an issue or contact the maintainers for any questions.

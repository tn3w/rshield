# Contributing to rshield

Thanks for your interest in contributing! Every contribution matters, whether it's a bug report, feature idea, docs fix, or code change.

## Ways to contribute

- **Report bugs**: [open an issue](https://github.com/tn3w/rshield/issues/new) with steps to reproduce
- **Suggest features**: [open an issue](https://github.com/tn3w/rshield/issues/new) describing the use case
- **Fix bugs**: browse [open issues](https://github.com/tn3w/rshield/issues) and submit a PR
- **Improve docs**: typos, examples, clarifications, all welcome
- **Add translations**: see `scripts/translation`

## Getting started

```bash
git clone https://github.com/tn3w/rshield.git
cd rshield
cargo test
```

Tests needing network access and Redis (`127.0.0.1:6379`) are ignored by default:

```bash
cargo test -- --ignored --skip benchmark
```

## Submitting a pull request

1. Fork the repo and create a branch from `main`
2. Write or update tests for your changes
3. Run the checks:
   ```bash
   cargo fmt
   cargo clippy --all-targets -- -D warnings
   cargo test
   ```
4. Keep commits focused: one logical change per PR
5. Open the PR with a clear description of what and why

## Code style

- Follow existing patterns in the codebase
- Max 90 characters per line (`rustfmt.toml`)
- No comments unless absolutely necessary: write self-documenting code
- Use early returns and keep nesting below 4 levels
- Minimum supported Rust version is 1.80

## Reporting security issues

Please do **not** open a public issue for security vulnerabilities. See [SECURITY.md](SECURITY.md).

## License

By contributing, you agree that your contributions will be licensed under the [Apache-2.0 License](LICENSE).

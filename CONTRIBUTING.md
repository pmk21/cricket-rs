# Contributing

Thank you for considering to contribute to `cricket-rs`!

- [Contributing](#contributing)
  - [Development](#development)
    - [Dependencies](#dependencies)
    - [Process](#process)
    - [Add Entry to Changelog](#add-entry-to-changelog)


## Development

### Dependencies

1. Install Rust using [`rustup`](https://www.rust-lang.org/tools/install).
2. [`cargo`](https://doc.rust-lang.org/cargo/guide/index.html) is a command line utility that is part of the Rust toolchain that can be used to install additional tools.

### Process

1. Select an issue to work on, and inform the maintainers.
2. Fork the project.
3. `git clone` the forked version of the project.
4. Work on the `main` branch for smaller patches and a separate branch for new features
5. After writing some code, run -
   - `cargo fmt`
   - `cargo clippy -- -D warnings`
   - `cargo test`
6. If all tests are passing, pull changes from the original remote with a rebase, and push the changes to your remote repository.
7. Use the GitHub website to create a Pull Request and wait for the maintainers to review it.

### Add Entry to Changelog

If your contribution changes the behaviour of `cricket-rs`(as opposed to documentation fixes etc.), please update [`CHANGELOG.md`](CHANGELOG.md) file and describe your changes.

The top of the `CHANGELOG` contains a *"unreleased"* section with a few subsections (Features, Bugfixes, …). Please add your entry to the subsection that best describes your change.

Entries follow this format:

```markdown
- Short description of what has been changed, see [#123](<link-to-issue>) (@user)
```

Here, `#123` is the number of the original issue and/or your pull request.
Please replace `@user` by your GitHub username.

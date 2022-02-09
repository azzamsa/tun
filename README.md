<div align="center">
<h1>Tun</h1>

Rust REST API Boilerplate 🏗

<a href="https://github.com/azzamsa/tun/actions/workflows/ci.yml">
  <img src="https://github.com/azzamsa/tun/actions/workflows/ci.yml/badge.svg" alt="Build status" />
</a>

</div>

---

## Features

- [Axum](https://github.com/tokio-rs/axum) web framework
- [git-cliff](https://github.com/orhun/git-cliff) Changelog Generator
- GitHub Action for CI and release
- Git hooks for continuous development (format, lint, test)
  - Uses [Cargo Husky](https://github.com/rhysd/cargo-husky)
- Consistent formatting using [dprint](https://github.com/dprint/dprint)
- [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) support

## Checklist

When you use this template, try to follow the checklist to update your info properly

- [ ] Change the author name in `LICENSE`
- [ ] Change the package info in `Cargo.toml`
- [ ] Change the application name:
  - [ ] App name in `release.sh`
  - [ ] App name in `release.yml`
  - [ ] App name in the import statements across Rust source and tests files.
- [ ] Clean up the READMEs and remove routes

And, enjoy :)

## Usage

```shell
$ # Clone the repository

$ just dev  # See also `just setup`
```

## Navigating the Code

All the features can be found in the [CHANGELOG](CHANGELOG.md) file tagged with `feat`.
The file only contains user-facing changes, so you won't get lost navigating the code.

## Credits

- Icons and emoji from [Noto Emoji](https://github.com/googlefonts/noto-emoji)

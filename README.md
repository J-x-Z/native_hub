# NativeHub

🚀 **Native Rust GitHub Client** - 100% Rust, 0% Browser Engine

A cyberpunk-themed, native desktop client for GitHub built with egui. Designed to be fast, lightweight, and keyboard-friendly.

## ⚠️ Alpha Status

This is an **early alpha** version. Currently only the following features work:
- ✅ Login via `gh` CLI (zero-config if you already use `gh auth login`)
- ✅ View repository list
- ✅ Click to open repo in browser

**Known Bugs**: Many features are incomplete or buggy. Use at your own risk!

## Requirements

- [Rust](https://rustup.rs/) (2024 edition)
- [GitHub CLI (`gh`)](https://cli.github.com/) - must be authenticated (`gh auth login`)

## Build & Run

```bash
git clone https://github.com/YOUR_USERNAME/native_hub.git
cd native_hub
cargo run
```

## Tech Stack

- **GUI**: [egui](https://github.com/emilk/egui) (Immediate mode, OpenGL)
- **Async Runtime**: tokio
- **HTTP Client**: reqwest
- **Keyring**: keyring (secure token storage)

## Roadmap

- [ ] Issue/PR list
- [ ] Code viewer with syntax highlighting
- [ ] Contribution graph
- [ ] i18n (Internationalization)

## License

MIT

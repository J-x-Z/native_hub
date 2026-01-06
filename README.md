# NativeHub

⚡ **Native Rust GitHub Client** - Cyberpunk Theme, Pure Rust, Zero Browser Engine

<p align="center">
  <img src="https://img.shields.io/badge/Rust-2024%20Edition-orange?style=flat-square&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/GUI-egui-blue?style=flat-square" alt="egui">
  <img src="https://img.shields.io/badge/Platform-Windows-0078D6?style=flat-square&logo=windows" alt="Windows">
  <img src="https://img.shields.io/badge/Status-Alpha-red?style=flat-square" alt="Alpha">
</p>

A fast, lightweight native desktop GitHub client built with Rust and egui. Features a distinctive cyberpunk aesthetic with neon cyan accents and a dark theme.

## ✨ Features

### Core Functionality
- ✅ **Authentication** - Zero-config login via `gh` CLI
- ✅ **Repository Browser** - View your GitHub repositories with cyberpunk-styled cards
- ✅ **File Browser** - Navigate repository files and directories
- ✅ **Code Viewer** - View file contents with syntax highlighting icons
- ✅ **README Display** - Automatic README loading when entering a repository
- ✅ **Search** - Search GitHub repositories globally

### Issues Management
- ✅ **Issues List** - View repository issues with labels and status
- ✅ **Issue Details** - Read issue body and comments
- ✅ **Add Comments** - Post new comments on issues
- ✅ **Close/Reopen** - Change issue state

### Pull Requests
- ✅ **PR List** - View open/closed pull requests
- ✅ **PR Details** - Branch info, stats (commits, additions, deletions)
- ✅ **Merge Options** - Merge, Squash, or Rebase
- ✅ **Close PR** - Close pull requests

### UI Features
- ✅ **Cyberpunk Theme** - Neon cyan accents, dark backgrounds, tactical corners
- ✅ **Chinese Localization** - Full Chinese interface support
- ✅ **System Status Bar** - HUD-style bottom bar
- ✅ **Tabbed Navigation** - Switch between Issues and PRs

## 🖥️ Screenshots

*Coming soon*

## 📦 Requirements

- [Rust](https://rustup.rs/) 1.70+ (2024 edition)
- [GitHub CLI (`gh`)](https://cli.github.com/) - Must be authenticated:
  ```bash
  gh auth login
  ```

## 🚀 Build & Run

### Development
```bash
git clone https://github.com/AhogeK/native_hub.git
cd native_hub
cargo run
```

### Release (No Console)
```bash
cargo build --release
./target/release/native_hub.exe
```

## 🛠️ Tech Stack

| Component | Technology |
|-----------|------------|
| GUI Framework | [egui](https://github.com/emilk/egui) + eframe |
| Async Runtime | Tokio |
| HTTP Client | Reqwest |
| Token Storage | keyring (OS-native secure storage) |
| Serialization | serde + serde_json |

## 📋 Roadmap

### Coming Soon
- [ ] Create new issues
- [ ] Review PR diffs
- [ ] Notifications
- [ ] Contribution graph
- [ ] Code syntax highlighting
- [ ] Repository settings
- [ ] Starring/Forking repos

### Future
- [ ] Linux/macOS support
- [ ] Custom themes
- [ ] Keyboard shortcuts
- [ ] Multiple account support

## ⚠️ Alpha Status

This is an **early alpha** release. Some features may be incomplete or buggy. 
Bug reports and contributions are welcome!

## 📄 License

MIT License - See [LICENSE](LICENSE) for details.

---

<p align="center">
  Made with ❤️ and Rust
</p>

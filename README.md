# cosmic-clip-minimal 📋

> A minimal, event-driven clipboard history manager for **Pop!_OS COSMIC** (Wayland).

Monitors everything you copy with `Ctrl+C` since boot. Press `Super+V` to open a fuzzy picker and paste any previous clip back into your clipboard.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)
![Wayland](https://img.shields.io/badge/Wayland-native-brightgreen.svg)
![Platform](https://img.shields.io/badge/platform-Pop!__OS%20COSMIC-teal.svg)

---

## ✨ Features

- 🚀 **Zero polling** — event-driven via `wlr-data-control` Wayland protocol, minimal CPU usage
- 🧠 **Volatile history** — stored in `/tmp/`, auto-cleared on reboot (privacy-friendly)
- 📝 **Plain text focus** — UTF-8 only, fast and simple
- 🎨 **COSMIC-style UI** — rounded wofi window matching the desktop theme
- ⚡ **Tiny footprint** — single native Rust binary, no Electron, no GTK app required
- 🔧 **50-item rolling cap** — newest items bubble to top, oldest are dropped automatically

---

## 🖥️ How it works

```
[You press Ctrl+C]
        │
        ▼
  [daemon] ← listens to Wayland data-control protocol
        │
        ▼
  /tmp/cosmic-clip-minimal.json  (volatile, max 50 items)
        │
[You press Super+V]
        │
        ▼
  [show] → wofi picker (newest first)
        │
  [You select an item]
        │
        ▼
  wl-copy → clipboard updated → Ctrl+V wherever you want
```

---

## 📋 Requirements

| Dependency | Install | Purpose |
|---|---|---|
| `wofi` | `sudo apt install wofi` | Fuzzy clipboard picker UI |
| `wl-clipboard` | `sudo apt install wl-clipboard` | Write to Wayland clipboard (`wl-copy`) |
| `libwayland-dev` | `sudo apt install libwayland-dev` | Build-time Wayland headers |
| `libxkbcommon-dev` | `sudo apt install libxkbcommon-dev` | Build-time keyboard headers |
| Rust stable | See below | To compile the binary |

> **Note:** This tool targets **Pop!_OS 24.04 LTS with the COSMIC desktop**. It requires a Wayland compositor that implements the `wlr-data-control-unstable-v1` or `ext-data-control-v1` protocol. Does **not** work under Flatpak or X11.

---

## 🚀 Installation

### 1. Install system dependencies

```bash
sudo apt install wofi wl-clipboard libwayland-dev libxkbcommon-dev
```

### 2. Install Rust (if you don't have it)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup default stable
```

### 3. Clone and build

```bash
git clone https://github.com/SEU_USUARIO/cosmic-clip-minimal.git
cd cosmic-clip-minimal
```

### 4. Install with `just`

```bash
cargo install just      # install the task runner
just install            # build, install binary + systemd service
```

<details>
<summary>Manual install (without <code>just</code>)</summary>

```bash
cargo build --release
mkdir -p ~/.local/bin ~/.config/systemd/user ~/.config/wofi
cp target/release/cosmic-clip-minimal ~/.local/bin/
cp cosmic-clip-minimal.service ~/.config/systemd/user/
cp assets/wofi-style.css ~/.config/wofi/cosmic-clip.css
systemctl --user daemon-reload
systemctl --user enable --now cosmic-clip-minimal.service
```
</details>

### 5. Set up keyboard shortcut

In COSMIC: **Settings → Keyboard → Custom Shortcuts → Add shortcut**

| Field | Value |
|---|---|
| Name | `Clipboard History` |
| Command | `~/.local/bin/cosmic-clip-minimal show` |
| Shortcut | `Super + V` |

---

## 🔍 Verify it's working

```bash
# Check the daemon is active
systemctl --user status cosmic-clip-minimal

# Test the picker manually
~/.local/bin/cosmic-clip-minimal show
```

---

## 🗑️ Uninstall

```bash
just uninstall
```

---

## 🏗️ Project Structure

```
cosmic-clip-minimal/
├── src/
│   ├── main.rs       — CLI entry point (clap subcommands: daemon, show)
│   ├── daemon.rs     — Wayland clipboard listener (event-driven, no polling)
│   ├── show.rs       — Fuzzy picker UI (wofi/fuzzel) + wl-copy integration
│   └── state.rs      — Volatile JSON state in /tmp (load, save, push_item)
├── assets/
│   └── wofi-style.css — COSMIC-themed picker window (rounded, dark)
├── cosmic-clip-minimal.service — Systemd user service
├── justfile          — Build, install, uninstall recipes
└── LICENSE           — MIT
```

---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Bug reports and PRs are welcome!

---

## 📄 License

MIT — see [LICENSE](LICENSE).

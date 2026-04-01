# Contributing to cosmic-clip-minimal

Thanks for your interest! This is a minimal tool — contributions should stay minimal too.

## Philosophy

- **No feature creep**: this is a clipboard history manager, not a Swiss Army knife
- **Wayland-native**: no X11 fallbacks, no Flatpak workarounds
- **Zero polling**: event-driven only, CPU-friendly
- **Plain text**: image/file clipboard support is out of scope

## Development Setup

```bash
# System dependencies
sudo apt install libwayland-dev libxkbcommon-dev wofi wl-clipboard

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# Clone
git clone https://github.com/SEU_USUARIO/cosmic-clip-minimal.git
cd cosmic-clip-minimal

# Check it compiles
cargo check

# Build release
cargo build --release
```

## How to Submit Changes

1. Fork the repo on GitHub
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Make your changes with clear, focused commits
4. Run `cargo check` and `cargo clippy` — no new warnings allowed
5. Open a Pull Request with a clear description of what and why

## Reporting Bugs

Please open an Issue with:

- Output of `systemctl --user status cosmic-clip-minimal`
- Your compositor: `echo $XDG_CURRENT_DESKTOP && echo $WAYLAND_DISPLAY`
- Exact steps to reproduce the issue
- Expected vs actual behavior

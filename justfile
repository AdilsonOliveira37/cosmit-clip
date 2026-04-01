install:
	cargo build --release
	mkdir -p ~/.local/bin ~/.config/systemd/user ~/.config/wofi
	cp target/release/cosmic-clip-minimal ~/.local/bin/
	cp cosmic-clip-minimal.service ~/.config/systemd/user/
	cp assets/wofi-style.css ~/.config/wofi/cosmic-clip.css
	systemctl --user daemon-reload
	systemctl --user enable --now cosmic-clip-minimal.service
	@echo ""
	@echo "✅ Instalação concluída!"
	@echo "⚠️  Dependência necessária (se ainda não instalada): sudo apt install wl-clipboard"

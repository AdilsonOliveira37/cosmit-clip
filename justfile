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
	@echo "⚠️  Dependências necessárias (se ainda não instaladas): sudo apt install wl-clipboard wofi"

uninstall:
	-systemctl --user disable --now cosmic-clip-minimal.service
	rm -f ~/.local/bin/cosmic-clip-minimal
	rm -f ~/.config/systemd/user/cosmic-clip-minimal.service
	rm -f ~/.config/wofi/cosmic-clip.css
	rm -f /tmp/cosmic-clip-minimal.json
	systemctl --user daemon-reload
	@echo "✅ Desinstalação concluída!"

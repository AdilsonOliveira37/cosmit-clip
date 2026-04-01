install:
	cargo build --release
	mkdir -p ~/.local/bin ~/.config/systemd/user
	cp target/release/cosmic-clip-minimal ~/.local/bin/
	cp cosmic-clip-minimal.service ~/.config/systemd/user/
	systemctl --user daemon-reload
	systemctl --user enable --now cosmic-clip-minimal.service

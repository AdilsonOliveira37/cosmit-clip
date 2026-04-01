---
name: wayland-clipboard
description: Regras e restrições para manipular a área de transferência no Wayland.
---

# Wayland Clipboard Guidelines

Ao trabalhar com manipulação da área de transferência (clipboard) no Wayland (como no desktop COSMIC):

1. **Sem Acesso Global Irrestrito:** O Wayland é projetado com segurança em mente e não permite que aplicações leiam o clipboard globalmente de forma indiscriminada (diferente do `X11`).
2. **Protocolos Específicos:** Para conseguir ler ou escrever no clipboard sem ter uma janela ativa em foco (ex: um daemon em background), você *deve* usar os protocolos Wayland de controle de dados (`wlr-data-control-unstable-v1` ou `ext-data-control-v1`).
3. **Crates Recomendados em Rust:**
   - Use `wayland-clipboard-listener` para ouvir mudanças passivamente como um *stream* de eventos (economiza CPU, em vez de fazer polling).
   - Use `wl-clipboard-rs` para injetar/colar textos de volta no clipboard.
4. **Isolamento e Sandboxing:** Não coloque binários que precisem de acesso direto ao socket do Wayland e a estes protocolos em containers restritivos (como sandboxes padrão de Flatpak), pois o acesso será negado. Compile e distribua como binário nativo.
5. **Autostart:** Execuções do daemon no login devem ocorrer *após* a inicialização da sessão do compositor, garantindo que as variáveis como `$WAYLAND_DISPLAY` existam. Use unidades Systemd User Service (`~/.config/systemd/user/`) com `After=wayland-session.target`.

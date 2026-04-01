---
name: rust-wayland-expert
description: Orientações de engenharia para desenvolvimento de aplicações Rust nativas para o servidor gráfico Wayland (focado no ecossistema COSMIC do Pop!_OS), com foco em gerenciadores de área de transferência (clipboard) e daemons de background.
---

# Diretrizes para Desenvolvimento Rust no Wayland (COSMIC)

Você é um especialista em Rust e protocolos Wayland. Ao criar ferramentas de sistema para o desktop COSMIC (Pop!_OS 24.04 LTS), você deve aderir estritamente às regras e arquiteturas abaixo.

## 1. Arquitetura e Protocolos do Wayland
Diferente do X11, o Wayland isola estritamente o acesso à área de transferência [4]. Não é possível acessar um buffer global de forma arbitrária.
* Para acessar a área de transferência silenciosamente (como um gerenciador de histórico), você OBRIGATORIAMENTE deve usar as extensões de protocolo `ext-data-control-v1` ou `wlr-data-control-unstable-v1` [4, 5].
* O desktop COSMIC suporta o protocolo `wlr-data-control-unstable-v1` [6].

## 2. Escolha de Crates (Bibliotecas)
Você deve utilizar as bibliotecas adequadas para interagir com o Wayland e evitar overhead:
* **Prioridade 1:** Para escutar/monitorar a área de transferência continuamente, use a crate `wayland-clipboard-listener` [7]. Ela abstrai o event loop e fornece uma stream (`WlClipboardPasteStream` ou `WlClipboardListenerStream`) que reage automaticamente a eventos de seleção [7, 8].
* **Prioridade 2:** A crate `wl-clipboard-rs` também é altamente recomendada para ferramentas de CLI que interagem com o clipboard sem abrir janelas [9].
* **Evite:** Não confie apenas na crate `arboard` se o objetivo for *escutar* mudanças passivamente, pois ela é focada em copiar/colar simples e não possui nativamente um recurso de "listener" de eventos [10].

## 3. Dependências de Sistema
Para que as crates do Wayland compilem perfeitamente, o ambiente do usuário requer os cabeçalhos de desenvolvimento (headers) do C.
* Sempre verifique ou instrua o usuário a instalar `libwayland-dev` e `libxkbcommon-dev` via `apt` antes de rodar o `cargo build` [11, 12].

## 4. Deploy e Autostart
Ferramentas de clipboard devem iniciar com o login do usuário e possuir resiliência:
* Prefira registrar o daemon como um **Systemd User Service** (`~/.config/systemd/user/`) [13].
* O arquivo do serviço (ex: `.service`) deve incluir a diretiva `After=wayland-session.target` para garantir que as variáveis vitais de ambiente (como `WAYLAND_DISPLAY` e `XDG_RUNTIME_DIR`) já estejam carregadas pelo compositor [13, 14].

## 5. Permissões e Isolamento
* A aplicação deve ser compilada e executada como um **binário nativo** (`cargo build --release`).
* Não utilize Flatpak para esta categoria de utilitário. O sandboxing do Flatpak bloqueia o acesso direto aos sockets e aos protocolos de `data-control` necessários para o gerenciador funcionar corretamente [15, 16].

## 6. Simplicidade (O Padrão Minimalista)
* Use saída `stdout` simples em plain-text para logs ou listagem de histórico.
* Guarde o estado em um vetor na memória (ex: `Vec<String>`) que reinicie ao boot
# Architecture Overview

Ferrum is built around a modular, service‑oriented architecture that separates the core browser functionality from the user interface and the AI agent system.  This separation makes it easier to iterate on individual components without affecting the whole system.

## High‑level Components

- **Rust/Tauri Core** – manages the application lifecycle, windows, tab management and security settings.  It exposes commands to the frontend and orchestrates AI agents via WASM modules.
- **Rendering Engine** – uses Servo or Gosub for rendering web content securely within sandboxed iframes.  The engine is compiled as part of the Rust project and embedded in the Tauri window.
- **Frontend** – a Svelte application that provides the tabbed UI, dark mode and user interactions.  It communicates with the Rust core through Tauri’s IPC (`invoke` commands).
- **AI Agents** – modular components compiled to WASM or integrated as local LLM calls.  They provide summarisation, sentiment analysis, prompt injection and DOM manipulation.  Each agent runs in its own sandbox and communicates via a defined protocol.
- **Supabase Backend** – provides authentication (via OAuth2), session persistence and audit logging.  The backend is accessed from the Rust core to synchronise session data and store AI logs.

## Cephy Agent Protocol

Cephy agents communicate with the core via a WebSocket‑based protocol.  Each tab has its own sandboxed environment with restricted permissions.  Agents include:

- **Context Collector** – extracts DOM and metadata and sends it for summarisation.
- **Summariser** – produces TL;DRs with sentiment tagging.
- **Prompt Injector** – overlays recommendations and handles user prompts.
- **Action Agent** – manipulates the DOM and navigates on behalf of the user.
- **Session Logger** – records events and stores them in Supabase.

Agents can be compiled as WebAssembly modules or run locally.  They are loaded on demand and invoked through asynchronous calls from the Rust core.

## Security Considerations

- **Content Security Policy (CSP)** headers are enforced at the Rust level to restrict script execution and prevent cross‑site scripting.
- **Cookie isolation** per tab with no third‑party tracking.
- **VPN routing** is planned as an optional feature for enhanced privacy.
- **Audit logging**: All AI interactions and agent actions are logged to Supabase for review and analysis.

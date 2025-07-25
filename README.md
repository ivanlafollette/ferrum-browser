# 🧠 VibeNav: AI-Enhanced Rust Browser

**VibeNav** is a security-first, minimalist browser built in Rust with native AI agents and modular expansion. Born from TeamInnovationAI, it’s the codename for Ivan’s MVP vision of blending Rust, Supabase, and Cephy-powered intelligence.

## 🚀 Features
- 🧭 Tabbed browsing with dark mode
- 🔐 Security architecture: CSP headers, sandboxed iframe, ad blocking
- 🧠 Built-in AI agents: summarization, sentiment, and prompt injection
- 🔌 SSO support via Google, GitHub, and Entra ID
- 🧬 Offline mode using service workers
- 🎨 Team Innovation branding with animation
- 🔄 Supabase backend for auth/session/AI logs
- 🧩 Future expansion: Cephy agents, terminal shell tab

## 🛠️ Tech Stack
- **Language**: Rust
- **Engine**: Servo or Gosub
- **Frontend**: Tauri + Svelte
- **AI Layer**: WASM modules or local LLMs
- **Backend**: Supabase
- **Auth**: OAuth2
- **Dev Environment**: Cursor AI

## 📦 Build Instructions
```bash
cargo build --release

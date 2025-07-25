# 🧠 Ferrum Browser

Ferrum is a security‑first, minimalist browser built in Rust with native AI agents and modular expansion. Born from TeamInnovationAI, it blends Rust performance with Supabase‑backed session management and Cephy‑powered intelligence.

## 🚀 Features

- 🧭 **Tabbed browsing** with dark mode.
- 🔐 **Security architecture**: CSP headers, sandboxed iframes and ad blocking.
- 🧠 **Built‑in AI agents**: summarisation, sentiment analysis and prompt injection to enrich your browsing experience.
- 🔌 **SSO support** via Google, GitHub and Entra ID.
- 🧬 **Offline mode** using service workers to cache pages.
- 🎨 **Branding and animation** reflecting TeamInnovationAI.
- 🔄 **Supabase backend** for authentication, session state and AI log storage.
- 🧩 **Future expansion**: Cephy agents, terminal shell tab and plugin system.

## 🛠️ Tech Stack

- **Language:** Rust (edition 2021)
- **Engine:** Servo or Gosub web rendering engine
- **Frontend:** Tauri + Svelte
- **AI Layer:** WASM modules or local LLMs for agent workflows
- **Backend:** Supabase
- **Auth:** OAuth2 (Google/GitHub/Entra ID)
- **Dev Environment:** Cursor AI or any Rust‑friendly IDE

## 📁 Repository Structure

```
/Cargo.toml            – Rust crate manifest
/src/main.rs           – Tauri application entry point
/tauri.conf.json       – Tauri configuration
/frontend/             – Svelte frontend application
  /package.json        – Node project manifest
  /src/                – Svelte components
  /public/             – Static assets and HTML entry point
/docs/                 – Project documentation
  architecture.md      – High‑level system architecture
  production.md        – Build and deployment notes
/README.md             – This file
/LICENSE               – Project licensing
/.gitignore            – Git exclusions
```

## 🧑‍💻 Development

### Prerequisites

- Node.js ≥ 16 and npm for the frontend
- The Rust toolchain (via [rustup](https://rustup.rs)) and Cargo
- The Tauri CLI (`cargo install tauri-cli`)

### Getting started

```bash
# install frontend dependencies
cd frontend
npm install

# run the frontend in dev mode
npm run dev

# in a separate terminal run the Rust application (spawns Tauri window)
cd ..
cargo tauri dev
```

When you’re ready for a production build, run:

```bash
npm run build     # build the Svelte frontend into static assets
cd ..
cargo build --release
```

See the documentation in `/docs/` for a detailed description of the architecture and production considerations.

## 📝 License

Ferrum is licensed under the MIT License. See [LICENSE](./LICENSE) for details.

# Building Ferrum on Windows

This guide explains how to build the Ferrum browser on a Windows machine using the Tauri toolchain.

## Prerequisites

* **Operating system:** Windows 10/11 (64‑bit).
* **Rust toolchain:** Install Rust stable with rustup (`https://rustup.rs`). Make sure the MSVC build tools are installed; the easiest way is to install **Visual Studio 2022 Build Tools** with the *Desktop development with C++* workload. During installation choose the **Windows 10 SDK**.
* **Node.js:** Install Node.js LTS (version 16 or higher). This is needed to build the Svelte frontend.
* **Tauri CLI:** Install the Tauri CLI and dependencies with `cargo install tauri-cli`.
* **WebView2 runtime:** Microsoft’s WebView2 runtime is required; it comes with Edge on current Windows 10/11 versions. If not present, install it from Microsoft.
* **Git:** Ensure Git is installed to clone the repository.

> **Note:** Tauri relies on native toolchains per operating system. Meaningful cross‑compilation is not currently possible. To build the Windows binaries you must run the build on Windows itself or configure a CI pipeline that runs on Windows【18784233107624†L80-L86】.

## Steps

1. **Clone the repository**

   ```shell
   git clone https://github.com/ivanlafollette/ferrum-browser.git
   cd ferrum-browser
   ```

2. **Install frontend dependencies**

   The frontend resides in `frontend/`. Install its dependencies via npm or pnpm:

   ```shell
   cd frontend
   npm install    # or: pnpm install
   cd ..
   ```

3. **Run the app in development mode (optional)**

   To run Ferrum in dev mode (with hot‑reloading):

   ```shell
   npm run tauri dev
   ```

   This command will:

   - Build the Svelte frontend using Vite.
   - Launch the Rust/Tauri backend.
   - Open a debug window with developer tools.

4. **Build the production package**

   To produce a distributable Windows binary (MSI installer and portable `.exe`), run:

   ```shell
   npm run tauri build
   ```

   Under the hood, this calls `cargo tauri build`, which compiles the Rust code and bundles the frontend. After completion you’ll find the packages in `src-tauri/target/release/bundle/` (e.g., `bundle/msi` for the installer and `bundle/portable` for the standalone executable).

5. **(Optional) Sign the installer**

   If you plan to distribute Ferrum publicly, you should code‑sign the installer to avoid Windows SmartScreen warnings. Follow the [Tauri Windows code signing guide](https://tauri.app/v1/guides/distribution/windows-code-signing/) for instructions.

## Building via CI

If you develop on Linux or macOS and need Windows binaries, configure the [Tauri GitHub Action](https://github.com/tauri-apps/tauri-action). The action runs on Windows, macOS and Linux runners and produces the correct artifacts for each platform. See the Cross‑Platform Compilation documentation for setup details【18784233107624†L87-L100】.

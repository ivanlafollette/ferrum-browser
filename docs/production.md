# Production & Deployment Notes

This document describes how to build, package and distribute the Ferrum browser for end users.

## Build pipeline

1. **Install dependencies**
   - Install Node.js (v16 or later) and the Rust toolchain via rustup.
   - Navigate to the `frontend/` directory and run `npm install` to install the front‑end dependencies.

2. **Build the front‑end**
   - From within `frontend/`, run `npm run build`.  The static assets will be output to `frontend/dist/`.

3. **Build the Tauri application**
   - Install the Tauri CLI if you haven’t already: `cargo install tauri-cli`.
   - Return to the project root and run `cargo tauri build`.  This produces a platform‑specific binary in `src-tauri/target/release/`.  The build command will automatically run the front‑end build step via the `beforeBuildCommand` defined in `tauri.conf.json`.

## Distribution

- **Linux builds** produce an AppImage and Debian packages, depending on the target.  These files are generated in the `src-tauri/target/release/` directory.
- **Windows builds** produce an MSI installer and a portable executable.  Code signing certificates should be configured in `tauri.conf.json` before release.
- **macOS builds** (future) require a macOS host or CI runner and will produce a `.app` bundle and a DMG.

## Environment variables

Ferrum relies on several environment variables at runtime:

- `SUPABASE_URL` – the URL of your Supabase instance
- `SUPABASE_KEY` – service role or anon key used to connect
- `OAUTH_GOOGLE_CLIENT_ID` / `OAUTH_GITHUB_CLIENT_ID` / `OAUTH_ENTRA_CLIENT_ID` – credentials for the OAuth providers configured in Supabase

These variables should be defined in a `.env` file or passed into the application by your build pipeline.  Never commit secrets to version control.

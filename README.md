# AetherPhotos Desktop App

A premium, local-first, high-performance desktop application for photo de-duplication and centralization across **macOS Photos**, **Amazon Photos**, and **Google Takeout**.

Built with a **Tauri v2 (Rust)** core, a **React & TypeScript** frontend, and a compiled **Python FastAPI** pipeline engine running as a zero-latency sidecar.

---

## Key Features

1. **Multi-Source Scan & Hashing:**
   - Indices macOS `.photoslibrary` (SQLite backend extraction).
   - Walks raw directory structures (e.g. Amazon Photos backups).
   - Zero-disk-extraction streaming ZIP parsing of Google Takeout archives.

2. **Deduplication & Resolution Engine:**
   - Group identical assets by SHA-256 content hashes.
   - Smart choice selection: selects the "best copy" based on file size, metadata completeness (EXIF timestamps, GPS coordinates), and source hierarchy.

3. **Safe Centralization:**
   - Chronological directory organization: `YYYY/YYYY-MM/filename.ext`.
   - UTC epoch absolute timestamp preservation with Finder timezone-shift prevention.
   - Sidecar JSON parser for Google Takeout EXIF injection.

4. **100% Local Privacy:**
   - No cloud uploads, no server calls, 100% offline security.

---

## Tech Stack & Architecture

```mermaid
graph TD
    A[React/TS Frontend] -- Local HTTP API (localhost:8000) --> B[Python FastAPI Server Sidecar]
    C[Tauri Core (Rust)] -- Spawns/Manages Lifecycle --> B
    B -- Scan & Read --> D[macOS Photos DB]
    B -- Stream Stream Zip --> E[Google Takeout Zip]
    B -- Scan Directories --> F[Amazon Photos]
    B -- Write & Preserves EXIF --> G[Centralized Target Folder]
```

- **Frontend:** React, TypeScript, Vite, Harmony Dark Glassmorphic Theme.
- **Backend Core:** Tauri v2 (Rust) managing native window handles and launching/killing the Python sidecar.
- **Pipeline Engine:** Python 3.13 + FastAPI + SQLite3 + PyInstaller (compiled standalone binary `photos-backend`).

---

## Development & Build Setup

### Prerequisites
- Node.js >= 18
- Rustc / Cargo >= 1.88.0
- Python 3.13 (with `uvicorn`, `fastapi`, `pyinstaller` in virtualenv)

### Local Dev Server
1. Start the React & Tauri app in development mode:
   ```bash
   npm run tauri dev
   ```
   Tauri will automatically boot Vite on port `1420` and launch the compiled Python `photos-backend` sidecar in the background.

### Packaging Release Bundles
To package the standalone `.dmg` installer and `.app` bundle:
1. Re-compile the Python backend engine:
   ```bash
   cd ../photos-cleanup
   ./venv/bin/pyinstaller api.spec --clean
   cp dist/api ../aetherphotos-desktop/src-tauri/binaries/photos-backend-aarch64-apple-darwin
   ```
2. Build the Tauri installer:
   ```bash
   cd ../aetherphotos-desktop
   npm run tauri build
   ```
3. Locate the finished bundles:
   - Standalone App: `src-tauri/target/release/bundle/macos/aetherphotos-desktop.app`
   - DMG Installer: `src-tauri/target/release/bundle/dmg/aetherphotos-desktop_0.1.0_aarch64.dmg`

---

## License
Proprietary / Personal Local Archive Utility.

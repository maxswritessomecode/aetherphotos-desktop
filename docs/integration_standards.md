# AetherPhotos Desktop Integration & Architectural Standards

Any agent working on this codebase must adhere to the following integration principles to avoid silent failures and layout regressions:

## 1. CORS & WebKit Mixed-Content Bypasses
- **The Issue:** macOS WebKit treats secure custom schemes (`tauri://`) strictly and blocks insecure local HTTP requests (`http://localhost:8000`) under Mixed-Content rules.
- **The Rule:** Never use standard browser `fetch()` for backend API calls. Always route local queries via Tauri's native Rust HTTP client (`@tauri-apps/plugin-http`) which executes in the system thread, completely bypassing CORS and WebKit sandboxes.

## 2. Loud-Failure Frontend Design
- **The Rule:** Do not allow the frontend to silently catch API exceptions and fall back to mock simulator data in production. If the FastAPI backend is unreachable, display a prominent red network connection warning banner in the interface.

## 3. Scoped Session Transaction Tracking
- **The Rule:** Never poll for background thread scan progress using global database counts (e.g. `total_assets > 0`). Always poll specific run IDs (`GET /run/{run_id}`) to track thread execution and support multi-session library changes cleanly.

## 4. macOS Folder Package Selectors
- **The Issue:** macOS prevents standard directory pickers (`directory: true`) from selecting package bundles like `.photoslibrary`.
- **The Rule:** Treat Photos Libraries as file packages. Set `directory: false` and filter specifically for the `["photoslibrary"]` extension.

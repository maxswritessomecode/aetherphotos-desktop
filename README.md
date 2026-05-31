# 📷 AetherPhotos Desktop App

> **A premium, 100% local-first, high-performance desktop application for photo de-duplication and centralization.**

AetherPhotos safely scans, deduplicates, and consolidates your scattered photo archives from **macOS Photos Libraries**, **Amazon Photos Backups**, and **Google Takeout ZIP Archives** into a single, beautifully organized master backup folder.

Built with a **Tauri v2 (Rust)** shell, a beautiful **React & TypeScript** glassmorphic frontend, and a compiled **Python FastAPI** pipeline engine running as a zero-latency native sidecar.

---

## 🛡️ Safety First: The Non-Destructive Principle

> [!IMPORTANT]
> **AetherPhotos is 100% Non-Destructive.**
> - It **never deletes, moves, or modifies** your original files or libraries.
> - All matching, hashing, and indexing are performed **in-memory** and logged to a local SQLite database.
> - Organizing and centralizing is done by **copying** your files to your new targeted destination folder.
> - Your original macOS Libraries, Amazon backup directories, and Google Takeout files remain **entirely untouched**. You can run this app with zero anxiety of data loss.

---

## ✨ Premium Features

### 1. Unified Multi-Source Centralization
*   **📷 macOS Photos Libraries:** Directly parses the internal SQLite database (`Photos.sqlite`) inside opaque `.photoslibrary` packages.
*   **📦 Amazon Photos Directories:** Recursively walks local directories downloaded via Amazon Photos.
*   **☁️ Google Takeout ZIP Streamer:** Streams and parses nested files directly inside Google Takeout ZIP archives **without extracting them to disk first** (saving hundreds of gigabytes of disk space).

### 2. Smart Deduplication & Best-Candidate Resolution
*   **Exact SHA-256 Hashing:** Groups identical files by content hashes, not by name or modification date.
*   **Priority Candidate Resolution:** If identical photos are found across multiple sources, AetherPhotos selects the "best copy" based on:
    1.  **Metadata completeness:** Prefers files with valid EXIF dates, camera makes/models, and GPS coordinates.
    2.  **Completeness:** Automatically extracts EXIF data from Google Takeout sidecar `.json` files and injects it back into the copied media.
    3.  **Source preference:** Prioritizes macOS Photos -> Amazon Photos -> Google Takeout.
*   **Amazon Filename Normalization:** Automatically strips parenthesized timestamps (e.g. `(2018-07-15T05_22_28.426)`) added by Amazon Photos during duplicate matching, recognizing matches even when filenames differ.

### 3. Chronological Canonical Layout
All resolved canonical files are copied and organized into a clean, human-readable hierarchy:
```text
Destination/
└── YYYY/
    └── YYYY-MM/
        └── YYYYMMDD_HHMMSS_[sha256_prefix].ext
```
*Unknown dates are safely placed in an `unknown/` directory.*

### 4. Microsecond Timezone-Preserved Metadata Restoration
*   **Native Birthtime Preservation:** Uses low-level macOS system calls (`setattrlist`) to write file creation dates (birthtime) and modification dates natively in microseconds. 
*   **No Spawning Overhead:** Avoids heavy process forking, ensuring blazing-fast file copying and touch operations.
*   **Graceful Fallback:** If native system calls are unavailable, falls back cleanly to standard macOS `touch -t` commands.

---

## ⚡ How to Use AetherPhotos

### Step 1: Connect Your Sources
Open the application and specify the folders you want to scan:
1.  **macOS Photos Library:** Select your standard `.photoslibrary` package (AetherPhotos handles package directory browsing seamlessly).
2.  **Amazon Photos Directory:** Select your local Amazon backup folder.
3.  **Google Takeout ZIP Folder:** Point directly to the folder containing your downloaded Takeout `.zip` files (no need to unzip them!).
4.  **Consolidated Target Folder:** Choose the external drive or directory where you want your new master master archive to live.

### Step 2: Scan & Hashing
Click **Start Scan & Hashing**. AetherPhotos will launch its background engine, showing a pulsing sonar radar as it walks your sources, builds file content hashes, and indexes them into a local, temporary SQLite registry.

### Step 3: Clutter & Duplicate Analysis
Once the scan finishes, AetherPhotos presents a breakdown of your files:
*   Total indexed assets.
*   Total unique canonical photos.
*   Total duplicates identified.
*   **Exact storage space reclaimable** by omitting the duplicates.

### Step 4: Centralize & Copy
*   **🔍 Dry-Run:** Performs a full simulation of the copy process to verify file structures and metadata without writing anything to disk.
*   **🚀 Centralize & Copy:** Safely executes the copy operation with a real-time progress bar displaying files copied, files processed, and any errors.

---

## 🍏 macOS Gatekeeper Security Notice (For Downloads)

Because AetherPhotos is distributed directly from GitHub without being signed by an official Apple Developer account, macOS will flag it on first launch:

> [!WARNING]
> **"AetherPhotos" is damaged and can’t be opened. You should move it to the Trash.**
> This is a standard macOS warning for unsigned applications. The app is 100% safe, runs completely locally on your machine, and makes no network requests.

### How to Bypass:
1.  Open your **Terminal** app.
2.  Run the following command to strip the download quarantine attribute:
    ```bash
    xattr -cr /Applications/aetherphotos-desktop.app
    ```
3.  Alternatively, locate `aetherphotos-desktop.app` in Finder, **right-click** it, select **Open**, and then click **Open Anyway**.

---

## 💻 Developer & Compilation Guide

### Architecture Overview
```mermaid
graph TD
    A[React/TS Frontend] -- Local HTTP API (localhost:8000) --> B[Python FastAPI Server Sidecar]
    C[Tauri Core (Rust)] -- Spawns/Manages Lifecycle --> B
    B -- Scan & Read --> D[macOS Photos DB]
    B -- Stream Stream Zip --> E[Google Takeout Zip]
    B -- Scan Directories --> F[Amazon Photos]
    B -- Write & Preserves EXIF --> G[Centralized Target Folder]
```

### Setup & Compilation from Source

If you want to run AetherPhotos in development mode or build it from source:

#### Prerequisites
*   Node.js >= 18
*   Rust / Cargo >= 1.88
*   Python >= 3.13 (with `uvicorn`, `fastapi`, and `pyinstaller` installed in a virtual environment)

#### 1. Compile the Python Sidecar
Clone the backend repo `photos-cleanup` and compile the sidecar:
```bash
cd photos-cleanup
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
./venv/bin/pyinstaller api.spec --clean
```
Copy the compiled binary to the Tauri application binaries directory:
```bash
cp dist/api ../aetherphotos-desktop/src-tauri/binaries/photos-backend-aarch64-apple-darwin
```

#### 2. Run in Development Mode
```bash
cd ../aetherphotos-desktop
npm install
npm run tauri dev
```

#### 3. Build Standalone Installer
```bash
npm run tauri build
```
The finished installer will be bundled at:
`src-tauri/target/release/bundle/dmg/aetherphotos-desktop_0.1.0_aarch64.dmg`

---

## 🔒 100% Private, Local, and Secure
AetherPhotos is completely offline. It **never** transmits your photo hashes, filenames, metadata, or coordinates to the internet. All calculations are executed solely on your CPU and local disks. Your memories stay entirely yours.

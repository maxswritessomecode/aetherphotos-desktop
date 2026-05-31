# 📷 AetherPhotos Desktop App

> **A premium, 100% local-first, high-performance desktop application for photo de-duplication and centralization.**

AetherPhotos safely scans, deduplicates, and consolidates your scattered photo archives from **macOS Photos Libraries**, **Amazon Photos Backups**, and **Google Takeout ZIP Archives (the official way to download all your Google Photos)** into a single, beautifully organized master backup folder.

Built with a **Tauri v2 (Rust)** shell, a beautiful **React & TypeScript** glassmorphic frontend, and a compiled **Python FastAPI** pipeline engine running as a zero-latency native sidecar.

---

## 🗑️➡️🎒 Stop Kicking the Can Down the Road

If you are like most of us, your digital memories are scattered in a dozen different messy places. You have a couple of old macOS Photos libraries on external drives, random backups uploaded to Amazon Photos, and a massive dump of fragmented, split-ZIP files from Google Takeout (which is the official service Google provides to download your entire Google Photos library) that you've been meaning to sort through.

You've probably been **kicking this can down the road for years** because doing it manually is an absolute nightmare. It’s too messy, too time-consuming, and standard cloud tools make it incredibly difficult to decouple from their subscription lock-ins so you can just own and save your pictures the way you want to.

**AetherPhotos was built to solve exactly this.** It is a non-destructive, blazing-fast local centralization engine designed to take the friction out of sorting your digital life. It indexes everything, finds duplicate clutter, restores microsecond-level metadata timezone shifts, and copies a clean, chronological master folder tree to your external drive or local disk. 

No cloud fees. No corporate lock-in. Just your memories, organized perfectly and kept 100% private.

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

## 📥 Step-by-Step Guide: How to Get Your Photos

To consolidate all your photos, you first need to download them to your computer. Here is the absolute simplest way to gather your Google and Amazon photos—even if you are completely non-technical.

---

### ☁️ Method A: How to Download from Google Photos (Google Takeout)

Google Takeout allows you to request a complete export of all your photos and videos currently stored in Google Cloud.

1.  **Go to Google Takeout:**
    Open your web browser and navigate to [takeout.google.com](https://takeout.google.com). Log in with your Google account.
2.  **Deselect All Other Data:**
    By default, Google selects everything. Click the **Deselect all** button at the top of the list so you only export your photos.
3.  **Select Google Photos Only:**
    Scroll down the list until you find **Google Photos**, and check the box next to it.
    *   *Tip:* You can click "All Photo albums included" to deselect specific years or trash folders if you want to speed up the download.
4.  **Go to the Next Step:**
    Scroll to the very bottom of the page and click **Next step**.
5.  **Choose Delivery Method & Size:**
    *   **Transfer to:** Keep it as **Send download link via email**.
    *   **Frequency:** Keep it as **Export once**.
    *   **File type & size:** Choose **.zip** and set the size limit to **50 GB** (this minimizes the number of zip files you have to download; choosing 2GB means you'll have dozens of separate files to click).
6.  **Create Export:**
    Click **Create export**. Google will start gathering your photos. Depending on the size of your library, this can take a few hours or up to a day.
7.  **Download the ZIP Files:**
    Google will email you when the export is ready. Go to the email, click the download links, and save the `.zip` files into a single folder on your computer (e.g. create a folder named `Google Takeout` inside your external drive).
    *   *Do NOT unzip them!* AetherPhotos will stream the pictures directly from the ZIP files to save you hundreds of gigabytes of disk space.

---

### 📦 Method B: How to Download from Amazon Photos (Mac Desktop Client)

To download your photos from Amazon, use the official Amazon Photos desktop app to sync everything directly to a folder on your Mac.

1.  **Download Amazon Photos for Mac:**
    Go to [amazon.com/photos/download](https://www.amazon.com/photos/download) on your web browser, download the desktop application, and install it on your Mac.
2.  **Log In:**
    Open the **Amazon Photos** app on your Mac and log in using your Amazon account details.
3.  **Navigate to the Download Section:**
    In the Amazon Photos window, click on the **Download** tab or select **Backup/Download Settings** in the app menu.
4.  **Download Your Entire Library:**
    *   Select **Download entire library** or select specific folders (like your phone backup album).
    *   Choose a destination folder. *Highly recommended:* Choose a folder on your external drive (e.g., `/Volumes/YourDrive/Amazon Photos`) to avoid filling up your Mac's internal hard drive.
5.  **Let the Sync Run:**
    Click **Download**. The Amazon Photos app will run in your menu bar and download all your photos at full resolution. Wait until the sync status shows **Sync Complete**.
6.  **Point AetherPhotos to this Folder:**
    Once finished, open **AetherPhotos** and point the **Amazon Photos Directory** field directly to that synced folder.

---

## ⚡ How to Use AetherPhotos

### Step 1: Connect Your Sources
Open the application and specify the folders you want to scan:
1.  **macOS Photos Library:** Select your standard `.photoslibrary` package (AetherPhotos handles package directory browsing seamlessly).
2.  **Amazon Photos Directory:** Select your local Amazon backup folder.
3.  **Google Takeout ZIP Folder:** Point directly to the folder containing your downloaded Google Takeout `.zip` files (Google Takeout is the official tool Google provides to download your entire Google Photos library—no need to unzip them!).
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

---

## 📄 License
This project is open-source and free to use under the terms of the [MIT License](LICENSE).


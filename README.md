# OAuth Token Generator

A cross-platform desktop application built with Tauri for generating OAuth access tokens.

## Features

- 🔐 Complete OAuth 2.0 PKCE flow implementation
- 🖥️ Cross-platform desktop app (Windows, macOS, Linux)
- 🎨 Modern and intuitive user interface
- 📋 One-click copy to clipboard
- 🌐 Direct browser integration
- ⚡ Fast and lightweight (built with Rust + Web technologies)

## Prerequisites

Before building or running this application, you need to install:

1. **Rust** - Install from [rustup.rs](https://rustup.rs/)
2. **Node.js** - Install from [nodejs.org](https://nodejs.org/)

## Installation

### Prerequisites

1. **Install Rust**:
   ```bash
   # Windows (PowerShell)
   Invoke-WebRequest -Uri https://win.rustup.rs/ -OutFile rustup-init.exe
   .\rustup-init.exe

   # macOS/Linux
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Node.js**:
   - Download from [nodejs.org](https://nodejs.org/)
   - Or use package manager (e.g., `winget install OpenJS.NodeJS`)

3. **Install Tauri CLI**:
   ```bash
   cargo install tauri-cli
   ```

### Quick Build (Recommended)

#### Windows:
```powershell
cd tauri-oauth-app
.\build.ps1
```

#### macOS/Linux:
```bash
cd tauri-oauth-app
chmod +x build.sh
./build.sh
```

### Manual Build

#### Development Mode:
```bash
cd tauri-oauth-app
npm install          # Install frontend dependencies
cargo tauri dev      # Start development server
```

#### Release Build:
```bash
cd tauri-oauth-app
npm install          # Install frontend dependencies
cargo tauri build    # Build for production
```

### Build Outputs

#### Windows:
- `src-tauri/target/release/tauri-oauth-app.exe` - Portable executable
- `src-tauri/target/release/bundle/msi/*.msi` - Windows installer

#### macOS:
- `src-tauri/target/release/bundle/macos/*.app` - Application bundle
- `src-tauri/target/release/bundle/dmg/*.dmg` - Disk image installer

#### Linux:
- `src-tauri/target/release/tauri-oauth-app` - Executable
- `src-tauri/target/release/bundle/deb/*.deb` - Debian package
- `src-tauri/target/release/bundle/appimage/*.AppImage` - Portable app

## Automated Builds

### GitHub Actions

This project includes GitHub Actions workflows for automated building:

1. **Release Build** (`build.yml`): Triggered on git tags
2. **Manual Build** (`manual-build.yml`): Manually triggered from GitHub Actions tab

To use GitHub Actions:
1. Push your code to GitHub
2. Go to Actions tab
3. Run "Manual Build" workflow
4. Choose platform (Windows/macOS/Linux/All)
5. Download artifacts when complete

## How to Use

1. **Generate Authorization URL**
   - Click "Generate Auth URL" button
   - Copy the generated URL or click "Open" to open in browser

2. **Authorize in Browser**
   - Complete the OAuth authorization process
   - Copy the JSON response from the browser

3. **Get Access Token**
   - Paste the JSON response into the text area
   - Click "Get Access Token"
   - Copy the generated access token and tenant URL

## Project Structure

```
tauri-oauth-app/
├── src/                    # Frontend (HTML/CSS/JS)
│   ├── index.html         # Main UI
│   ├── style.css          # Styling
│   └── script.js          # Frontend logic
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── main.rs        # Tauri app entry point
│   │   └── oauth.rs       # OAuth implementation
│   ├── Cargo.toml         # Rust dependencies
│   ├── tauri.conf.json    # Tauri configuration
│   └── build.rs           # Build script
└── README.md
```

## Technical Details

- **Frontend**: Vue 3 + Vite
- **Backend**: Rust with Tauri framework
- **HTTP Client**: reqwest (async)
- **Cryptography**: SHA-256, Base64 URL encoding
- **Security**: PKCE (Proof Key for Code Exchange) flow
- **Build Tool**: Vite (fast HMR and bundling)

## Dependencies

### Frontend Dependencies
- `vue` - Vue 3 framework
- `@tauri-apps/api` - Tauri JavaScript API
- `@vitejs/plugin-vue` - Vite Vue plugin
- `vite` - Build tool and dev server

### Rust Dependencies
- `tauri` - Desktop app framework
- `reqwest` - HTTP client
- `serde` - JSON serialization
- `base64` - Base64 encoding
- `sha2` - SHA-256 hashing
- `rand` - Random number generation
- `url` - URL parsing

## License

This project is open source and available under the MIT License.

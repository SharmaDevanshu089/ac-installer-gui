# 🚀 AutoCrate Installer GUI


A **modern, reactive, and cross-platform desktop installer** for the [AutoCrate CLI](https://github.com/SharmaDevanshu089/ac-installer).
Built with **Rust + Tauri + SvelteKit + DaisyUI**, this app merges native performance with a smooth, reactive UI.

***

## 🌟 Overview

**AutoCrate Installer GUI** eliminates the need for manual terminal commands, letting users install the CLI tool with a few clicks. It automatically fetches release data from GitHub, autodetects the right binary, and guides users through a step-by-step animated installation with real-time progress.

***

## 🧩 Core Features

### Backend (Rust + Tauri)

- **Automatic Release Fetching:** Always fetches the latest version from GitHub.
- **Smart Asset Detection:** Selects the appropriate binary by MIME type or naming.
- **In-Memory Caching:** Reduces redundant API calls during a session for speed.
- **Async Downloading:** Utilizes `reqwest` and `tokio` for non-blocking downloads.
- **System Integration:** Optionally adds AutoCrate to PATH for seamless use.


### Frontend (SvelteKit + DaisyUI)

- **Reactive UI:** Built on Svelte’s fast reactivity to mirror backend progress instantly.
- **Modern Design:** Uses Tailwind CSS and DaisyUI for a clean, themeable interface.
- **Animated User Flow:** Interactive modals, progress bars, and completion visuals.
- **Real-Time Progress:** Backend emits events directly to the UI for live updates.
- **External Links:** Quick access to GitHub and docs via Tauri's URL APIs.

***

## 🧠 Technical Architecture

| Layer | Technology | Responsibility |
| :-- | :-- | :-- |
| 🦀 Backend | Rust + Tauri | API requests, caching, file I/O, installation logic |
| 🧩 Frontend | SvelteKit | Reactive UI, user input, event listeners |
| 🎨 Styling | Tailwind + DaisyUI | Modern, themeable interface |
| ⚡ Runtime | Tokio | Async networking and background tasks |

**Flow:**
GitHub API → Rust (`reqwest` + async/await)
↓
AppState (Mutex Cache)
↓
Tauri Commands ↔ SvelteKit invoke()
↓
Live UI Updates via Events

***

## 🧭 User Flow

1. App opens → backend fetches latest release from GitHub.
2. "Install" button activates when ready.
3. Modal displays release details for confirmation.
4. User confirms → animated installation progress.
5. Backend streams download → frontend displays progress and status.
6. Success = Animated checkmark with “Installation Complete!”

***

## 🔩 Code Highlights

**Stateful Backend Example:**

```rust
pub struct AppState {
    pub download_url: Mutex<Option<String>>,
}
```

**Async Fetch Example:**

```rust
let release = client
    .get(URL)
    .header("User-Agent", "AutoCrate Installer")
    .send()
    .await?
    .json::<ReleaseInfo>()
    .await?;
```

**Frontend Invoking Backend:**

```ts
import { invoke } from "@tauri-apps/api/core";
await invoke("download_binary");
```


***

## 🧰 Getting Started

### Run Locally

```bash
git clone https://github.com/SharmaDevanshu089/ac-installer-gui
cd ac-installer-gui
npm install
cargo tauri dev
```


### Build Production Binary

```bash
cargo tauri build
```


***

## 🧾 Roadmap

- [x] Smart API fetching
- [x] Reactive Svelte UI
- [x] Automatic binary detection
- [ ] Implement real, streaming progress bar
- [ ] Support for .zip/.msi releases
- [ ] Cross-platform (Linux/macOS) support
- [ ] Persistent cache for offline mode
- [ ] Settings modal (install directory, theme)
- [ ] Updater logic for AutoCrate CLI

***

## 📸 Visual Demo

[![Watch the Gif Demo](gif.gif)](gif.gif)

***

## ⚡ Command-Line Version

Prefer the terminal?
Check out the [AutoCrate CLI](https://github.com/SharmaDevanshu089/ac-installer) — the same fast engine, now scriptable.

***

## 🧑‍💻 Author

**Devanshu Sharma**
GitHub: [SharmaDevanshu089](https://github.com/SharmaDevanshu089)
Builds high-quality Rust tools, open-source enthusiast.

***

## 📜 License

MIT License — free to use, modify, and share. See [LICENSE](./LICENSE).

***

## ℹ️ Notes / References

- Powered by Rust for speed and reliability.[^1]
- Secure and lightweight hybrid desktop app built with Tauri 2.0.[^2]
- Universal, reactive frontend with SvelteKit and DaisyUI.

***

Copy, customize, and extend! Great docs welcome more users and contributors.

<div align="center">⁂</div>

[^1]: https://www.rust-lang.org

[^2]: https://tauri.app


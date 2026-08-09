# Friday AI - Autonomous System Automation & Engineering Co-worker

<div align="center">

![Friday AI Logo](./logo.jpg)

**Friday AI** is an advanced, high-performance local AI assistant and autonomous engineering co-worker built from scratch in **Rust** for maximum memory safety, concurrency, and speed. It integrates real-time voice streams, file/ZIP processors, Whisper Flow prompt refinement, and sandboxed developer co-worker capabilities.

</div>

---

## 📄 Project Proposal & Product Architecture
Read the complete 28-section product architecture proposal:
* 📥 **[Download Executive Proposal PDF (PROPOSAL.pdf)](./PROPOSAL.pdf)**
* 📖 **[View Product Proposal Markdown (PROPOSAL.md)](./PROPOSAL.md)**

---

## 🚀 Interactive Live Demo
Try the glassmorphic control console directly on the web:
👉 **[Friday AI Live Dashboard Demo](https://hamz963.github.io/Friday-AI-Assistant/)**

---

## 🎓 Educational Purpose & Security Architecture

Friday AI is developed to study the safety, behavior, and security implications of letting autonomous LLM agents execute local system commands. Since raw command execution presents high security risks, the project explores several mitigation strategies:
* **Whisper Flow speech normalization**: Pre-processes raw inputs to filter noise and map requests to predefined action boundaries.
* **Terminal Sandbox (Command Filtering)**: Implements regex parsers to detect and block destructive operations (e.g. recursive overrides or root privilege escalations).
* **Safe Diagnostics Loop**: Confines compilation tests to restricted workspace folders to avoid side-channel modifications.

---

## 🛠️ Ways of Using Friday AI

1. **Desktop Command Center (Dashboard Mode)**:
   Launch the Axum daemon server locally (`cargo run --bin friday`) to open the interactive Web UI. This serves as a control center where you can view live telemetry (CPU/RAM load), run console commands, and interact with the assistant.
2. **Autonomous Developer Co-worker (CI/Diagnostics Mode)**:
   Integrate Friday AI with your project build system to automatically run checks, parse errors, and suggest diagnostic revisions.
3. **Local Speech Assistant (Voice Mode)**:
   Configure local CPAL input streams to run hotkey voice activation, processing speech prompts in the background.

---

## 📊 Comparison: Friday AI vs. OpenJarvis vs. OpenClaw

| Feature | **Friday AI (This Project)** | **OpenJarvis** | **OpenClaw** |
|:---|:---|:---|:---|
| **Core Language** | **Rust (Native)** | Python | Python |
| **System Drivers**| Direct OS Key/Mouse & Headless Browser | App Shell / Python SDK | Chat Application Hooks |
| **Safety Sandbox**| Integrated Terminal & Command Filters | External Containers | User Approvals |
| **Prompt Pipeline**| Whisper Flow Real-time Refiner | Standard LLM Templates | Vector Database RAG |
| **Real-time Audio**| Native CPAL/Rodio Stream Capture | External API Calls | App Voice Notes |
| **Diagnostics** | Auto-debugging Rust Compiler loop | Standard trace prints | External LSP plugins |
| **Distribution** | Statically linked `.exe`, `.msi` | Local source run | Local source run |

---

## 🛠️ System Architecture & Workspace Modules

Friday AI is organized as a modular Cargo workspace containing 16 crates:

*   **`friday-core`**: Configuration manager (wake word, LLM model choice) and hardware profiles detector. Contains the `SystemMetricsTracker` for CPU and RAM diagnostics.
*   **`friday-llm`**: Multi-provider adapters (OpenAI, Gemini, Ollama, Anthropic) and `html2md` URL crawler/scraper.
*   **`friday-memory`**: High-performance SQLite agent message history and telemetry database.
*   **`friday-refiner`**: Whisper Flow speech filler refiner that transforms raw voice requests into target actions.
*   **`friday-generator`**: Direct vector SVG layout writers and directory boilerplate creators.
*   **`friday-terminal`**: Command line execution sandbox that sanitizes commands to prevent destructive shell operations.
*   **`friday-git`**: Repository manager that tracks branch logs and commit history.
*   **`friday-diagnostics`**: Automates compiler error resolution, running diagnostic checks to debug files.
*   **`friday-api`**: High-performance Axum REST API web server.
*   **`friday-voice`**: CPAL and Rodio hooks for recording and playing audio files.
*   **`friday-files`**: Zip compressions and recursive directory traversal.
*   **`friday-desktop`**: System keyboard and mouse keypress event driver.
*   **`friday-browser`**: Headless Chrome controller for scraping and interacting with web apps.
*   **`friday-agents`**: Orchestrates workflows between browser drivers and filesystem crates.

---

## 📦 Installation & Getting Started

### Windows (Recommended)
1. Download the **`friday-installer.msi`** from the **[Releases Tab](https://github.com/hamz963/Friday-AI-Assistant/releases)**.
2. Run the installer to place Friday AI on your local environment.
3. Open a terminal and run `friday` to launch the Axum web server and dashboard automatically.

### macOS & Linux
1. Download the latest tarball release (`friday-macos-x64.tar.gz` or `friday-linux-x64.tar.gz`).
2. Extract the archive:
   ```bash
   tar -xzf friday-*.tar.gz
   ./friday
   ```

---

## 📡 REST API Reference

The embedded Axum server exposes the following endpoints:

*   **`GET /`**: Serves the embedded HTML/CSS/JS glassmorphism dashboard.
*   **`GET /assets/logo.jpg`**: Serves the embedded brand logo image asset.
*   **`GET /api/metrics`**: Captures CPU load, used RAM, and command execution latency.
*   **`GET /api/git`**: Queries local workspace repository tree logs.
*   **`POST /api/terminal`**: Safely executes terminal commands through the sandboxing filter.
*   **`POST /api/chat`**: Evaluates chat prompts through the Whisper Flow refiner and triggers OS/browser automations.

---

## 📄 License
This project is open-source under the MIT License.

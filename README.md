# F.R.I.D.A.Y. AI - Autonomous System Automation & Engineering Co-worker

<div align="center">

![Friday AI Logo](./logo.jpg)

### **F.R.I.D.A.Y.**
**F**ast, **R**esilient & **I**ntelligent **D**esktop **A**gent **Y**ield-Engine

*Your AI. Your Computer. Your Projects. Your Agent.*

</div>

---

---

## 📦 Downloads & Releases (v1.5.0 Latest - 100% Local Privacy Mode)

Download the pre-compiled, zero-dependency production binaries for your platform:

| Platform | Download Asset | Description |
|:---|:---|:---|
| **Windows 64-bit** | 💻 **[friday.exe (11.9 MB)](https://github.com/hamz963/Friday-AI/releases/download/v1.5.0/friday.exe)** | Native zero-dependency Windows executable |
| **Windows Zip Package** | 📦 **[friday-windows-x64.zip (5.5 MB)](https://github.com/hamz963/Friday-AI/releases/download/v1.5.0/friday-windows-x64.zip)** | Compressed release bundle for Windows |
| **macOS 64-bit / ARM** | 🍏 **[friday-macos-x64.tar.gz (5.6 MB)](https://github.com/hamz963/Friday-AI/releases/download/v1.5.0/friday-macos-x64.tar.gz)** | Compressed release tarball for macOS |
| **All Releases** | 🚀 **[View Release Page (v1.5.0)](https://github.com/hamz963/Friday-AI/releases/tag/v1.5.0)** | Tagged GitHub Release Page |

---

## 📄 Project Proposal, Architecture & Presentation Deck
Read the complete product specifications and executive pitch deck:
* 📊 **[Download PowerPoint Deck (SLIDES.pptx)](./SLIDES.pptx)** | **[PDF Version (SLIDES.pdf)](./SLIDES.pdf)** | **[View Markdown (SLIDES.md)](./SLIDES.md)**
* 📥 **[Download Product Proposal PDF (PROPOSAL.pdf)](./PROPOSAL.pdf)** | **[View Proposal (PROPOSAL.md)](./PROPOSAL.md)**

---

## 🚀 Interactive Live Demo
Try the glassmorphic control console directly on the web:
👉 **[Friday AI Live Web Portal Demo](https://hamz963.github.io/Friday-AI/)**

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

## 🌟 Key Architecture & Highlights

- ⚡ **Native Rust Architecture**: High-speed, memory-safe system performance with a single binary deployment (`friday.exe`).
- 🤖 **Multi-Provider LLM Core**: Supports OpenAI GPT-4o, Anthropic Claude 3.7 Sonnet, DeepSeek R1, Google Gemini 2.5 Pro, xAI Grok 3, and custom model IDs.
- 🎨 **Nano Banana AI Media Studio**: Photorealistic AI art & motion generation powered by Nano Banana, Midjourney v6, SDXL Turbo, and FLUX.1 Pro engines.
- 💬 **Inline Artifact Viewports**: Embedded interactive application runner inside message bubbles.
- 🔒 **100% On-Device Local Privacy**: System metrics, command executions, RAG vector searches, and diagnostic logging remain strictly on your machine.

---

## 🏗️ Workspace Crate Architecture

The Friday AI project is structured as a modular Rust workspace (`Cargo.toml`):

```
Friday-AI/
├── Cargo.toml (Root Workspace)
├── index.html (SaaS Web Portal Demo)
├── vercel.json (Vercel Cloud Deployment Config)
├── .github/workflows/pages.yml (GitHub Pages Deployment Workflow)
└── crates/
    ├── api/        # REST API endpoints & Axum web server
    ├── cli/        # Command-line launcher & CLI tools
    ├── core/       # Task queue, agent execution loop, state management
    ├── diagnostics/# Codebase auto-patching & diagnostics subsystem
    ├── generator/  # Nano Banana & FLUX.1 Media Generator Engine
    ├── git/        # Workspace Git status & versioning helper
    ├── llm/        # Multi-provider LLM client abstractions
    ├── mcp/        # Model Context Protocol (MCP) Stdio JSON-RPC hub
    ├── memory/     # Persistent conversation & agent memory
    ├── rag/        # Local SQLite Vector RAG search engine
    ├── security/   # Workspace sandbox & command safety rules
    ├── terminal/   # Interactive sandboxed shell execution
    ├── tts/        # Local text-to-speech voice engine
    ├── vision/     # Multimodal desktop screenshot analyzer
    └── whisper/    # Speech-to-text audio pipeline
```

---

## 💻 Building from Source

Ensure you have Rust and Cargo installed:

```bash
# Clone the repository
git clone https://github.com/hamz963/Friday-AI.git
cd Friday-AI

# Check workspace compilation
cargo check --workspace

# Run Friday AI locally
cargo run --bin friday
```

---

## 👨‍💻 Author & System Architect

Designed, architected, and built by **Hamza Abdul Karim**.
- GitHub: [github.com/hamz963](https://github.com/hamz963)
- Project Repo: [github.com/hamz963/Friday-AI](https://github.com/hamz963/Friday-AI)
- Live Web Demo: [hamz963.github.io/Friday-AI](https://hamz963.github.io/Friday-AI/)

*Licensed under MIT.*

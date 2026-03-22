# SimpleVoice — Project Rules

## Project Overview
Cross-platform desktop voice input tool. Global hotkey → mic → local Whisper transcription → text injected at cursor.

## Tech Stack
- **Frontend**: Tauri v2 + React (TypeScript)
- **Backend**: Rust (Tauri core) + whisper.cpp via `whisper-rs` crate
- **Input simulation**: `enigo` crate
- **Global hotkey**: `tauri-plugin-global-shortcut`
- **Tray**: `tauri-plugin-tray`

## Directory Layout
```
SimpleVoice/
├── src/                  # React frontend (TypeScript)
│   ├── components/
│   ├── hooks/
│   ├── pages/
│   └── main.tsx
├── src-tauri/            # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── audio.rs      # mic capture
│   │   ├── transcribe.rs # whisper integration
│   │   ├── inject.rs     # keyboard injection via enigo
│   │   ├── hotkey.rs     # global shortcut registration
│   │   └── tray.rs       # system tray
│   ├── Cargo.toml
│   └── tauri.conf.json
├── models/               # whisper GGML model files (gitignored)
├── CLAUDE.md
├── package.json
└── .gitignore
```

## Naming Conventions
- Rust: `snake_case` for files, functions, variables; `PascalCase` for types/structs
- TypeScript/React: `camelCase` for variables/functions; `PascalCase` for components; `kebab-case` for files
- Tauri commands: `snake_case` (e.g., `start_recording`, `stop_recording`)

## Commit Convention (Conventional Commits)
```
feat: add global hotkey toggle
fix: audio buffer overflow on long recordings
chore: update whisper-rs to 0.11
docs: update README with model download instructions
```

## Code Style
- Rust: `cargo fmt` + `cargo clippy --deny warnings` before commit
- TypeScript: ESLint + Prettier (2-space indent, single quotes)
- No `unwrap()` in production paths — use `?` or explicit error handling
- No `console.log` left in committed code — use structured logging (`tracing` crate in Rust)

## Forbidden
- No cloud API calls without explicit user opt-in (privacy-first)
- No telemetry or analytics without consent
- No storing audio files to disk (process in memory only)
- No `unsafe` Rust blocks without a comment explaining why

## Performance Targets
- End-to-end latency: < 1.5s (hotkey release → text injected)
- Model: whisper `small` or `base` by default (balance speed/accuracy)
- Preferred language: Chinese (zh) primary, English (en) secondary

## Common Commands
```bash
# Dev
pnpm tauri dev

# Build
pnpm tauri build

# Rust checks
cd src-tauri && cargo fmt && cargo clippy

# Frontend checks
pnpm lint && pnpm typecheck

# Tests
cd src-tauri && cargo test
pnpm test
```

## Models Directory
- Place GGML model files in `models/` (gitignored, ~150MB for `small`)
- Download: `scripts/download_model.sh <tiny|base|small>`
- Default model path configured in `tauri.conf.json` → `app.modelPath`

## Error Handling Policy
- All Tauri commands return `Result<T, String>` — never panic in command handlers
- Frontend shows user-facing error toasts for all command failures
- Audio errors (device not found, permission denied) must surface to UI immediately

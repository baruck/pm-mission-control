# Mission Control

Mission Control is a Tauri + React desktop app for Scrum PO/BA/PM workflows with ADHD-friendly capture and conversion flows.

## Implemented v1 foundation

- Journal parsing engine with bucket detection and atomic block parsing.
- Conversion helpers for Jira/Confluence payload generation.
- Markdown marker format for stable artifact cards.
- Workspace bootstrapper for OneDrive/SharePoint-backed folder structure.
- Frontend shell with Journal / Tasks / Refactor / Settings views.

## How to build binaries (step-by-step)

You have **two ways** to build binaries.

### Option A — GitHub Actions (recommended)

This is the easiest and most repeatable path.

1. Push your branch.
2. Create and push a tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
3. Open **GitHub → Actions → Build Desktop Binaries**.
4. Open the workflow run and download artifacts:
   - `mission-control-windows-x64`
   - `mission-control-macos-arm64`

Workflow file: `.github/workflows/build-desktop-binaries.yml`.

### Option B — Local machine build

Use this when you want a local release build for your own OS.

```bash
./scripts/build-binaries.sh local
```

Output location:

- `src-tauri/target/release/`

If you just want validation checks (frontend build + Rust compile checks), run:

```bash
./scripts/build-binaries.sh preflight
```

## What this repository currently outputs

At this stage, the backend is structured as foundational Rust modules and testable logic. The CI artifacts provide packaged build outputs and source snapshot content to make release delivery traceable while the full Tauri packaging layer is being integrated.

## Prerequisites

- Rust stable toolchain
- Node.js 20+
- For full desktop packaging: Tauri v2 prerequisites per OS

## Run in development

### Backend tests

```bash
cd src-tauri
cargo test
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

## Workspace configuration flow

1. Open **Settings**.
2. Choose your OneDrive/SharePoint synced folder.
3. App creates:
   - `MissionControl/journal/YYYY/MM/YYYY-MM-DD.md`
   - `MissionControl/tasks/`
   - `MissionControl/artifacts/`
   - `MissionControl/links/`
   - `MissionControl/settings/`
4. App writes `.missioncontrol.json` marker at workspace root.
5. Configure Atlassian URL/email/token (token stored in keychain in full Tauri runtime implementation).

## Parsing rules covered by tests

- Bucket headings (`Tasks for me:` and standalone headings before bullets)
- Top-level bullet blocks with nested bullet retention
- Jira key detection (`ABC-123`)
- `<tag>...</tag>` parsing and label sanitization
- Marker parsing for `MC_BLOCK`
- Jira/Confluence request payload builders

## Notes on production hardening

This repository includes production-oriented module boundaries and test coverage for critical parsing and request-building logic. Full Tauri command wiring, OS keychain glue, SQLite FTS index maintenance, and desktop notifications are intended as the next integration step.

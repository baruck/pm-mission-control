# Mission Control

Mission Control is a Tauri + React desktop app for Scrum PO/BA/PM workflows with ADHD-friendly capture and conversion flows.

## Implemented v1 foundation

- Journal parsing engine with bucket detection and atomic block parsing.
- Conversion helpers for Jira/Confluence payload generation.
- Markdown marker format for stable artifact cards.
- Workspace bootstrapper for OneDrive/SharePoint-backed folder structure.
- Frontend shell with Journal / Tasks / Refactor / Settings views.

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

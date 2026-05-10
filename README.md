# CloudLite Frontend

CloudLite Frontend is a Tauri desktop app built with Vue and TypeScript. It is part of CloudLite, a lightweight self-hosted cloud storage project inspired by Nextcloud-style file sync, built mainly for learning and experimentation.

The app provides the client UI for logging in, browsing synced files and folders, and sending local file or directory actions to the Rust backend.

## Scope

This project currently focuses on the desktop frontend shell, authentication flow, file browsing UI, drag-and-drop file ingestion, synchronization with the CloudLite backend, and sync command integration with the Tauri backend.

CloudLite backend repository: TODO: add backend repository URL.

## Architecture

- Vue.js and TypeScript provide the desktop UI.
- Tauri wraps the web UI as a desktop app and exposes native APIs.
- Rust handles local backend commands, filesystem access, and sync orchestration.
- SQLite stores local sync metadata and changelog state.
- Keycloak OAuth2 is used for authentication.

## Commands

```bash
npm run dev
npm run build
npm run tauri
```

## Screenshots

### Login Page

![Login page screenshot placeholder](docs/screenshots/login-page.png)

### Files Page

![Files page screenshot placeholder](docs/screenshots/files-page.png)

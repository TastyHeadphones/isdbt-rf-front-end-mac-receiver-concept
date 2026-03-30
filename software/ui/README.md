# UI (Tauri + TypeScript Skeleton)

This UI provides a diagnostics and signal metrics skeleton for the host backend API.

## Current Scope

- Diagnostics section: backend health and runtime mode
- Signal metrics section: packet/byte counters and latest replay file
- Device section: simulated or connected device list

## Run in Browser Dev Mode

```bash
cd software/ui
npm install
npm run dev
```

## Tauri Shell Scaffold

`src-tauri/` includes a Tauri desktop shell starter for future desktop packaging and integration.

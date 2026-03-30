# API and Interface Definitions

## Host Backend Base URL

- Default: `http://127.0.0.1:8088`

## HTTP Endpoints

### `GET /health`

Response:

```json
{
  "status": "ok",
  "mode": "lab_demo"
}
```

### `GET /api/v1/devices`

Response:

```json
[
  {
    "id": "sim-demod-001",
    "mode": "lab_demo",
    "locked": false,
    "snr_db": 0.0,
    "ber": 1.0
  }
]
```

### `POST /api/v1/tune`

Request:

```json
{
  "device_id": "sim-demod-001",
  "frequency_hz": 569142857,
  "bandwidth_mhz": 6
}
```

Response:

```json
{
  "device_id": "sim-demod-001",
  "accepted": true,
  "message": "tune request accepted in lab simulation mode"
}
```

### `GET /api/v1/metrics`

Response:

```json
{
  "packets_total": 5000,
  "bytes_total": 940000,
  "last_replay_file": "../tests/data/synthetic_demo.ts",
  "last_update_unix_ms": 1760000000000
}
```

### `POST /api/v1/replay`

Request:

```json
{
  "file": "../tests/data/synthetic_demo.ts"
}
```

Response:

```json
{
  "packets_ingested": 5000,
  "bytes_ingested": 940000,
  "file": "../tests/data/synthetic_demo.ts"
}
```

## Control Interface Abstraction

- Device discovery and tuning are API-level abstractions over simulated or hardware-backed bridge flows.
- Replay endpoint is an explicit lab/demo path for legal stream validation.

## Transport Stream Intake Contract

- Ingestor expects binary payload where packet alignment may be 188-byte MPEG-TS style.
- Current scaffold counts bytes and packet-equivalent units (`bytes / 188`).
- Exact demux/decode integration is intentionally deferred.

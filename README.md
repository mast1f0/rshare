# rshare

`rshare` is a small local file-sharing tool written in Rust (Axum).
It serves a simple web gallery and upload page in your local network.

## Requirements

- Rust toolchain (`cargo`, `rustc`)
- `make`
- `sudo` (only for install/uninstall to system directories)

## Build

```bash
make build
```

## Install

Build as your user, then install as root:

```bash
make build
sudo make install CARGO=true
```

Default install paths:

- app data: `/opt/rshare`
- binary: `/usr/local/bin/rshare`

## Run

```bash
rshare
```

With startup files:

```bash
rshare /path/to/file1 /path/to/file2
```

Clean shared directory before start:

```bash
rshare --clean
```

## Configuration

- `RSHARE_APP_DIR` — base directory for templates, shared files, and QR image.
  Default: `/opt/rshare`.

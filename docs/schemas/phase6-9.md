# ROADMAP-v1.0 Phases 6-9: Schemas and Pseudocode

This document provides schemas, pseudocode, and high-level plans for the remaining phases of ROADMAP-v1.0. It focuses on generating structured outlines ready for implementation, including optimizations and preparation for Raspberry Pi coding.

## 6. Frontend Proxy Schema (next.config WS)

### Description
Configure a Next.js frontend to proxy WebSocket (WS) connections, enabling real-time communication (e.g., for agent updates or live data streams). This involves updating `next.config.js` to handle WS proxying, potentially integrating with Vercel or custom servers.

### Pseudocode/Schema (JSON-like for next.config.js)
```javascript
// next.config.js schema (pseudocode)
module.exports = {
  // Base configuration
  reactStrictMode: true,
  // WebSocket proxy setup
  async rewrites() {
    return [
      {
        source: '/ws/:path*',  // Match WS routes
        destination: 'ws://backend-service:port/:path*'  // Proxy to backend WS server
      },
      {
        source: '/api/:path*',  // Optional API proxy for consistency
        destination: 'http://backend-service:port/api/:path*'
      }
    ];
  },
  // Environment variables for dynamic config
  env: {
    WS_BACKEND_URL: process.env.WS_BACKEND_URL || 'ws://localhost:3001',
    PROXY_ENABLED: process.env.PROXY_ENABLED === 'true'
  },
  // Optimizations: Enable compression for WS traffic
  compress: true,
  // Headers for WS security
  async headers() {
    return [
      {
        source: '/ws/:path*',
        headers: [
          { key: 'Upgrade', value: 'websocket' },
          { key: 'Connection', value: 'Upgrade' },
          { key: 'Sec-WebSocket-Protocol', value: 'chat' }  // Custom protocol if needed
        ]
      }
    ];
  }
};

// Usage pseudocode
if (process.env.NODE_ENV === 'production') {
  // Add production-specific proxy rules
}
```

### Implementation Notes
- Use `http-proxy-middleware` for advanced proxying if built-in rewrites are insufficient.
- Test with WebSocket clients to ensure no connection drops.

## 7. Install.sh (Hardware Detect RTX/Jetson Vars/Debug)

### Description
A bash installation script that detects hardware (e.g., NVIDIA RTX, Jetson), sets environment variables, enables debug modes, and installs dependencies. Supports Raspberry Pi compatibility checks.

### Pseudocode (Bash-like)
```bash
#!/bin/bash

# Constants
DEBUG=false
HARDWARE_TYPE="unknown"
RTX_VARS=""
JETSON_VARS=""

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --debug) DEBUG=true; shift ;;
    *) echo "Unknown option: $1"; exit 1 ;;
  esac
done

# Detect hardware
if command -v nvidia-smi &> /dev/null; then
  if nvidia-smi | grep -q "Jetson"; then
    HARDWARE_TYPE="jetson"
    JETSON_VARS="JETSON=1 CUDA_HOME=/usr/local/cuda"
  else
    HARDWARE_TYPE="rtx"
    RTX_VARS="RTX=1 GPU_ARCH=sm_86"
  fi
elif [[ $(uname -m) == "aarch64" && -f /proc/device-tree/model && grep -q "Raspberry Pi" /proc/device-tree/model ]]; then
  HARDWARE_TYPE="pi"
  echo "Raspberry Pi detected - limited GPU support"
else
  echo "No supported hardware detected"
  exit 1
fi

# Set variables and debug
if $DEBUG; then
  echo "Debug mode: HARDWARE_TYPE=$HARDWARE_TYPE"
  echo "Exporting: $RTX_VARS $JETSON_VARS"
fi

# Install dependencies
if [[ $HARDWARE_TYPE == "rtx" || $HARDWARE_TYPE == "jetson" ]]; then
  sudo apt update
  sudo apt install -y cuda-toolkit libnvrtc
fi

# Export vars
export $RTX_VARS $JETSON_VARS

# Completion
echo "Installation complete for $HARDWARE_TYPE"
```

### Implementation Notes
- Add error handling for missing commands.
- For Pi: Include cross-compilation flags if needed.

## 8. CI.yml (Binary Size/Cargo Test)

### Description
GitHub Actions workflow for testing Cargo crates, checking binary sizes, and running tests. Ensures builds are optimized and under size limits.

### Schema (YAML)
```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run Cargo Test
        run: cargo test --verbose
      - name: Check Binary Size
        run: |
          cargo build --release
          SIZE=$(stat --printf="%s" target/release/my-binary)
          if [ $SIZE -gt 10485760 ]; then  # 10MB limit
            echo "Binary too large: $SIZE bytes"
            exit 1
          fi
          echo "Binary size OK: $SIZE bytes"

  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run Cargo Clippy
        run: cargo clippy -- -D warnings
```

### Implementation Notes
- Add matrix for multi-platform builds (e.g., arm64 for Pi).
- Integrate with Crates.io publishing in a release job.

## 9. Crate.io (Cargo.toml Lib/Examples)

### Description
Configure Cargo.toml for publishing a Rust library to Crates.io, including examples and documentation.

### Schema (TOML)
```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"
description = "A library for AI agent optimizations"
license = "MIT"
repository = "https://github.com/user/my-crate"
documentation = "https://docs.rs/my-crate"
keywords = ["ai", "optimization", "cuda"]
categories = ["algorithms", "science"]

[lib]
name = "my_lib"
path = "src/lib.rs"

[[example]]
name = "baton_lineage"
path = "examples/baton_lineage.rs"

[[example]]
name = "cuda_graphs"
path = "examples/cuda_graphs.rs"

[dependencies]
anyhow = "1.0"
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
criterion = "0.5"  # For benchmarking optimizations
```

### Implementation Notes
- Ensure `cargo publish` works by testing locally.
- Add README.md with usage examples.

## High-Level Optimizations

### Baton Lineage
- **Concept**: Track computation "batons" (tasks) with lineage metadata (parent tasks, dependencies) to enable efficient resuming and error recovery in agent workflows.
- **Pseudocode**:
  ```rust
  struct Baton {
      id: Uuid,
      lineage: Vec<Uuid>,  // Parent batons
      task: fn() -> Result<(), Error>,
  }

  fn pass_baton(baton: Baton) -> Result<Baton, Error> {
      let mut new_baton = baton.clone();
      new_baton.lineage.push(baton.id);
      (baton.task)()?;  // Execute
      Ok(new_baton)
  }
  ```
- **Benefits**: Reduces recomputation in distributed systems.

### CUDA Graphs
- **Concept**: Use CUDA graphs for batched GPU operations, capturing sequences of kernels for faster execution (e.g., in AI inference on RTX/Jetson).
- **Pseudocode**:
  ```rust
  use cuda_rs::{Graph, Stream};

  fn optimize_with_graph(stream: &Stream) -> Graph {
      let mut graph = Graph::new();
      graph.begin_capture(stream);
      // Add kernels: e.g., matrix_multiply(kernel_args)
      graph.end_capture(stream);
      graph  // Replayable graph for perf
  }
  ```
- **Benefits**: Lowers overhead for repetitive GPU tasks.

## Preparation for Pi Coding
- **Cross-Compilation**: Use `cross` crate for building on x86 for arm64 (Pi).
- **Hardware Vars**: Extend install.sh to set `PI=1` and use CPU-only fallbacks (no CUDA).
- **Testing**: Add CI matrix for arm64.
- **Optimizations**: Focus on lightweight batons without GPU deps for Pi compatibility.
- **Next Steps**: Ready for coding - implement schemas in respective files/directories.

Generated and ready for implementation.

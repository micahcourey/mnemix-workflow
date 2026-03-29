#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"

require_command() {
  local command_name="$1"
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "required command not found: $command_name" >&2
    exit 1
  fi
}

require_command docker

docker run --rm \
  --platform linux/amd64 \
  -v "$repo_root:/work:ro" \
  ubuntu:22.04 \
  bash -lc '
    set -euo pipefail
    export DEBIAN_FRONTEND=noninteractive
    apt-get update
    apt-get install -y --no-install-recommends \
      build-essential \
      ca-certificates \
      curl \
      git \
      libssl-dev \
      pkg-config \
      python3 \
      python3-pip \
      python3-venv
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.85.0 --profile minimal
    . "$HOME/.cargo/env"
    cp -a /work /tmp/mnemix-workflow-linux-release-src
    export CARGO_TARGET_DIR=/tmp/mnemix-workflow-linux-release-target
    cd /tmp/mnemix-workflow-linux-release-src
    cargo build --release --bin mnemix-workflow --bin mxw --bin mnx
    PYTHON_BIN=python3 \
    TP_WORKFLOW_BINARY=/tmp/mnemix-workflow-linux-release-target/release/mnemix-workflow \
    TP_MXW_BINARY=/tmp/mnemix-workflow-linux-release-target/release/mxw \
    TP_MNX_BINARY=/tmp/mnemix-workflow-linux-release-target/release/mnx \
      ./scripts/check-python-bundled-wheel.sh
  '

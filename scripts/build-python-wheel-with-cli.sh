#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
python_root="$repo_root/python"
python_bin="${PYTHON_BIN:-$python_root/.venv/bin/python}"
workflow_binary="${MNEMIX_WORKFLOW_CLI_BINARY:-${TP_WORKFLOW_BINARY:-$repo_root/target/debug/mnemix-workflow}}"
mxw_binary="${MXW_CLI_BINARY:-${TP_MXW_BINARY:-$repo_root/target/debug/mxw}}"
mnx_binary="${MNX_CLI_BINARY:-${TP_MNX_BINARY:-$repo_root/target/debug/mnx}}"
staging_dir="$python_root/mnemix_workflow/_bin"
build_venv="$python_root/.build-wheel-venv"

resolve_python() {
    local candidate="$1"
    if [[ -x "$candidate" ]]; then
        echo "$candidate"
        return 0
    fi
    if command -v "$candidate" >/dev/null 2>&1; then
        command -v "$candidate"
        return 0
    fi
    return 1
}

venv_python() {
    local venv_dir="$1"
    if [[ -x "$venv_dir/bin/python" ]]; then
        echo "$venv_dir/bin/python"
    else
        echo "$venv_dir/Scripts/python.exe"
    fi
}

ensure_default_binaries() {
    if [[ "${MNEMIX_WORKFLOW_CLI_BINARY:-}" != "" || "${MXW_CLI_BINARY:-}" != "" || "${MNX_CLI_BINARY:-}" != "" || "${TP_WORKFLOW_BINARY:-}" != "" || "${TP_MXW_BINARY:-}" != "" || "${TP_MNX_BINARY:-}" != "" ]]; then
        return 0
    fi

    if [[ -f "$workflow_binary" && -f "$mxw_binary" && -f "$mnx_binary" ]]; then
        return 0
    fi

    echo "debug binaries missing; building CLI binaries first"
    cargo build --manifest-path "$repo_root/Cargo.toml" --bin mnemix-workflow --bin mxw --bin mnx
}

stage_binary() {
    local source_path="$1"
    local target_name="$2"

    if [[ ! -f "$source_path" ]]; then
        echo "CLI binary not found: $source_path" >&2
        exit 1
    fi

    local target_path="$staging_dir/$target_name"
    if [[ "$source_path" == *.exe ]]; then
        target_path="$target_path.exe"
    fi

    cp "$source_path" "$target_path"
    chmod +x "$target_path" 2>/dev/null || true
}

python_bin="$(resolve_python "$python_bin")" || {
    echo "python interpreter not found: $python_bin" >&2
    exit 1
}

cleanup() {
    rm -f "$python_root/mnemix_workflow/_bin/mnemix-workflow"
    rm -f "$python_root/mnemix_workflow/_bin/mnemix-workflow.exe"
    rm -f "$python_root/mnemix_workflow/_bin/mxw"
    rm -f "$python_root/mnemix_workflow/_bin/mxw.exe"
    rm -f "$python_root/mnemix_workflow/_bin/mnx"
    rm -f "$python_root/mnemix_workflow/_bin/mnx.exe"
    rm -rf "$build_venv"
    rmdir "$staging_dir" 2>/dev/null || true
}

trap cleanup EXIT

ensure_default_binaries

mkdir -p "$staging_dir"
stage_binary "$workflow_binary" "mnemix-workflow"
stage_binary "$mxw_binary" "mxw"
stage_binary "$mnx_binary" "mnx"

rm -rf "$build_venv"
"$python_bin" -m venv "$build_venv"
build_python="$(venv_python "$build_venv")"

cd "$python_root"
rm -rf dist build
"$build_python" -m pip install --quiet --upgrade pip build
"$build_python" -m build --wheel

wheel_path="$(ls dist/*.whl)"
case "$wheel_path" in
    *-any.whl)
        echo "expected a platform-specific wheel, got: $wheel_path" >&2
        exit 1
        ;;
esac

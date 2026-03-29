#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
python_root="$repo_root/python"
python_bin="${PYTHON_BIN:-$python_root/.venv/bin/python}"
verify_venv="$python_root/.bundled-wheel-venv"

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

python_bin="$(resolve_python "$python_bin")" || {
    echo "python interpreter not found: $python_bin" >&2
    exit 1
}

cleanup() {
    rm -rf "$verify_venv"
}

trap cleanup EXIT

"$repo_root/scripts/build-python-wheel-with-cli.sh"

rm -rf "$verify_venv"
"$python_bin" -m venv "$verify_venv"
verify_python="$(venv_python "$verify_venv")"
"$verify_python" -m pip install --upgrade pip
"$verify_python" -m pip install "$python_root"/dist/*.whl

cd "$repo_root"
"$verify_python" <<'PY'
import os
import subprocess
from pathlib import Path
import sys
import sysconfig

from mnemix_workflow._runner import bundled_binaries

for env_name in ("MNEMIX_WORKFLOW_BINARY", "MXW_BINARY", "MNX_BINARY"):
    os.environ.pop(env_name, None)

paths = bundled_binaries()
assert len(paths) == 3, paths
for path in paths:
    assert Path(path).exists(), path

commands = {
    "mnemix-workflow": ["mnemix-workflow", "--help"],
    "mxw": ["mxw", "--help"],
    "mnx": ["mnx", "--help"],
}

scripts_dir = Path(sysconfig.get_path("scripts"))

for name, command in commands.items():
    script_name = f"{name}.exe" if os.name == "nt" else name
    script_path = scripts_dir / script_name
    assert script_path.exists(), script_path
    result = subprocess.run([str(script_path), "--help"], capture_output=True, text=True, check=False)
    print(result.stdout)
    assert result.returncode == 0, (name, result.stdout, result.stderr)
    combined = (result.stdout + result.stderr).lower()
    assert "usage" in combined or name in combined, (name, combined)
PY

"""Console-script runner for the bundled Mnemix Workflow binaries."""

from __future__ import annotations

from importlib import resources
import os
from pathlib import Path
import shutil
import subprocess
import sys

_CANONICAL_BINARY = "mnemix-workflow"
_ALIAS_BINARY = "mxw"
_TUI_BINARY = "mnx"


class WorkflowBinaryNotFoundError(RuntimeError):
    """Raised when the requested workflow CLI binary cannot be found."""


def _platform_binary_name(binary_name: str) -> str:
    if sys.platform == "win32":
        return f"{binary_name}.exe"
    return binary_name


def _env_var_for_binary(binary_name: str) -> str:
    return f"{binary_name.upper().replace('-', '_')}_BINARY"


def _find_bundled_binary(binary_name: str) -> str | None:
    candidate = resources.files("mnemix_workflow").joinpath(
        "_bin", _platform_binary_name(binary_name)
    )
    if candidate.is_file():
        return os.fspath(candidate)
    return None


def _find_binary(binary_name: str) -> str:
    from_env = os.environ.get(_env_var_for_binary(binary_name))
    if from_env:
        return from_env

    bundled = _find_bundled_binary(binary_name)
    if bundled:
        return bundled

    found = shutil.which(_platform_binary_name(binary_name))
    if found:
        return found

    raise WorkflowBinaryNotFoundError(
        f"Could not find the '{_platform_binary_name(binary_name)}' binary. "
        f"Install a mnemix-workflow wheel that bundles the CLI, install the "
        f"CLI separately, or set {_env_var_for_binary(binary_name)} to the "
        "absolute path of the binary."
    )


def _run_cli_entrypoint(binary_name: str) -> int:
    try:
        binary = _find_binary(binary_name)
    except WorkflowBinaryNotFoundError as exc:
        print(str(exc), file=sys.stderr)
        return 1

    completed = subprocess.run([binary, *sys.argv[1:]], check=False)
    return completed.returncode


def bundled_binaries() -> list[Path]:
    """Return bundled binary paths that exist in the installed package."""
    result = []
    for binary_name in (_CANONICAL_BINARY, _ALIAS_BINARY, _TUI_BINARY):
        bundled = _find_bundled_binary(binary_name)
        if bundled:
            result.append(Path(bundled))
    return result


def main() -> int:
    """Entry point for the packaged ``mnemix-workflow`` console script."""
    return _run_cli_entrypoint(_CANONICAL_BINARY)


def main_alias() -> int:
    """Entry point for the packaged ``mxw`` console script."""
    return _run_cli_entrypoint(_ALIAS_BINARY)


def main_tui() -> int:
    """Entry point for the packaged ``mnx`` console script."""
    return _run_cli_entrypoint(_TUI_BINARY)

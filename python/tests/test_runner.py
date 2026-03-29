from __future__ import annotations

import subprocess
from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest

from mnemix_workflow._runner import (
    WorkflowBinaryNotFoundError,
    _find_binary,
    bundled_binaries,
    main,
    main_alias,
    main_tui,
)


def _make_result(returncode: int = 0) -> MagicMock:
    mock = MagicMock(spec=subprocess.CompletedProcess)
    mock.returncode = returncode
    return mock


def test_env_var_used_for_canonical_binary(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv("MNEMIX_WORKFLOW_BINARY", "/custom/mnemix-workflow")
    assert _find_binary("mnemix-workflow") == "/custom/mnemix-workflow"


def test_env_var_used_for_tui_binary(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv("MNX_BINARY", "/custom/mnx")
    assert _find_binary("mnx") == "/custom/mnx"


def test_raises_when_binary_is_missing(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.delenv("MNEMIX_WORKFLOW_BINARY", raising=False)
    with patch("mnemix_workflow._runner._find_bundled_binary", return_value=None), patch(
        "shutil.which", return_value=None
    ):
        with pytest.raises(WorkflowBinaryNotFoundError):
            _find_binary("mnemix-workflow")


def test_bundled_binary_is_used_when_present(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.delenv("MNEMIX_WORKFLOW_BINARY", raising=False)
    with patch("mnemix_workflow._runner._find_bundled_binary", return_value="/wheel/mxw"), patch(
        "shutil.which", return_value=None
    ):
        assert _find_binary("mxw") == "/wheel/mxw"


def test_main_entrypoint_runs_canonical_binary() -> None:
    with patch("mnemix_workflow._runner._find_binary", return_value="/wheel/mnemix-workflow") as mock_find, patch(
        "subprocess.run", return_value=_make_result()
    ) as mock_run, patch("sys.argv", ["mnemix-workflow", "--help"]):
        exit_code = main()

    assert exit_code == 0
    mock_find.assert_called_once_with("mnemix-workflow")
    mock_run.assert_called_once_with(["/wheel/mnemix-workflow", "--help"], check=False)


def test_main_alias_runs_mxw_binary() -> None:
    with patch("mnemix_workflow._runner._find_binary", return_value="/wheel/mxw") as mock_find, patch(
        "subprocess.run", return_value=_make_result()
    ) as mock_run, patch("sys.argv", ["mxw", "--help"]):
        exit_code = main_alias()

    assert exit_code == 0
    mock_find.assert_called_once_with("mxw")
    mock_run.assert_called_once_with(["/wheel/mxw", "--help"], check=False)


def test_main_tui_runs_mnx_binary() -> None:
    with patch("mnemix_workflow._runner._find_binary", return_value="/wheel/mnx") as mock_find, patch(
        "subprocess.run", return_value=_make_result()
    ) as mock_run, patch("sys.argv", ["mnx", "--help"]):
        exit_code = main_tui()

    assert exit_code == 0
    mock_find.assert_called_once_with("mnx")
    mock_run.assert_called_once_with(["/wheel/mnx", "--help"], check=False)


def test_bundled_binaries_returns_existing_paths() -> None:
    with patch(
        "mnemix_workflow._runner._find_bundled_binary",
        side_effect=["/wheel/mnemix-workflow", "/wheel/mxw", None],
    ):
        result = bundled_binaries()

    assert result == [Path("/wheel/mnemix-workflow"), Path("/wheel/mxw")]

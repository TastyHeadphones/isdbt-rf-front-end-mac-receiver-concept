from pathlib import Path
import tomllib


def test_default_config_exists() -> None:
    cfg_path = Path("software/shared/config/default.toml")
    assert cfg_path.exists()

    cfg = tomllib.loads(cfg_path.read_text())
    assert cfg["mode"] == "lab_demo"
    assert cfg["http"]["port"] == 8088


def test_required_docs_present() -> None:
    required = [
        Path("docs/system_architecture.md"),
        Path("docs/block_diagram.md"),
        Path("docs/compliance_and_legal.md"),
        Path("docs/validation_plan.md"),
        Path("docs/lab_demo.md"),
    ]
    for doc in required:
        assert doc.exists(), f"missing {doc}"

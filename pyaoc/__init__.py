from pathlib import Path


def get_input(filename: str) -> Path:
    return Path("__file__").parent / "input" / filename


def get_input_string(filename: str) -> str:
    file_path = get_input(filename)
    with open(file_path) as fin:
        return fin.read()


def test_one():
    assert True

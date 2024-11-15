from typer import Typer, Option

from lint_ebi import rust

app = Typer()


@app.command()
def main(
        directories: list[str],
        parallel: bool = Option(None, "--parallel", "-p"),
        config: str = Option("pyproject.toml", "--config", "-c"),
):
    for directory in directories:
        rust.lint_ebi(directory, parallel=bool(parallel), config=config)

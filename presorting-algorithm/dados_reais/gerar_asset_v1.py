"""Gera o asset real v1 (congelado) para ArrayType::Real.

Procedimento (documentado e repetível):
  1. Lista arquivos trackeados do git (`git ls-files -z`) na raiz do repo.
  2. Filtra por extensões de texto-fonte/prosa, excluindo saídas de medição.
  3. Concatena os bytes crus na ordem do `git ls-files` -> real_bytes_v1.bin.

O .bin é carregado em tempo de compilação via `include_bytes!`, de modo que
o conteúdo é idêntico para qualquer clone do commit registrado abaixo.
NUNCA regenerar silenciosamente: nova fonte => novo asset versionado (v2...).
"""
import hashlib
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
OUT_DIR = Path(__file__).resolve().parent
EXTS = {".tex", ".md", ".rs", ".toml", ".bib", ".cls", ".bst", ".py"}
EXCLUIR_PREFIXOS = (
    "artigo/resultados/",  # saídas de medição (circularidade)
    "artigo/bench2026/",  # derivado do artigo (circularidade)
    "presorting-algorithm/dados_reais/",  # o próprio asset
)


def main() -> None:
    commit = (
        subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=REPO,
            capture_output=True,
            text=True,
            check=True,
        )
        .stdout.strip()
    )
    raw = subprocess.run(
        ["git", "ls-files", "-z"],
        cwd=REPO,
        capture_output=True,
        check=True,
    ).stdout.split(b"\0")
    arquivos = [
        p.decode("utf-8")
        for p in raw
        if p
        and Path(p.decode("utf-8")).suffix.lower() in EXTS
        and not p.decode("utf-8").startswith(EXCLUIR_PREFIXOS)
    ]
    blob = bytearray()
    for rel in arquivos:
        blob += (REPO / rel).read_bytes()
    destino = OUT_DIR / "real_bytes_v1.bin"
    destino.write_bytes(bytes(blob))
    (OUT_DIR / "real_bytes_v1.files.txt").write_text(
        f"commit: {commit}\n" + "\n".join(arquivos) + "\n", encoding="utf-8"
    )
    sha = hashlib.sha256(bytes(blob)).hexdigest()
    print(f"arquivos: {len(arquivos)}")
    print(f"bytes (N nativo): {len(blob)}")
    print(f"sha256: {sha}")
    print(f"commit: {commit}")


if __name__ == "__main__":
    sys.exit(main())

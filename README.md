stt_cli
=======

Outil CLI pour la transcription de fichiers `.m4a` (AAC) via `whisper-rs`.

Usage rapide
------------

Build release :

```bash
cargo build --release
```

Exemple d'exécution :

```bash
cargo run --release -- --model ./ggml-base.bin --m4a ./audio.m4a --lang fr
```

Créer un livrable (archive tar.gz) :

```bash
bash scripts/package_release.sh
```

Voir `docs/USAGE.md` pour plus de détails.

Fichiers inclus dans le bundle : binaire `stt_cli`, `docs/USAGE.md`, `README.md`, `Cargo.toml`.

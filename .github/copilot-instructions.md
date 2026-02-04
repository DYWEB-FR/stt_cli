# Instructions pour les agents AI (Copilot)

But court
- Aider un développeur à être productif rapidement dans ce dépôt Rust `stt_cli` (un CLI de STT basé sur whisper-rs).

Architecture & flux de données
- Entrée : un fichier audio AAC/M4A fourni via `--m4a`.
- Décodage : `src/main.rs::decode_m4a_to_f32_mono` utilise `symphonia` pour produire des samples mono `f32`.
- Resampling : `src/main.rs::resample_to_16k_mono` utilise `rubato::FftFixedIn` pour convertir en 16 kHz (format attendu par `whisper-rs`).
- Transcription : `whisper-rs` est initialisé dans `main()` ; la transcription est effectuée par `state.full(...)` puis les segments sont lus avec `state.full_get_segment_text(i)`.

Commandes de build & exécution
- Build : `cargo build --release` ou `cargo build` pour debug.
- Run (exemple reproductible) :

```
cargo run --release -- --model ./ggml-base.bin --m4a ./audio.m4a --lang fr
```

Paramètres CLI importants
- `--model`: chemin vers le fichier ggml (ex: `ggml-base.bin`) — fourni par l'utilisateur, non embarqué.
- `--m4a`: fichier audio d'entrée (AAC/M4A).
- `--lang`: langue (par défaut `fr`; chaîne vide => auto-détection).
- `--threads`: contrôle le nombre de threads `whisper` (0 => valeur par défaut de la lib).

Conventions et patterns du projet
- Erreurs : utilisation systématique de `anyhow` + `.context(...)` pour enrichir les erreurs (voir `main.rs`).
- CLI : `clap` v4 avec `derive` — modifications d'arguments se font dans la struct `Args` au début de `src/main.rs`.
- Découpage audio : le code fait une copie des métadonnées nécessaires (id, sr, channels) avant d'instancier le décodeur pour éviter les emprunts prolongés sur `format`.
- Downmix stéréo -> mono : moyenne simple des canaux (implémentée dans `decode_m4a_to_f32_mono`).

Points d'attention pour les contributions AI
- Tests et samples : il n'y a pas de suite de tests; pour valider les changements, reproduisez localement avec un petit fichier `audio.m4a` et un modèle ggml.
- Modèle externe : le binaire `ggml-*.bin` n'est pas géré par Cargo — toute tâche d'intégration doit documenter où obtenir le modèle.
- Paramètres de performance : `WhisperContextParameters` peut être ajusté (ex. `use_gpu`) — la ligne est présente mais commentée dans `main.rs`.
- Resampler : les valeurs `chunk_size` et `sub_chunks` sont des points d'équilibrage qualité/latence (modifiez avec prudence).

Fichiers-clés à consulter
- `Cargo.toml` : dépendances essentielles (`whisper-rs`, `symphonia`, `rubato`, `clap`, `anyhow`).
- `src/main.rs` : logique entière du pipeline audio -> transcription.

Exemples de modifications courantes demandées aux agents
- Ajouter une option pour écrire la sortie en JSON : modifier `main()` pour construire une structure et sérialiser avec `serde_json`.
- Exposer des options de resampling et d'optimisation (buffer sizes, GPU flag) via la CLI (`Args`).
- Ajouter logging (ex: `log` + `env_logger`) : insérer initialisation dans `main()` et remplacer `println!` par `info!`/`debug!`.

Quand demander au développeur
- Fournir un modèle ggml de test ou indiquer où le récupérer si l'agent a besoin de tester.
- Donner des exemples d'audio d'entrée représentatifs (durée, canaux, bitrate) si vous ajustez le resampler ou le pipeline.

Merci — demandez un retour si une section nécessite plus de détails ou des exemples supplémentaires.

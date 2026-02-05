# Installation et déploiement sur Linux (Ubuntu / Linux Mint)

> Guide rapide pour déployer l'exécutable `stt_cli` et ses ressources sur une machine Linux (Ubuntu / Linux Mint).

## 1 — Prérequis système

Sur une distribution basée sur Ubuntu / Mint, installez les paquets nécessaires à la compilation et au runtime :

```bash
sudo apt update
sudo apt install -y cmake build-essential pkg-config clang libclang-dev
```

Remarque : `libclang-dev` est parfois requis pour `whisper-rs` (bindgen). Si la compilation échoue, vérifiez `LIBCLANG_PATH`.

## 2 — Compilation (sur la machine de build)

Depuis la racine du dépôt :

```bash
cd /path/to/stt_cli
cargo build --release
```

Le binaire optimisé sera créé dans `target/release/stt_cli`.

## 3 — Installer le binaire et les ressources

Conseil d'arborescence :

- Binaire : `/usr/local/bin/stt_cli`
- Modèles et ressources : `/opt/stt_cli`

Commandes d'exemple :

```bash
sudo install -m 0755 target/release/stt_cli /usr/local/bin/stt_cli
sudo mkdir -p /opt/stt_cli/models /opt/stt_cli/audio
sudo cp models/ggml-base.bin /opt/stt_cli/models/    # adapter le nom du modèle
sudo chown -R $USER:$USER /opt/stt_cli
```

Le dossier `/opt/stt_cli` contient les gros fichiers modèles (ggml) et vos audios.

## 4 — Exécution basique

Exemple d'utilisation après installation :

```bash
stt_cli --model /opt/stt_cli/models/ggml-base.bin --m4a /opt/stt_cli/audio/sample.m4a --lang fr
```

Si `stt_cli` n'est pas trouvé, vérifiez que `/usr/local/bin` est dans votre `PATH`.

## 5 — Service systemd (optionnel)

Pour lancer une transcription via systemd (oneshot) : créez `/etc/systemd/system/stt_cli.service` :

```ini
[Unit]
Description=stt_cli transcription service
After=network.target

[Service]
Type=oneshot
ExecStart=/usr/local/bin/stt_cli --model /opt/stt_cli/models/ggml-base.bin --m4a /opt/stt_cli/audio/input.m4a --lang fr
User=youruser
WorkingDirectory=/opt/stt_cli
TimeoutSec=600

[Install]
WantedBy=multi-user.target
```

Puis :

```bash
sudo systemctl daemon-reload
sudo systemctl start stt_cli.service
# sudo systemctl enable stt_cli.service  # si souhaité (faire attention au type oneshot)
```

Pour un service en écoute continue (daemon), il faut envelopper `stt_cli` dans un script qui surveille un répertoire d'entrée ou exposer une API — je peux fournir un exemple si besoin.

## 6 — Packaging (facultatif)

Créer un paquet Debian avec `cargo-deb` :

```bash
cargo install cargo-deb
cargo deb --no-build
```

Le paquet `.deb` résultant peut être installé sur d'autres machines avec `dpkg -i`.

## 7 — Dépannage

- Erreur bindgen / libclang : assurez-vous que `libclang-dev` est installé et, si besoin, exportez `LIBCLANG_PATH` vers la bibliothèque `libclang.so`.
- Modèle manquant : `stt_cli` n'inclut pas le modèle ggml ; téléchargez un modèle compatible (ex. `ggml-base.bin`) et placez-le dans `/opt/stt_cli/models`.
- Problème d'audio : vérifiez que le fichier `.m4a` est encodé en AAC ou supporté par Symphonia (utilisez `ffprobe` pour inspecter).

## 8 — Notes pratiques

- Préférez `/usr/local/bin` pour un binaire global et `/opt/stt_cli` pour les ressources.
- Pour automatisation: un script shell qui copie les modèles et installe le binaire facilite le déploiement.

---

Fichier créé pour déploiement rapide. Si vous voulez, je peux :

- fournir un script d'installation (`install.sh`) automatisé,
- créer une unité `systemd` plus avancée pour surveiller un répertoire,
- générer un paquet `.deb` configuré avec dépendances.

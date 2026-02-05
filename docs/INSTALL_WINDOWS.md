# Installation et déploiement sur Windows

Guide rapide pour compiler et déployer `stt_cli` sur une machine Windows (natif, non WSL).

## 1 — Rappel

`stt_cli` est un binaire Rust qui dépend de crates natives (`whisper-rs`, `symphonia`, etc.). Sur Windows la compilation peut nécessiter des outils natifs (Visual C++ Build Tools, LLVM/Clang) pour que les bindings et le linking fonctionnent.

## 2 — Pré-requis

- Installer Rust (rustup) : https://www.rust-lang.org/tools/install
- Choisir la toolchain MSVC recommandée : `rustup default stable-x86_64-pc-windows-msvc`
- Installer Visual C++ Build Tools (ou Visual Studio avec workload "Desktop development with C++").
- Installer LLVM/Clang (nécessaire si bindgen se déclenche) et s'assurer que `libclang.dll` est accessible.
- Optionnel : `choco` (Chocolatey) pour installations automatisées.

Exemples via Chocolatey (exécution PowerShell en administrateur) :

```powershell
choco install -y visualstudio2019buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools"
choco install -y llvm
```

Après installation de LLVM, pointez `LIBCLANG_PATH` si nécessaire :

```powershell
$env:LIBCLANG_PATH = 'C:\Program Files\LLVM\bin\libclang.dll'
# Pour rendre permanent, ajoutez la variable à vos variables d'environnement système
```

## 3 — Compilation

Depuis la racine du dépôt (PowerShell) :

```powershell
# S'assurer d'utiliser la toolchain MSVC
rustup show
cargo build --release
```

Le binaire optimisé sera dans `target\release\stt_cli.exe`.

## 4 — Emplacements recommandés et modèles

- Binaire : `C:\Program Files\stt_cli\stt_cli.exe` (ou `C:\stt_cli\stt_cli.exe` si vous préférez éviter le dossier Program Files).
- Modèles : `C:\Program Files\stt_cli\models` ou `C:\stt_cli\models`.

Copier le binaire et les modèles (exemple PowerShell) :

```powershell
New-Item -ItemType Directory -Force -Path 'C:\Program Files\stt_cli\models'
Copy-Item -Path target\release\stt_cli.exe -Destination 'C:\Program Files\stt_cli\stt_cli.exe'
Copy-Item -Path models\ggml-base.bin -Destination 'C:\Program Files\stt_cli\models\'
```

## 5 — Exécution

Exemple d'utilisation depuis PowerShell :

```powershell
"C:\Program Files\stt_cli\stt_cli.exe" --model "C:\Program Files\stt_cli\models\ggml-base.bin" --m4a "C:\path\to\sample.m4a" --lang fr
```

Si vous voulez pouvoir lancer `stt_cli` depuis n'importe quel endroit, ajoutez `C:\Program Files\stt_cli` au `PATH` système.

## 6 — Service Windows / automatisation

Options communes pour exécuter automatiquement :

- Utiliser NSSM (Non-Sucking Service Manager) pour wrapper l'exécutable en service Windows.
- Créer une tâche planifiée (Scheduled Task) qui exécute un script PowerShell pour traiter un répertoire d'entrée.

Exemple rapide avec NSSM (après avoir téléchargé NSSM) :

```powershell
# nssm.exe install stt_cli "C:\Program Files\stt_cli\stt_cli.exe" --model "C:\Program Files\stt_cli\models\ggml-base.bin" --m4a "C:\stt_cli\audio\input.m4a" --lang fr
```

Ou créer une tâche planifiée : utilisez `schtasks.exe` ou l'interface GUI pour déclencher un script PowerShell.

## 7 — Dépannage

- Erreur liée à `bindgen` / `libclang` : vérifier que `libclang.dll` est installé et que l'architecture (x64) correspond à la toolchain Rust. Contrôlez `LIBCLANG_PATH`.
- Erreurs de linking (LNK) : assurez-vous que Visual C++ Build Tools sont installés et que l'environnement de build est correctement configuré (redémarrer la session peut aider).
- Crash au runtime lié à `whisper-rs` : vérifier la version de `whisper-rs` et les issues upstream ; tester avec un modèle plus petit.
- Si l'audio `.m4a` n'est pas décodable, testez avec `ffmpeg`/`ffprobe` pour confirmer le codec (le projet utilise `symphonia` et supporte AAC via feature `aac`).

## 8 — Alternatives

- WSL2 : si vous préférez l'environnement Linux existant (bash/systemd), installer WSL2 et suivre les instructions Linux décrites dans [docs/INSTALL_LINUX.md](INSTALL_LINUX.md).
- Cross-build : compiler sur une machine Linux en target `x86_64-pc-windows-msvc` est possible mais plus complexe (toolchain cross-linking). Pour la majorité des utilisateurs, compiler localement sur Windows est plus simple.

## 9 — Veux‑tu que je fasse ça pour toi ?

Je peux :
- fournir un script `install_windows.ps1` pour automatiser l'installation des dépendances, la compilation et la copie des modèles ;
- fournir un petit script PowerShell de démonstration pour surveiller un répertoire et lancer `stt_cli` sur chaque nouveau fichier ;
- préparer un exemple de service utilisant NSSM ou les instructions pour une tâche planifiée.

Indique ce que tu veux et je le génère.

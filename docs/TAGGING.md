Générateur de tag — `create_and_push_tag.sh`
=========================================

But
---
Script utilitaire pour créer un tag Git au format semver préfixé par `v` (ex: `v1.2.3`). Peut incrémenter le dernier tag existant et optionnellement le pousser vers le remote `origin`.

Usage
-----

- Créer automatiquement un nouveau tag (incrémente le patch):

```bash
bash scripts/create_and_push_tag.sh
```

- Créer un tag précis (le préfixe `v` est ajouté si absent) :

```bash
bash scripts/create_and_push_tag.sh 1.2.0
bash scripts/create_and_push_tag.sh v1.2.0
```

- Pousser le tag vers le remote `origin` :

```bash
bash scripts/create_and_push_tag.sh --push
bash scripts/create_and_push_tag.sh v1.2.0 --push
```

Comportement
-----------
- Si aucun tag `v*` n'existe, le script crée `v0.1.0`.
- Si un tag `vMAJ.MIN.PATCH` existe, le script incrémente `PATCH`.
- Le tag est créé localement avec un message annoté `Release <tag>`.
- `--push` pousse le tag vers le remote `origin`. Le script échoue si `origin` n'existe pas.

Erreurs courantes
-----------------
- "Latest tag ... doesn't look like semver": le tag existant n'est pas au format `vMAJ.MIN.PATCH`.
- Pas de remote `origin`: utilisez `git remote add origin <url>` ou poussez manuellement.

Conseils CI / Workflow
---------------------
- Utiliser ce script localement pour créer et pousser des tags qui déclencheront le workflow GitHub Actions.
- Dans CI, préférez créer le tag via l'API GitHub ou via un job qui possède un token avec permissions `contents: write`.

Sécurité
--------
- Le script exécute des commandes `git` locales : revérifiez le tag avant de pousser en production.

Fichier
------
- Script: `scripts/create_and_push_tag.sh`

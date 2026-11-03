# Rust → moteur d'inférence

Apprentissage de Rust, **15 à 30 minutes par jour**, avec des exercices vérifiés
automatiquement (tests + CI) et relus par un agent IA.

**Objectif final** : écrire un petit moteur d'inférence de LLM (type `llama.cpp`) en Rust.

## Démarrage

1. Installer Rust : <https://rustup.rs>
2. Cloner ce dépôt, puis vérifier que tout est en place : `cargo --version`
3. Ouvrir le dossier de l'exercice du jour (`exercises/dXX-...`) et lire son `README.md`.

> ⚠️ Ne lance **jamais** `cargo test` à la racine : les exercices pas encore faits
> échouent volontairement (`todo!()`). Toujours cibler : `cargo test -p dXX-nom`.

## Routine quotidienne

| Format | Contenu |
|---|---|
| **15 min** | mini-leçon (3-5 min, dans le README de l'exercice ou expliquée par l'agent) → exercice jusqu'au vert (~8 min) → validation + question de l'agent (2 min) |
| **30 min** | idem + **bonus** (tests `#[ignore]`) + une question de compréhension plus poussée |

Si tu bloques plus de 5 minutes, demande un **indice** à l'agent (pas la solution).

## Workflow Git

1. Une **issue** par exercice (créées par `scripts/setup-github.sh`).
2. Une **branche** par exercice : `git switch -c j01-variables-types`.
3. Une **Pull Request** qui référence l'issue (`Closes #N`).
4. La **CI** lance les tests / clippy sur les exercices modifiés. Ajoute le label `bonus`
   à la PR pour que la CI exécute aussi les tests bonus.
5. L'agent relit la PR et te pose la question de compréhension.
6. Merge → l'issue se ferme → la carte passe en « Validé » dans le Project.

Tiens aussi `JOURNAL.md` à jour (2 lignes par session) : ça sert à l'agent pour te
reproposer des notions à revoir.

## Plan global

| Phase | Contenu | Durée estimée |
|---|---|---|
| 1 — Fondamentaux | ownership, enums, traits, erreurs, itérateurs, closures (**ce dépôt contient J01 à J14**) | ~3 semaines |
| 2 — Rust courant | lifetimes, modules, tests, gestion d'erreurs propre, tokenizer simple | ~4 semaines |
| 3 — Briques d'inférence | Tensor, matmul optimisé, `memmap2`, parser GGUF, quantization, `rayon`/SIMD, BPE | ~8 semaines |
| 4 — Mini moteur | forward pass transformer, KV cache, sampling, benchmarks | ~6-8 semaines |

## Exercices de la phase 1

| Jour | Dossier | Sujet |
|---|---|---|
| J01 | `d01-variables-types` | Variables, types et fonctions |
| J02 | `d02-ownership` | Ownership : qui possède quoi ? |
| J03 | `d03-borrowing-slices` | Emprunts et tranches (slices) |
| J04 | `d04-structs` | Structs et méthodes |
| J05 | `d05-enums-match` | Enums et pattern matching |
| J06 | `d06-option` | Option et combinateurs |
| J07 | `d07-result-errors` | Result et gestion d'erreurs |
| J08 | `d08-vec-hashmap` | Vec et HashMap |
| J09 | `d09-strings` | Chaînes de caractères et UTF-8 |
| J10 | `d10-traits` | Traits et polymorphisme |
| J11 | `d11-generics` | Génériques : une pile `Stack<T>` |
| J12 | `d12-iterators` | Itérateurs |
| J13 | `d13-closures` | Closures |
| J14 | `d14-mini-matrix` | Mini projet : une matrice `f32` |

Les phases 2 à 4 seront ajoutées au fur et à mesure.

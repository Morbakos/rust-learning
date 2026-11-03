#!/usr/bin/env bash
# Crée sur GitHub : labels, milestones (phases), issues J01..J14 et, en option, un Project.
#
# Prérequis : gh installé et authentifié (`gh auth login`), à lancer depuis le dépôt cloné.
# Pour le Project : `gh auth refresh -s project` puis `CREATE_PROJECT=1 ./scripts/setup-github.sh`
set -euo pipefail

M1="Phase 1 — Fondamentaux"
M2="Phase 2 — Rust courant"
M3="Phase 3 — Briques d'inférence"
M4="Phase 4 — Mini moteur d'inférence"

echo "→ Labels"
gh label create base --color 0E8A16 --description "Exercice de base" --force
gh label create bonus --color FBCA04 --description "Exercice bonus" --force
gh label create phase-1 --color 1D76DB --description "Phase 1" --force
gh label create phase-2 --color 1D76DB --description "Phase 2" --force
gh label create phase-3 --color 1D76DB --description "Phase 3" --force
gh label create phase-4 --color 1D76DB --description "Phase 4" --force
gh label create "borrowing" --color C5DEF5 --description "Thème : borrowing" --force
gh label create "closures" --color C5DEF5 --description "Thème : closures" --force
gh label create "collections" --color C5DEF5 --description "Thème : collections" --force
gh label create "enums" --color C5DEF5 --description "Thème : enums" --force
gh label create "errors" --color C5DEF5 --description "Thème : errors" --force
gh label create "generics" --color C5DEF5 --description "Thème : generics" --force
gh label create "iterators" --color C5DEF5 --description "Thème : iterators" --force
gh label create "matrices" --color C5DEF5 --description "Thème : matrices" --force
gh label create "option" --color C5DEF5 --description "Thème : option" --force
gh label create "ownership" --color C5DEF5 --description "Thème : ownership" --force
gh label create "strings" --color C5DEF5 --description "Thème : strings" --force
gh label create "structs" --color C5DEF5 --description "Thème : structs" --force
gh label create "traits" --color C5DEF5 --description "Thème : traits" --force
gh label create "types" --color C5DEF5 --description "Thème : types" --force

echo "→ Milestones (phases)"
create_milestone() {
  gh api "repos/{owner}/{repo}/milestones" -f title="$1" -f description="$2" --silent \
    || echo "  (existe déjà ? $1)"
}
create_milestone "$M1" "Ownership, enums, traits, erreurs, itérateurs, closures"
create_milestone "$M2" "Lifetimes, modules, tests, erreurs propres, tokenizer simple"
create_milestone "$M3" "Tensor, matmul, mmap, GGUF, quantization, rayon/SIMD, BPE"
create_milestone "$M4" "Forward pass transformer, KV cache, sampling, benchmarks"

echo "→ Issues (phase 1)"
ISSUES=(
  "d01-variables-types|J01 — Variables, types et fonctions|types"
  "d02-ownership|J02 — Ownership : qui possède quoi ?|ownership"
  "d03-borrowing-slices|J03 — Emprunts et tranches (slices)|borrowing"
  "d04-structs|J04 — Structs et méthodes|structs"
  "d05-enums-match|J05 — Enums et pattern matching|enums"
  "d06-option|J06 — Option et combinateurs|option"
  "d07-result-errors|J07 — Result et gestion d'erreurs|errors"
  "d08-vec-hashmap|J08 — Vec et HashMap|collections"
  "d09-strings|J09 — Chaînes de caractères et UTF-8|strings"
  "d10-traits|J10 — Traits et polymorphisme|traits"
  "d11-generics|J11 — Génériques : une pile Stack<T>|generics"
  "d12-iterators|J12 — Itérateurs|iterators"
  "d13-closures|J13 — Closures|closures"
  "d14-mini-matrix|J14 — Mini projet : une matrice f32|matrices"
)
for entry in "${ISSUES[@]}"; do
  IFS='|' read -r dir title theme <<< "$entry"
  gh issue create \
    --title "$title" \
    --label "base,phase-1,$theme" \
    --milestone "$M1" \
    --body "Dossier : \`exercises/$dir\`

Durée : 15 min (base) · +15 min (bonus)
Consigne : voir le README du dossier.

Validation : tests au vert, clippy sans warning, question de compréhension répondue."
done

if [[ "${CREATE_PROJECT:-0}" == "1" ]]; then
  echo "→ Project"
  PROJECT_NUMBER=$(gh project create --owner "@me" --title "Rust → moteur d'inférence" --format json --jq '.number')
  gh project field-create "$PROJECT_NUMBER" --owner "@me" --name "Durée (min)" --data-type NUMBER
  gh project field-create "$PROJECT_NUMBER" --owner "@me" --name "Difficulté" \
    --data-type SINGLE_SELECT --single-select-options "Facile,Moyen,Dur"
  gh issue list --limit 100 --json url --jq '.[].url' | while read -r url; do
    gh project item-add "$PROJECT_NUMBER" --owner "@me" --url "$url"
  done
  echo "Project n°$PROJECT_NUMBER créé. Lie-le au dépôt depuis l'interface GitHub (Projects → Link a repository)."
fi

echo "✔ Terminé."

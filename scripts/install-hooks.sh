set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
HOOK="$ROOT/.git/hooks/pre-push"

mkdir -p "$ROOT/.git/hooks"

cat > "$HOOK" <<'EOF'
#!/usr/bin/env bash
exec "$(git rev-parse --show-toplevel)/scripts/preflight.sh"
EOF

chmod +x "$HOOK"
chmod +x "$ROOT/scripts/preflight.sh"

echo "✓ pre-push hook installed at .git/hooks/pre-push"
echo "  Every \`git push\` will now run: fmt --check, clippy, test"
echo "  Skip once with:  git push --no-verify"

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

bold=$(tput bold 2>/dev/null || true)
reset=$(tput sgr0 2>/dev/null || true)
green=$(tput setaf 2 2>/dev/null || true)
red=$(tput setaf 1 2>/dev/null || true)

step() {
    printf '\n%s→ %s%s\n' "$bold" "$1" "$reset"
}

step "cargo fmt --check"
if ! cargo fmt --all -- --check; then
    printf '\n%s✗ formatting issues. Run `cargo fmt --all` to auto-fix.%s\n' "$red" "$reset" >&2
    exit 1
fi

step "cargo clippy"
cargo clippy --all-targets --all-features --locked -- -D warnings

step "cargo test"
cargo test --all --locked

printf '\n%s%s✓ preflight passed — safe to push%s\n' "$bold" "$green" "$reset"

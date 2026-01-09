#!/bin/bash

# Pre-commit hook para verificar formato y linting
# Para instalarlo: cp scripts/pre-commit.sh .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit

set -e

echo "🔍 Ejecutando verificaciones pre-commit..."

echo ""
echo "📝 Verificando formato con cargo fmt..."
if ! cargo fmt --all -- --check; then
    echo "❌ Error: El código no está formateado correctamente."
    echo "💡 Ejecuta 'cargo fmt --all' para corregir el formato."
    exit 1
fi
echo "✅ Formato correcto"

echo ""
echo "🔎 Ejecutando clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Error: Clippy encontró warnings o errores."
    echo "💡 Corrige los warnings antes de hacer commit."
    exit 1
fi
echo "✅ Clippy pasó sin warnings"

echo ""
echo "🧪 Ejecutando tests..."
if ! cargo test --all-features; then
    echo "❌ Error: Algunos tests fallaron."
    echo "💡 Corrige los tests antes de hacer commit."
    exit 1
fi
echo "✅ Todos los tests pasaron"

echo ""
echo "✨ ¡Todas las verificaciones pasaron! Procediendo con el commit..."
exit 0

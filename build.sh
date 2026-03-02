#!/bin/bash

# Nome do seu projeto (conforme definido no Cargo.toml)
APP_NAME="gnome_xfce"

echo "🔨 Compilando $APP_NAME..."
RUSTFLAGS="-C target-cpu=native"  cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Compilação concluída!"
    # Copia o binário para a raiz com um nome amigável
    cp target/release/$APP_NAME ./$APP_NAME
    echo "🚀 Para rodar, use: ./$APP_NAME"
else
    echo "❌ Erro na compilação."
    exit 1
fi
#!/bin/bash
set -e

# 安装 wasm-pack 如果尚未安装
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# 使用 wasm-pack 构建项目
echo "Building WebAssembly module..."
wasm-pack build --target web --out-dir pkg

# 复制 HTML 文件到 pkg 目录
echo "Copying HTML file to pkg directory..."
cp index.html pkg/

echo "Build completed successfully!"
echo "To test locally, you can run: cd pkg && python3 -m http.server" 
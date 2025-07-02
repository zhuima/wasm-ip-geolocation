# IP 地址查询工具

这是一个使用 Rust 和 WebAssembly 构建的简单 IP 地址查询工具。它可以显示用户的 IP 地址，并且可以部署在任何静态网站托管服务上，如 Vercel、Netlify、GitHub Pages 等。

## 功能

- 显示用户的 IP 地址
- 纯客户端实现，使用 WebAssembly
- 响应式设计，适配移动设备

## 本地开发

### 前提条件

- Rust 和 Cargo（https://www.rust-lang.org/tools/install）
- wasm-pack（会在构建脚本中自动安装）

### 构建步骤

1. 克隆仓库：

```bash
git clone <repository-url>
cd ip-geolocation
```

2. 运行构建脚本：

```bash
chmod +x build.sh
./build.sh
```

3. 本地测试：

```bash
cd pkg
python3 -m http.server
```

然后在浏览器中访问 http://localhost:8000

## 部署到 Vercel

1. 安装 Vercel CLI：

```bash
npm install -g vercel
```

2. 构建项目：

```bash
./build.sh
```

3. 部署到 Vercel：

```bash
cd pkg
vercel deploy
```

按照提示进行操作，完成部署。

## 部署到其他平台

由于这是一个纯静态网站项目，你可以将 `pkg` 目录部署到任何支持静态网站托管的平台，如 Netlify、GitHub Pages、Cloudflare Pages 等。

## 许可证

MIT 



```python

{
origin: "127.0.0.1",
headers: {
sec-fetch-dest: "document",
spin-component-route: "",
spin-matched-route: "/...",
sec-ch-ua-mobile: "?0",
spin-raw-component-route: "/...",
accept-encoding: "gzip, deflate, br, zstd",
sec-fetch-mode: "navigate",
sec-fetch-site: "none",
sec-ch-ua-platform: ""macOS"",
spin-base-path: "/",
accept: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
spin-path-info: "/",
accept-language: "en-US,en;q=0.9,cy;q=0.8,da;q=0.7,zh-CN;q=0.6,zh;q=0.5",
user-agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36",
sec-ch-ua: ""Google Chrome";v="135", "Not-A.Brand";v="8", "Chromium";v="135"",
spin-client-addr: "127.0.0.1:51202",
cache-control: "max-age=0",
sec-fetch-user: "?1",
spin-full-url: "http://127.0.0.1:3000/",
upgrade-insecure-requests: "1"
}
}

```

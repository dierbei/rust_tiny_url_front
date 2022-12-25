## 1. 依赖
```shell
# wasm
rustup target add wasm32-unknown-unknown

# trunk
cargo install trunk
```

## 2. trunk快速开始
```shell
# 创建页面
touch index.html

# 页面内容
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Tiny URL</title>
</head>
<body>

</body>
</html>

# 添加配置
touch Trunk.toml

# 配置内容
[build]
target = "index.html"
dist = "dist"

# 启动服务
trunk serve

# 访问
localhost:8080
```

## 3. tailwind css
```shell
# 官网
https://tailwindcss.com/docs/installation

# 安装
npm install -D tailwindcss
npm install -D @tailwindcss/forms
npx tailwindcss init

# 编辑 tailwind.config.js

# 新建 input.css
touch input.css

# css 内容
@tailwind base;
@tailwind components;
@tailwind utilities;

# 生成css样式
npx tailwindcss -i input.css -o output.css --watch

# 引入样式
<link href="/dist/output.css" rel="stylesheet">

# 更换 home body
<h1 class="text-3xl font-bold underline">
    Hello world!
</h1>
```
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
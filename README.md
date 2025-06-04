# Rust Crypto Chat

> **终端 × Rust × 端到端加密** — 一款跨平台、零依赖、可在本地或 VPS 一键部署的即时通讯应用。

---

## ✨ 项目概述

`rust_chat` 由 **客户端** 和 **服务器** 两部分组成，均使用 Rust **Tokio 异步运行时** 实现。项目聚焦在「轻量级 + 强安全 + 高可玩性」：内置房间系统、邀请码、TUI 聊天界面、图片预览与可插拔的加密层。

只需要稍微改一下`initialization.rs`可以为好友们提供懒人启动方式。

<div align="center">
  <img src="https://github.com/Vrepol/Rust_Crypto_Chat/blob/main/demo.gif" width="600" alt="Demo GIF"/>
</div>

---

## 📂 目录结构

```text
rust_chat/
├── Cargo.toml
├── README.md
├── src/
│   ├── client/            # 客户端逻辑
│   │   ├── crypto.rs      # 加解密部分
│   │   ├── handshake.rs   # 认证 + 密钥生成
│   │   ├── keyboard.rs    # 按键交互部分
│   │   ├── network.rs     # 读写 + 心跳
│   │   ├── receiver.rs    # 消息通道 → UI
│   │   ├── utils.rs       # 工具函数部分
│   │   ├── crypto.rs      # 加密算法部分
│   │   ├── network.rs     # 客户端通信收发部分
│   │   ├── clipboard.rs   # 剪切板部分
│   │   └── initialization.rs  # 初始化部分
│   └── bin/         
│       ├── client.rs      # 客户端部分
│       └── server.rs      # 服务端部分
│
└── LICENSE
```

---

## 🚀 快速开始

### 1. 安装依赖

| 平台                  | 必备工具                                                                                             | 说明                                       |
| ------------------- | ------------------------------------------------------------------------------------------------ | ---------------------------------------- |
| **Ubuntu / Debian** | `build-essential pkg-config libasound2-dev`                                                      | `alsa-sys` 依赖，如无需提示音可省略 `libasound2-dev` |
| **Arch**            | `base-devel pkgconf`                                                                             |                                          |
| **macOS**           | `brew install pkg-config`                                                                        |                                          |
| **Windows**         | [MSVC Build Tools](https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/) / MinGW64 | 推荐 MSVC 以获得最佳兼容性                         |

> Rust 版本需 **1.77** 及以上。

### 2. 编译

```bash
# 克隆代码
$ git clone https://github.com/yourname/rust_chat.git
$ cd rust_chat

# Release 构建
$ cargo build --release
```

### 3. 运行服务器

```bash
./server --port 6655 --password 'Password'
```

| 参数           | 作用              | 默认       |
| ------------ | --------------- | -------- |
| `--port`     | 监听端口            | `6655`   |
| `--password` | 服务器主密码（同时作为根密钥） | `Vrepol` |


### 4. 运行客户端

```bash
./client
或
client.exe
```

启动流程：
1. **昵称**（留空则为随机法语昵称）
2. **服务器地址 / 邀请码**（可直接粘贴以 `/INVITE:` 开头的一次性链接）
3. **服务器密码**（仅本地使用，不会明文上传）
4. **房间号码**（留空则为大厅，输入/q退回到第2步，输入单引号 ' 为加强的随机房间，32位密码，配合邀请码使用）
5. **房间密码**

---

## 🔑 信息安全

| 阶段   | 说明                                                        | 特性                                    |
| ------- | --------------------------------------------------------- | --------------------------------------- |
| 握手阶段 | 使用chacha20poly1305加密算法，本地将服务器密码哈希值作为对称密钥进行握手。密钥的生命周期为30秒。 ||
| 聊天阶段 | 本地将房间密码的哈希值作为对称密钥，外部再包一层服务器加密形成双重加密。                      | |
| 邀请码  | 当邀请码的生成时间与生成逻辑同时暴露时，会泄露服务器密码的哈希值，房间号和房间密码以及IP地址。邀请码生命周期为500秒。        | 被邀请的成员无法生成正确的邀请码并且退出房间后退回到选择服务器界面，可以理解为被邀请人只有房间使用权没有服务器使用权。|
| 图片缓存 | 会临时创建一个文件夹保存图片，退出房间后自动删除。                                 | 在房间中直接退出应用会导致临时文件无法正确清理。|

综上所述，邀请码的发送渠道/方式 是最薄弱的环节。

> 加密/解密逻辑位于 `src/client/server/crypto.rs`，可自由替换为 TLS、Noise 等其它协议。

---

## 🖥️ TUI 操作快捷键（聊天界面）

| 快捷键            | 功能      | 快捷键            | 功能      |
| -------------- | ------- | -------------- | ------- |
| **Ctrl+H/J**   | 中文/英文提示 | **Crtl+↑ / ↓** | 加速滚动    |
| **Ctrl+I**     | 生成邀请码   | **Ctrl+X**     | 粘贴图片或文字 |
| ← / →          | 移动光标    | **Ctrl+Z**     | 撤销  |
| **Crtl+← / →** | 加速移动    | **Ctrl+C**     | 复制消息文本  |
| ↑ / ↓          | 滚动消息    | **Ctrl+A**     | 清空输入框   |
| Tab            | 打开图片    | ESC            | 退出房间    |

---

## 💻 常见问题 FAQ

<details>
<summary>编译时报错 <code>alsa-sys</code> 找不到库？</summary>
安装 `libasound2-dev`，或在 <code>Cargo.toml</code> 中为 <code>rodio</code> 关闭默认特性：

```toml
rodio = { version = "0.18", default-features = false }
```

</details>

<details>
<summary>PowerShell 显示 Emoji/彩色字符为乱码？</summary>
请使用 **Windows Terminal** 并选择支持 Emoji 的字体（如 *Cascadia Code PL*）。
</details>

<details>
<summary>如何跨编译到 Windows 可执行文件？</summary>
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```
</details>

---

## 🛣️ Roadmap / TODO

* [ ] 断点续传 / 大文件分片
* [ ] 移动端 (Flutter/Fyne) GUI
* [ ] 单次邀请码
* [ ] 可靠的邀请码


## 📄 许可证

本项目基于 **MIT License** - 详见 [LICENSE](LICENSE)。

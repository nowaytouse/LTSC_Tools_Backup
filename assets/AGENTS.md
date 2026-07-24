# Windows LTSC Setup - Start Here (Rust GUI 预编译版)

## 直接双击运行

在本项目根目录下已为您直接预编译好了 **Windows 原生可执行程序 [ltsc_setup_gui.exe](file:///Users/nyamiiko/Downloads/GitHub/LTSC_Tools_Backup/ltsc_setup_gui.exe)**。

1. 在 Windows 系统中，直接找到项目根目录下的 `ltsc_setup_gui.exe`。
2. 右键选择 **以管理员身份运行** (Run as Administrator)。
3. 点击界面中的 **▶️ 一键开始全套配置** 按钮即可！

---

## 包含的部署项

1. **网络与 TLS 硬化**：强制 TLS 1.2/1.3、刷新 DNS、优化 TCP 协议栈。
2. **包管理器自动化**：部署 Microsoft Store、Winget、Scoop、Chocolatey。
3. **LTSC 内置 UWP 应用恢复**：恢复计算器、照片、画图、截图工具、Windows Terminal。
4. **核心桌面软件**：Chrome、VLC、7-Zip、Notepad++、ShareX、IrfanView、VS Code、Cursor、Podman Desktop、Krita、PureRef、Bitwarden、LocalSend。
5. **开发者与 AI 套件**：Git, Python, Node (`fnm`/`bun`/`pnpm`), Rust, `rtk` (Rust Token Killer), `@anthropic-ai/claude-code`, `opencode-ai`, `kimi-cli`, `ruff`, `fastfetch` 等。
6. **系统优化**：开启 NTFS 长路径支持 (`LongPathsEnabled = 1`) 与开发者模式。
7. **实时日志与进度**：可视进度条与彩色分级日志终端。

---

## 重新编译（可选）

如需自行修改源码后重新编译：

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

编译产物位于 `target/x86_64-pc-windows-gnu/release/ltsc_setup_gui.exe`。

Last Updated: 2026-07-24

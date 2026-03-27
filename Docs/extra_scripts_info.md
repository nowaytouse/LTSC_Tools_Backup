# 额外脚本说明

这些脚本来自 `D:\Backup\Downloads` 目录，提供了更多安装选项。

---

## 📦 脚本列表

### 01_bootstrap_ltsc.ps1
**用途**：安装包管理器和基础依赖

**功能**：
- ✅ 启用 TLS 1.2
- ✅ 安装 NuGet 提供者
- ✅ 更新 PowerShellGet
- ✅ 安装 VCLibs 和 UI.Xaml（Winget 依赖）
- ✅ 安装 Winget
- ✅ 安装 Scoop（并添加 extras/versions/nerd-fonts buckets）
- ✅ 安装 Chocolatey
- ✅ 刷新 PATH 环境变量

**使用方法**：
```powershell
# 以管理员身份运行
Set-ExecutionPolicy Bypass -Scope Process -Force
.\01_bootstrap_ltsc.ps1
```

**适用场景**：
- 全新安装的 LTSC 系统
- 需要 Scoop 和 Chocolatey 的用户
- 作为其他安装脚本的前置步骤

---

### 02_install.ps1
**用途**：Scoop 官方安装脚本

**功能**：
- 从 GitHub 克隆或下载 Scoop
- 创建 Scoop shim（快捷方式）
- 配置 Scoop 目录和缓存
- 将 Scoop 添加到 PATH

**使用方法**：
```powershell
# 普通用户安装
.\02_install.ps1

# 自定义目录安装
.\02_install.ps1 -ScoopDir "D:\Scoop" -ScoopGlobalDir "D:\Scoop\Global"

# 使用代理安装
.\02_install.ps1 -Proxy "http://proxy:8080"
```

**参数说明**：
- `-ScoopDir`: Scoop 根目录（默认：`$env:USERPROFILE\scoop`）
- `-ScoopGlobalDir`: 全局应用目录（默认：`$env:ProgramData\scoop`）
- `-ScoopCacheDir`: 缓存目录（默认：`$ScoopDir\cache`）
- `-Proxy`: 代理地址
- `-ProxyCredential`: 代理凭据
- `-RunAsAdmin`: 强制以管理员运行（不推荐）

**注意**：
- 默认不需要管理员权限
- 需要 PowerShell 5.0 或更高版本
- 需要 .NET Framework 4.5+

---

### 03_install_windows.ps1
**用途**：完整开发环境一键安装

**功能**：
1. **包管理器检查**
   - Scoop（自动安装）
   - Winget（检查）

2. **CLI 工具（通过 Scoop）**
   - 开发：git, gh, node, python, go, zig, deno, fnm, cmake, ninja
   - 文档：pandoc
   - 搜索：ripgrep, fd
   - 下载：wget, aria2, yt-dlp, gallery-dl
   - 媒体：ffmpeg, imagemagick, exiftool
   - 备份：restic
   - 压缩：p7zip
   - 工具：tree, sqlite, nasm, yasm, topgrade
   - AI：ollama
   - OCR：tesseract
   - PDF：poppler
   - 压缩算法：lz4, zstd, xz, brotli

3. **GUI 应用（通过 Winget）**
   - Bitwarden CLI
   - LocalSend
   - GnuPG
   - OpenJDK
   - Certbot

4. **Rust 工具链**
   - Rust + Cargo
   - Cargo 包：bkmr, cargo-expand, cargo-audit 等 17 个工具

5. **NPM 全局包**
   - @anthropic-ai/claude-code
   - acp-ts, lodash, openclaw
   - opencode-ai, run-deepseek-cli, uipro-cli

6. **Python pip 包**
   - 数据科学：numpy, scipy, scikit-learn, pandas
   - 图像处理：pillow, opencv-python
   - AI/ML：torch, lightgbm, openvino
   - 工具：flask, flask-cors, tqdm, joblib, sympy, networkx

7. **UV 工具**
   - uv 包管理器
   - kimi-cli

**使用方法**：
```powershell
# 以管理员身份运行
Set-ExecutionPolicy Bypass -Scope Process -Force
.\03_install_windows.ps1
```

**执行时间**：约 30-60 分钟（取决于网络速度）

**适用场景**：
- 开发环境快速搭建
- 需要完整工具链的用户
- AI/机器学习开发环境

**注意**：
- 需要稳定的网络连接
- 需要约 10GB 磁盘空间
- 部分工具仅支持 macOS（已跳过）

---

## 🔀 脚本选择指南

### 场景 1：只需要基础工具
```powershell
.\00_QuickInstall.ps1
```

### 场景 2：需要 Scoop 和更多 CLI 工具
```powershell
.\01_bootstrap_ltsc.ps1
.\03_install_windows.ps1
```

### 场景 3：只需要 Winget 和常用软件
```powershell
# 使用原有脚本
.\install_ltsc_essential.ps1
```

### 场景 4：完整开发环境
```powershell
# 步骤 1：安装包管理器
.\01_bootstrap_ltsc.ps1

# 步骤 2：安装开发工具
.\03_install_windows.ps1

# 步骤 3：安装额外软件
winget install <软件 ID>
```

---

## 📊 包管理器对比

| 特性 | Winget | Scoop | Chocolatey |
|------|--------|-------|------------|
| 来源 | 微软官方 | 社区 | 社区 |
| GUI 应用 | ✅ | ❌ | ✅ |
| CLI 工具 | ⚠️ | ✅ | ✅ |
| 开发者工具 | ⚠️ | ✅ | ✅ |
| 安装速度 | 快 | 快 | 中 |
| 需要管理员 | 否 | 否 | 是 |
| 推荐用途 | 日常软件 | 开发工具 | 企业软件 |

---

## 🛠️ 组合使用示例

### 最小化安装
```powershell
# 1. 安装包管理器
.\01_bootstrap_ltsc.ps1

# 2. 安装必要工具
scoop install git nodejs-lts python

# 3. 安装日常软件
winget install 7zip.Google.Chrome
```

### 完整开发环境
```powershell
# 1. 安装所有包管理器
.\01_bootstrap_ltsc.ps1

# 2. 安装完整开发工具
.\03_install_windows.ps1

# 3. 验证安装
winget --version
scoop --version
choco --version
git --version
node --version
python --version
```

---

## ⚠️ 注意事项

1. **网络要求**
   - 所有脚本都需要稳定的网络连接
   - 建议使用有线连接或稳定的 WiFi
   - 如遇下载失败，检查代理设置

2. **磁盘空间**
   - 最小安装：2GB
   - 完整开发环境：10GB+
   - 建议预留 20GB 空间

3. **执行权限**
   ```powershell
   # 临时允许脚本执行
   Set-ExecutionPolicy Bypass -Scope Process -Force
   
   # 恢复默认设置（安装后）
   Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
   ```

4. **PATH 刷新**
   - 安装完成后重启终端
   - 或运行：`refreshenv`（Chocolatey）
   - 或重新登录 Windows

---

## 🔧 故障排查

### Scoop 安装失败
```powershell
# 检查 PowerShell 版本
$PSVersionTable.PSVersion

# 检查执行策略
Get-ExecutionPolicy

# 手动安装 Scoop
Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression
```

### Winget 不可用
```powershell
# 检查是否安装
Get-AppxPackage -Name Microsoft.DesktopAppInstaller

# 从商店安装
start ms-windows-store://pdp/?ProductId=9NBLGGH4NNS1
```

### PATH 未更新
```powershell
# 刷新环境变量
$env:PATH = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
```

---

*文档更新时间：2026-03-25*

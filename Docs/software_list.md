# 软件与环境一致性

事实来源分成三层：

- [macos_inventory.json](../src/assets/macos_inventory.json)：源 Mac 的实际工具名称快照。
- [parity_rules.json](../src/assets/parity_rules.json)：非同名工具、WSL 对等项、macOS 专属项和手动项。
- [setup_profile.json](../src/assets/setup_profile.json)：Windows 自动安装矩阵和系统配置。

当前 Windows 自动矩阵包括 34 个 WinGet 包、83 个 Scoop 工具，以及 Cargo、NPM、Pip/UV 工具。新增了 PowerToys、Sysinternals、x64/x86 Visual C++ Runtime、Codex CLI、Repomix、Krew、uutils-coreutils、libvips 和源 Mac 缺失的 Cargo 工具。

部署前会计算完整覆盖报告：

- 自动：可由已配置 provider 安装。
- 对等：Windows 内置、WSL 或已安装的替代工具。
- macOS 专属：不能也不应复制到 Windows。
- 手动：上游可能有 Windows 产物，但没有经过验证的 provider 清单。
- 未映射：配置错误；任务直接停止。

`.github/workflows/ci.yml` 每周在 Windows 上逐项验证所有 provider 端点和精确 WinGet ID，避免清单长期腐烂。

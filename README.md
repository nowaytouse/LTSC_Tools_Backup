# LTSC Workspace

个人自用的 Windows LTSC 工作站部署器：从源 Mac 采集 Homebrew、Cargo、NPM、UV 等工具清单，把可维护的 Windows 对等项追加到 LTSC，同时配置开发环境、Windows 可选功能、网络、电源、存储、隐私和 Agent 资源。

Windows GUI 只构建为原生 `.exe`。它没有 HTML、WebView、Electron 或浏览器运行时；窗口、状态机、任务编排、回滚和 Windows 操作都在 Rust 内实现。macOS 端只有一个无 GUI 的清单采集命令。

## 核心改进

- 删除仓库外置 `00_QuickSetup.ps1`；失败、取消、panic 和成功都用结构化完成事件收尾，不会再永久卡在“正在运行”。
- 每个子进程都有超时与取消；stdout/stderr 并发排空，输出有 512 KiB 上限，取消或超时会终止 Windows 子进程树。
- 发布版带 Windows `requireAdministrator` UAC manifest，启动即请求所需权限。
- 注册表由 `winreg` 直接读写；DISM、WinGet、`netsh.exe`、`powercfg.exe`、`fsutil.exe` 和 `defrag.exe` 均由 Rust 以参数数组调用，不拼接 PowerShell 命令。
- 系统变更前记录原始注册表值、TCP 配置、TRIM 状态和原电源计划；“撤销上次调优”会按账本逆序恢复。
- 存储优化使用 Windows 官方 `/O` 策略，让系统按 SSD/HDD 类型选择 retrim 或 defrag，不对 SSD 盲目套传统碎片整理。
- AC 极限电源档只修改接通电源参数，不改电池策略；休眠默认不关闭。
- 网络 Extreme 档使用 RSS/RSC、正常自动调优、Fast Open、CTCP 与 ECN，不再暗中导入或修改 WinHTTP 代理。
- Profile 与源 Mac 清单逐项核对：自动安装、Windows/WSL 对等、macOS 专属和待手动项目都会明确列出，未知项会阻止部署。
- 每周 CI 在 Windows 上重新检查全部 WinGet、Scoop、Cargo、NPM、Pip/UV 项，并构建 Windows 发布产物。

Scoop 本身按上游设计以 PowerShell 实现；本项目只通过受控 provider 适配器调用它，不下载或执行项目部署 PS1，也不把 Scoop 的内部脚本当成本项目控制流。

## Mac → Windows 同步

在源 Mac 的仓库目录运行：

```text
cargo run --bin macos_inventory -- src/assets/macos_inventory.json
```

该命令只记录用于映射的工具名称，不写入主机名、系统用户名、绝对路径、环境变量或 Homebrew tap 列表；tap 可能泄露个人/私有仓库命名空间，而且不是 Windows 安装目标。然后提交并推送 `macos_inventory.json`，Windows 端拉取新提交并重新构建/下载 CI 产物。映射规则在 [parity_rules.json](src/assets/parity_rules.json)，安装矩阵在 [setup_profile.json](src/assets/setup_profile.json)。

当前快照包含 103 个 Homebrew formula、15 个 cask、31 个 Cargo 工具、13 个 NPM 全局包和 3 个 UV 工具。

## Windows 使用

1. 在 Windows LTSC 拉取最新仓库或下载 CI 生成的 `ltsc_setup_gui.exe`。
2. 双击程序；UAC 会自动请求管理员权限。
3. 选择“完整部署”或专项任务，检查勾选项后开始。
4. 右侧以最终状态为准：已完成、完成但有失败项、已取消或异常终止。
5. 系统调优不合适时选择“撤销上次调优”。软件安装和 Windows 功能不会被自动卸载。

专项任务包括开发工具、网络、存储、Windows 功能、IDE、Agent、系统调优和回滚。

## 代码结构

```text
src/
├── app.rs                 # Windows 原生 GUI、状态与取消
├── installer.rs           # 有序部署、调优、审计与回滚执行
├── utils.rs               # 有界/可取消子进程与安全文件更新
├── platform.rs            # Windows 注册表与用户环境
├── rollback.rs            # 持久化原值账本
├── inventory.rs           # Mac 清单、映射规则与覆盖报告
├── config.rs              # Profile 模型与完整校验
├── bin/macos_inventory.rs # macOS 无 GUI 清单采集器
├── bin/profile_audit.rs   # Windows 全量 provider 在线审计
└── assets/                # 编译进 exe 的 Profile、清单、字体和 Agent 资源
```

## 验证

```text
cargo fmt --all -- --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo clippy --target x86_64-pc-windows-gnu --all-targets -- -D warnings
cargo audit
cargo build --release --target x86_64-pc-windows-gnu --bin ltsc_setup_gui
```

Windows CI 还会运行 `profile_audit`，全量查询配置中的 provider 项。

## 安全边界

- 这是个人机器部署器，不是通用企业镜像或“删得越多越快”的 debloat 工具。
- 待手动或 macOS 专属项目不会从未知下载站点强装。
- 回滚账本覆盖本项目修改的注册表、TCP、TRIM 与电源计划；它不等同于完整系统镜像，也不会卸载软件或自动关闭已启用的 Windows 功能。
- LTSC 的 App Installer、Features on Demand 和硬件能力存在版本差异；不支持的可选功能会明确警告。

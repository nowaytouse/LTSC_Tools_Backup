# Changelog

## 2.1.0 - 2026-08-24

- 将 GUI 限定为 Windows 原生构建，关闭 eframe 的 Web/X11/Wayland 默认功能，并内置 UAC 管理员 manifest。
- 新增 macOS 无 GUI 清单采集器、165 项当前快照、逐项 parity 规则和未知项阻断；公开快照默认省略可能暴露个人仓库命名空间的 Homebrew tap 列表。
- 扩充 PowerToys、Sysinternals、VC++ Runtime、Codex CLI、Repomix、Krew、uutils、libvips 与本机 Cargo 工具。
- 新增网络原配置快照，移除 WinHTTP 代理导入；Extreme 改为 RSS/RSC、Fast Open、CTCP 与 ECN。
- 新增 TRIM、按介质 `/O` 存储优化、Windows Features 专项任务和可选 Sandbox/.NET 3.5/Hyper-V。
- 新增可复用的受管卓越性能计划和仅修改 AC 参数的极限电源档。
- 新增持久化回滚账本与“撤销上次调优”；休眠默认关闭选项为 false，无法读到原状态时拒绝变更。
- 新增 Windows/macOS CI、每周全量 provider 在线审计和发布产物上传。
- Windows release 静态链接 VC Runtime，并在 CI 中阻断缺失 UAC 清单或重新引入动态 `VCRUNTIME*.dll` 的产物。

## 2.0.0 - 2026-08-24

- 重做 GUI：任务导航、配置卡片、独立运行记录、明确结果摘要与取消按钮。
- 用结构化 `SetupEvent::Finished` 取代“看到 End 日志才结束”的脆弱状态机。
- 新增可取消、有界输出的命令执行层；取消和超时会终止 Windows 进程树。
- 删除 `src/Scripts/00_QuickSetup.ps1`，移除核心流程中的 PowerShell 命令拼接。
- 网络、WSL、UWP、注册表、电源计划与最终审计改用 Rust/Windows 原生实现。
- WinGet 改用 curl + DISM 原生引导；Scoop 由 Rust 部署官方仓库和 shim。
- 修正执行顺序：先准备 WSL、镜像与运行时，再安装依赖它们的工具和 IDE 扩展。
- Profile 加入 Git 与 Rustup 基础包，版本升至 2.0.0。
- 全量核对官方 WinGet/Scoop 清单；修正 Podman Desktop 与 PeaZip ID，移除无可用 Windows 清单的映射，并把 ChatGPT/UWP 改为 Microsoft Store 产品 ID。
- 配置写入改为保守合并；检测到无法安全合并的 JSONC 时不会覆盖用户文件。
- Agent 资源释放范围收紧为 Skills、Plugins 与 MCP 配置。
- 新增取消、管道排空、受管区块、Profile 校验、包识别、GUID 解析与 JSON 合并测试。

## 1.1.0 - 2026-08-24

- 并发排空子进程 stdout/stderr。
- 增加 PATH 刷新、包存在检查、有限重试与失败计数。
- 根据本机 Homebrew 快照更新 Windows 等价工具矩阵。

## 1.0.0 - 2026-07-24

- 初始 Rust/egui GUI 与内置资源版本。

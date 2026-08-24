# 2.1 重构交付说明

## 已实现

- Windows-only 原生 `.exe` 与自动 UAC 权限请求；没有网页 GUI 或项目部署 PS1。
- 结构化完成事件、可取消进程树、有界并发输出和明确失败汇总。
- 源 Mac 清单生成器、逐项 parity 规则、未知项阻断和 Windows 在线 provider 审计。
- WinGet/Scoop/Cargo/NPM/Python 安装矩阵与 LTSC 常用 Store 应用。
- WSL2、.NET 3.5、Sandbox 和可选 Hyper-V 的能力感知启用。
- TCP 快照、RSS/RSC/自动调优/Fast Open/CTCP/ECN 分档；不修改 WinHTTP 代理。
- TRIM 和 Windows `/O` 存储优化。
- 受管卓越性能计划、只影响 AC 的极限参数和默认关闭的休眠开关。
- 注册表/TCP/TRIM/电源原值账本与 GUI 一键逆序回滚。
- Windows/macOS CI、依赖安全审计与每周全量 provider 检查。

## 目标 Windows 仍需实机验证

- 具体 LTSC 版本的 App Installer/MSIX 依赖与 Store 源可用性。
- Sandbox、NetFx3、WSL2 等 Features on Demand 是否存在或需要安装介质。
- 显卡、网卡、SSD/HDD 和电源固件对相应设置的实际支持。
- 第三方安装器在目标网络、地区和 x64 架构上的行为。

这些差异只能在目标 LTSC 设备上得到最终证据。程序会逐项保留真实结果，不把跨目标编译冒充实机部署成功。

# 实现与安全说明

## Windows-only GUI

`eframe` 仅作为 Windows 目标依赖启用，并关闭默认的 Web screen reader、X11 和 Wayland 功能。发布物是带 UAC manifest 的 Windows 原生进程，不包含 WebView/Electron。非 Windows 的主程序只输出说明；Mac 仅运行清单采集 CLI。

## 任务生命周期

后台线程无论成功、部分失败、取消还是 panic，都会发送 `Finished`。GUI 只依赖这个事件退出运行态。退出窗口时先取消，再等待 worker 收尾，避免留下孤儿安装进程。

子进程的 stdout/stderr 从启动起并发读取；每条命令独立超时；输出只保留最后 512 KiB；Windows 取消使用进程树终止。

## 原生 Windows 操作

- 注册表：`winreg` 直接读写并先保存原值。
- 可选功能：`dism.exe`，可选功能不受支持时警告，WSL2 必要组件失败时停止。
- 网络：`ipconfig.exe` / `netsh.exe`；持久变更前保存可由 `netsh exec` 恢复的快照。
- 电源：`powercfg.exe`；复用名为 `LTSC Workspace Ultimate` 的受管计划。
- 存储：`fsutil.exe` 查询/启用 TRIM，`defrag.exe /C /O /U` 按介质类型优化。
- 软件：WinGet 使用精确 ID 与非交互参数；Scoop 仅通过 provider shim 调用。

## 回滚

账本位于 `%LOCALAPPDATA%\LTSCWorkspace\rollback`，每次变更后立即落盘。注册表只记录同一键的第一次原值；命令按应用顺序记录、恢复时逆序执行。空账本不会成为“最近可恢复项”。

回滚不删除软件，不反向禁用 Windows 可选功能，也不是用户数据备份。

## 文件安全

Git ignore、PowerShell Profile 与 `.npmrc` 使用受管区块。VS Code/Cursor 严格 JSON 递归合并；检测到无法无损处理的 JSONC 时跳过并警告。已有个人键和不受管内容保持不变。

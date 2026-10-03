# 开始使用

## Windows 第一次运行

1. 获取最新的 `ltsc_setup_gui.exe`；程序只支持 Windows。
2. 双击后确认 UAC。发布版已内置管理员权限清单，不需要手动右键寻找入口。
3. 默认是“开发工具”：按 Mac 清单补齐已映射的工具。网络、存储、电源等调优需另选任务或显式勾选，不会随工具同步自动执行。
4. 检查清单后再点“开始”。默认先用 GitHub Raw，连接失败或内容无效时自动尝试 GitHub Contents API；每个来源连接最多等 8 秒、下载最多 20 秒，外层等待上限 25 秒（不是两次尝试合计），只跟随 HTTPS 重定向。自定义 HTTPS 地址会独占使用，不会回退到官方地址。网络受限或官方 API 达到访问限额时，可填自己信任且可访问的 HTTPS 地址，或输入从其他设备传来的 `macos_inventory.json` 本地路径并导入。导入文件最大 1 MiB，且与在线清单使用相同的格式、映射和版本检查；失败或取消更新时保留现有缓存并清理临时文件。根目录 `.exe` 已含这些清单功能，但未包含后续源码改动，见 [验证状态](READINESS.md)。
5. 右侧最终状态是唯一完成依据，进度 100% 本身不代表没有失败项。

## 任务

- 开发工具：WinGet、Scoop、Cargo、NPM、Pip 与 UV。
- 网络优化：保存原 TCP 配置后应用 Basic / Optimized / Extreme 档。
- 存储优化：启用 TRIM，并用 `/O` 按介质类型优化固定卷。
- Windows 功能：WSL2、.NET 3.5、Sandbox；完整 Hyper-V 默认关闭。
- IDE / Agent：保守合并配置和释放明确的内置资源。
- 系统优化：回滚账本、卓越性能、AC 极限参数、隐私和 Explorer 设置。
- 撤销上次调优：逆序恢复最近的非空账本。

## 状态

- 已完成：没有失败项。
- 完成，但有失败项：线程已退出，可按红色日志修复后重试。
- 已取消：当前子进程树已经停止。
- 异常终止：后台线程 panic 或事件通道断开；GUI 已退出运行态。

## 从 Mac 刷新工具

在源 Mac 运行：

```text
cargo run --bin macos_inventory -- --sync
```

此命令要求工作区干净，会快进拉取、采集并校验映射，仅在清单有实际变化时提交和推送。Windows 可使用默认 GitHub Raw/Contents API 来源、可访问的 HTTPS 镜像，或导入本地 JSON 文件；自定义地址保存在 `%LOCALAPPDATA%\LTSCWorkspace\inventory_source_url.txt`，清单缓存保存在同目录。仅清单变化时不需要重新构建程序。若新工具尚无当前程序支持的映射，导入/更新会失败并保留旧缓存；先更新 Windows 程序再重试。首次运行及缓存不可用时仍可使用内置清单。

清单可离线导入或使用内置副本；首次引导 WinGet 仍需访问 GitHub API 和微软的 GitHub Release 文件，Scoop 需要访问 GitHub 仓库和软件发布地址，其他包管理器也需要访问各自的软件源。

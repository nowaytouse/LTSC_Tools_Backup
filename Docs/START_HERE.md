# 开始使用

## Windows 第一次运行

1. 获取最新的 `ltsc_setup_gui.exe`；程序只支持 Windows。
2. 双击后确认 UAC。发布版已内置管理员权限清单，不需要手动右键寻找入口。
3. 首次选择“完整部署”；也可以先分别运行“Windows 功能”和“开发工具”。
4. 检查中间的选项和风险说明，然后开始。
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
cargo run --bin macos_inventory -- src/assets/macos_inventory.json
```

提交并推送清单后，让 Windows 拉取最新提交并使用新构建。程序会在部署前拒绝任何没有映射规则的新工具，防止静默漏同步。

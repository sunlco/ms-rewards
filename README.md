# MS Rewards Bot

MS Rewards Bot 是一个基于 Rust 的应用程序，旨在通过模拟人类行为自动化搜索和与搜索结果的交互。它使用指定 API 的热词进行搜索并与结果进行交互。

## 功能

- 模拟人类的输入和滚动行为。
- 自动打开和关闭浏览器标签页。
- 从 API 获取热词并标记为已搜索。
- 支持测试模式以验证键盘交互。

## 前提条件

- 系统上已安装 Rust 和 Cargo。
- 兼容的操作系统（Windows 或 macOS）用于浏览器自动化。

## 安装

1. 克隆仓库：

   ```bash
   git clone https://github.com/yourusername/ms-rewards.git
   cd ms-rewards
   ```

2. 构建项目：

   ```bash
   cargo build --release
   ```

## 使用

### 运行机器人

在正常模式下运行机器人，执行：
```bash
cargo run --release
```

### 测试模式
在测试模式下运行机器人，测试键盘交互，使用：

```bash
cargo run --release -- --test
```

## 配置

- 机器人使用 `hot_words.json` 来存储和管理热词。该文件由机器人自动创建和更新。
- 机器人从 API `https://uapis.cn/api/hotlist?type=douyin` 获取热词。

## 依赖

- [Enigo](https://crates.io/crates/enigo) 用于模拟键盘和鼠标操作。
- [Rand](https://crates.io/crates/rand) 用于生成随机数。
- [Reqwest](https://crates.io/crates/reqwest) 用于进行 HTTP 请求。
- [Serde](https://crates.io/crates/serde) 和 [Serde JSON](https://crates.io/crates/serde_json) 用于 JSON 序列化和反序列化。
- [Tokio](https://crates.io/crates/tokio) 用于异步编程。
- [Chrono](https://crates.io/crates/chrono) 用于日期和时间处理。

## 许可证

此项目根据 MIT 许可证授权。有关详细信息，请参阅 [LICENSE](LICENSE) 文件。

## 贡献

欢迎贡献！如有任何改进或错误修复，请打开问题或提交拉取请求。

## 致谢

- 感谢在本项目中使用的库的开发者们的出色工作。
"""
# ProjMan - 项目管理 CLI 工具

一个用 Rust 编写的项目管理命令行工具，用于快速管理和操作多个项目。

## 功能特性

- 📁 **项目扫描与发现** - 自动扫描工作目录中的项目
- ⚡ **快速启动项目** - 一键启动配置好的项目
- 🔧 **自定义命令** - 为每个项目配置启动命令
- 🌿 **Git 集成** - 支持 Git 仓库操作和分支管理
- 📝 **交互式选择** - 使用交互式界面选择项目
- ⚙️ **配置文件管理** - 使用 YAML 配置文件管理项目设置

## 安装

### 从源码构建

```bash
git clone <repository-url>
cd projman
cargo build --release
```

### 安装到系统路径

```bash
cargo install --path .
```

## 快速开始

### 1. 配置项目

编辑 `projman.yml` 文件，添加你的项目配置：

```yaml
projects:
  - name: frontend
    path: /path/to/frontend
    start: "pnpm dev"
    branch: dev
  - name: backend
    path: /path/to/backend
    start: "cargo run"
    branch: main
  - name: projman
    path: /path/to/projman
    start: "cargo run list"
    branch: master
git:
  before:
    is_pull: true
```

### 2. 使用命令

#### 列出所有项目
```bash
projman list
```

#### 启动指定项目
```bash
projman start <项目名称>
```

#### 使用交互式界面选择项目
```bash
projman list
# 然后选择要启动的项目
```

## 配置说明

### 项目配置字段

- `name`: 项目名称（用于命令行引用）
- `path`: 项目路径
- `start`: 启动命令
- `branch`: Git 分支

### Git 配置

- `git.before.is_pull`: 在启动项目前是否执行 `git pull`

## 项目结构

```
projman/
├── Cargo.toml          # Rust 项目配置
├── projman.yml         # 项目配置文件
├── src/
│   ├── main.rs         # 主程序入口
│   ├── lib.rs          # 核心库
│   ├── git_commands.rs # Git 操作模块
│   └── my_err.rs       # 错误处理模块
└── README.md          # 项目文档
```

## 依赖

- `clap` - 命令行参数解析
- `dialoguer` - 交互式命令行界面
- `serde` - 序列化/反序列化
- `serde_yaml` - YAML 配置文件处理

## 开发

### 构建项目
```bash
cargo build
```

### 运行测试
```bash
cargo test
```

### 运行开发版本
```bash
cargo run -- list
```

## 许可证

[添加许可证信息]

## 贡献

欢迎提交 Issue 和 Pull Request！
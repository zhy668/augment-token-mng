# Augment Token Manager

一个基于 Tauri 构建的跨平台桌面应用程序，用于生成和管理 Augment Code 访问令牌。


## 安装指南

### 环境准备

1. **安装 Rust**：
   ```bash
   # Windows (PowerShell)
   Invoke-WebRequest -Uri https://win.rustup.rs/ -OutFile rustup-init.exe
   .\rustup-init.exe

   # macOS/Linux
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **安装 Node.js**：
   - 从 [nodejs.org](https://nodejs.org/) 下载安装
   - 或使用包管理器（例如：`winget install OpenJS.NodeJS`）

3. **安装 Tauri CLI**：
   ```bash
   cargo install tauri-cli
   ```

### 快速构建（推荐）

#### Windows：
```powershell
cd tauri-oauth-app
.\build.ps1
```

#### macOS/Linux：
```bash
cd tauri-oauth-app
chmod +x build.sh
./build.sh
```

### 手动构建

#### 开发模式：
```bash
cd tauri-oauth-app
npm install          # 安装前端依赖
cargo tauri dev      # 启动开发服务器
```

#### 发布构建：
```bash
cd tauri-oauth-app
npm install          # 安装前端依赖
cargo tauri build    # 构建生产版本
```


## 使用方法

1. **生成授权 URL**
   - 点击"生成 Augment 授权 URL"按钮
   - 复制生成的 URL 或点击"打开授权 URL"在浏览器中打开

2. **浏览器授权**
   - 完成 OAuth 授权流程
   - 从浏览器复制 JSON 响应

3. **获取访问令牌**
   - 将 JSON 响应粘贴到文本区域
   - 点击"获取访问令牌"
   - 复制生成的访问令牌和租户 URL

4. **令牌管理**
   - 点击"保存 Token"保存当前令牌
   - 点击"已保存 Token"查看和管理所有保存的令牌
   - 支持编辑、删除和复制令牌

## 许可证

本项目是开源项目，采用 MIT 许可证。

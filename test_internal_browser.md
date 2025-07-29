# Tauri 内置浏览器功能测试

## 新增功能

### 1. 多窗口 OAuth 登录
- 添加了 `start_forum_oauth_login_internal` 命令
- 在应用内创建新的 WebView 窗口进行 OAuth 授权
- 自动监听回调并完成登录流程
- 登录完成后自动关闭授权窗口

### 2. 通用内置浏览器
- 添加了 `open_internal_browser` 命令
- 可以打开任意 URL 在新窗口中
- 支持自定义窗口标题
- 返回窗口标识符用于后续操作

### 3. 窗口管理
- 添加了 `close_window` 命令
- 可以通过窗口标识符关闭指定窗口

## 前端集成

### App.vue 更新
- 添加了内置浏览器登录按钮
- 实现了事件监听机制
- 支持两种登录方式：系统浏览器和内置浏览器

### EmailManager.vue 更新
- 书签卡片添加了两个按钮：外部打开和内置打开
- 支持在内置浏览器中浏览邮箱网站
- 保持原有的系统浏览器打开功能

### TokenGenerator.vue 更新
- OAuth 授权 URL 支持内置浏览器打开
- 添加了图标和更清晰的按钮标识

## 使用方法

### 1. OAuth 登录
```javascript
// 系统浏览器登录（原有功能）
await invoke('start_forum_oauth_login')

// 内置浏览器登录（新功能）
await invoke('start_forum_oauth_login_internal')
```

### 2. 打开内置浏览器
```javascript
// 打开指定 URL
const windowLabel = await invoke('open_internal_browser', {
  url: 'https://example.com',
  title: '示例网站'
})

// 关闭窗口
await invoke('close_window', { window_label: windowLabel })
```

### 3. 事件监听
```javascript
import { listen } from '@tauri-apps/api/event'

// 监听 OAuth 完成事件
const unlisten = await listen('oauth_completed', (event) => {
  console.log('OAuth 登录成功:', event.payload)
})

// 监听 OAuth 错误事件
const unlistenError = await listen('oauth_error', (event) => {
  console.error('OAuth 登录失败:', event.payload)
})
```

## 技术实现

### 后端 (Rust)
- 使用 `WindowBuilder` 创建新的 WebView 窗口
- 通过 `tokio::spawn` 异步监听 OAuth 回调
- 使用 `app.emit` 发送事件到前端
- 支持窗口的创建、管理和关闭

### 前端 (Vue.js)
- 使用 `@tauri-apps/api/event` 监听后端事件
- 动态更新 UI 状态
- 支持多种浏览器打开方式

## 优势

1. **用户体验**: 用户无需离开应用即可完成 OAuth 授权
2. **安全性**: 内置浏览器仍然使用系统的 WebView 组件
3. **灵活性**: 支持系统浏览器和内置浏览器两种模式
4. **一致性**: 保持应用的视觉和交互一致性

## 注意事项

1. 内置浏览器窗口会占用额外的系统资源
2. 某些网站可能对 WebView 有特殊限制
3. 需要确保回调 URL 的正确配置
4. 建议为用户提供两种选择，让用户根据需要选择合适的方式

<template>
  <div class="app">
    <!-- Header -->
    <header class="app-header">
      <div class="header-left">
        <h1>Token 管理器</h1>
        <div class="user-status">
          <span v-if="isGuest" class="status-badge guest">游客模式</span>
          <div v-else-if="isAuthenticated" class="status-badge authenticated user-info">
            <img
              v-if="userAvatarUrl"
              :src="userAvatarUrl"
              :alt="currentUser.username"
              class="user-avatar"
              @error="onAvatarError"
            />
            <div v-else class="user-avatar-placeholder">
              {{ currentUser.username.charAt(0).toUpperCase() }}
            </div>
            <span class="username">{{ currentUser.username }}</span>
          </div>
          <span v-else class="status-badge loading">加载中...</span>
        </div>
      </div>
      <div class="header-buttons">
        <!-- Feature buttons -->
        <button
          @click="showEmailManager = true"
          :class="['btn', 'secondary', { disabled: !canAccessEmailFeatures }]"
          :disabled="!canAccessEmailFeatures"
          :title="canAccessEmailFeatures ? '邮箱获取' : '需要登录后使用'"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"/>
          </svg>
          邮箱获取
        </button>
        <button @click="showTokenGenerator = true" class="btn primary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
          </svg>
          获取新Token
        </button>

        <!-- User mode controls -->
        <div class="user-controls">
          <button
            v-if="isAuthenticated"
            @click="logoutUser"
            class="btn secondary small"
            title="退出登录"
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
              <path d="M17 7l-1.41 1.41L18.17 11H8v2h10.17l-2.58 2.59L17 17l5-5zM4 5h8V3H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h8v-2H4V5z"/>
            </svg>
            退出
          </button>
          <div v-if="isGuest" class="login-buttons">
            <button
              @click="showOAuthLogin"
              class="btn primary small login-btn"
              title="OAuth登录（系统浏览器）"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
              </svg>
              登录
            </button>
            <button
              @click="showOAuthLoginInternal"
              class="btn secondary small login-btn-internal"
              title="OAuth登录（内置浏览器）"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 4H5c-1.11 0-2 .9-2 2v12c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-5 14H5V8h9v10z"/>
              </svg>
              内置
            </button>
          </div>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="main-content">
      <!-- Empty State -->
      <div v-if="tokens.length === 0 && !isLoading" class="empty-state">
        <div class="empty-icon">
          <svg width="64" height="64" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
          </svg>
        </div>
        <h2>还没有保存的Token</h2>
        <p>点击右上角的"获取新Token"按钮来添加你的第一个Token</p>
      </div>

      <!-- Loading State -->
      <div v-if="isLoading" class="loading-state">
        <div class="spinner"></div>
        <p>正在加载Token...</p>
      </div>

      <!-- Token List -->
      <div v-if="tokens.length > 0" class="token-list">
        <div class="list-header">
          <h2>已保存的Token ({{ tokens.length }})</h2>
          <button @click="cleanupExpired" class="btn secondary small">
            清理过期Token
          </button>
        </div>

        <TokenCard
          v-for="token in tokens"
          :key="token.id"
          :token="token"
          @delete="deleteToken"
          @copy-success="showStatus"
        />
      </div>
    </main>

    <!-- Token Generator Modal -->
    <TokenGenerator
      v-if="showTokenGenerator"
      @close="showTokenGenerator = false"
      @token-saved="onTokenSaved"
    />

    <!-- Email Manager Modal -->
    <EmailManager
      v-if="showEmailManager"
      @close="showEmailManager = false"
    />

    <!-- Status Messages -->
    <div
      v-if="statusMessage"
      :class="['status-toast', statusType]"
    >
      {{ statusMessage }}
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import TokenCard from './components/TokenCard.vue'
import TokenGenerator from './components/TokenGenerator.vue'
import EmailManager from './components/EmailManager.vue'

// Reactive data
const tokens = ref([])
const isLoading = ref(false)
const showTokenGenerator = ref(false)
const showEmailManager = ref(false)
const statusMessage = ref('')
const statusType = ref('info')

// User state management
const userMode = ref({ type: "Guest" })
const isLoadingUserState = ref(false)

// Computed properties
const isAuthenticated = computed(() => {
  return userMode.value && userMode.value.type === "Authenticated"
})

const isGuest = computed(() => {
  return userMode.value && userMode.value.type === "Guest"
})

const currentUser = computed(() => {
  if (isAuthenticated.value && userMode.value.data) {
    return userMode.value.data.user_info
  }
  return null
})

const userAvatarUrl = computed(() => {
  if (currentUser.value && currentUser.value.avatar_url) {
    return currentUser.value.avatar_url
  }
  return null
})

const canAccessEmailFeatures = computed(() => {
  return isAuthenticated.value
})

// Methods
const showStatus = (message, type = 'info') => {
  statusMessage.value = message
  statusType.value = type

  setTimeout(() => {
    statusMessage.value = ''
  }, 3000)
}

const loadTokens = async () => {
  isLoading.value = true
  try {
    const result = await invoke('get_all_tokens')
    tokens.value = result
  } catch (error) {
    showStatus(`加载Token失败: ${error}`, 'error')
  } finally {
    isLoading.value = false
  }
}

const deleteToken = async (tokenId) => {
  try {
    const success = await invoke('delete_token', { id: tokenId })
    if (success) {
      tokens.value = tokens.value.filter(token => token.id !== tokenId)
      showStatus('Token删除成功!', 'success')
    } else {
      showStatus('Token删除失败', 'error')
    }
  } catch (error) {
    showStatus(`删除Token失败: ${error}`, 'error')
  }
}

const cleanupExpired = async () => {
  try {
    const removedCount = await invoke('cleanup_expired_tokens')
    if (removedCount > 0) {
      await loadTokens()
      showStatus(`已清理 ${removedCount} 个过期Token`, 'success')
    } else {
      showStatus('没有过期的Token需要清理', 'info')
    }
  } catch (error) {
    showStatus(`清理过期Token失败: ${error}`, 'error')
  }
}

const onTokenSaved = () => {
  loadTokens()
  showStatus('新Token已保存!', 'success')
}

// User state management methods
const loadUserMode = async () => {
  isLoadingUserState.value = true
  try {
    const mode = await invoke('get_user_mode')
    console.log('Received user mode:', mode)
    userMode.value = mode
  } catch (error) {
    console.error('Failed to load user mode:', error)
    showStatus(`加载用户状态失败: ${error}`, 'error')
  } finally {
    isLoadingUserState.value = false
  }
}

const setGuestMode = async () => {
  try {
    await invoke('set_guest_mode')
    await loadUserMode()
    showStatus('已切换到游客模式', 'info')
  } catch (error) {
    showStatus(`切换到游客模式失败: ${error}`, 'error')
  }
}

const logoutUser = async () => {
  try {
    await invoke('logout_user')
    await loadUserMode()
    showStatus('已退出登录', 'info')
  } catch (error) {
    showStatus(`退出登录失败: ${error}`, 'error')
  }
}

const showOAuthLogin = async () => {
  try {
    showStatus('正在启动OAuth登录...', 'info')
    const loginResult = await invoke('start_forum_oauth_login')

    // 登录成功，重新加载用户状态
    await loadUserMode()
    showStatus(`登录成功！欢迎 ${loginResult.user_info.username}`, 'success')
  } catch (error) {
    showStatus(`登录失败: ${error}`, 'error')
  }
}

const showOAuthLoginInternal = async () => {
  try {
    showStatus('正在启动内置浏览器OAuth登录...', 'info')
    const windowLabel = await invoke('start_forum_oauth_login_internal')

    // 监听OAuth完成事件
    const { listen } = await import('@tauri-apps/api/event')

    const unlistenSuccess = await listen('oauth_completed', (event) => {
      showStatus(`登录成功！欢迎 ${event.payload.user_info.username}`, 'success')
      loadUserMode()
      unlistenSuccess()
      unlistenError()
    })

    const unlistenError = await listen('oauth_error', (event) => {
      showStatus(`登录失败: ${event.payload}`, 'error')
      unlistenSuccess()
      unlistenError()
    })

  } catch (error) {
    showStatus(`启动内置浏览器失败: ${error}`, 'error')
  }
}

const onAvatarError = (event) => {
  // 头像加载失败时隐藏图片，显示占位符
  event.target.style.display = 'none'
}

// Initialize
onMounted(() => {
  loadTokens()
  loadUserMode()
})
</script>

<style scoped>
.app {
  height: 100vh;
  background: #f8f9fa;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* 隐藏所有滚动条 */
* {
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
}

*::-webkit-scrollbar {
  width: 0px;
  background: transparent;
}

/* 确保body和html不产生滚动条 */
html, body {
  overflow: hidden;
  height: 100%;
  margin: 0;
  padding: 0;
}

.app-header {
  background: white;
  border-bottom: 1px solid #e1e5e9;
  padding: 12px 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  flex-shrink: 0;
  min-height: 60px;
  flex-wrap: wrap;
  gap: 8px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
  min-width: 0;
}

.app-header h1 {
  margin: 0;
  color: #333;
  font-size: 20px;
  font-weight: 600;
  white-space: nowrap;
}

.user-status {
  display: flex;
  align-items: center;
}

.status-badge {
  padding: 4px 12px;
  border-radius: 16px;
  font-size: 12px;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.status-badge.guest {
  background: #f8f9fa;
  color: #6c757d;
  border: 1px solid #dee2e6;
}

.status-badge.authenticated {
  background: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px !important;
}

.user-avatar {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  object-fit: cover;
  border: 2px solid #c3e6cb;
}

.user-avatar-placeholder {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: #28a745;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: bold;
  border: 2px solid #c3e6cb;
}

.username {
  font-weight: 500;
  white-space: nowrap;
}

.status-badge.loading {
  background: #fff3cd;
  color: #856404;
  border: 1px solid #ffeaa7;
}

.header-buttons {
  display: flex;
  gap: 8px;
  align-items: center;
  flex-wrap: wrap;
  justify-content: flex-end;
}

.user-controls {
  display: flex;
  gap: 8px;
  align-items: center;
  margin-left: 12px;
  padding-left: 12px;
  border-left: 1px solid #e1e5e9;
}

.main-content {
  padding: 20px 16px;
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.btn {
  padding: 10px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.btn.primary {
  background: #007bff;
  color: white;
}

.btn.primary:hover {
  background: #0056b3;
}

.btn.secondary {
  background: #6c757d;
  color: white;
}

.btn.secondary:hover {
  background: #545b62;
}

.btn.small {
  padding: 6px 12px;
  font-size: 12px;
}

.btn.disabled,
.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

.login-buttons {
  display: flex;
  gap: 6px;
  align-items: center;
}

.login-btn {
  background: linear-gradient(135deg, #007bff, #0056b3) !important;
  border: none !important;
  box-shadow: 0 2px 8px rgba(0, 123, 255, 0.3) !important;
  transition: all 0.2s ease !important;
}

.login-btn:hover {
  transform: translateY(-1px) !important;
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.4) !important;
}

.login-btn-internal {
  background: linear-gradient(135deg, #6c757d, #545b62) !important;
  border: none !important;
  box-shadow: 0 2px 8px rgba(108, 117, 125, 0.3) !important;
  transition: all 0.2s ease !important;
}

.login-btn-internal:hover {
  transform: translateY(-1px) !important;
  box-shadow: 0 4px 12px rgba(108, 117, 125, 0.4) !important;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .app-header {
    padding: 8px 12px;
    min-height: 56px;
  }

  .app-header h1 {
    font-size: 18px;
  }

  .header-buttons {
    gap: 6px;
  }

  .btn {
    padding: 8px 12px;
    font-size: 13px;
  }

  .btn.small {
    padding: 6px 10px;
    font-size: 12px;
  }

  .user-status {
    order: 2;
    width: 100%;
    justify-content: flex-start;
    margin-top: 4px;
  }

  .status-badge {
    font-size: 11px;
    padding: 3px 8px;
  }

  .user-avatar,
  .user-avatar-placeholder {
    width: 20px;
    height: 20px;
    font-size: 10px;
  }

  .username {
    font-size: 11px;
  }
}

@media (max-width: 480px) {
  .app-header {
    flex-direction: column;
    align-items: stretch;
    gap: 8px;
    padding: 8px;
  }

  .header-left {
    justify-content: space-between;
    width: 100%;
  }

  .header-buttons {
    justify-content: space-between;
    width: 100%;
  }

  .user-controls {
    margin-left: auto;
    padding-left: 8px;
    border-left: 1px solid #e1e5e9;
  }
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: #666;
}

.empty-icon {
  margin-bottom: 24px;
  color: #ccc;
}

.empty-state h2 {
  margin: 0 0 12px 0;
  color: #333;
  font-size: 24px;
  font-weight: 600;
}

.empty-state p {
  margin: 0;
  font-size: 16px;
  line-height: 1.5;
}

.loading-state {
  text-align: center;
  padding: 40px 20px;
  color: #666;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #007bff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.token-list {
  width: 100%;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.list-header h2 {
  margin: 0;
  color: #333;
  font-size: 20px;
  font-weight: 600;
}

.status-toast {
  position: fixed;
  top: 20px;
  right: 20px;
  padding: 12px 20px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  animation: slideIn 0.3s ease;
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

.status-toast.info {
  background: #d1ecf1;
  color: #0c5460;
  border: 1px solid #bee5eb;
}

.status-toast.success {
  background: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
}

.status-toast.error {
  background: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}

@media (max-width: 768px) {
  .app-header {
    padding: 16px 20px;
    flex-direction: column;
    gap: 16px;
    text-align: center;
  }

  .header-buttons {
    flex-direction: column;
    width: 100%;
  }

  .header-buttons .btn {
    width: 100%;
    justify-content: center;
  }

  .main-content {
    padding: 20px 16px;
  }

  .list-header {
    flex-direction: column;
    gap: 16px;
    align-items: stretch;
  }

  .status-toast {
    left: 20px;
    right: 20px;
    top: auto;
    bottom: 20px;
  }
}
</style>

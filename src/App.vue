<template>
  <div class="app">
    <!-- Header -->
    <header class="app-header">
      <div class="header-left">
        <h1>Augment Token Manager</h1>
      </div>
      <div class="header-buttons">
        <!-- Feature buttons -->
        <button @click="showBookmarkManager = true" class="btn secondary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M17 3H7c-1.1 0-1.99.9-1.99 2L5 21l7-3 7 3V5c0-1.1-.9-2-2-2z"/>
          </svg>
          书签管理
        </button>
        <button @click="showTokenList = true" class="btn primary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z"/>
          </svg>
          已保存Token
        </button>


      </div>
    </header>

    <!-- Main Content -->
    <main class="main-content">
      <div class="token-generator-main">
        <div class="generator-header">
          <h2>生成Augment Token</h2>
          <p>按照以下步骤获取你的Augment访问令牌</p>
        </div>

        <div class="generator-body">
          <!-- Step 1: Generate Authorization URL -->
          <div class="section">
            <h3>步骤 1: 生成Augment授权URL</h3>
            <button
              @click="generateAuthUrl"
              :class="['btn', 'primary', { loading: isGenerating }]"
              :disabled="isGenerating"
            >
              生成Augment授权URL
            </button>

            <div v-if="authUrl" class="url-section">
              <label>授权URL:</label>
              <div class="input-with-copy">
                <input
                  type="text"
                  :value="authUrl"
                  readonly
                  ref="authUrlInput"
                  placeholder="点击上方按钮生成授权URL"
                >
                <button @click="copyAuthUrl" class="copy-icon-btn" title="复制URL">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
                  </svg>
                </button>
              </div>
              <div class="button-container">
                <button @click="showAuthUrlDialog = true" class="btn secondary">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
                  </svg>
                  打开授权URL
                </button>
              </div>
            </div>
          </div>

          <!-- Step 2: Enter Authorization Code -->
          <div class="section">
            <h3>步骤 2: 输入授权码</h3>
            <textarea
              v-model="authCode"
              placeholder="在此粘贴授权码JSON..."
              rows="4"
            ></textarea>
            <div class="button-container">
              <button
                @click="getAccessToken"
                :class="['btn', 'primary', { loading: isGettingToken }]"
                :disabled="!canGetToken || isGettingToken"
              >
                获取访问令牌
              </button>
            </div>
          </div>

          <!-- Step 3: Access Token -->
          <div class="section" v-if="tokenResult">
            <h3>步骤 3: Augment访问令牌</h3>
            <div class="token-section">
              <div class="result-container">
                <label>访问令牌:</label>
                <div class="token-container">
                  <input
                    type="text"
                    :value="tokenResult.access_token"
                    readonly
                    ref="accessTokenInput"
                  >
                  <button @click="copyAccessToken" class="btn secondary">复制</button>
                </div>
              </div>
              <div class="result-container">
                <label>租户URL:</label>
                <div class="token-container">
                  <input
                    type="text"
                    :value="tokenResult.tenant_url"
                    readonly
                    ref="tenantUrlInput"
                  >
                  <button @click="copyTenantUrl" class="btn secondary">复制</button>
                </div>
              </div>
              <div class="button-container">
                <button @click="saveToken" class="btn success">保存Token</button>
              </div>
            </div>
          </div>
        </div>

        <!-- Status Messages -->
        <div
          v-if="statusMessage"
          :class="['status', statusType]"
        >
          {{ statusMessage }}
        </div>
      </div>
    </main>

    <!-- Token List Modal -->
    <TokenList
      v-if="showTokenList"
      :tokens="tokens"
      :isLoading="isLoading"
      @close="showTokenList = false"
      @delete="deleteToken"
      @copy-success="showStatus"
      @add-token="showTokenForm"
      @refresh="loadTokens"
      @open-portal="handleOpenPortal"
      @copy-action="handleCopyAction"
      @edit="handleEditToken"
    />

    <!-- Token Form Modal -->
    <TokenForm
      v-if="showTokenFormModal"
      :token="editingToken"
      @close="closeTokenForm"
      @success="handleTokenFormSuccess"
    />

    <!-- Portal打开方式选择对话框 -->
    <div v-if="showPortalDialog" class="portal-dialog-overlay" @click="showPortalDialog = false">
      <div class="portal-dialog" @click.stop>
        <h3>选择打开方式</h3>
        <div class="dialog-buttons">
          <button @click="openPortalExternal" class="dialog-btn external">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
            </svg>
            外部打开
          </button>
          <button @click="openPortalInternal" class="dialog-btn internal">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
            </svg>
            内置打开
          </button>
          <button @click="showPortalDialog = false" class="dialog-btn cancel">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>
            不打开
          </button>
        </div>
      </div>
    </div>

    <!-- 复制操作选择对话框 -->
    <div v-if="showCopyDialog" class="portal-dialog-overlay" @click="showCopyDialog = false">
      <div class="portal-dialog" @click.stop>
        <h3>选择操作方式</h3>
        <div class="dialog-buttons">
          <button @click="copyContentToClipboard" class="dialog-btn external">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
            </svg>
            复制到剪贴板
          </button>
          <button @click="openInBrowser" class="dialog-btn internal">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
            </svg>
            在浏览器中打开
          </button>
          <button @click="showCopyDialog = false" class="dialog-btn cancel">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>
            取消
          </button>
        </div>
      </div>
    </div>

    <!-- Bookmark Manager Modal -->
    <BookmarkManager
      v-if="showBookmarkManager"
      @close="showBookmarkManager = false"
    />

    <!-- Status Messages -->
    <div
      v-if="statusMessage"
      :class="['status-toast', statusType]"
    >
      {{ statusMessage }}
    </div>

    <!-- 授权URL打开方式选择对话框 -->
    <div v-if="showAuthUrlDialog" class="portal-dialog-overlay" @click="showAuthUrlDialog = false">
      <div class="portal-dialog" @click.stop>
        <h3>选择打开方式</h3>
        <div class="dialog-buttons">
          <button @click="openAuthUrlExternal" class="dialog-btn external">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
            </svg>
            外部打开
          </button>
          <button @click="openAuthUrlInternal" class="dialog-btn internal">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
            </svg>
            内置打开
          </button>
          <button @click="showAuthUrlDialog = false" class="dialog-btn cancel">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>
            取消
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import TokenCard from './components/TokenCard.vue'
import TokenList from './components/TokenList.vue'
import TokenForm from './components/TokenForm.vue'
import BookmarkManager from './components/BookmarkManager.vue'

// Reactive data
const tokens = ref([])
const isLoading = ref(false)
const showTokenList = ref(false)
const showBookmarkManager = ref(false)
const statusMessage = ref('')
const statusType = ref('info')

// Token generator data
const authUrl = ref('')
const authCode = ref('')
const tokenResult = ref(null)
const isGenerating = ref(false)
const isGettingToken = ref(false)

// Template refs
const authUrlInput = ref(null)
const accessTokenInput = ref(null)
const tenantUrlInput = ref(null)



// Portal dialog
const showPortalDialog = ref(false)
const currentPortalToken = ref(null)

// Copy dialog
const showCopyDialog = ref(false)
const currentCopyAction = ref(null)

// Auth URL dialog
const showAuthUrlDialog = ref(false)

// Token form dialog
const showTokenFormModal = ref(false)
const editingToken = ref(null)

// Computed properties

const canGetToken = computed(() => {
  return authUrl.value && authCode.value.trim().length > 0
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



const onTokenSaved = () => {
  loadTokens()
  showStatus('新Token已保存!', 'success')
}

// Token generator methods
const copyToClipboard = async (text) => {
  try {
    await navigator.clipboard.writeText(text)
    return true
  } catch (error) {
    console.error('Failed to copy to clipboard:', error)
    return false
  }
}

const generateAuthUrl = async () => {
  isGenerating.value = true

  try {
    const url = await invoke('generate_augment_auth_url')
    authUrl.value = url
  } catch (error) {
    showStatus(`错误: ${error}`, 'error')
  } finally {
    isGenerating.value = false
  }
}

const copyAuthUrl = async () => {
  const success = await copyToClipboard(authUrl.value)
  showStatus(
    success ? 'URL已复制到剪贴板!' : '复制URL失败',
    success ? 'success' : 'error'
  )
}



const getAccessToken = async () => {
  isGettingToken.value = true
  showStatus('正在获取访问令牌...', 'info')

  try {
    const result = await invoke('get_augment_token', { code: authCode.value })
    tokenResult.value = result
    showStatus('访问令牌获取成功!', 'success')
  } catch (error) {
    showStatus(`错误: ${error}`, 'error')
  } finally {
    isGettingToken.value = false
  }
}

const copyAccessToken = async () => {
  const success = await copyToClipboard(tokenResult.value.access_token)
  showStatus(
    success ? '访问令牌已复制到剪贴板!' : '复制访问令牌失败',
    success ? 'success' : 'error'
  )
}

const copyTenantUrl = async () => {
  const success = await copyToClipboard(tokenResult.value.tenant_url)
  showStatus(
    success ? '租户URL已复制到剪贴板!' : '复制租户URL失败',
    success ? 'success' : 'error'
  )
}

const saveToken = async () => {
  try {
    const result = await invoke('save_token', {
      tenantUrl: tokenResult.value.tenant_url,
      accessToken: tokenResult.value.access_token,
      portalUrl: null
    })

    showStatus('Token保存成功!', 'success')
    await loadTokens()

    // Reset form
    authUrl.value = ''
    authCode.value = ''
    tokenResult.value = null
  } catch (error) {
    showStatus(`保存Token失败: ${error}`, 'error')
  }
}







// Token form methods
const showTokenForm = () => {
  editingToken.value = null
  showTokenFormModal.value = true
}

const handleEditToken = (token) => {
  editingToken.value = token
  showTokenFormModal.value = true
}

const closeTokenForm = () => {
  showTokenFormModal.value = false
  editingToken.value = null
}

const handleTokenFormSuccess = async () => {
  await loadTokens()
  showStatus(editingToken.value ? 'Token更新成功!' : 'Token保存成功!', 'success')
}

const saveTokenManually = async (tenantUrl, accessToken, portalUrl) => {
  try {
    const result = await invoke('save_token', {
      tenantUrl,
      accessToken,
      portalUrl: portalUrl || null
    })

    showStatus('Token保存成功!', 'success')
    await loadTokens()
  } catch (error) {
    showStatus(`保存Token失败: ${error}`, 'error')
  }
}

// Portal dialog methods
const handleOpenPortal = (token) => {
  currentPortalToken.value = token
  showPortalDialog.value = true
}

const openPortalExternal = async () => {
  showPortalDialog.value = false
  if (!currentPortalToken.value?.portal_url) return

  try {
    await invoke('open_url', { url: currentPortalToken.value.portal_url })
  } catch (error) {
    console.error('Failed to open portal externally:', error)
    showStatus('打开Portal失败', 'error')
  }
}

const openPortalInternal = async () => {
  showPortalDialog.value = false
  if (!currentPortalToken.value?.portal_url) return

  try {
    const displayUrl = currentPortalToken.value.tenant_url.replace(/^https?:\/\//, '').replace(/\/$/, '')
    await invoke('open_internal_browser', {
      url: currentPortalToken.value.portal_url,
      title: 'Portal - ' + displayUrl
    })
  } catch (error) {
    console.error('Failed to open portal internally:', error)
    showStatus('打开Portal失败', 'error')
  }
}

// Copy dialog methods
const handleCopyAction = (action) => {
  currentCopyAction.value = action
  showCopyDialog.value = true
}

const copyContentToClipboard = async () => {
  showCopyDialog.value = false
  if (!currentCopyAction.value) return

  const { type, token } = currentCopyAction.value
  const content = type === 'token' ? token.access_token : token.tenant_url
  const label = type === 'token' ? 'Token' : 'URL'

  try {
    await navigator.clipboard.writeText(content)
    showStatus(`${label}已复制到剪贴板!`, 'success')
  } catch (error) {
    // 备用方案
    const textArea = document.createElement('textarea')
    textArea.value = content
    document.body.appendChild(textArea)
    textArea.select()
    document.execCommand('copy')
    document.body.removeChild(textArea)
    showStatus(`${label}已复制到剪贴板!`, 'success')
  }
}

const openInBrowser = async () => {
  showCopyDialog.value = false
  if (!currentCopyAction.value) return

  const { type, token } = currentCopyAction.value
  const url = type === 'token' ? token.tenant_url : token.tenant_url

  try {
    await invoke('open_url', { url })
  } catch (error) {
    console.error('Failed to open URL:', error)
    showStatus('打开浏览器失败', 'error')
  }
}

// Auth URL dialog methods
const openAuthUrlExternal = async () => {
  showAuthUrlDialog.value = false
  if (!authUrl.value) return

  try {
    await invoke('open_url', { url: authUrl.value })
  } catch (error) {
    console.error('Failed to open auth URL externally:', error)
    showStatus('打开授权URL失败', 'error')
  }
}

const openAuthUrlInternal = async () => {
  showAuthUrlDialog.value = false
  if (!authUrl.value) return

  try {
    await invoke('open_internal_browser', {
      url: authUrl.value,
      title: 'Augment OAuth 授权'
    })
  } catch (error) {
    console.error('Failed to open auth URL internally:', error)
    showStatus('打开授权URL失败', 'error')
  }
}



// Initialize
onMounted(() => {
  loadTokens()
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







.header-buttons {
  display: flex;
  gap: 8px;
  align-items: center;
  flex-wrap: wrap;
  justify-content: flex-end;
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

/* 输入框样式 */
input[type="text"] {
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  transition: border-color 0.2s;
  width: 100%;
  box-sizing: border-box;
}

input[type="text"]:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.1);
}

input[type="text"]:read-only {
  background-color: #f8f9fa;
  color: #6c757d;
}

/* 带复制图标的输入框 */
.input-with-copy {
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;
}

.input-with-copy input {
  padding-right: 45px;
  flex: 1;
}

.copy-icon-btn {
  position: absolute;
  right: 8px;
  background: none;
  border: none;
  cursor: pointer;
  padding: 6px;
  border-radius: 4px;
  color: #6c757d;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.copy-icon-btn:hover {
  background: #e9ecef;
  color: #495057;
}

.copy-icon-btn:active {
  transform: scale(0.95);
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

/* Portal对话框样式 */
.portal-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 3000; /* 确保在所有其他元素之上 */
}

.portal-dialog {
  background: white;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
  min-width: 300px;
  max-width: 400px;
}

.portal-dialog h3 {
  margin: 0 0 20px 0;
  font-size: 18px;
  font-weight: 600;
  color: #333;
  text-align: center;
}

.dialog-buttons {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.dialog-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border: 2px solid transparent;
  border-radius: 8px;
  background: #f8f9fa;
  color: #495057;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  justify-content: center;
}

.dialog-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.dialog-btn.external {
  background: #e3f2fd;
  color: #1976d2;
  border-color: #90caf9;
}

.dialog-btn.external:hover {
  background: #bbdefb;
  border-color: #64b5f6;
}

.dialog-btn.internal {
  background: #e8f5e8;
  color: #2e7d32;
  border-color: #a5d6a7;
}

.dialog-btn.internal:hover {
  background: #c8e6c9;
  border-color: #81c784;
}

.dialog-btn.cancel {
  background: #fce4ec;
  color: #c2185b;
  border-color: #f8bbd9;
}

.dialog-btn.cancel:hover {
  background: #f8bbd9;
  border-color: #f48fb1;
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

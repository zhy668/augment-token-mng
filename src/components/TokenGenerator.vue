<template>
  <div class="token-generator">
    <div class="modal-overlay" @click="$emit('close')">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>生成Augment Token</h2>
          <button class="close-btn" @click="$emit('close')">×</button>
        </div>
        
        <div class="modal-body">
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
              <p>Augment授权URL已生成:</p>
              <div class="url-input-container">
                <input
                  type="text"
                  :value="authUrl"
                  readonly
                  ref="authUrlInput"
                >
              </div>
              <div class="url-buttons">
                <button @click="copyAuthUrl" class="btn secondary">复制</button>
                <button @click="openAuthUrl" class="btn secondary">打开</button>
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
                <button @click="saveAndClose" class="btn success">保存并关闭</button>
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
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Emits
const emit = defineEmits(['close', 'token-saved'])

// Reactive data
const authUrl = ref('')
const authCode = ref('')
const tokenResult = ref(null)
const isGenerating = ref(false)
const isGettingToken = ref(false)
const statusMessage = ref('')
const statusType = ref('info')

// Template refs
const authUrlInput = ref(null)
const accessTokenInput = ref(null)
const tenantUrlInput = ref(null)

// Computed properties
const canGetToken = computed(() => {
  return authUrl.value && authCode.value.trim().length > 0
})

// Methods
const showStatus = (message, type = 'info') => {
  statusMessage.value = message
  statusType.value = type
  
  if (type === 'success' || type === 'info') {
    setTimeout(() => {
      statusMessage.value = ''
    }, 3000)
  }
}

const copyToClipboard = async (text) => {
  try {
    await navigator.clipboard.writeText(text)
    return true
  } catch (err) {
    // Fallback for older browsers
    const textArea = document.createElement('textarea')
    textArea.value = text
    document.body.appendChild(textArea)
    textArea.select()
    const success = document.execCommand('copy')
    document.body.removeChild(textArea)
    return success
  }
}

const generateAuthUrl = async () => {
  isGenerating.value = true
  showStatus('正在生成Augment授权URL...', 'info')
  
  try {
    const url = await invoke('generate_augment_auth_url')
    authUrl.value = url
    showStatus('Augment授权URL生成成功!', 'success')
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

const openAuthUrl = async () => {
  try {
    await invoke('open_url', { url: authUrl.value })
    showStatus('正在浏览器中打开授权URL...', 'info')
  } catch (error) {
    showStatus(`打开URL错误: ${error}`, 'error')
  }
}

const getAccessToken = async () => {
  if (!authCode.value.trim()) {
    showStatus('请输入授权码', 'error')
    return
  }
  
  isGettingToken.value = true
  showStatus('正在获取Augment访问令牌...', 'info')
  
  try {
    const result = await invoke('get_augment_token', { code: authCode.value.trim() })
    tokenResult.value = result
    showStatus('Augment访问令牌获取成功!', 'success')
  } catch (error) {
    showStatus(`错误: ${error}`, 'error')
  } finally {
    isGettingToken.value = false
  }
}

const copyAccessToken = async () => {
  const success = await copyToClipboard(tokenResult.value.access_token)
  showStatus(
    success ? '访问令牌已复制到剪贴板!' : '复制令牌失败',
    success ? 'success' : 'error'
  )
}

const copyTenantUrl = async () => {
  const success = await copyToClipboard(tokenResult.value.tenant_url)
  showStatus(
    success ? '租户URL已复制到剪贴板!' : '复制URL失败',
    success ? 'success' : 'error'
  )
}

const saveAndClose = async () => {
  try {
    await invoke('save_token', {
      tenantUrl: tokenResult.value.tenant_url,
      accessToken: tokenResult.value.access_token
    })
    showStatus('Token保存成功!', 'success')
    emit('token-saved')
    setTimeout(() => {
      emit('close')
    }, 1000)
  } catch (error) {
    showStatus(`保存失败: ${error}`, 'error')
  }
}

// Initialize
// showStatus('准备生成OAuth令牌', 'info')
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: white;
  border-radius: 8px;
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid #eee;
}

.modal-header h2 {
  margin: 0;
  color: #333;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: #666;
  padding: 0;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-btn:hover {
  color: #333;
}

.modal-body {
  padding: 20px;
}

.section {
  margin-bottom: 30px;
}

.section h3 {
  margin: 0 0 15px 0;
  color: #333;
  font-size: 18px;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.btn.primary {
  background: #007bff;
  color: white;
}

.btn.primary:hover:not(:disabled) {
  background: #0056b3;
}

.btn.secondary {
  background: #6c757d;
  color: white;
}

.btn.secondary:hover {
  background: #545b62;
}

.btn.success {
  background: #28a745;
  color: white;
}

.btn.success:hover {
  background: #1e7e34;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn.loading {
  position: relative;
}

.btn.loading::after {
  content: '';
  position: absolute;
  width: 16px;
  height: 16px;
  margin: auto;
  border: 2px solid transparent;
  border-top-color: #ffffff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.url-section, .token-section {
  margin-top: 15px;
  padding: 0;
  background: transparent;
  border-radius: 4px;
  text-align: left;
}

.url-section p, .token-section p {
  margin: 0 0 10px 0;
  text-align: left;
  font-weight: 500;
  color: #333;
}

.url-input-container {
  margin-top: 10px;
}

.url-input-container input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-family: monospace;
  font-size: 12px;
}

.url-buttons {
  display: flex;
  gap: 10px;
  margin-top: 10px;
  justify-content: flex-start;
}

.url-buttons .btn {
  padding: 8px 16px;
  font-size: 14px;
  min-width: 80px;
}

.token-container {
  display: flex;
  gap: 8px;
  margin-top: 10px;
  align-items: center;
}

.token-container input {
  flex: 1;
  padding: 8px 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-family: monospace;
  font-size: 11px;
  min-width: 0;
}

.token-container .btn {
  flex-shrink: 0;
  padding: 8px 12px;
  font-size: 12px;
  white-space: nowrap;
}

.result-container {
  margin-bottom: 15px;
}

.result-container label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
  color: #333;
}

textarea {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-family: monospace;
  font-size: 12px;
  resize: vertical;
}

.button-container {
  margin-top: 15px;
}

.status {
  padding: 10px 20px;
  margin: 0 20px 20px;
  border-radius: 4px;
  font-size: 14px;
}

.status.info {
  background: #d1ecf1;
  color: #0c5460;
  border: 1px solid #bee5eb;
}

.status.success {
  background: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
}

.status.error {
  background: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}
</style>

<template>
  <div class="modal-overlay">
    <div class="modal-content outlook-manager" @click.stop>
      <div class="modal-header">
        <h3>Outlook 邮箱管理</h3>
        <button @click="$emit('close')" class="close-btn">×</button>
      </div>

      <div class="modal-body">
        <!-- 添加邮箱表单 -->
        <div class="add-account-section">
          <h4>添加邮箱账户</h4>
          <div class="session-notice">
            <span class="notice-icon">ℹ️</span>
            账户信息仅在当前会话中有效，关闭应用后需要重新添加
          </div>
          <div class="form-group">
            <label>账户信息:</label>
            <input
              v-model="accountInput"
              type="text"
              placeholder="邮箱地址----密码----Refresh Token----Client ID"
              class="form-input"
            >
            <div class="input-hint">
              请按格式输入：邮箱地址----密码----Refresh Token----Client ID
            </div>
          </div>
          <div class="form-actions">
            <button
              @click="addAccount"
              :disabled="!canAddAccount || isAdding"
              :class="['btn', 'primary', { loading: isAdding }]"
            >
              {{ isAdding ? '添加中...' : '添加账户' }}
            </button>
          </div>
        </div>

        <!-- 账户列表 -->
        <div class="accounts-section">
          <div class="section-header">
            <h4>当前会话账户 ({{ accounts.length }})</h4>
            <button
              @click="refreshAccounts"
              :disabled="isLoading"
              class="btn secondary small"
            >
              刷新
            </button>

          </div>

          <div v-if="isLoading" class="loading-state">
            <div class="spinner"></div>
            <p>加载账户中...</p>
          </div>

          <div v-else-if="accounts.length === 0" class="empty-state">
            <p>当前会话中暂无邮箱账户</p>
            <p class="empty-hint">添加账户后即可开始使用邮件功能</p>
          </div>

          <div v-else class="accounts-list">
            <div
              v-for="account in accounts"
              :key="account"
              class="account-item"
            >
              <div class="account-info">
                <div class="account-email">{{ account }}</div>
                <div class="account-status" :class="getStatusClass(account)">
                  {{ getStatusText(account) }}
                </div>
              </div>
              <div class="account-actions">
                <button
                  @click="viewEmails(account)"
                  class="btn primary small"
                >
                  查看邮件
                </button>
                <button
                  @click="checkStatus(account)"
                  :disabled="isCheckingStatus"
                  class="btn secondary small"
                >
                  检查状态
                </button>
                <button
                  @click="deleteAccount(account)"
                  class="btn danger small"
                >
                  移除
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 邮件查看器 -->
  <EmailViewer
    v-if="showEmailViewer"
    :email="selectedEmail"
    @close="showEmailViewer = false"
  />
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import EmailViewer from './EmailViewer.vue'

const emit = defineEmits(['close', 'show-status'])

// 响应式数据
const accounts = ref([])
const accountStatuses = ref({})
const isLoading = ref(false)
const isAdding = ref(false)
const isCheckingStatus = ref(false)
const showEmailViewer = ref(false)
const selectedEmail = ref('')

const accountInput = ref('')

// 计算属性
const canAddAccount = computed(() => {
  return accountInput.value.trim() &&
         accountInput.value.includes('----') &&
         accountInput.value.split('----').length === 4
})

// 方法
const showStatus = (message, type = 'info') => {
  emit('show-status', message, type)
}

const refreshAccounts = async () => {
  isLoading.value = true
  try {
    accounts.value = await invoke('outlook_get_all_accounts')
  } catch (error) {
    showStatus(`刷新失败: ${error}`, 'error')
  } finally {
    isLoading.value = false
  }
}

const addAccount = async () => {
  isAdding.value = true
  try {
    // 解析输入的账户信息（四字段格式：邮箱----密码----Refresh Token----Client ID）
    const parts = accountInput.value.trim().split('----')
    if (parts.length !== 4) {
      throw new Error('格式错误，请按照 邮箱地址----密码----Refresh Token----Client ID 的格式输入')
    }

    const [email, password, refreshToken, clientId] = parts.map(part => part.trim())

    if (!email || !password || !refreshToken || !clientId) {
      throw new Error('邮箱、密码、Refresh Token 和 Client ID 都不能为空')
    }

    // 后端仍然只接收 email、clientId、refreshToken 三个字段
    await invoke('outlook_save_credentials', {
      email,
      refreshToken,
      clientId
    })

    // 重置表单
    accountInput.value = ''

    // 刷新账户列表
    await refreshAccounts()
    showStatus('账户添加成功', 'success')
  } catch (error) {
    showStatus(`添加失败: ${error}`, 'error')
  } finally {
    isAdding.value = false
  }
}

const deleteAccount = async (email) => {
  if (!confirm(`确定要从当前会话中移除账户 ${email} 吗？`)) {
    return
  }

  try {
    const deleted = await invoke('outlook_delete_account', { email })
    if (deleted) {
      await refreshAccounts()
      delete accountStatuses.value[email]
      showStatus('账户已从当前会话中移除', 'success')
    } else {
      showStatus('账户不存在', 'warning')
    }
  } catch (error) {
    showStatus(`移除失败: ${error}`, 'error')
  }
}

const checkStatus = async (email) => {
  isCheckingStatus.value = true
  try {
    const status = await invoke('outlook_check_account_status', { email })
    accountStatuses.value[email] = status.status
    showStatus(`${email} 状态: ${status.status}`, 'info')
  } catch (error) {
    accountStatuses.value[email] = 'error'
    showStatus(`状态检查失败: ${error}`, 'error')
  } finally {
    isCheckingStatus.value = false
  }
}

const viewEmails = (email) => {
  selectedEmail.value = email
  showEmailViewer.value = true
}



const getStatusClass = (email) => {
  const status = accountStatuses.value[email]
  return {
    'status-active': status === 'active',
    'status-inactive': status === 'inactive',
    'status-error': status === 'error',
    'status-unknown': !status
  }
}

const getStatusText = (email) => {
  const status = accountStatuses.value[email]
  switch (status) {
    case 'active': return '活跃'
    case 'inactive': return '非活跃'
    case 'error': return '错误'
    default: return '未知'
  }
}

// 生命周期
onMounted(() => {
  refreshAccounts()
})
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
  border-radius: 12px;
  max-width: 90vw;
  max-height: 95vh;
  overflow-y: auto;
  position: relative;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #e5e7eb;
  background: #f9fafb;
  border-radius: 12px 12px 0 0;
}

.modal-header h3 {
  margin: 0;
  color: #374151;
  font-size: 18px;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: #6b7280;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  transition: all 0.2s;
}

.close-btn:hover {
  background: #e5e7eb;
  color: #374151;
}

.outlook-manager {
  width: 90vw;
  max-width: 800px;
  max-height: 90vh;
}

.modal-body {
  padding: 24px;
  overflow-y: auto;
}

.add-account-section {
  background: #f8f9fa;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 24px;
}

.add-account-section h4 {
  margin: 0 0 16px 0;
  color: #333;
  font-size: 16px;
  font-weight: 600;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-weight: 500;
  color: #374151;
  font-size: 14px;
}

.form-input,
.form-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 14px;
  transition: border-color 0.2s ease;
  box-sizing: border-box;
}

.form-input:focus,
.form-textarea:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.form-textarea {
  resize: vertical;
  min-height: 80px;
}

.form-actions {
  margin-top: 20px;
}

.input-hint {
  margin-top: 6px;
  font-size: 12px;
  color: #6b7280;
  font-style: italic;
}

.session-notice {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: #fef3c7;
  border: 1px solid #f59e0b;
  border-radius: 6px;
  margin-bottom: 16px;
  font-size: 13px;
  color: #92400e;
}

.notice-icon {
  font-size: 16px;
  flex-shrink: 0;
}

.empty-hint {
  font-size: 12px;
  color: #9ca3af;
  margin-top: 8px;
}

.accounts-section h4 {
  margin: 0;
  color: #333;
  font-size: 16px;
  font-weight: 600;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.accounts-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.account-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
}

.account-info {
  flex: 1;
}

.account-email {
  font-weight: 500;
  color: #374151;
  margin-bottom: 4px;
}

.account-status {
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 12px;
  display: inline-block;
}

.status-active {
  background: #d1fae5;
  color: #065f46;
}

.status-inactive {
  background: #fee2e2;
  color: #991b1b;
}

.status-error {
  background: #fef3c7;
  color: #92400e;
}

.status-unknown {
  background: #f3f4f6;
  color: #6b7280;
}

.account-actions {
  display: flex;
  gap: 8px;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  text-decoration: none;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

.btn.primary {
  background: #3b82f6;
  color: white;
}

.btn.primary:hover:not(:disabled) {
  background: #2563eb;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(59, 130, 246, 0.3);
}

.btn.secondary {
  background: #6b7280;
  color: white;
}

.btn.secondary:hover:not(:disabled) {
  background: #4b5563;
}

.btn.danger {
  background: #dc2626;
  color: white;
}

.btn.danger:hover:not(:disabled) {
  background: #b91c1c;
}

.btn.small {
  padding: 6px 12px;
  font-size: 12px;
}

.btn.loading {
  opacity: 0.7;
  cursor: wait;
}

.btn.small {
  padding: 6px 12px;
  font-size: 12px;
}

.btn.danger {
  background: #dc2626;
  color: white;
}

.btn.danger:hover {
  background: #b91c1c;
}

.loading-state,
.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: #6b7280;
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid #f3f3f3;
  border-top: 3px solid #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 16px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>

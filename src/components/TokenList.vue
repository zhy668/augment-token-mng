<template>
  <div class="token-list-modal">
    <div class="modal-overlay">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <div class="header-title">
            <h2>{{ $t('tokenList.title') }}</h2>
            <div :class="['status-badge', storageStatusClass]">
              <span :class="['status-dot', storageStatusClass]"></span>
              <span class="status-text">{{ storageStatusText }}</span>
            </div>
          </div>
          <div class="header-actions">

            <button @click="handleSave" class="btn success small" :disabled="isSaving">
              <svg v-if="!isSaving" width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z"/>
              </svg>
              {{ isSaving ? $t('loading.saving') : $t('tokenList.save') }}
            </button>
            <!-- 数据库配置按钮 -->
            <button @click="showDatabaseConfig = true" class="btn info small">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 3C7.58 3 4 4.79 4 7s3.58 4 8 4 8-1.79 8-4-3.58-4-8-4zM4 9v3c0 2.21 3.58 4 8 4s8-1.79 8-4V9c0 2.21-3.58 4-8 4s-8-1.79-8-4zM4 16v3c0 2.21 3.58 4 8 4s8-1.79 8-4v-3c0 2.21-3.58 4-8 4s-8-1.79-8-4z"/>
              </svg>
              {{ $t('tokenList.databaseConfig') }}
            </button>
            <button @click="handleAddToken" class="btn primary small">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
              </svg>
              {{ $t('tokenList.addToken') }}
            </button>
            <button @click="handleRefresh" class="btn secondary small" :disabled="isRefreshing">
              <svg v-if="!isRefreshing" width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
              </svg>
              {{ isRefreshing ? $t('loading.refreshing') : $t('tokenList.refresh') }}
            </button>
            <button class="close-btn" @click="handleClose">×</button>
          </div>
        </div>
        
        <div class="modal-body">
          <!-- Empty State -->
          <div v-if="tokens.length === 0 && !isLoading" class="empty-state">
            <div class="empty-icon">
              <svg width="64" height="64" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
              </svg>
            </div>
            <h3>{{ $t('tokenList.empty') }}</h3>
            <p>{{ $t('tokenList.emptyDescription') }}</p>
          </div>

          <!-- Loading State -->
          <div v-if="isLoading" class="loading-state">
            <div class="spinner"></div>
            <p>{{ $t('tokenList.loading') }}</p>
          </div>

          <!-- Token List -->
          <div v-if="tokens.length > 0" class="token-list">
            <div class="list-header">
              <h3>{{ $t('tokenList.listTitle', { count: tokens.length }) }}</h3>
            </div>

            <div class="token-grid">
              <TokenCard
                v-for="token in tokens"
                :key="token.id"
                :ref="el => setTokenCardRef(el, token.id)"
                :token="token"
                @delete="deleteToken"
                @edit="handleEditToken"
                @token-updated="hasUnsavedChanges = true"
              />
            </div>


          </div>
        </div>
      </div>
    </div>

    <!-- Database Config Modal -->
    <DatabaseConfig
      v-if="showDatabaseConfig"
      @close="showDatabaseConfig = false"
      @config-saved="handleDatabaseConfigSaved"
      @config-deleted="handleDatabaseConfigDeleted"
    />

    <!-- Token Form Modal -->
    <TokenForm
      v-if="showTokenFormModal"
      :token="editingToken"
      @close="closeTokenForm"
      @success="handleTokenFormSuccess"
      @update-token="handleUpdateToken"
      @add-token="handleAddTokenFromForm"
    />
  </div>
</template>

<script setup>
import { ref, nextTick, onMounted, computed, readonly } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import TokenCard from './TokenCard.vue'
import DatabaseConfig from './DatabaseConfig.vue'
import TokenForm from './TokenForm.vue'

const { t } = useI18n()

// Props - 移除存储状态相关的props，TokenList自主管理
const props = defineProps({
  // 如果将来需要其他props可以在这里添加
})

// 内部状态管理 - TokenList直接管理tokens和存储状态
const tokens = ref([])
const isLoading = ref(false)
const hasUnsavedChanges = ref(false)
const isDatabaseAvailable = ref(false)

const emit = defineEmits(['close'])

// Additional state for new components
const showDatabaseConfig = ref(false)
const isSaving = ref(false)
const isRefreshing = ref(false)

// TokenForm state management
const showTokenFormModal = ref(false)
const editingToken = ref(null)

// Token card refs for accessing child methods
const tokenCardRefs = ref({})

// Computed properties for storage status display
const storageStatusText = computed(() => {
  const baseText = isDatabaseAvailable.value ? t('storage.dualStorage') : t('storage.localStorage')
  return hasUnsavedChanges.value ? `${baseText}-${t('storage.unsaved')}` : baseText
})

const storageStatusClass = computed(() => {
  return hasUnsavedChanges.value ? 'unsaved' : 'saved'
})



// 存储状态管理方法
const getStorageStatus = async () => {
  try {
    const status = await invoke('get_storage_status')
    isDatabaseAvailable.value = status?.is_database_available || false
  } catch (error) {
    console.error('Failed to get storage status:', error)
    isDatabaseAvailable.value = false
  }
}

// 设置ref的函数
const setTokenCardRef = (el, tokenId) => {
  if (el) {
    tokenCardRefs.value[tokenId] = el
  } else {
    // 当组件被移除时，清理引用
    delete tokenCardRefs.value[tokenId]
  }
}



// 检查所有Token的账号状态
const checkAllAccountStatus = async () => {
  if (tokens.value.length === 0) {
    return { success: true, message: t('messages.noTokensToCheck') }
  }

  try {
    // 准备批量检测的数据
    const tokenInfos = tokens.value.map(token => ({
      id: token.id,
      access_token: token.access_token,
      tenant_url: token.tenant_url,
      portal_url: token.portal_url || null
    }))

    console.log('Starting batch token status check for', tokenInfos.length, 'tokens')

    // 单次批量API调用
    const results = await invoke('batch_check_tokens_status', {
      tokens: tokenInfos
    })

    console.log('Batch check results:', results)

    // 批量更新tokens状态
    updateTokensFromResults(results)

    // 统计成功和失败的数量
    let successCount = 0
    let failureCount = 0

    results.forEach(result => {
      const statusResult = result.status_result
      if (statusResult.status === 'ERROR') {
        failureCount++
      } else {
        successCount++
      }
    })

    // 等待DOM更新后保存tokens
    await nextTick()
    await saveTokens()

    if (failureCount === 0) {
      return {
        success: true,
        message: t('messages.accountStatusCheckComplete', { success: successCount, total: tokenInfos.length })
      }
    } else if (successCount === 0) {
      return {
        success: false,
        message: t('messages.accountStatusCheckFailed', { failed: failureCount, total: tokenInfos.length })
      }
    } else {
      return {
        success: true,
        message: t('messages.accountStatusCheckPartial', { success: successCount, total: tokenInfos.length })
      }
    }
  } catch (error) {
    console.error('Batch check error:', error)
    return {
      success: false,
      message: `${t('messages.accountStatusCheckError')}: ${error}`
    }
  }
}

// 根据批量检测结果更新tokens状态
const updateTokensFromResults = (results) => {
  results.forEach(result => {
    const token = tokens.value.find(t => t.id === result.token_id)
    if (token) {
      const statusResult = result.status_result
      // 更新ban_status
      token.ban_status = statusResult.status
      
      // 更新Portal信息（如果有）
      if (result.portal_info) {
        token.portal_info = {
          credits_balance: result.portal_info.credits_balance,
          expiry_date: result.portal_info.expiry_date,
          is_active: result.portal_info.is_active
        }
        console.log(`Updated token ${token.id} portal info:`, token.portal_info)
      } else if (result.portal_error) {
        console.warn(`Failed to get portal info for token ${token.id}:`, result.portal_error)
      }
      
      // 更新时间戳
      token.updated_at = new Date().toISOString()
      console.log(`Updated token ${token.id} status to: ${statusResult.status}`)
    }
  })
  
  // 标记有未保存的更改
  hasUnsavedChanges.value = true
}

// TokenList内部的数据管理方法
const loadTokens = async (showSuccessMessage = false) => {
  isLoading.value = true
  try {
    const jsonString = await invoke('load_tokens_json')
    tokens.value = JSON.parse(jsonString)
    hasUnsavedChanges.value = false
    if (showSuccessMessage) {
      window.$notify.success(t('messages.tokenLoadSuccess'))
    }
  } catch (error) {
    window.$notify.error(`${t('messages.tokenLoadFailed')}: ${error}`)
    if (tokens.value.length === 0) {
      tokens.value = []
    }
    hasUnsavedChanges.value = false
  } finally {
    isLoading.value = false
  }
}

const saveTokens = async () => {
  try {
    const jsonString = JSON.stringify(tokens.value, null, 2)
    await invoke('save_tokens_json', { jsonString })
    hasUnsavedChanges.value = false
    window.$notify.success(t('messages.tokenSaved'))
  } catch (error) {
    window.$notify.error(`${t('messages.tokenSaveFailed')}: ${error}`)
    throw error
  }
}

// 删除token
const deleteToken = (tokenId) => {
  const tokenIndex = tokens.value.findIndex(token => token.id === tokenId)
  if (tokenIndex === -1) {
    window.$notify.error(t('messages.tokenNotFound'))
    return
  }

  // 从内存中删除
  tokens.value = tokens.value.filter(token => token.id !== tokenId)
  window.$notify.success(t('messages.tokenDeleted'))
  hasUnsavedChanges.value = true
  
  
  // 异步删除后端数据（不阻塞UI）
  invoke('delete_token', { tokenId }).catch(error => {
    console.log('Backend delete failed:', error)
  })
}

// TokenForm event handlers
const handleAddToken = () => {
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

const handleTokenFormSuccess = () => {
  closeTokenForm()
  window.$notify.success(t('messages.tokenUpdated'))
}

const handleUpdateToken = (updatedTokenData) => {
  const index = tokens.value.findIndex(token => token.id === updatedTokenData.id)
  if (index !== -1) {
    // Update the token in the list
    tokens.value[index] = {
      ...tokens.value[index],
      tenant_url: updatedTokenData.tenantUrl,
      access_token: updatedTokenData.accessToken,
      portal_url: updatedTokenData.portalUrl || null,
      email_note: updatedTokenData.emailNote || null
    }
    hasUnsavedChanges.value = true
  }
}

const handleAddTokenFromForm = (tokenData) => {
  addToken(tokenData)
}


// 添加token
const addToken = (tokenData) => {
  const newToken = {
    id: crypto.randomUUID(),
    tenant_url: tokenData.tenantUrl,
    access_token: tokenData.accessToken,
    created_at: new Date().toISOString(),
    portal_url: tokenData.portalUrl || null,
    ban_status: null,
    portal_info: null,
    email_note: tokenData.emailNote || null
  }
  
  tokens.value.push(newToken)
  hasUnsavedChanges.value = true
  return newToken
}

// 处理关闭事件
const handleClose = () => {
  // 如果有未保存的更改，显示提示并阻止关闭
  if (hasUnsavedChanges.value) {
    window.$notify.warning(t('messages.unsavedChangesClose'))
    return
  }

  // 没有未保存更改，正常关闭
  emit('close')
}

// 处理刷新事件 - 智能刷新逻辑
const handleRefresh = async () => {
  // 如果有未保存的更改，警告用户
  if (hasUnsavedChanges.value) {
    window.$notify.warning(t('messages.unsavedChangesRefresh'))
    return
  }

  if (isRefreshing.value) return
  isRefreshing.value = true

  try {
    if (isDatabaseAvailable.value) {
      // 双向存储模式：执行双向同步
      window.$notify.info(t('messages.bidirectionalSyncing'))

      const result = await invoke('bidirectional_sync_tokens')
      window.$notify.success(t('messages.bidirectionalSyncComplete'))

      // 刷新本地token列表显示
      await loadTokens(false)
      await nextTick() // 等待DOM更新

      // 只执行账号状态检查
      const statusResult = await Promise.allSettled([
        checkAllAccountStatus()
      ])

      if (statusResult[0].status === 'fulfilled') {
        if (!statusResult[0].value.success) {
          window.$notify.error(`${t('messages.syncCompleteButStatusFailed')}: ${statusResult[0].value.message}`)
        }
      } else {
        window.$notify.error(`${t('messages.syncCompleteButStatusFailed')}: ${statusResult[0].reason}`)
      }
    } else {
      // 本地存储模式：直接刷新和保存
      window.$notify.info(t('messages.refreshingTokenStatus'))

      // 直接加载tokens
      await loadTokens()
      await nextTick() // 等待DOM更新

      // 只有在有tokens时才执行后续操作和保存
      if (tokens.value.length > 0) {
        const statusResult = await Promise.allSettled([
          checkAllAccountStatus()
        ])

        if (statusResult[0].status === 'fulfilled') {
          const result = statusResult[0].value
          window.$notify[result.success ? 'success' : 'error'](result.message)
        } else {
          window.$notify.error(`${t('messages.accountStatusCheckError')}: ${statusResult[0].reason}`)
        }

        await saveTokens() // 直接调用内部方法
      } else {
        window.$notify.error(t('messages.refreshFailedNoTokens'))
      }
    }
  } catch (error) {
    window.$notify.error(`${t('messages.refreshFailed')}: ${error.message}`)
  } finally {
    isRefreshing.value = false
  }
}



const handleDatabaseConfigSaved = async () => {
  window.$notify.success(t('messages.databaseConfigSaved'))
  // 重新获取存储状态
  await getStorageStatus()
  // 自动执行刷新操作
  await handleRefresh()
}

const handleDatabaseConfigDeleted = async () => {
  window.$notify.info(t('messages.databaseConfigDeleted'))
  // 重新获取存储状态
  await getStorageStatus()
}



// 智能保存方法
const handleSave = async () => {
  if (isSaving.value) return

  isSaving.value = true
  try {
    if (isDatabaseAvailable.value) {
      // 🔴 修复双向存储模式逻辑：
      // 不要先保存到本地，因为bidirectional_sync会清空本地存储后重新写入
      // 直接执行双向同步，它会自动处理本地和远程数据的合并
      
      window.$notify.info(t('messages.bidirectionalSyncing'))
      const result = await invoke('bidirectional_sync_tokens')
      window.$notify.success(t('messages.bidirectionalSyncSaveComplete'))
      
      // 双向同步完成后刷新本地显示
      await loadTokens(false)
    } else {
      // 本地存储模式：直接保存
      await saveTokens()
    }
  } catch (error) {
    window.$notify.error(`${t('messages.saveFailed')}: ${error}`)
  } finally {
    isSaving.value = false
  }
}

// 组件挂载时自动加载tokens和存储状态
onMounted(async () => {
  // 首先获取存储状态
  await getStorageStatus()
  await loadTokens(false) // 显示成功消息
})

// 暴露方法给父组件
defineExpose({
  addToken,    // 允许App.vue添加token
  deleteToken, // 允许App.vue删除token  
  tokens: readonly(tokens), // 只读访问tokens
  saveTokens   // 允许App.vue保存tokens
})
</script>

<style scoped>
.token-list-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 2000;
}

.modal-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
}

.modal-content {
  background: var(--color-surface, #ffffff);
  border-radius: 12px;
  width: 100%;
  max-width: 900px;
  height: 90vh;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
}

/* 移除旧的 modal-header 样式，使用新的样式 */

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--color-text-muted, #6b7280);
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.close-btn:hover {
  background: var(--color-surface-hover, #f3f4f6);
  color: var(--color-text-primary, #374151);
}

.modal-body {
  padding: 24px;
  flex: 1;
  overflow-y: auto;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
}

.empty-icon {
  color: var(--color-border-strong, #d1d5db);
  margin-bottom: 16px;
}

.empty-state h3 {
  color: var(--color-text-primary, #374151);
  margin: 0 0 8px 0;
  font-size: 1.25rem;
}

.empty-state p {
  color: var(--color-text-muted, #6b7280);
  margin: 0;
}

.loading-state {
  text-align: center;
  padding: 40px 20px;
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--color-border, #e5e7eb);
  border-top: 3px solid var(--color-accent, #3b82f6);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 16px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.loading-state p {
  color: var(--color-text-muted, #6b7280);
  margin: 0;
}

.token-list {
  /* 移除内部滚动，使用外层 modal-body 的滚动 */
}

.token-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
  padding: 4px;
}

/* 响应式处理 */
@media (max-width: 768px) {
  .modal-content {
    margin: 10px;
    max-width: calc(100vw - 20px);
  }

  .modal-header {
    padding: 16px;
  }

  .modal-body {
    padding: 16px;
  }

  .header-actions {
    gap: 6px;
  }

  .token-grid {
    grid-template-columns: 1fr;
    gap: 12px;
  }

  .list-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }

  .list-header h3 {
    font-size: 1rem;
  }
}

@media (max-width: 480px) {
  .modal-overlay {
    padding: 10px;
  }

  .modal-content {
    max-height: 95vh;
  }

  .modal-header h2 {
    font-size: 1.25rem;
  }

  .empty-state {
    padding: 20px 10px;
  }

  .empty-state h3 {
    font-size: 1.125rem;
  }

  .btn.small {
    padding: 4px 8px;
    font-size: 11px;
  }

  .sync-actions {
    flex-direction: column;
  }

  .btn.sync-btn {
    min-width: auto;
  }
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--color-border, #e5e7eb);
}

.list-header h3 {
  margin: 0;
  color: var(--color-text-primary, #374151);
  font-size: 1.125rem;
  font-weight: 600;
}

.btn {
  padding: 8px 16px;
  border: 1px solid transparent;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  height: 36px;
  box-sizing: border-box;
}

.btn.secondary {
  background: var(--color-surface-hover, #f3f4f6);
  color: var(--color-text-primary, #374151);
  border: 1px solid var(--color-border-strong, #d1d5db);
}

.btn.secondary:hover {
  background: var(--color-border, #e5e7eb);
  border-color: var(--color-border-hover, #9ca3af);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(55, 65, 81, 0.2);
}

.btn.success {
  background: var(--color-success-surface, #d1fae5);
  color: var(--color-success-text, #065f46);
  border: 1px solid var(--color-success-border, #a7f3d0);
}

.btn.success:hover:not(:disabled) {
  background: var(--color-success-surface, #a7f3d0);
  border-color: var(--color-success-border, #6ee7b7);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(6, 95, 70, 0.3);
}

.btn.success:disabled {
  background: var(--color-border-strong, #d1d5db);
  color: var(--color-text-soft, #9ca3af);
  border-color: var(--color-border-strong, #d1d5db);
  cursor: not-allowed;
}

.btn.info {
  background: var(--color-info-surface, #dbeafe);
  color: var(--color-info-text, #1e40af);
  border: 1px solid var(--color-info-border, #93c5fd);
}

.btn.info:hover:not(:disabled) {
  background: var(--color-info-surface, #bfdbfe);
  border-color: var(--color-info-border, #60a5fa);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(30, 64, 175, 0.3);
}

.btn.small {
  padding: 6px 12px;
  font-size: 12px;
  height: 32px;
}

/* Header layout */
.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-border, #e5e7eb);
  background: var(--color-surface-alt, #f9fafb);
  min-height: 60px;
}

.header-title {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}

.header-title h2 {
  margin: 0;
  color: var(--color-text-strong, #111827);
  font-size: 1.25rem;
  font-weight: 600;
  line-height: 1.2;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
  flex-wrap: wrap;
}



/* Status badge styles */
.status-badge {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
  height: fit-content;
}

.status-badge.saved {
  background-color: var(--color-success-surface, #d1fae5);
  color: var(--color-success-text, #065f46);
}

.status-badge.unsaved {
  background-color: var(--color-warning-surface, #fef3c7);
  color: var(--color-warning-text, #92400e);
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  display: inline-block;
}

.status-dot.saved {
  background-color: var(--color-success-bg, #10b981);
}

.status-dot.unsaved {
  background-color: var(--color-warning-bg, #f59e0b);
}

.status-text {
  font-size: 11px;
  font-weight: 500;
}



@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.btn.loading {
  opacity: 0.7;
  pointer-events: none;
}

/* 浅色主题按钮样式统一 */
.btn.primary {
  background: var(--color-blue-soft-bg, #e3f2fd);
  color: var(--color-blue-soft-text, #1976d2);
  border: 1px solid var(--color-blue-soft-border, #90caf9);
}

.btn.primary:hover {
  background: var(--color-blue-soft-bg, #bbdefb);
  border-color: var(--color-blue-soft-hover, #64b5f6);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(25, 118, 210, 0.3);
}

/* 黑暗主题下的按钮样式 */
[data-theme='dark'] .btn.secondary {
  background: rgba(148, 163, 184, 0.2);
  color: #cbd5e1;
  border: 1px solid rgba(148, 163, 184, 0.4);
}

[data-theme='dark'] .btn.secondary:hover {
  background: rgba(148, 163, 184, 0.3);
  border-color: rgba(148, 163, 184, 0.6);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(148, 163, 184, 0.4);
}

[data-theme='dark'] .btn.success {
  background: rgba(34, 197, 94, 0.2);
  color: #86efac;
  border: 1px solid rgba(134, 239, 172, 0.4);
}

[data-theme='dark'] .btn.success:hover:not(:disabled) {
  background: rgba(34, 197, 94, 0.3);
  border-color: rgba(110, 231, 183, 0.6);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(34, 197, 94, 0.4);
}

[data-theme='dark'] .btn.success:disabled {
  background: rgba(100, 116, 139, 0.2);
  color: rgba(148, 163, 184, 0.6);
  border-color: rgba(100, 116, 139, 0.4);
  cursor: not-allowed;
}

[data-theme='dark'] .btn.info {
  background: rgba(14, 165, 233, 0.2);
  color: #7dd3fc;
  border: 1px solid rgba(125, 211, 252, 0.4);
}

[data-theme='dark'] .btn.info:hover:not(:disabled) {
  background: rgba(14, 165, 233, 0.3);
  border-color: rgba(56, 189, 248, 0.6);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(14, 165, 233, 0.4);
}

[data-theme='dark'] .btn.primary {
  background: rgba(59, 130, 246, 0.2);
  color: #93c5fd;
  border: 1px solid rgba(147, 197, 253, 0.4);
}

[data-theme='dark'] .btn.primary:hover {
  background: rgba(59, 130, 246, 0.3);
  border-color: rgba(96, 165, 250, 0.6);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(59, 130, 246, 0.4);
}

</style>

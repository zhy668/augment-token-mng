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
            <!-- 紧凑按钮组 -->
            <div class="compact-actions">
              <button @click="$emit('add-token')" class="btn primary compact">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
                </svg>
                {{ $t('tokenList.addToken') }}
              </button>
              <button @click="handleSave" class="btn success compact" :disabled="isSaving">
                <svg v-if="!isSaving" width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z"/>
                </svg>
                {{ isSaving ? $t('loading.saving') : $t('tokenList.save') }}
              </button>
              <button @click="exportTokens" :disabled="tokens.length === 0" class="btn export compact">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z"/>
                </svg>
                {{ $t('tokenList.exportTokens') }}
              </button>
              <button @click="handleRefresh" class="btn secondary compact" :disabled="isRefreshing">
                <svg v-if="!isRefreshing" width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
                </svg>
                {{ isRefreshing ? $t('loading.refreshing') : $t('tokenList.refresh') }}
              </button>
              <button @click="showDatabaseConfig = true" class="btn info compact">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 3C7.58 3 4 4.79 4 7s3.58 4 8 4 8-1.79 8-4-3.58-4-8-4zM4 9v3c0 2.21 3.58 4 8 4s8-1.79 8-4V9c0 2.21-3.58 4-8 4s-8-1.79-8-4zM4 16v3c0 2.21 3.58 4 8 4s8-1.79 8-4v-3c0 2.21-3.58 4-8 4s-8-1.79-8-4z"/>
                </svg>
                {{ $t('tokenList.databaseConfig') }}
              </button>
            </div>
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
                @delete="$emit('delete', $event)"
                @copy-success="$emit('copy-success', $event)"
              @open-portal="$emit('open-portal', $event)"
              @edit="$emit('edit', $event)"
              @token-updated="$emit('token-updated')"
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
      @show-status="handleShowStatus"
      @config-saved="handleDatabaseConfigSaved"
      @config-deleted="handleDatabaseConfigDeleted"
    />
  </div>
</template>

<script setup>
import { ref, nextTick, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import TokenCard from './TokenCard.vue'
import DatabaseConfig from './DatabaseConfig.vue'

const { t } = useI18n()

// Props
const props = defineProps({
  tokens: {
    type: Array,
    default: () => []
  },
  isLoading: {
    type: Boolean,
    default: false
  },
  hasUnsavedChanges: {
    type: Boolean,
    default: false
  },
  isDatabaseAvailable: {
    type: Boolean,
    default: false
  }
})

// Emits
const emit = defineEmits(['close', 'delete', 'copy-success', 'add-token', 'refresh', 'open-portal', 'edit', 'save', 'token-updated', 'storage-config-changed'])

// Additional state for new components
const showDatabaseConfig = ref(false)
const isSaving = ref(false)
const isRefreshing = ref(false)

// Token card refs for accessing child methods
const tokenCardRefs = ref({})

// Computed properties for storage status display
const storageStatusText = computed(() => {
  const baseText = props.isDatabaseAvailable ? t('storage.dualStorage') : t('storage.localStorage')
  return props.hasUnsavedChanges ? `${baseText}-${t('storage.unsaved')}` : baseText
})

const storageStatusClass = computed(() => {
  return props.hasUnsavedChanges ? 'unsaved' : 'saved'
})



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
  await nextTick() // 确保DOM已更新

  const allTokens = Object.entries(tokenCardRefs.value).filter(([tokenId, cardRef]) => {
    // 确保 token 还存在于当前列表中
    const tokenExists = props.tokens.find(t => t.id === tokenId)
    return tokenExists && cardRef && typeof cardRef.refreshAccountStatus === 'function'
  })

  if (allTokens.length === 0) {
    return { success: true, message: t('messages.noTokensToCheck') }
  }

  try {
    // 并行检查所有Token的账号状态
    const checkPromises = allTokens.map(async ([tokenId, cardRef]) => {
      try {
        await cardRef.refreshAccountStatus()
        return { tokenId, success: true }
      } catch (error) {
        console.error(`Failed to check account status for token ${tokenId}:`, error)
        return { tokenId, success: false, error: error.message }
      }
    })

    const results = await Promise.allSettled(checkPromises)

    // 统计成功和失败的数量
    let successCount = 0
    let failureCount = 0

    results.forEach(result => {
      if (result.status === 'fulfilled' && result.value.success) {
        successCount++
      } else {
        failureCount++
      }
    })

    if (failureCount === 0) {
      return {
        success: true,
        message: t('messages.accountStatusCheckComplete', { success: successCount, total: allTokens.length })
      }
    } else if (successCount === 0) {
      return {
        success: false,
        message: t('messages.accountStatusCheckFailed', { failed: failureCount, total: allTokens.length })
      }
    } else {
      return {
        success: true,
        message: t('messages.accountStatusCheckPartial', { success: successCount, total: allTokens.length })
      }
    }
  } catch (error) {
    return {
      success: false,
      message: `${t('messages.accountStatusCheckError')}: ${error.message}`
    }
  }
}

// 处理关闭事件
const handleClose = () => {
  // 如果有未保存的更改，显示提示并阻止关闭
  if (props.hasUnsavedChanges) {
    emit('copy-success', t('messages.unsavedChangesClose'), 'error')
    return
  }

  // 没有未保存更改，正常关闭
  emit('close')
}

// 处理刷新事件 - 智能刷新逻辑
const handleRefresh = async () => {
  // 如果有未保存的更改，警告用户
  if (props.hasUnsavedChanges) {
    emit('copy-success', t('messages.unsavedChangesRefresh'), 'error')
    return
  }

  if (isRefreshing.value) return

  isRefreshing.value = true

  try {
    if (props.isDatabaseAvailable) {
      // 双向存储模式：执行双向同步
      emit('copy-success', t('messages.bidirectionalSyncing'), 'info')

      const result = await invoke('bidirectional_sync_tokens')
      emit('copy-success', t('messages.bidirectionalSyncComplete'), 'success')

      // 刷新本地token列表显示
      emit('refresh', false) // 不显示成功消息，因为已经有同步完成的消息
      await nextTick() // 等待DOM更新

      // 只执行账号状态检查（包含Portal信息刷新）
      const statusResult = await Promise.allSettled([
        checkAllAccountStatus()
      ])

      // 处理检查结果
      if (statusResult[0].status === 'fulfilled') {
        if (!statusResult[0].value.success) {
          emit('copy-success', `${t('messages.syncCompleteButStatusFailed')}: ${statusResult[0].value.message}`, 'error')
        }
      } else {
        emit('copy-success', `${t('messages.syncCompleteButStatusFailed')}: ${statusResult[0].reason}`, 'error')
      }
    } else {
      // 本地存储模式：使用原有逻辑
      emit('copy-success', t('messages.refreshingTokenStatus'), 'info')

      // 先刷新token列表
      emit('refresh')
      await nextTick() // 等待DOM更新

      // 只执行账号状态检查（包含Portal信息刷新）
      const statusResult = await Promise.allSettled([
        checkAllAccountStatus()
      ])

      // 处理结果
      if (statusResult[0].status === 'fulfilled') {
        const result = statusResult[0].value
        emit('copy-success', result.message, result.success ? 'success' : 'error')
      } else {
        emit('copy-success', `${t('messages.accountStatusCheckError')}: ${statusResult[0].reason}`, 'error')
      }

      emit('save')
    }
  } catch (error) {
    // 显示错误通知
    emit('copy-success', `${t('messages.refreshFailed')}: ${error.message}`, 'error')
  } finally {
    isRefreshing.value = false
  }
}



// 新增的事件处理方法
const handleShowStatus = (message, type = 'info') => {
  emit('copy-success', message, type)
}

const handleDatabaseConfigSaved = async () => {
  emit('copy-success', t('messages.databaseConfigSaved'), 'success')
  // 通知父组件重新获取存储状态
  emit('storage-config-changed')
  // 自动执行刷新操作
  await handleRefresh()
}

const handleDatabaseConfigDeleted = () => {
  emit('copy-success', t('messages.databaseConfigDeleted'), 'info')
  // 通知父组件重新获取存储状态
  emit('storage-config-changed')
}



// 智能保存方法
const handleSave = async () => {
  if (isSaving.value) return

  isSaving.value = true
  try {
    if (props.isDatabaseAvailable) {
      // 双向存储模式：先保存到本地文件，然后执行双向同步
      emit('save')
      await nextTick() // 等待本地保存完成

      // 执行双向同步
      const result = await invoke('bidirectional_sync_tokens')
      emit('copy-success', t('messages.bidirectionalSyncSaveComplete'), 'success')
    } else {
      // 本地存储模式：只保存到本地文件
      emit('save')
    }
  } catch (error) {
    emit('copy-success', `${t('messages.saveFailed')}: ${error}`, 'error')
  } finally {
    isSaving.value = false
  }
}

// 导出Token到TXT文件（只导出Portal URL、租户URL和Token）
const exportTokens = () => {
  if (tokens.length === 0) {
    emit('copy-success', t('tokenList.noTokensToExport'), 'error')
    return
  }

  try {
    const exportData = tokens.map(token => {
      return `Portal URL: ${token.portalUrl || ''} | Tenant URL: ${token.tenantUrl || ''} | Token: ${token.accessToken || ''}`
    }).join('\n')

    const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19)
    const filename = `tokens_export_${timestamp}.txt`

    const blob = new Blob([exportData], { type: 'text/plain;charset=utf-8' })
    const url = URL.createObjectURL(blob)

    const link = document.createElement('a')
    link.href = url
    link.download = filename
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    URL.revokeObjectURL(url)

    emit('copy-success', t('tokenList.exportSuccess'), 'success')
  } catch (error) {
    emit('copy-success', `${t('tokenList.exportFailed')}: ${error.message}`, 'error')
  }
}

// 组件挂载时只在没有未保存更改时才刷新
onMounted(async () => {
  // 如果有未保存的更改，不要重新加载文件数据，避免覆盖内存中的新token
  if (!props.hasUnsavedChanges) {
    // emit('refresh', true) // 传递 true 表示显示成功消息
    await handleRefresh()
  }
})

// 暴露方法给父组件
defineExpose({
  // 目前没有需要暴露的方法
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
    justify-content: center;
  }

  .compact-actions {
    gap: 3px;
    padding: 3px;
    flex-wrap: wrap;
  }

  .header-title {
    margin-right: 0;
    margin-bottom: 8px;
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
  border: none;
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
}

.btn.secondary:hover {
  background: var(--color-border, #e5e7eb);
}

.btn.success {
  background: var(--color-success-bg, #10b981);
  color: var(--color-text-inverse, #ffffff);
  border: 1px solid var(--color-success-bg, #10b981);
}

.btn.success:hover:not(:disabled) {
  background: var(--color-success-bg-hover, #059669);
  border-color: var(--color-success-bg-hover, #059669);
}

.btn.success:disabled {
  background: var(--color-border-strong, #d1d5db);
  color: var(--color-text-soft, #9ca3af);
  border-color: var(--color-border-strong, #d1d5db);
  cursor: not-allowed;
}

.btn.info {
  background: var(--color-info-bg, #0ea5e9);
  color: var(--color-text-inverse, #ffffff);
}

.btn.info:hover:not(:disabled) {
  background: var(--color-info-bg-hover, #0284c7);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(14, 165, 233, 0.3);
}

.btn.export {
  background: var(--color-purple-bg, #8b5cf6);
  color: var(--color-text-inverse, #ffffff);
  border: 1px solid var(--color-purple-bg, #8b5cf6);
}

.btn.export:hover:not(:disabled) {
  background: var(--color-purple-bg-hover, #7c3aed);
  border-color: var(--color-purple-bg-hover, #7c3aed);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(139, 92, 246, 0.3);
}

.btn.export:disabled {
  background: var(--color-border-strong, #d1d5db);
  color: var(--color-text-soft, #9ca3af);
  border-color: var(--color-border-strong, #d1d5db);
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.btn.small {
  padding: 6px 12px;
  font-size: 12px;
  height: 32px;
}

.btn.compact {
  padding: 4px 8px;
  font-size: 11px;
  height: 28px;
  border-radius: 4px;
  font-weight: 500;
  min-width: auto;
}

.btn.compact svg {
  width: 12px;
  height: 12px;
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
  gap: 16px;
  flex: 1;
  margin-right: 8px;
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
  justify-content: flex-end;
  gap: 8px;
  flex-shrink: 0;
  flex-wrap: wrap;
}

.compact-actions {
  display: flex;
  align-items: center;
  gap: 4px;
  background: var(--color-surface-hover, #f8f9fa);
  border-radius: 8px;
  padding: 4px;
  border: 1px solid var(--color-border, #e5e7eb);
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


</style>

<template>
  <div class="sync-status-component">
    <div class="status-header">
      <h3>存储状态</h3>
      <button @click="refreshStatus" class="refresh-btn" :disabled="isRefreshing">
        <svg :class="{ spinning: isRefreshing }" width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
          <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
        </svg>
      </button>
    </div>

    <div class="status-content">
      <!-- 存储类型显示 -->
      <div class="storage-info">
        <div class="info-item">
          <span class="label">存储模式:</span>
          <span :class="['value', storageTypeClass]">{{ storageTypeText }}</span>
        </div>
        
        <div class="info-item" v-if="storageStatus">
          <span class="label">数据库状态:</span>
          <span :class="['status-badge', databaseStatusClass]">
            <span :class="['status-dot', databaseStatusClass]"></span>
            {{ databaseStatusText }}
          </span>
        </div>
      </div>

      <!-- 同步操作按钮 -->
      <div class="sync-actions" v-if="showSyncActions">
        <button
          @click="syncToDatabase"
          :class="['btn', 'primary', 'small', { loading: isSyncingToDb }]"
          :disabled="!canSync || isSyncingToDb"
        >
          <svg v-if="!isSyncingToDb" width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 4l-1.41 1.41L16.17 11H4v2h12.17l-5.58 5.59L12 20l8-8z"/>
          </svg>
          同步到数据库
        </button>

        <button
          @click="syncFromDatabase"
          :class="['btn', 'secondary', 'small', { loading: isSyncingFromDb }]"
          :disabled="!canSync || isSyncingFromDb"
        >
          <svg v-if="!isSyncingFromDb" width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
            <path d="M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20z"/>
          </svg>
          从数据库同步
        </button>

        <button
          @click="bidirectionalSync"
          :class="['btn', 'success', 'small', { loading: isBidirectionalSyncing }]"
          :disabled="!canSync || isBidirectionalSyncing"
        >
          <svg v-if="!isBidirectionalSyncing" width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6.99 11L3 15l3.99 4v-3H14v-2H6.99v-3zM21 9l-3.99-4v3H10v2h7.01v3L21 9z"/>
          </svg>
          双向同步
        </button>
      </div>

      <!-- 最后同步状态 -->
      <div class="last-sync-info" v-if="lastSyncStatus">
        <div class="sync-result">
          <span class="label">最后同步:</span>
          <span :class="['sync-status', lastSyncStatus.status]">
            {{ formatSyncStatus(lastSyncStatus) }}
          </span>
        </div>
        
        <div class="sync-details" v-if="lastSyncStatus.tokens_synced > 0">
          <span class="sync-count">{{ lastSyncStatus.tokens_synced }} 个Token</span>
          <span class="sync-direction">{{ formatSyncDirection(lastSyncStatus.sync_direction) }}</span>
        </div>

        <div class="sync-error" v-if="lastSyncStatus.error_message">
          <span class="error-text">{{ lastSyncStatus.error_message }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Emits
const emit = defineEmits(['show-status'])

// Reactive data
const storageStatus = ref(null)
const lastSyncStatus = ref(null)
const isRefreshing = ref(false)
const isSyncingToDb = ref(false)
const isSyncingFromDb = ref(false)
const isBidirectionalSyncing = ref(false)

// Computed properties
const storageTypeText = computed(() => {
  if (!storageStatus.value) return '加载中...'
  
  switch (storageStatus.value.storage_type) {
    case 'dual_storage':
      return '双重存储 (本地 + 数据库)'
    case 'local_only':
      return '仅本地存储'
    case 'postgresql':
      return '仅数据库存储'
    default:
      return '未知'
  }
})

const storageTypeClass = computed(() => {
  if (!storageStatus.value) return ''
  
  switch (storageStatus.value.storage_type) {
    case 'dual_storage':
      return 'dual'
    case 'local_only':
      return 'local'
    case 'postgresql':
      return 'database'
    default:
      return ''
  }
})

const databaseStatusText = computed(() => {
  if (!storageStatus.value) return '检查中...'
  return storageStatus.value.is_database_available ? '已连接' : '未连接'
})

const databaseStatusClass = computed(() => {
  if (!storageStatus.value) return 'checking'
  return storageStatus.value.is_database_available ? 'connected' : 'disconnected'
})

const showSyncActions = computed(() => {
  return storageStatus.value?.is_database_available
})

const canSync = computed(() => {
  return storageStatus.value?.is_database_available && 
         !isSyncingToDb.value && 
         !isSyncingFromDb.value && 
         !isBidirectionalSyncing.value
})

// Methods
const refreshStatus = async () => {
  isRefreshing.value = true
  try {
    const status = await invoke('get_storage_status')
    storageStatus.value = status

    // 同时获取同步状态
    try {
      const syncStatus = await invoke('get_sync_status')
      if (syncStatus) {
        lastSyncStatus.value = syncStatus
      }
    } catch (syncError) {
      console.error('Failed to get sync status:', syncError)
    }
  } catch (error) {
    console.error('Failed to get storage status:', error)
    emit('show-status', `获取存储状态失败: ${error}`, 'error')
  } finally {
    isRefreshing.value = false
  }
}

const syncToDatabase = async () => {
  isSyncingToDb.value = true
  try {
    const result = await invoke('sync_tokens_to_database')
    lastSyncStatus.value = result
    emit('show-status', '同步到数据库完成', 'success')
  } catch (error) {
    emit('show-status', `同步到数据库失败: ${error}`, 'error')
  } finally {
    isSyncingToDb.value = false
  }
}

const syncFromDatabase = async () => {
  isSyncingFromDb.value = true
  try {
    const result = await invoke('sync_tokens_from_database')
    lastSyncStatus.value = result
    emit('show-status', '从数据库同步完成', 'success')
  } catch (error) {
    emit('show-status', `从数据库同步失败: ${error}`, 'error')
  } finally {
    isSyncingFromDb.value = false
  }
}

const bidirectionalSync = async () => {
  isBidirectionalSyncing.value = true
  try {
    const result = await invoke('bidirectional_sync_tokens')
    lastSyncStatus.value = result
    emit('show-status', '双向同步完成', 'success')
  } catch (error) {
    emit('show-status', `双向同步失败: ${error}`, 'error')
  } finally {
    isBidirectionalSyncing.value = false
  }
}

const formatSyncStatus = (syncStatus) => {
  if (!syncStatus.last_sync_at) return '从未同步'
  
  const date = new Date(syncStatus.last_sync_at)
  const now = new Date()
  const diffMs = now - date
  const diffMins = Math.floor(diffMs / 60000)
  
  let timeText = ''
  if (diffMins < 1) {
    timeText = '刚刚'
  } else if (diffMins < 60) {
    timeText = `${diffMins}分钟前`
  } else if (diffMins < 1440) {
    timeText = `${Math.floor(diffMins / 60)}小时前`
  } else {
    timeText = date.toLocaleDateString()
  }
  
  const statusText = syncStatus.status === 'success' ? '成功' : '失败'
  return `${timeText} (${statusText})`
}

const formatSyncDirection = (direction) => {
  switch (direction) {
    case 'local_to_remote':
      return '本地 → 数据库'
    case 'remote_to_local':
      return '数据库 → 本地'
    case 'bidirectional':
      return '双向同步'
    default:
      return direction
  }
}

// Lifecycle
onMounted(() => {
  refreshStatus()
})
</script>

<style scoped>
.sync-status-component {
  background: white;
  border-radius: 8px;
  padding: 16px;
  border: 1px solid #e5e7eb;
}

.status-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.status-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #374151;
}

.refresh-btn {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  color: #6b7280;
  transition: all 0.2s;
}

.refresh-btn:hover:not(:disabled) {
  background: #f3f4f6;
  color: #374151;
}

.refresh-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.storage-info {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 16px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.label {
  font-size: 14px;
  color: #6b7280;
  font-weight: 500;
}

.value {
  font-size: 14px;
  font-weight: 600;
}

.value.dual {
  color: #059669;
}

.value.local {
  color: #d97706;
}

.value.database {
  color: #3b82f6;
}

.status-badge {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  font-weight: 500;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.status-dot.connected {
  background: #10b981;
}

.status-dot.disconnected {
  background: #ef4444;
}

.status-dot.checking {
  background: #f59e0b;
}

.status-badge.connected {
  color: #065f46;
}

.status-badge.disconnected {
  color: #991b1b;
}

.status-badge.checking {
  color: #92400e;
}

.sync-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 16px;
}

.btn {
  padding: 6px 12px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 500;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.btn.small {
  padding: 6px 10px;
  font-size: 11px;
}

.btn.primary {
  background: #3b82f6;
  color: white;
}

.btn.primary:hover:not(:disabled) {
  background: #2563eb;
}

.btn.secondary {
  background: #6b7280;
  color: white;
}

.btn.secondary:hover:not(:disabled) {
  background: #4b5563;
}

.btn.success {
  background: #10b981;
  color: white;
}

.btn.success:hover:not(:disabled) {
  background: #059669;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn.loading {
  opacity: 0.7;
  pointer-events: none;
}

.last-sync-info {
  border-top: 1px solid #f3f4f6;
  padding-top: 12px;
}

.sync-result {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.sync-status {
  font-size: 12px;
  font-weight: 500;
}

.sync-status.success {
  color: #059669;
}

.sync-status.failed {
  color: #dc2626;
}

.sync-details {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 11px;
  color: #6b7280;
  margin-bottom: 8px;
}

.sync-error {
  font-size: 11px;
  color: #dc2626;
  background: #fef2f2;
  padding: 6px 8px;
  border-radius: 4px;
  border: 1px solid #fecaca;
}

@media (max-width: 768px) {
  .sync-actions {
    flex-direction: column;
  }
  
  .btn {
    width: 100%;
    justify-content: center;
  }
  
  .info-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }
}
</style>

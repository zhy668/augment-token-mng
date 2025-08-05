<template>
  <div class="token-list-modal">
    <div class="modal-overlay" @click="$emit('close')">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>已保存Token</h2>
          <div class="header-actions">
            <button @click="$emit('add-token')" class="btn primary small">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
              </svg>
              添加
            </button>
            <button @click="handleRefresh" class="btn secondary small">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
              </svg>
              刷新
            </button>
            <button class="close-btn" @click="$emit('close')">×</button>
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
            <h3>还没有保存的Token</h3>
            <p>关闭此窗口，在主页面生成你的第一个Token</p>
          </div>

          <!-- Loading State -->
          <div v-if="isLoading" class="loading-state">
            <div class="spinner"></div>
            <p>正在加载Token...</p>
          </div>

          <!-- Token List -->
          <div v-if="tokens.length > 0" class="token-list">
            <div class="list-header">
              <h3>Token列表 ({{ tokens.length }})</h3>
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
              @copy-action="$emit('copy-action', $event)"
              @edit="$emit('edit', $event)"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, nextTick } from 'vue'
import TokenCard from './TokenCard.vue'

// Props
const props = defineProps({
  tokens: {
    type: Array,
    default: () => []
  },
  isLoading: {
    type: Boolean,
    default: false
  }
})

// Emits
const emit = defineEmits(['close', 'delete', 'copy-success', 'add-token', 'refresh', 'open-portal', 'copy-action', 'edit'])

// Token card refs for accessing child methods
const tokenCardRefs = ref({})

// 设置ref的函数
const setTokenCardRef = (el, tokenId) => {
  if (el) {
    tokenCardRefs.value[tokenId] = el
  }
}

// 刷新所有Portal信息
const refreshAllPortalInfo = async () => {
  await nextTick() // 确保DOM已更新

  Object.entries(tokenCardRefs.value).forEach(([tokenId, cardRef]) => {
    if (cardRef && typeof cardRef.refreshPortalInfo === 'function') {
      cardRef.refreshPortalInfo()
    }
  })
}

// 处理刷新事件
const handleRefresh = async () => {
  emit('refresh') // 先刷新token列表
  await nextTick() // 等待DOM更新
  refreshAllPortalInfo() // 立即刷新Portal信息
}

// 暴露方法给父组件
defineExpose({
  refreshAllPortalInfo
})
</script>

<style scoped>
.token-list-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 1000;
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
  background: white;
  border-radius: 12px;
  width: 100%;
  max-width: 900px;
  height: 85vh; /* 固定高度，更大 */
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
  margin-top: -60px; /* 向上移动到红框位置 */
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  border-bottom: 1px solid #e5e7eb;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.modal-header h2 {
  margin: 0;
  color: #1f2937;
  font-size: 1.5rem;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: #6b7280;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s;
}

.close-btn:hover {
  background: #f3f4f6;
  color: #374151;
}

.modal-body {
  padding: 24px;
  height: calc(85vh - 80px); /* 固定高度 */
  overflow-y: auto;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
}

.empty-icon {
  color: #d1d5db;
  margin-bottom: 16px;
}

.empty-state h3 {
  color: #374151;
  margin: 0 0 8px 0;
  font-size: 1.25rem;
}

.empty-state p {
  color: #6b7280;
  margin: 0;
}

.loading-state {
  text-align: center;
  padding: 40px 20px;
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid #e5e7eb;
  border-top: 3px solid #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 16px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.loading-state p {
  color: #6b7280;
  margin: 0;
}

.token-list {
  max-height: 600px;
  overflow-y: auto;
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
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid #e5e7eb;
}

.list-header h3 {
  margin: 0;
  color: #374151;
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
  gap: 6px;
}

.btn.secondary {
  background: #f3f4f6;
  color: #374151;
}

.btn.secondary:hover {
  background: #e5e7eb;
}

.btn.small {
  padding: 6px 12px;
  font-size: 12px;
}
</style>

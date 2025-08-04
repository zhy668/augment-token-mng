<template>
  <div class="token-card">
    <div class="card-main">
      <div class="token-info">
        <h3 class="tenant-name">{{ displayUrl }}</h3>
        <div class="token-meta">
          <span class="created-date">{{ formatDate(token.created_at) }}</span>
          <span :class="['remaining-days', remainingDaysClass]">
            {{ remainingDays }}天
          </span>
        </div>
      </div>

      <div class="actions">
        <button @click="copyToken" class="btn-action" title="复制Token">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
          </svg>
          Token
        </button>
        <button @click="copyUrl" class="btn-action" title="复制URL">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M3.9 12c0-1.71 1.39-3.1 3.1-3.1h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-1.9H7c-1.71 0-3.1-1.39-3.1-3.1zM8 13h8v-2H8v2zm9-6h-4v1.9h4c1.71 0 3.1 1.39 3.1 3.1s-1.39 3.1-3.1 3.1h-4V17h4c2.76 0 5-2.24 5-5s-2.24-5-5-5z"/>
          </svg>
          URL
        </button>
        <button @click="deleteToken" class="btn-action delete" title="删除Token">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'

// Props
const props = defineProps({
  token: {
    type: Object,
    required: true
  }
})

// Emits
const emit = defineEmits(['delete', 'copy-success'])

// Computed properties
const displayUrl = computed(() => {
  try {
    const url = new URL(props.token.tenant_url)
    return url.hostname
  } catch {
    return props.token.tenant_url
  }
})

const maskedToken = computed(() => {
  const token = props.token.access_token
  if (token.length <= 20) return token
  return token.substring(0, 10) + '...' + token.substring(token.length - 10)
})

const remainingDays = computed(() => {
  const createdAt = new Date(props.token.created_at)
  const expiryDate = new Date(createdAt.getTime() + 14 * 24 * 60 * 60 * 1000)
  const now = new Date()
  const diffTime = expiryDate - now
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
  return Math.max(0, diffDays)
})

const remainingDaysClass = computed(() => {
  const days = remainingDays.value
  if (days <= 1) return 'danger'
  if (days <= 3) return 'warning'
  return 'success'
})

// Methods
const formatDate = (dateString) => {
  const date = new Date(dateString)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
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

const copyToken = async () => {
  const success = await copyToClipboard(props.token.access_token)
  emit('copy-success', success ? 'Token已复制到剪贴板!' : '复制Token失败')
}

const copyUrl = async () => {
  const success = await copyToClipboard(props.token.tenant_url)
  emit('copy-success', success ? 'URL已复制到剪贴板!' : '复制URL失败')
}

const deleteToken = () => {
  if (confirm('确定要删除这个Token吗？')) {
    emit('delete', props.token.id)
  }
}
</script>

<style scoped>
.token-card {
  background: white;
  border: 1px solid #e1e5e9;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transition: all 0.2s ease;
  height: fit-content;
  min-height: 120px;
}

.token-card:hover {
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.15);
  border-color: #3b82f6;
  transform: translateY(-2px);
}

.card-main {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: 100%;
}

.token-info {
  flex: 1;
  min-width: 0;
}

.tenant-name {
  margin: 0 0 6px 0;
  font-size: 16px;
  font-weight: 600;
  color: #333;
  word-break: break-all;
  line-height: 1.3;
}

.token-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
}

.created-date {
  font-size: 12px;
  color: #666;
}

.remaining-days {
  font-size: 12px;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: 12px;
  background: #e9ecef;
  color: #495057;
}

.remaining-days.warning {
  background: #fff3cd;
  color: #856404;
}

.remaining-days.danger {
  background: #f8d7da;
  color: #721c24;
}

.remaining-days.success {
  background: #d4edda;
  color: #155724;
}

.actions {
  display: flex;
  flex-direction: row;
  gap: 8px;
  justify-content: flex-end;
  margin-top: auto;
}

.btn-action {
  background: #f8f9fa;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  padding: 8px 12px;
  cursor: pointer;
  color: #495057;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  font-weight: 500;
  flex: 1;
  justify-content: center;
  min-height: 36px;
}

.btn-action:hover {
  background: #e9ecef;
  border-color: #adb5bd;
  transform: translateY(-1px);
}

.btn-action.delete {
  color: #dc3545;
}

.btn-action.delete:hover {
  background: #f8d7da;
  border-color: #f5c6cb;
}

/* 响应式处理 */
@media (max-width: 768px) {
  .token-card {
    padding: 16px;
    min-height: 100px;
  }

  .card-main {
    gap: 12px;
  }

  .tenant-name {
    font-size: 14px;
  }

  .actions {
    gap: 6px;
  }

  .btn-action {
    padding: 6px 8px;
    font-size: 11px;
    min-height: 32px;
  }
}

@media (max-width: 480px) {
  .token-card {
    padding: 12px;
  }

  .actions {
    flex-direction: column;
    gap: 4px;
  }

  .btn-action {
    padding: 8px;
    font-size: 10px;
    min-height: 28px;
  }

  .token-meta {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }
}


</style>

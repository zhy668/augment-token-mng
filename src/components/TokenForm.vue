<template>
  <div class="token-form-modal">
    <div class="modal-overlay">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>{{ isEditing ? '编辑Token' : '添加Token' }}</h2>
          <button class="close-btn" @click="handleCancel">×</button>
        </div>
        
        <div class="modal-body">
          <form @submit.prevent="handleSubmit">
            <div class="form-group">
              <label for="tenantUrl">租户URL *</label>
              <input
                id="tenantUrl"
                v-model="formData.tenantUrl"
                type="url"
                placeholder="https://example.augmentcode.com/"
                required
                :disabled="isLoading"
              >
              <div v-if="errors.tenantUrl" class="error-message">{{ errors.tenantUrl }}</div>
            </div>

            <div class="form-group">
              <label for="accessToken">访问令牌 *</label>
              <textarea
                id="accessToken"
                v-model="formData.accessToken"
                placeholder="请输入访问令牌..."
                rows="3"
                required
                :disabled="isLoading"
              ></textarea>
              <div v-if="errors.accessToken" class="error-message">{{ errors.accessToken }}</div>
            </div>

            <div class="form-group">
              <label for="portalUrl">Portal URL (可选)</label>
              <input
                id="portalUrl"
                v-model="formData.portalUrl"
                type="url"
                placeholder="https://portal.withorb.com/view?token=xxx"
                :disabled="isLoading"
              >
              <div class="help-text">用于查看账户余额和过期时间</div>
              <div v-if="errors.portalUrl" class="error-message">{{ errors.portalUrl }}</div>
            </div>

            <div class="form-group">
              <label for="emailNote">邮箱备注 (可选)</label>
              <input
                id="emailNote"
                v-model="formData.emailNote"
                type="text"
                placeholder="请输入邮箱相关备注"
                :disabled="isLoading"
              >
              <div class="help-text">用于记录与此Token相关的邮箱信息</div>
              <div v-if="errors.emailNote" class="error-message">{{ errors.emailNote }}</div>
            </div>

            <div class="form-actions">
              <button type="button" @click="handleCancel" class="btn secondary" :disabled="isLoading">
                取消
              </button>
              <button type="submit" class="btn primary" :disabled="isLoading || !isFormValid">
                <span v-if="isLoading" class="loading-spinner"></span>
                {{ isLoading ? '保存中...' : (isEditing ? '更新' : '保存') }}
              </button>
            </div>
          </form>
        </div>


      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Props
const props = defineProps({
  token: {
    type: Object,
    default: null
  }
})

// Emits
const emit = defineEmits(['close', 'success', 'show-status', 'update-token', 'add-token'])

// Reactive data
const formData = ref({
  tenantUrl: '',
  accessToken: '',
  portalUrl: '',
  emailNote: ''
})

const errors = ref({
  tenantUrl: '',
  accessToken: '',
  portalUrl: '',
  emailNote: ''
})

const isLoading = ref(false)

// Computed properties
const isEditing = computed(() => !!props.token)

const isFormValid = computed(() => {
  return formData.value.tenantUrl.trim() &&
         formData.value.accessToken.trim() &&
         !errors.value.tenantUrl &&
         !errors.value.accessToken &&
         !errors.value.portalUrl &&
         !errors.value.emailNote
})

// Watch for token prop changes (for editing)
watch(() => props.token, (newToken) => {
  if (newToken) {
    formData.value = {
      tenantUrl: newToken.tenant_url || '',
      accessToken: newToken.access_token || '',
      portalUrl: newToken.portal_url || '',
      emailNote: newToken.email_note || ''
    }
  } else {
    // Reset form for adding new token
    formData.value = {
      tenantUrl: '',
      accessToken: '',
      portalUrl: '',
      emailNote: ''
    }
  }
  // Clear errors when token changes
  errors.value = {
    tenantUrl: '',
    accessToken: '',
    portalUrl: '',
    emailNote: ''
  }
}, { immediate: true })

// Methods
const showStatus = (message, type = 'info') => {
  emit('show-status', message, type)
}

const validateForm = () => {
  errors.value = {
    tenantUrl: '',
    accessToken: '',
    portalUrl: ''
  }

  // Validate tenant URL
  if (!formData.value.tenantUrl.trim()) {
    errors.value.tenantUrl = '租户URL不能为空'
  } else {
    try {
      new URL(formData.value.tenantUrl)
    } catch {
      errors.value.tenantUrl = '请输入有效的URL格式'
    }
  }

  // Validate access token
  if (!formData.value.accessToken.trim()) {
    errors.value.accessToken = '访问令牌不能为空'
  }

  // Validate portal URL (optional)
  if (formData.value.portalUrl && formData.value.portalUrl.trim()) {
    try {
      new URL(formData.value.portalUrl)
    } catch {
      errors.value.portalUrl = '请输入有效的URL格式'
    }
  }

  return !errors.value.tenantUrl && !errors.value.accessToken && !errors.value.portalUrl
}

const handleSubmit = async () => {
  if (!validateForm()) {
    return
  }

  isLoading.value = true
  showStatus(isEditing.value ? '正在更新Token...' : '正在保存Token...', 'info')

  try {
    const tokenData = {
      tenantUrl: formData.value.tenantUrl.trim(),
      accessToken: formData.value.accessToken.trim(),
      portalUrl: formData.value.portalUrl ? formData.value.portalUrl.trim() || null : null,
      emailNote: formData.value.emailNote ? formData.value.emailNote.trim() || null : null
    }

    if (isEditing.value) {
      // Update existing token - 通知父组件更新内存中的数据
      emit('update-token', {
        id: props.token.id,
        ...tokenData
      })
      showStatus('Token已更新到内存，请手动保存', 'success')
    } else {
      // Add new token - 通知父组件添加到内存中的数据
      emit('add-token', tokenData)
      showStatus('Token已添加到内存，请手动保存', 'success')
    }

    emit('success')
    emit('close')
  } catch (error) {
    showStatus(`${isEditing.value ? '更新' : '保存'}Token失败: ${error}`, 'error')
  } finally {
    isLoading.value = false
  }
}

const handleCancel = () => {
  emit('close')
}
</script>

<style scoped>
.token-form-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 1100;
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
  max-width: 500px;
  max-height: 98vh;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
  background: var(--color-surface-muted, #f8f9fa);
}

.modal-header h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-heading, #333);
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  color: var(--color-text-muted, #666);
  cursor: pointer;
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
  background: var(--color-surface-muted, #e9ecef);
  color: var(--color-text-heading, #333);
}

.modal-body {
  padding: 24px;
  max-height: calc(98vh - 120px);
  overflow-y: auto;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-weight: 500;
  color: var(--color-text-heading, #333);
  font-size: 14px;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--color-btn-secondary-border, #ddd);
  border-radius: 6px;
  font-size: 14px;
  transition: border-color 0.2s;
  box-sizing: border-box;
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: var(--color-accent, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.form-group input:disabled,
.form-group textarea:disabled {
  background: var(--color-btn-tertiary-bg, #f5f5f5);
  color: var(--color-text-muted, #666);
  cursor: not-allowed;
}

.form-group textarea {
  resize: vertical;
  min-height: 80px;
}

.help-text {
  font-size: 12px;
  color: var(--color-text-muted, #666);
  margin-top: 4px;
}

.error-message {
  color: var(--color-danger-bg, #dc3545);
  font-size: 12px;
  margin-top: 4px;
}

.form-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  margin-top: 24px;
  padding-top: 20px;
  border-top: 1px solid var(--color-divider, #e1e5e9);
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 80px;
  justify-content: center;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn.primary {
  background: var(--color-accent, #3b82f6);
  color: var(--color-text-inverse, #ffffff);
}

.btn.primary:hover:not(:disabled) {
  background: var(--color-accent-hover, #2563eb);
}

.btn.secondary {
  background: var(--color-surface-muted, #f8f9fa);
  color: var(--color-text-secondary, #495057);
  border: 1px solid var(--color-border-strong, #dee2e6);
}

.btn.secondary:hover:not(:disabled) {
  background: var(--color-surface-muted, #e9ecef);
}

.loading-spinner {
  width: 14px;
  height: 14px;
  border: 2px solid transparent;
  border-top: 2px solid currentColor;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}


</style>

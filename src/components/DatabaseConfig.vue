<template>
  <div class="database-config-modal">
    <div class="modal-overlay">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>数据库配置</h2>
          <button class="close-btn" @click="$emit('close')">×</button>
        </div>
        
        <div class="modal-body">
          <div class="config-form">
            <div class="form-group">
              <label for="host">主机地址:</label>
              <input
                id="host"
                v-model="config.host"
                type="text"
                placeholder="localhost"
                :disabled="isLoading"
              >
            </div>

            <div class="form-group">
              <label for="port">端口:</label>
              <input
                id="port"
                v-model.number="config.port"
                type="number"
                placeholder="5432"
                :disabled="isLoading"
              >
            </div>

            <div class="form-group">
              <label for="database">数据库名:</label>
              <input
                id="database"
                v-model="config.database"
                type="text"
                placeholder="augment_tokens"
                :disabled="isLoading"
              >
            </div>

            <div class="form-group">
              <label for="username">用户名:</label>
              <input
                id="username"
                v-model="config.username"
                type="text"
                placeholder="postgres"
                :disabled="isLoading"
              >
            </div>

            <div class="form-group">
              <label for="password">密码:</label>
              <input
                id="password"
                v-model="config.password"
                type="password"
                placeholder="请输入数据库密码"
                :disabled="isLoading"
              >
            </div>

            <div class="form-group">
              <label for="sslMode">SSL模式:</label>
              <select
                id="sslMode"
                v-model="config.sslMode"
                :disabled="isLoading"
              >
                <option value="require">Require (强制SSL)</option>
                <option value="disable">Disable (禁用SSL)</option>
              </select>
            </div>

          </div>
        </div>

        <div class="modal-footer">
          <button
            @click="testConnection"
            :class="['btn', 'secondary', { loading: isTesting }]"
            :disabled="!canTest || isTesting"
          >
            <svg v-if="!isTesting" width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
            </svg>
            测试连接
          </button>
          
          <button
            @click="saveConfig"
            :class="['btn', 'primary', { loading: isSaving }]"
            :disabled="!canSave || isSaving"
          >
            <svg v-if="!isSaving" width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z"/>
            </svg>
            保存配置
          </button>

          <button
            v-if="hasExistingConfig"
            @click="deleteConfig"
            :class="['btn', 'danger', { loading: isDeleting }]"
            :disabled="isDeleting"
          >
            <svg v-if="!isDeleting" width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
            </svg>
            删除配置
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Props
const props = defineProps({
  initialConfig: {
    type: Object,
    default: () => ({})
  }
})

// Emits
const emit = defineEmits(['close', 'config-saved', 'config-deleted', 'show-status'])

// Reactive data
const config = ref({
  host: 'localhost',
  port: 5432,
  database: 'augment_tokens',
  username: 'postgres',
  password: '',
  sslMode: 'require',
  enabled: true
})

const isLoading = ref(false)
const isTesting = ref(false)
const isSaving = ref(false)
const isDeleting = ref(false)
const hasExistingConfig = ref(false)
const isConnectionTested = ref(false)

// Computed properties
const canTest = computed(() => {
  return config.value.host && 
         config.value.port && 
         config.value.database && 
         config.value.username && 
         config.value.password
})

const canSave = computed(() => {
  return canTest.value && isConnectionTested.value
})

// Methods
const loadConfig = async () => {
  isLoading.value = true
  try {
    const loadedConfig = await invoke('load_database_config')
    if (loadedConfig && loadedConfig.enabled) {
      config.value = {
        host: loadedConfig.host || 'localhost',
        port: loadedConfig.port || 5432,
        database: loadedConfig.database || 'augment_tokens',
        username: loadedConfig.username || 'postgres',
        password: '', // 不显示已保存的密码
        sslMode: loadedConfig.ssl_mode || 'require',
        enabled: loadedConfig.enabled || false
      }
      hasExistingConfig.value = true
    }
  } catch (error) {
    console.error('Failed to load database config:', error)
  } finally {
    isLoading.value = false
  }
}

const testConnection = async () => {
  isTesting.value = true
  isConnectionTested.value = false

  try {
    await invoke('test_database_connection', {
      host: config.value.host,
      port: config.value.port,
      database: config.value.database,
      username: config.value.username,
      password: config.value.password,
      ssl_mode: config.value.sslMode
    })

    // 连接成功时发送toast通知
    emit('show-status', '数据库连接测试成功！', 'success')
    isConnectionTested.value = true
  } catch (error) {
    // 连接失败时发送toast通知
    emit('show-status', `数据库连接测试失败: ${error}`, 'error')
    isConnectionTested.value = false
  } finally {
    isTesting.value = false
  }
}

const saveConfig = async () => {
  isSaving.value = true
  
  try {
    await invoke('save_database_config', {
      host: config.value.host,
      port: config.value.port,
      database: config.value.database,
      username: config.value.username,
      password: config.value.password,
      ssl_mode: config.value.sslMode
    })
    
    emit('show-status', '数据库配置保存成功！', 'success')
    emit('config-saved')
    emit('close')
  } catch (error) {
    emit('show-status', `保存配置失败: ${error}`, 'error')
  } finally {
    isSaving.value = false
  }
}

const deleteConfig = async () => {
  if (!confirm('确定要删除数据库配置吗？这将禁用数据库存储功能。')) {
    return
  }
  
  isDeleting.value = true
  
  try {
    await invoke('delete_database_config')
    
    emit('show-status', '数据库配置已删除', 'success')
    emit('config-deleted')
    emit('close')
  } catch (error) {
    emit('show-status', `删除配置失败: ${error}`, 'error')
  } finally {
    isDeleting.value = false
  }
}

// Watch for config changes to reset connection test status
watch(config, () => {
  isConnectionTested.value = false
}, { deep: true })

// Lifecycle
onMounted(() => {
  loadConfig()
})
</script>



<style scoped>
.database-config-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
}

.modal-content {
  background: var(--color-surface, #ffffff);
  border-radius: 12px;
  width: 90%;
  max-width: 500px;
  max-height: 90vh;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-border, #e5e7eb);
  background: var(--color-surface-alt, #f9fafb);
  border-radius: 12px 12px 0 0;
  flex-shrink: 0;
}

.modal-header h2 {
  margin: 0;
  color: var(--color-text-primary, #374151);
  font-size: 18px;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--color-text-muted, #6b7280);
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
  background: var(--color-border, #e5e7eb);
  color: var(--color-text-primary, #374151);
}

.modal-body {
  padding: 24px;
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.config-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-group label {
  font-weight: 500;
  color: var(--color-text-primary, #374151);
  font-size: 14px;
}

.form-group input {
  padding: 10px 12px;
  border: 1px solid var(--color-border-strong, #d1d5db);
  border-radius: 6px;
  font-size: 14px;
  transition: border-color 0.2s ease;
}

.form-group input:focus {
  outline: none;
  border-color: var(--color-accent, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.form-group input:disabled {
  background-color: var(--color-surface-alt, #f9fafb);
  color: var(--color-text-muted, #6b7280);
  cursor: not-allowed;
}

.form-group select {
  padding: 10px 12px;
  border: 1px solid var(--color-border-strong, #d1d5db);
  border-radius: 6px;
  font-size: 14px;
  transition: border-color 0.2s ease;
  background-color: var(--color-surface, #ffffff);
}

.form-group select:focus {
  outline: none;
  border-color: var(--color-accent, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.form-group select:disabled {
  background-color: var(--color-surface-alt, #f9fafb);
  color: var(--color-text-muted, #6b7280);
  cursor: not-allowed;
}



.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 20px 24px;
  border-top: 1px solid var(--color-border, #e5e7eb);
  background: var(--color-surface-alt, #f9fafb);
  border-radius: 0 0 12px 12px;
  flex-shrink: 0;
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
  background: var(--color-accent, #3b82f6);
  color: var(--color-text-inverse, #ffffff);
}

.btn.primary:hover:not(:disabled) {
  background: var(--color-accent-hover, #2563eb);
}

.btn.secondary {
  background: var(--color-text-muted, #6b7280);
  color: var(--color-text-inverse, #ffffff);
}

.btn.secondary:hover:not(:disabled) {
  background: var(--color-text-secondary, #4b5563);
}

.btn.danger {
  background: var(--color-danger-bg, #dc2626);
  color: var(--color-text-inverse, #ffffff);
}

.btn.danger:hover:not(:disabled) {
  background: var(--color-danger-bg-hover, #b91c1c);
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn.loading {
  opacity: 0.7;
  pointer-events: none;
}

@media (max-width: 768px) {
  .modal-content {
    width: 95%;
    margin: 20px;
  }
  
  .modal-footer {
    flex-direction: column;
  }
  
  .btn {
    width: 100%;
    justify-content: center;
  }
}
</style>

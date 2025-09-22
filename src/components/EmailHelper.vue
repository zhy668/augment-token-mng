<template>
  <div class="modal-overlay">
    <div class="modal-content email-helper" @click.stop>
      <div class="modal-header">
        <h3>{{ $t('emailHelper.title') }}</h3>
        <div class="header-actions">
          <button @click="showSettings = true" class="btn secondary small">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <circle cx="12" cy="12" r="3"/>
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
            </svg>
            {{ $t('emailHelper.settings') }}
          </button>
          <button @click="$emit('close')" class="close-btn">×</button>
        </div>
      </div>

      <div class="modal-body">
        <!-- 创建邮箱区域 -->
        <div class="create-section">
          <div v-if="!config.token" class="config-notice">
            <span class="notice-icon">⚠️</span>
            {{ $t('emailHelper.tokenRequired') }}
          </div>

          <!-- 创建邮箱表单 -->
          <div v-if="config.token" class="create-form">
            <div class="create-form-row">
              <div class="form-group type-group">
                <label>{{ $t('emailHelper.emailType') }}:</label>
                <select v-model="createType" class="form-select">
                  <option value="random">{{ $t('emailHelper.randomEmail') }}</option>
                  <option value="custom">{{ $t('emailHelper.customEmail') }}</option>
                </select>
              </div>

              <div v-if="createType === 'custom'" class="form-group custom-name-group">
                <label>{{ $t('emailHelper.customEmailName') }}:</label>
                <input
                  v-model="customEmailName"
                  type="text"
                  class="form-input"
                  :placeholder="$t('emailHelper.customEmailPlaceholder')"
                >
              </div>

              <div class="form-group create-btn-group">
                <label class="invisible-label">{{ $t('emailHelper.createEmailBtn') }}:</label>
                <button
                  @click="createEmail"
                  :disabled="!canCreateEmail || isCreating"
                  :class="['btn', 'primary', { loading: isCreating }]"
                >
                  {{ isCreating ? $t('emailHelper.creating') : $t('emailHelper.createEmailBtn') }}
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- 邮箱列表区域 -->
        <div class="emails-section">
          <div class="section-header">
            <h4>{{ $t('emailHelper.emailList') }} ({{ emails.length }})</h4>
            <div class="list-actions">
              <button
                v-if="isMonitoring"
                @click="stopMonitoring"
                class="btn warning small"
              >
                {{ $t('emailHelper.stopMonitor') }}
              </button>
            </div>
          </div>

          <div v-if="emails.length === 0" class="empty-state">
            <p>{{ $t('emailHelper.emptyState') }}</p>
            <p class="empty-hint">{{ $t('emailHelper.emptyDescription') }}</p>
          </div>

          <div v-else class="emails-grid">
            <div
              v-for="emailInfo in sortedEmails"
              :key="emailInfo.email"
              class="email-card"
            >
              <div class="email-info">
                <div class="email-header">
                  <div class="email-address">{{ emailInfo.email }}</div>
                  <!-- 按钮放在邮箱名旁边 -->
                  <div class="email-actions-inline">
                    <button
                      @click="copyEmail(emailInfo.email)"
                      class="btn secondary small"
                    >
                      {{ $t('emailHelper.copyEmail') }}
                    </button>
                    <button
                      @click="copyPassword(emailInfo.password)"
                      class="btn secondary small"
                    >
                      {{ $t('emailHelper.copyPassword') }}
                    </button>
                    <button
                      @click="startMonitoring(emailInfo.email, emailInfo.password)"
                      :disabled="isMonitoring && monitoringEmail !== emailInfo.email"
                      :class="['btn', monitoringEmail === emailInfo.email ? 'warning' : 'primary', 'small']"
                    >
                      {{ monitoringEmail === emailInfo.email ? $t('emailHelper.monitoring') : $t('emailHelper.startMonitor') }}
                    </button>
                    <button
                      @click="viewEmails(emailInfo.email, emailInfo.password)"
                      class="btn info small"
                    >
                      {{ $t('emailHelper.viewEmails') }}
                    </button>
                    <button
                      @click="deleteEmail(emailInfo.email)"
                      class="btn danger small"
                    >
                      {{ $t('emailHelper.delete') }}
                    </button>
                  </div>
                </div>
                <div class="email-meta">
                  <div class="email-password">{{ $t('emailHelper.password') }}: {{ emailInfo.password }}</div>
                  <div class="email-created">{{ formatDate(emailInfo.created_at) }}</div>
                </div>
                <!-- 验证码显示在邮箱名密码下面 -->
                <div v-if="emailInfo.verificationCodes && emailInfo.verificationCodes.length > 0" class="verification-codes-section">
                  <div
                    v-for="(code, index) in emailInfo.verificationCodes"
                    :key="index"
                    class="verification-code-item"
                  >
                    <span class="code-value">{{ code.code }}</span>
                    <button @click="copyCode(code.code)" class="btn-copy-code" :title="$t('emailHelper.copyCode')">
                      📋
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 设置模态框 -->
  <div v-if="showSettings" class="settings-overlay" @click="showSettings = false">
    <div class="settings-content" @click.stop>
      <div class="settings-header">
        <h3>{{ $t('emailHelper.settingsTitle') }}</h3>
        <button @click="showSettings = false" class="close-btn">×</button>
      </div>
      
      <div class="settings-body">
        <div class="form-group">
          <label>{{ $t('emailHelper.serverUrl') }}:</label>
          <input
            v-model="config.serverUrl"
            type="text"
            class="form-input"
            :placeholder="$t('emailHelper.serverUrlPlaceholder')"
          >
        </div>
        
        <div class="form-group">
          <label>{{ $t('emailHelper.adminEmail') }}:</label>
          <input
            v-model="config.adminEmail"
            type="text"
            class="form-input"
            :placeholder="$t('emailHelper.adminEmailPlaceholder')"
          >
        </div>
        
        <div class="form-group">
          <label>{{ $t('emailHelper.adminPassword') }}:</label>
          <input
            v-model="config.adminPassword"
            type="password"
            class="form-input"
            :placeholder="$t('emailHelper.adminPasswordPlaceholder')"
          >
        </div>
        
        <div class="form-group">
          <label>{{ $t('emailHelper.emailDomain') }}:</label>
          <div class="domain-input-group">
            <select v-model="selectedDomain" @change="onDomainChange" class="form-select domain-select">
              <option value="">{{ $t('emailHelper.selectDomain') }}</option>
              <option v-for="domain in domainOptions" :key="domain" :value="domain">{{ domain }}</option>
              <option value="custom">{{ $t('emailHelper.customDomain') }}</option>
            </select>
            <input
              v-if="selectedDomain === 'custom'"
              v-model="config.emailDomain"
              type="text"
              class="form-input domain-input"
              :placeholder="$t('emailHelper.emailDomainPlaceholder')"
            >
          </div>
        </div>

        <div class="settings-actions">
          <button @click="saveSettingsWithToken" :disabled="!canGetToken || isGettingToken" class="btn primary">
            {{ isGettingToken ? $t('emailHelper.gettingToken') : $t('emailHelper.saveSettings') }}
          </button>
          <button @click="exportEmails" :disabled="emails.length === 0" class="btn secondary">
            {{ $t('emailHelper.exportEmails') }}
          </button>
          <button @click="clearAllEmails" :disabled="emails.length === 0" class="btn danger">
            {{ $t('emailHelper.clearAll') }}
          </button>
        </div>

        <div v-if="config.token" class="token-display">
          <label>{{ $t('emailHelper.currentToken') }}:</label>
          <div class="token-value">{{ (config.token || '').substring(0, 50) }}...</div>
        </div>
      </div>
    </div>
  </div>

  <!-- 邮件查看模态框 -->
  <div v-if="showEmailViewer" class="email-viewer-overlay" @click="closeEmailViewer">
    <div class="email-viewer-content" @click.stop>
      <div class="email-viewer-header">
        <h3>{{ $t('emailHelper.emailList') }} - {{ currentViewingEmail }}</h3>
        <button @click="closeEmailViewer" class="close-btn">×</button>
      </div>

      <div class="email-viewer-body">
        <div v-if="isLoadingEmails" class="loading-state">
          {{ $t('emailHelper.loadingEmails') }}
        </div>

        <div v-else-if="emailList.length === 0" class="empty-state">
          {{ $t('emailHelper.noEmails') }}
        </div>

        <div v-else class="email-list">
          <div
            v-for="email in emailList"
            :key="email.id"
            class="email-item"
            @click="viewEmailContent(email)"
          >
            <div class="email-item-header">
              <div class="email-from">{{ email.from }}</div>
              <div class="email-time">{{ formatEmailTime(email.date) }}</div>
            </div>
            <div class="email-subject">{{ email.subject }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 邮件内容查看模态框 -->
  <div v-if="showEmailContent" class="email-content-overlay" @click="closeEmailContent">
    <div class="email-content-modal" @click.stop>
      <div class="email-content-header">
        <h3>{{ $t('emailHelper.emailContent') }}</h3>
        <button @click="closeEmailContent" class="close-btn">×</button>
      </div>

      <div class="email-content-body">
        <div v-if="isLoadingEmailContent" class="loading-state">
          {{ $t('emailHelper.loadingEmailContent') }}
        </div>

        <div v-else-if="currentEmailContent">
          <div class="email-meta-info">
            <div><strong>{{ $t('emailHelper.from') }}:</strong> {{ currentEmailContent.from }}</div>
            <div><strong>{{ $t('emailHelper.subject') }}:</strong> {{ currentEmailContent.subject }}</div>
            <div><strong>{{ $t('emailHelper.date') }}:</strong> {{ formatEmailTime(currentEmailContent.date) }}</div>
          </div>

          <div class="email-content-html" v-html="currentEmailContent.content"></div>
        </div>
      </div>

      <div class="email-content-footer">
        <button @click="closeEmailContent" class="btn secondary">
          {{ $t('emailHelper.back') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { useI18n } from 'vue-i18n'

const emit = defineEmits(['close', 'show-status'])

// i18n
const { t } = useI18n()

// 响应式数据
const emails = ref([])
const config = ref({
  serverUrl: 'https://wyattzheng.eu.org',
  adminEmail: '',
  adminPassword: '',
  token: '',
  emailDomain: '@wyatt.x10.mx'
})

// 邮箱域名选项
const domainOptions = ref([
  '@siemens.eu.org',
  '@wuka.eu.org',
  '@siemensapp.tech',
  '@wyatt.x10.mx',
  '@augment.elementfx.com',
  '@siemens.elementfx.com'
])

const isCreating = ref(false)
const isGettingToken = ref(false)
const isMonitoring = ref(false)
const monitoringEmail = ref('')
const verificationCode = ref('')
const showSettings = ref(false)
const monitorInterval = ref(null)
const monitorStartTime = ref(null) // 监控开始时间

// 新增的响应式变量
const createType = ref('random') // 'random' 或 'custom'
const customEmailName = ref('')

// 邮件查看相关状态
const showEmailViewer = ref(false)
const showEmailContent = ref(false)
const currentViewingEmail = ref('')
const emailList = ref([])
const currentEmailContent = ref(null)
const isLoadingEmails = ref(false)
const isLoadingEmailContent = ref(false)
const selectedDomain = ref('@wyatt.x10.mx')

// 计算属性
const canCreateEmail = computed(() => {
  if (!config.value.token || !config.value.emailDomain) return false
  if (createType.value === 'custom') {
    return customEmailName.value.trim().length > 0
  }
  return true
})

const canGetToken = computed(() => {
  return config.value.serverUrl && config.value.adminEmail && config.value.adminPassword
})

// 按创建时间倒序排列的邮箱列表（最新的在前面）
const sortedEmails = computed(() => {
  return [...emails.value].sort((a, b) => {
    const timeA = new Date(a.created_at).getTime()
    const timeB = new Date(b.created_at).getTime()
    return timeB - timeA // 倒序排列
  })
})

// 方法
const showStatus = (message, type = 'info') => {
  emit('show-status', message, type)
}

// 生成邮箱
const generateEmail = () => {
  // 确保域名格式正确（去掉开头的@，然后添加@）
  const domain = config.value.emailDomain.startsWith('@')
    ? config.value.emailDomain.substring(1)
    : config.value.emailDomain

  if (createType.value === 'custom') {
    return `${customEmailName.value.trim()}@${domain}`
  } else {
    const chars = 'abcdefghijklmnopqrstuvwxyz0123456789'
    let result = ''
    for (let i = 0; i < 8; i++) {
      result += chars.charAt(Math.floor(Math.random() * chars.length))
    }
    return `augment${result}@${domain}`
  }
}

// 生成随机密码
const generateRandomPassword = () => {
  const chars = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'
  let result = ''
  for (let i = 0; i < 11; i++) {
    result += chars.charAt(Math.floor(Math.random() * chars.length))
  }
  return result
}

// 格式化日期
const formatDate = (dateString) => {
  const date = new Date(dateString)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

// 格式化时间
const formatTime = (timestamp) => {
  const date = new Date(timestamp)
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

// 复制密码
const copyPassword = (password) => {
  navigator.clipboard.writeText(password).then(() => {
    showStatus(t('emailHelper.passwordCopied'), 'success')
  }).catch(() => {
    showStatus(t('emailHelper.copyFailed'), 'error')
  })
}

// 复制验证码
const copyCode = (code) => {
  navigator.clipboard.writeText(code).then(() => {
    showStatus(t('emailHelper.codeCopied'), 'success')
  }).catch(() => {
    showStatus(t('emailHelper.copyFailed'), 'error')
  })
}

// 域名选择变化
const onDomainChange = () => {
  if (selectedDomain.value && selectedDomain.value !== 'custom') {
    config.value.emailDomain = selectedDomain.value
  }
}

// 复制到剪贴板
const copyToClipboard = async (text) => {
  try {
    await navigator.clipboard.writeText(text)
    return true
  } catch (error) {
    console.error('Failed to copy to clipboard:', error)
    return false
  }
}

// 复制邮箱地址
const copyEmail = async (email) => {
  const success = await copyToClipboard(email)
  showStatus(
    success ? t('emailHelper.emailCopied') : t('emailHelper.copyFailed'),
    success ? 'success' : 'error'
  )
}



// 保存邮箱列表到本地存储
const saveEmailsList = () => {
  localStorage.setItem('emailHelper_emails', JSON.stringify(emails.value))
}

// 从本地存储加载邮箱列表
const loadEmailsList = () => {
  try {
    const saved = localStorage.getItem('emailHelper_emails')
    if (saved) {
      emails.value = JSON.parse(saved)
    }
  } catch (error) {
    console.error('Failed to load emails list:', error)
    emails.value = []
  }
}

// 保存配置到本地存储
const saveConfig = () => {
  localStorage.setItem('emailHelper_config', JSON.stringify(config.value))
}

// 从本地存储加载配置
const loadConfig = () => {
  try {
    const saved = localStorage.getItem('emailHelper_config')
    if (saved) {
      const savedConfig = JSON.parse(saved)
      // 确保所有必要的字段都存在
      config.value = {
        serverUrl: savedConfig.serverUrl || 'https://wyattzheng.eu.org',
        adminEmail: savedConfig.adminEmail || '',
        adminPassword: savedConfig.adminPassword || '',
        token: savedConfig.token || '',
        emailDomain: savedConfig.emailDomain || '@wyatt.x10.mx'
      }
    }
  } catch (error) {
    // 如果配置损坏，重置为默认配置
    localStorage.removeItem('emailHelper_config')
    config.value = {
      serverUrl: 'https://wyattzheng.eu.org',
      adminEmail: '',
      adminPassword: '',
      token: '',
      emailDomain: '@wyatt.x10.mx'
    }
  }
}

// 获取Token
const getToken = async () => {
  isGettingToken.value = true
  try {
    const response = await fetch(`${config.value.serverUrl}/api/public/genToken`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        email: config.value.adminEmail,
        password: config.value.adminPassword
      })
    })

    const result = await response.json()
    if (result.code === 200) {
      config.value.token = result.data.token
      saveConfig()
      showStatus(t('emailHelper.tokenSuccess'), 'success')
      return result.data.token
    } else {
      throw new Error(result.message || 'Login failed')
    }
  } catch (error) {
    showStatus(`${t('emailHelper.tokenFailed')}: ${error.message}`, 'error')
    throw error
  } finally {
    isGettingToken.value = false
  }
}

// 通用认证请求函数 - 自动处理token失效和重新获取
const makeAuthenticatedRequest = async (url, options = {}) => {
  // 检查是否有token
  if (!config.value.token) {
    throw new Error(t('emailHelper.tokenRequired'))
  }

  // 设置默认headers
  const headers = {
    'Content-Type': 'application/json',
    'Authorization': config.value.token,
    ...options.headers
  }

  // 第一次尝试请求
  try {
    const response = await fetch(url, {
      ...options,
      headers
    })

    const result = await response.json()

    // 检查是否是token失效错误
    if (isTokenExpiredError(response, result)) {
      console.log('🔄 Token失效，尝试重新获取token...')

      // 重新获取token
      try {
        await getToken()

        // 使用新token重试请求
        const retryHeaders = {
          ...headers,
          'Authorization': config.value.token
        }

        const retryResponse = await fetch(url, {
          ...options,
          headers: retryHeaders
        })

        const retryResult = await retryResponse.json()

        // 如果重试后仍然失败，抛出错误
        if (isTokenExpiredError(retryResponse, retryResult)) {
          throw new Error('Token重新获取后仍然失效')
        }

        return { response: retryResponse, result: retryResult }
      } catch (tokenError) {
        console.error('❌ 重新获取token失败:', tokenError)
        throw new Error(`重新获取token失败: ${tokenError.message}`)
      }
    }

    return { response, result }
  } catch (error) {
    // 网络错误或其他异常
    if (error.name === 'TypeError' && error.message.includes('fetch')) {
      throw new Error('网络连接失败，请检查网络设置')
    }
    throw error
  }
}

// 检查是否是token失效错误
const isTokenExpiredError = (response, result) => {
  // 检查HTTP状态码
  if (response.status === 401) {
    return true
  }

  // 检查响应结果中的错误码和消息
  if (result && (
    result.code === 401 ||
    result.code === 403 ||
    (result.message && (
      result.message.includes('token') ||
      result.message.includes('unauthorized') ||
      result.message.includes('expired') ||
      result.message.includes('invalid')
    ))
  )) {
    return true
  }

  return false
}

// 创建邮箱
const createEmail = async () => {
  isCreating.value = true
  try {
    const email = generateEmail()
    const password = generateRandomPassword()

    showStatus(`${t('emailHelper.creatingEmail')}: ${email}`, 'info')

    const userData = {
      list: [{
        email: email,
        password: password,
        roleName: ''
      }]
    }

    const { result } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/public/addUser`, {
      method: 'POST',
      body: JSON.stringify(userData)
    })

    if (result.code === 200) {
      const emailInfo = {
        email: email,
        password: password,
        created_at: new Date().toISOString(),
        verificationCodes: [] // 添加验证码数组
      }

      emails.value.push(emailInfo)
      saveEmailsList()
      showStatus(t('emailHelper.createSuccess'), 'success')

      // 清空自定义邮箱名称
      if (createType.value === 'custom') {
        customEmailName.value = ''
      }

      // 自动开始监控新创建的邮箱
      startMonitoring(email, password)
    } else {
      throw new Error(result.message || 'Create email failed')
    }
  } catch (error) {
    showStatus(`${t('emailHelper.createFailed')}: ${error.message}`, 'error')
  } finally {
    isCreating.value = false
  }
}

// 删除邮箱
const deleteEmail = async (email) => {
  try {
    showStatus(t('emailHelper.deletingEmail'), 'info')

    // 如果有Token，先尝试删除云端邮箱
    if (config.value.token) {
      try {
        const { result } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/user/delete?email=${encodeURIComponent(email)}`, {
          method: 'DELETE'
        })

        if (result.code !== 200) {
          // 云端删除失败，询问用户是否继续删除本地记录
          const confirmDelete = confirm(`${t('emailHelper.cloudDeleteFailed')}: ${result.message}\n\n${t('emailHelper.confirmDeleteLocal')}`)
          if (!confirmDelete) {
            showStatus(t('emailHelper.deleteAborted'), 'info')
            return
          }
        }
      } catch (error) {
        // 网络错误或其他异常，询问用户是否继续删除本地记录
        const confirmDelete = confirm(`${t('emailHelper.cloudDeleteError')}: ${error.message}\n\n${t('emailHelper.confirmDeleteLocal')}`)
        if (!confirmDelete) {
          showStatus(t('emailHelper.deleteAborted'), 'info')
          return
        }
      }
    }

    // 云端删除成功或用户确认删除本地记录，从本地列表中移除
    const originalLength = emails.value.length
    emails.value = emails.value.filter(emailInfo => emailInfo.email !== email)

    if (emails.value.length < originalLength) {
      saveEmailsList()

      // 如果正在监控这个邮箱，停止监控
      if (monitoringEmail.value === email) {
        stopMonitoring()
      }

      if (config.value.token) {
        showStatus(t('emailHelper.deleteSuccess'), 'success')
      } else {
        showStatus(t('emailHelper.deleteLocalSuccess'), 'success')
      }
    } else {
      showStatus(t('emailHelper.emailNotFound'), 'warning')
    }
  } catch (error) {
    showStatus(`${t('emailHelper.deleteFailed')}: ${error.message}`, 'error')
  }
}

// 清空所有邮箱
const clearAllEmails = async () => {
  if (!confirm(t('emailHelper.confirmClearAll'))) {
    return
  }

  try {
    const totalEmails = emails.value.length

    // 停止监控
    stopMonitoring()

    let cloudDeleteErrors = []

    // 如果有Token，批量删除云端用户
    if (config.value.token) {
      showStatus(`${t('emailHelper.deletingEmails')}: ${totalEmails}`, 'info')

      for (const emailInfo of emails.value) {
        try {
          const { result } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/user/delete?email=${encodeURIComponent(emailInfo.email)}`, {
            method: 'DELETE'
          })

          if (result.code !== 200) {
            cloudDeleteErrors.push(`${emailInfo.email}: ${result.message}`)
            console.error('Failed to delete email from server:', emailInfo.email, result.message)
          }
        } catch (error) {
          cloudDeleteErrors.push(`${emailInfo.email}: ${error.message}`)
          console.error('Failed to delete email from server:', emailInfo.email, error)
        }
      }

      // 如果有云端删除失败的邮箱，询问用户是否继续
      if (cloudDeleteErrors.length > 0) {
        const errorMessage = `${t('emailHelper.someCloudDeletesFailed')}:\n${cloudDeleteErrors.slice(0, 3).join('\n')}${cloudDeleteErrors.length > 3 ? `\n...${t('emailHelper.andMore', { count: cloudDeleteErrors.length - 3 })}` : ''}\n\n${t('emailHelper.confirmClearLocal')}`

        if (!confirm(errorMessage)) {
          showStatus(t('emailHelper.clearAllAborted'), 'info')
          return
        }
      }
    }

    // 清空本地列表
    emails.value = []
    saveEmailsList()

    if (cloudDeleteErrors.length > 0) {
      showStatus(`${t('emailHelper.clearAllPartialSuccess')} (${cloudDeleteErrors.length}/${totalEmails} ${t('emailHelper.cloudDeletesFailed')})`, 'warning')
    } else {
      showStatus(t('emailHelper.clearAllSuccess'), 'success')
    }
  } catch (error) {
    showStatus(`${t('emailHelper.clearAllFailed')}: ${error.message}`, 'error')
  }
}

// 开始监控验证码
const startMonitoring = (email, password) => {
  // 停止当前监控
  stopMonitoring()

  monitoringEmail.value = email
  isMonitoring.value = true
  verificationCode.value = ''

  // 记录监控开始时间 (UTC时间)
  monitorStartTime.value = new Date()
  console.log('⏰ 监控开始时间设置为:', monitorStartTime.value)
  console.log('⏰ 监控开始时间 UTC:', monitorStartTime.value.toISOString())
  console.log('⏰ 监控开始时间 本地:', monitorStartTime.value.toLocaleString())

  // 清除该邮箱之前的验证码（开始新的监控时）
  const emailIndex = emails.value.findIndex(e => e.email === email)
  if (emailIndex !== -1) {
    if (!emails.value[emailIndex].verificationCodes) {
      emails.value[emailIndex].verificationCodes = []
    }
    // 清空旧的验证码，开始新的监控
    emails.value[emailIndex].verificationCodes = []
    saveEmailsList()
    console.log('🗑️ 已清除邮箱的旧验证码，开始新的监控')
  }

  showStatus(`${t('emailHelper.startMonitoringEmail')}: ${email}`, 'info')

  // 开始定时检查
  monitorInterval.value = setInterval(async () => {
    await checkForVerificationCode(email, password)
  }, 5000) // 每5秒检查一次
}

// 停止监控
const stopMonitoring = () => {
  if (monitorInterval.value) {
    clearInterval(monitorInterval.value)
    monitorInterval.value = null
  }

  isMonitoring.value = false
  monitoringEmail.value = ''
  verificationCode.value = ''
  monitorStartTime.value = null // 清除监控开始时间
}

// 检查验证码
const checkForVerificationCode = async (email, password) => {
  try {
    const { result } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/public/emailList`, {
      method: 'POST',
      body: JSON.stringify({
        toEmail: email,
        fromEmail: 'support@augmentcode.com',
        num: 1,
        size: 10, // 增加查询数量，确保能获取到最新邮件
        timeSort: 'desc'
      })
    })

    if (result.code === 200 && result.data && result.data.length > 0) {
      console.log('📧 收到邮件数据:', result.data.length, '封邮件')

      // 过滤出监控开始时间之后的邮件
      const filteredEmails = result.data.filter(emailData => {
        if (!monitorStartTime.value) {
          console.log('⏰ 没有监控开始时间，不过滤邮件')
          return true // 如果没有开始时间，则不过滤
        }

        // 根据API文档，邮件时间字段是 createTime，格式为 "2099-12-30 23:99:99" (UTC时间)
        if (!emailData.createTime) {
          console.log('⚠️ 邮件没有createTime字段，不过滤此邮件')
          return true
        }

        // 解析邮件时间 (UTC时间) - 需要添加 'Z' 后缀表示UTC时间
        const emailTimeStr = emailData.createTime.replace(' ', 'T') + 'Z'
        const emailTime = new Date(emailTimeStr)

        // 如果无法解析邮件时间，则不过滤（保持原有行为）
        if (isNaN(emailTime.getTime())) {
          console.log('⚠️ 无法解析邮件时间:', emailData.createTime, '转换后:', emailTimeStr, '不过滤此邮件')
          return true
        }

        const shouldInclude = emailTime >= monitorStartTime.value
        console.log(`⏰ 邮件时间: ${emailTime} (原始: ${emailData.createTime}), 监控开始时间: ${monitorStartTime.value}, 是否包含: ${shouldInclude}`)
        return shouldInclude
      })

      console.log('📧 过滤后邮件数量:', filteredEmails.length)

      // 查找来自 Augment 的邮件
      const augmentEmail = filteredEmails.find(emailData =>
        emailData.sendEmail === 'support@augmentcode.com' &&
        emailData.subject.includes('Welcome to Augment Code')
      )

      if (augmentEmail) {
        const code = extractVerificationCode(augmentEmail.content)
        if (code) {
          console.log('🎉 找到验证码:', code)

          // 找到对应的邮箱对象并添加验证码
          const emailIndex = emails.value.findIndex(e => e.email === email)
          if (emailIndex !== -1) {
            if (!emails.value[emailIndex].verificationCodes) {
              emails.value[emailIndex].verificationCodes = []
            }

            // 检查是否已经存在相同的验证码（避免重复添加）
            const existingCode = emails.value[emailIndex].verificationCodes.find(c => c.code === code)
            if (!existingCode) {
              // 添加新的验证码到列表
              emails.value[emailIndex].verificationCodes.push({
                code: code,
                time: Date.now(),
                emailTime: augmentEmail.createTime // 保存邮件的原始时间
              })
              saveEmailsList() // 保存到本地存储
              console.log('✅ 验证码已添加到邮箱列表')
            } else {
              console.log('⚠️ 验证码已存在，跳过添加')
            }
          }

          verificationCode.value = code
          showStatus(`${t('emailHelper.codeFound')}: ${code}`, 'success')
          stopMonitoring() // 找到验证码后停止监控
          return
        } else {
          console.log('⚠️ 未能从邮件内容中提取验证码')
        }
      } else {
        console.log('📧 未找到来自 Augment 的验证码邮件')
      }
    }
  } catch (error) {
    console.error('Failed to check verification code:', error)
  }
}

// 提取验证码
const extractVerificationCode = (htmlContent) => {
  if (!htmlContent) {
    return null
  }

  const patterns = [
    /Your verification code is:\s*<b>(\d{6})<\/b>/i,
    /Your verification code is:\s*(\d{6})/i,
    /verification code is:\s*<b>(\d{6})<\/b>/i,
    /verification code is:\s*(\d{6})/i,
    /<b>(\d{6})<\/b>/i,
    /(\d{6})/g
  ]

  for (let i = 0; i < patterns.length; i++) {
    const pattern = patterns[i]
    const match = htmlContent.match(pattern)
    if (match && match[1] && match[1].length === 6) {
      return match[1]
    }
  }

  return null
}

// 打开设置
const openSettings = () => {
  try {
    showSettings.value = true
  } catch (error) {
    console.error('Failed to open settings:', error)
  }
}

// 保存设置
const saveSettings = () => {
  saveConfig()
  showStatus(t('emailHelper.settingsSaved'), 'success')
}

// 保存设置并获取Token（邮箱服务Token）
const saveSettingsWithToken = async () => {
  isGettingToken.value = true
  try {
    // 先获取邮箱服务Token
    const response = await fetch(`${config.value.serverUrl}/api/public/genToken`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        email: config.value.adminEmail,
        password: config.value.adminPassword
      })
    })

    const result = await response.json()
    if (result.code === 200) {
      config.value.token = result.data.token
      // 保存配置
      saveConfig()
      showStatus(t('emailHelper.tokenSuccess'), 'success')
    } else {
      throw new Error(result.message || 'Get token failed')
    }
  } catch (error) {
    showStatus(`${t('emailHelper.tokenFailed')}: ${error.message}`, 'error')
  } finally {
    isGettingToken.value = false
  }
}

// 邮件查看相关函数
const viewEmails = async (email, password) => {
  // 检查是否有token
  if (!config.value.token) {
    showStatus(t('emailHelper.tokenRequired'), 'error')
    return
  }

  currentViewingEmail.value = email
  showEmailViewer.value = true
  isLoadingEmails.value = true
  emailList.value = []

  try {
    const { result } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/public/emailList`, {
      method: 'POST',
      body: JSON.stringify({
        toEmail: email,
        fromEmail: '', // 获取所有发件人的邮件
        num: 1,
        size: 50, // 获取更多邮件
        timeSort: 'desc'
      })
    })

    if (result.code === 200 && result.data && result.data.length > 0) {
      console.log('📧 收到邮件数据:', result.data.length, '封邮件')
      console.log('📧 第一封邮件数据结构:', result.data[0])

      // 转换邮件数据格式，使其与模板兼容
      emailList.value = result.data.map(emailData => ({
        id: emailData.id || emailData.messageId || Date.now() + Math.random(), // 确保有ID
        from: emailData.sendEmail || emailData.from || '未知发件人',
        subject: emailData.subject || '无主题',
        date: emailData.createTime || emailData.date || new Date().toISOString(),
        content: emailData.content || emailData.html || emailData.text || ''
      })).sort((a, b) => new Date(b.date) - new Date(a.date))
    } else {
      emailList.value = []
      if (result.code !== 200) {
        showStatus(`${t('emailHelper.loadEmailsFailed')}: ${result.message || 'Unknown error'}`, 'error')
      }
    }
  } catch (error) {
    showStatus(`${t('emailHelper.loadEmailsFailed')}: ${error.message}`, 'error')
  } finally {
    isLoadingEmails.value = false
  }
}

const viewEmailContent = async (email) => {
  showEmailContent.value = true
  isLoadingEmailContent.value = true
  currentEmailContent.value = null

  try {
    // 如果邮件已经包含内容，直接使用
    if (email.content) {
      currentEmailContent.value = {
        from: email.from,
        subject: email.subject,
        date: email.date,
        content: email.content
      }
      isLoadingEmailContent.value = false
      return
    }

    // 如果没有内容，尝试通过API获取
    const { result } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/public/emailContent`, {
      method: 'POST',
      body: JSON.stringify({
        toEmail: currentViewingEmail.value,
        messageId: email.id
      })
    })
    if (result.code === 200 && result.data) {
      currentEmailContent.value = {
        from: email.from,
        subject: email.subject,
        date: email.date,
        content: result.data.content || result.data.html || result.data.text || email.content || '邮件内容为空'
      }
    } else {
      // 如果API调用失败，使用邮件列表中的内容
      currentEmailContent.value = {
        from: email.from,
        subject: email.subject,
        date: email.date,
        content: email.content || '无法加载邮件内容'
      }
    }
  } catch (error) {
    // 如果出现错误，使用邮件列表中的内容
    currentEmailContent.value = {
      from: email.from,
      subject: email.subject,
      date: email.date,
      content: email.content || `加载邮件内容失败: ${error.message}`
    }
  } finally {
    isLoadingEmailContent.value = false
  }
}

const closeEmailViewer = () => {
  showEmailViewer.value = false
  currentViewingEmail.value = ''
  emailList.value = []
}

const closeEmailContent = () => {
  showEmailContent.value = false
  currentEmailContent.value = null
}

const formatEmailTime = (dateString) => {
  if (!dateString) return '未知时间'

  try {
    // 处理邮件服务器返回的时间格式 "2099-12-30 23:99:99"
    let date
    if (typeof dateString === 'string' && dateString.includes(' ') && !dateString.includes('T')) {
      // 转换为ISO格式
      const isoString = dateString.replace(' ', 'T') + 'Z'
      date = new Date(isoString)
    } else {
      date = new Date(dateString)
    }

    // 检查日期是否有效
    if (isNaN(date.getTime())) {
      return dateString // 如果无法解析，返回原始字符串
    }

    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit'
    })
  } catch (error) {
    return dateString || '时间解析错误'
  }
}

// 导出邮箱列表
const exportEmails = () => {
  if (emails.value.length === 0) {
    showStatus(t('emailHelper.noEmailsToExport'), 'warning')
    return
  }

  // 生成导出内容
  let exportContent = ''
  emails.value.forEach((emailInfo) => {
    exportContent += `${emailInfo.email}  |  ${emailInfo.password}\n`
  })

  // 创建下载链接
  const blob = new Blob([exportContent], { type: 'text/plain;charset=utf-8' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = `augment_emails_${new Date().toISOString().split('T')[0]}.txt`
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  URL.revokeObjectURL(url)

  showStatus(t('emailHelper.exportSuccess'), 'success')
}

// 生命周期
onMounted(() => {
  loadConfig()
  loadEmailsList()

  // 初始化域名选择器
  if (config.value.emailDomain && domainOptions.value.includes(config.value.emailDomain)) {
    selectedDomain.value = config.value.emailDomain
  } else if (config.value.emailDomain) {
    selectedDomain.value = 'custom'
  }
})

onBeforeUnmount(() => {
  if (monitorInterval.value) {
    clearInterval(monitorInterval.value)
  }
})
</script>

<style scoped>
/* 基础模态框样式 */
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
  z-index: 2000;
}

.modal-content {
  background: var(--color-surface, #ffffff);
  border-radius: 12px;
  max-width: 90vw;
  max-height: 95vh;
  overflow-y: auto;
  position: relative;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
}

.email-helper {
  width: 95vw;
  max-width: 1000px;
  max-height: 90vh;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-border, #e5e7eb);
  background: var(--color-surface-alt, #f9fafb);
  border-radius: 12px 12px 0 0;
}

.modal-header h3 {
  margin: 0;
  color: var(--color-text-primary, #374151);
  font-size: 18px;
  font-weight: 600;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
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
  overflow-y: auto;
}

/* 创建邮箱区域 */
.create-section {
  background: var(--color-surface-muted, #f8f9fa);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-header h4 {
  margin: 0;
  color: var(--color-text-heading, #333);
  font-size: 16px;
  font-weight: 600;
}

.config-notice {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: var(--color-warning-surface, #fef3c7);
  border: 1px solid var(--color-warning-bg, #f59e0b);
  border-radius: 6px;
  font-size: 13px;
  color: var(--color-warning-text, #92400e);
}

.notice-icon {
  font-size: 16px;
  flex-shrink: 0;
}

/* 邮箱列表区域 */
.emails-section h4 {
  margin: 0;
  color: var(--color-text-heading, #333);
  font-size: 16px;
  font-weight: 600;
}

.list-actions {
  display: flex;
  gap: 8px;
}

.emails-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.email-item {
  background: var(--color-surface, #ffffff);
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 8px;
  padding: 16px;
  transition: all 0.2s ease;
}

.email-item:hover {
  border-color: var(--color-border-strong, #d1d5db);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.email-info {
  margin-bottom: 8px;
}

.email-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  margin-bottom: 4px;
  flex-wrap: wrap;
}

.email-address {
  font-weight: 500;
  color: var(--color-text-primary, #374151);
  font-size: 14px;
  flex: 1;
  min-width: 200px;
}

.email-actions-inline {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}

.verification-codes-section {
  margin-top: 8px;
  padding: 8px;
  background: var(--color-surface-muted, #f8f9fa);
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 4px;
}

.verification-code-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  background: #e8f5e8;
  border: 1px solid #28a745;
  border-radius: 4px;
  margin-bottom: 4px;
}

.verification-code-item:last-child {
  margin-bottom: 0;
}

.verification-code-item .code-value {
  font-family: 'Courier New', monospace;
  font-weight: bold;
  color: #28a745;
  letter-spacing: 1px;
  font-size: 14px;
  flex: 1;
}

.btn-copy-code {
  background: none;
  border: none;
  cursor: pointer;
  padding: 1px;
  border-radius: 2px;
  font-size: 10px;
  opacity: 0.7;
  transition: opacity 0.2s;
  line-height: 1;
}

.btn-copy-code:hover {
  opacity: 1;
  background: rgba(40, 167, 69, 0.1);
}

.email-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  margin-top: 4px;
}

.email-password {
  font-size: 11px;
  color: var(--color-text-muted, #6b7280);
  background: var(--color-surface-hover, #f3f4f6);
  padding: 2px 6px;
  border-radius: 3px;
  font-family: 'Courier New', monospace;
}

.email-created {
  font-size: 10px;
  color: var(--color-text-muted, #6b7280);
  background: var(--color-surface-hover, #f3f4f6);
  padding: 2px 6px;
  border-radius: 6px;
}

/* 验证码显示区�?*/
.verification-display {
  margin: 12px 0;
  padding: 16px;
  background: var(--color-success-surface, #f0f9ff);
  border: 2px solid var(--color-success-border, #28a745);
  border-radius: 8px;
  text-align: center;
}

.verification-code {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.code-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-success-text, #155724);
}

.code-value {
  font-size: 24px;
  font-weight: bold;
  color: var(--color-success-text, #28a745);
  font-family: 'Courier New', monospace;
  letter-spacing: 2px;
  padding: 8px 16px;
  background: var(--color-surface, #ffffff);
  border-radius: 6px;
  border: 1px solid var(--color-success-border, #c3e6cb);
}

.email-actions {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

/* 按钮样式 */
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
  gap: 4px;
  text-decoration: none;
  height: 32px;
  box-sizing: border-box;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

.btn.primary {
  background: var(--color-accent, #3b82f6);
  color: var(--color-text-inverse, #ffffff);
}

.btn.primary:hover:not(:disabled) {
  background: var(--color-accent-hover, #2563eb);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(59, 130, 246, 0.3);
}

.btn.secondary {
  background: var(--color-text-muted, #6b7280);
  color: var(--color-text-inverse, #ffffff);
}

.btn.secondary:hover:not(:disabled) {
  background: var(--color-text-secondary, #4b5563);
}

.btn.success {
  background: var(--color-success-bg, #10b981);
  color: var(--color-text-inverse, #ffffff);
}

.btn.success:hover:not(:disabled) {
  background: var(--color-success-bg-hover, #059669);
}

.btn.warning {
  background: var(--color-warning-bg, #f59e0b);
  color: var(--color-text-inverse, #ffffff);
}

.btn.warning:hover:not(:disabled) {
  background: var(--color-warning-bg-hover, #d97706);
}

.btn.danger {
  background: var(--color-danger-bg, #dc2626);
  color: var(--color-text-inverse, #ffffff);
}

.btn.danger:hover:not(:disabled) {
  background: var(--color-danger-bg-hover, #b91c1c);
}

.btn.small {
  padding: 6px 12px;
  font-size: 12px;
}

.btn.loading {
  opacity: 0.7;
  cursor: wait;
}

/* 空状态样�?*/
.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: var(--color-text-muted, #6b7280);
}

.empty-hint {
  font-size: 12px;
  color: var(--color-text-soft, #9ca3af);
  margin-top: 8px;
}

/* 设置模态框样式 */
.settings-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  z-index: 10001;
  display: flex;
  justify-content: center;
  align-items: center;
}

.settings-content {
  background: var(--color-surface, #ffffff);
  border-radius: 8px;
  padding: 16px;
  width: 700px;
  max-width: 90%;
  box-shadow: 0 4px 20px rgba(0,0,0,0.3);
  max-height: 80vh;
  overflow-y: auto;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--color-border, #e5e7eb);
}

.settings-header h3 {
  margin: 0;
  color: var(--color-text-primary, #374151);
  font-size: 18px;
  font-weight: 600;
}

.settings-body {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-group label {
  font-weight: 500;
  color: var(--color-text-primary, #374151);
  font-size: 14px;
}

.form-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--color-border-strong, #d1d5db);
  border-radius: 6px;
  font-size: 14px;
  transition: border-color 0.2s ease;
  box-sizing: border-box;
  height: 38px;
}

.form-input:focus {
  outline: none;
  border-color: var(--color-accent, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.settings-actions {
  display: flex;
  gap: 12px;
  margin-top: 20px;
  flex-wrap: nowrap;
  justify-content: flex-start;
}

.token-display {
  margin-top: 16px;
  padding: 12px;
  background: var(--color-surface-muted, #f8f9fa);
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 6px;
}

.token-display label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: var(--color-text-primary, #374151);
  font-size: 14px;
}

.token-value {
  font-family: 'Courier New', monospace;
  font-size: 12px;
  word-break: break-all;
  color: var(--color-text-muted, #6b7280);
  background: var(--color-surface, #ffffff);
  padding: 8px;
  border-radius: 4px;
  border: 1px solid var(--color-border, #e5e7eb);
}

/* 创建邮箱表单样式 */
.create-form {
  margin-top: 16px;
}

.create-form-row {
  display: flex;
  gap: 16px;
  align-items: center;
  flex-wrap: wrap;
}

.type-group {
  min-width: 150px;
  flex-shrink: 0;
}

.custom-name-group {
  min-width: 200px;
  flex: 1;
}

.create-btn-group {
  flex-shrink: 0;
}

.invisible-label {
  visibility: hidden;
  height: 20px;
  margin-bottom: 6px;
  display: block;
}

.form-row {
  display: flex;
  gap: 12px;
  margin-bottom: 12px;
  align-items: center;
}

.form-select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--color-border-strong, #d1d5db);
  border-radius: 6px;
  font-size: 14px;
  background: var(--color-surface, #ffffff);
  transition: border-color 0.2s ease;
  box-sizing: border-box;
  height: 38px;
}

.form-select:focus {
  outline: none;
  border-color: var(--color-accent, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

/* 域名输入组样�?*/
.domain-input-group {
  display: flex;
  gap: 8px;
  align-items: center;
}

.domain-select {
  flex: 1;
  min-width: 200px;
}

.domain-input {
  flex: 1;
  min-width: 150px;
}

/* 邮箱列表布局 */
.emails-grid {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-top: 16px;
}

.email-card {
  background: var(--color-surface, #ffffff);
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 6px;
  padding: 12px;
  transition: box-shadow 0.2s ease;
  min-height: auto;
}

.email-card:hover {
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1);
}

/* 验证码显示样�?*/
.verification-display {
  margin-top: 12px;
  padding: 12px;
  background: var(--color-surface-muted, #f8f9fa);
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 6px;
}

.verification-title {
  font-weight: 500;
  color: var(--color-text-primary, #374151);
  margin-bottom: 8px;
  font-size: 14px;
}

.verification-code {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  background: var(--color-surface, #ffffff);
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 4px;
  margin-bottom: 8px;
}

.verification-code:last-child {
  margin-bottom: 0;
}

.code-value {
  font-family: 'Courier New', monospace;
  font-size: 16px;
  font-weight: bold;
  color: var(--color-accent, #3b82f6);
  flex: 1;
}

.code-time {
  font-size: 12px;
  color: var(--color-text-muted, #6b7280);
  min-width: 80px;
}

/* 响应式设�?*/
@media (max-width: 768px) {
  .email-helper {
    width: 95vw;
    max-width: none;
  }

  .modal-body {
    padding: 16px;
  }

  .section-header {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }

  .email-meta {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }

  .email-actions {
    justify-content: space-between;
  }

  .settings-content {
    width: 90%;
    padding: 16px;
  }

  .settings-actions {
    flex-wrap: wrap;
    gap: 8px;
  }

  /* 邮箱列表在小屏幕上保持单列布局 */

  .create-form-row {
    flex-direction: column;
    align-items: stretch;
  }

  .form-row {
    flex-direction: column;
    align-items: stretch;
  }

  .domain-input-group {
    flex-direction: column;
  }

  .domain-select,
  .domain-input {
    min-width: auto;
  }
}

@media (max-width: 480px) {
  .email-actions {
    flex-direction: column;
  }

  .btn {
    justify-content: center;
  }
}

/* 邮件查看模态框样式 */
.email-viewer-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  z-index: 10002;
  display: flex;
  justify-content: center;
  align-items: center;
}

.email-viewer-content {
  background: var(--color-bg, #ffffff);
  border-radius: 8px;
  width: 90%;
  max-width: 800px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
}

.email-viewer-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--color-border, #e5e7eb);
}

.email-viewer-header h3 {
  margin: 0;
  color: var(--color-text, #1f2937);
  font-size: 18px;
}

.email-viewer-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.email-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.email-item {
  padding: 12px;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.email-item:hover {
  background: var(--color-bg-soft, #f9fafb);
  border-color: var(--color-primary, #3b82f6);
}

.email-item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.email-from {
  font-weight: 500;
  color: var(--color-text, #1f2937);
}

.email-time {
  font-size: 12px;
  color: var(--color-text-soft, #9ca3af);
}

.email-subject {
  color: var(--color-text-soft, #6b7280);
  font-size: 14px;
}

/* 邮件内容模态框样式 */
.email-content-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  z-index: 10003;
  display: flex;
  justify-content: center;
  align-items: center;
}

.email-content-modal {
  background: var(--color-bg, #ffffff);
  border-radius: 8px;
  width: 95%;
  max-width: 900px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
}

.email-content-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--color-border, #e5e7eb);
}

.email-content-header h3 {
  margin: 0;
  color: var(--color-text, #1f2937);
  font-size: 18px;
}

.email-content-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.email-meta-info {
  background: var(--color-bg-soft, #f9fafb);
  padding: 12px;
  border-radius: 6px;
  margin-bottom: 16px;
  border: 1px solid var(--color-border, #e5e7eb);
}

.email-meta-info div {
  margin-bottom: 4px;
  font-size: 14px;
}

.email-meta-info div:last-child {
  margin-bottom: 0;
}

.email-content-html {
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 6px;
  padding: 16px;
  background: var(--color-bg, #ffffff);
  min-height: 200px;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  line-height: 1.6;
}

.email-content-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--color-border, #e5e7eb);
  display: flex;
  justify-content: flex-end;
}

.loading-state, .empty-state {
  text-align: center;
  padding: 40px 20px;
  color: var(--color-text-soft, #9ca3af);
  font-size: 14px;
}
</style>



<template>
  <div class="token-card">
    <!-- 状态指示器 -->
    <div v-if="(token.portal_url && portalInfo.data) || token.ban_status" class="status-indicator">
      <!-- 账号状态优先显示 -->
      <span v-if="token.ban_status" :class="['status-badge', token.ban_status === 'SUSPENDED' ? 'banned' : 'active']">
        {{ token.ban_status === 'SUSPENDED' ? '已封禁' : '正常' }}
      </span>
      <!-- Portal状态作为备选 -->
      <span v-else-if="token.portal_url && portalInfo.data" :class="['status-badge', portalInfo.data.is_active ? 'active' : 'inactive']">
        {{ portalInfo.data.is_active ? '正常' : '失效' }}
      </span>
    </div>

    <div class="card-main">
      <div class="token-info">
        <h3 class="tenant-name">{{ displayUrl }}</h3>
        <div class="token-meta">
          <span class="created-date">{{ formatDate(token.created_at) }}</span>
          <!-- Portal信息直接显示在meta中 -->
          <template v-if="token.portal_url">
            <!-- 优先显示Portal数据，无论是来自本地缓存还是网络请求 -->
            <template v-if="portalInfo.data">
              <span class="portal-meta expiry">过期: {{ formatExpiryDate(portalInfo.data.expiry_date) }}</span>
              <span class="portal-meta balance">剩余: {{ portalInfo.data.credits_balance }}</span>
            </template>
            <!-- 如果没有数据且正在加载，显示加载状态 -->
            <span v-else-if="isLoadingPortalInfo" class="portal-meta loading">加载中...</span>
            <!-- 不显示错误信息，静默处理所有错误 -->
          </template>
        </div>
      </div>

      <div class="actions">
        <button @click="copyToken" class="btn-action" title="复制Token">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
          </svg>
        </button>
        <button @click="copyTenantUrl" class="btn-action" title="复制租户URL">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M3.9 12c0-1.71 1.39-3.1 3.1-3.1h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-1.9H7c-1.71 0-3.1-1.39-3.1-3.1zM8 13h8v-2H8v2zm9-6h-4v1.9h4c1.71 0 3.1 1.39 3.1 3.1s-1.39 3.1-3.1 3.1h-4V17h4c2.76 0 5-2.24 5-5s-2.24-5-5-5z"/>
          </svg>
        </button>
        <button @click="checkAccountStatus" :class="['btn-action', 'status-check', { loading: isCheckingStatus }]" :disabled="isCheckingStatus" title="检测账号状态">
          <svg v-if="!isCheckingStatus" width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
          </svg>
          <div v-else class="loading-spinner"></div>
        </button>
        <button v-if="token.portal_url" @click="$emit('open-portal', token)" class="btn-action portal" title="打开Portal">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
          </svg>
        </button>
        <button @click="$emit('edit', token)" class="btn-action edit" title="编辑Token">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
          </svg>
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
import { computed, ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Props
const props = defineProps({
  token: {
    type: Object,
    required: true
  }
})

// Emits
const emit = defineEmits(['delete', 'copy-success', 'open-portal', 'edit'])

// Reactive data
const isLoadingPortalInfo = ref(false)
const portalInfo = ref({ data: null, error: null })
const isCheckingStatus = ref(false)

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





const deleteToken = () => {
  if (confirm('确定要删除这个Token吗？')) {
    emit('delete', props.token.id)
  }
}

// 复制到剪贴板的通用方法
const copyToClipboard = async (text) => {
  try {
    await navigator.clipboard.writeText(text)
    return true
  } catch (error) {
    // 备用方案
    const textArea = document.createElement('textarea')
    textArea.value = text
    document.body.appendChild(textArea)
    textArea.select()
    document.execCommand('copy')
    document.body.removeChild(textArea)
    return true
  }
}

// 复制Token
const copyToken = async () => {
  const success = await copyToClipboard(props.token.access_token)
  if (success) {
    emit('copy-success', 'Token已复制到剪贴板!', 'success')
  } else {
    emit('copy-success', '复制Token失败', 'error')
  }
}

// 复制租户URL
const copyTenantUrl = async () => {
  const success = await copyToClipboard(props.token.tenant_url)
  if (success) {
    emit('copy-success', '租户URL已复制到剪贴板!', 'success')
  } else {
    emit('copy-success', '复制租户URL失败', 'error')
  }
}



const extractTokenFromPortalUrl = (portalUrl) => {
  try {
    const url = new URL(portalUrl)
    return url.searchParams.get('token')
  } catch {
    return null
  }
}

const loadPortalInfo = async (forceRefresh = false) => {
  console.log('loadPortalInfo called with forceRefresh:', forceRefresh)
  console.log('token.portal_url:', props.token.portal_url)
  console.log('token.portal_info:', props.token.portal_info)

  if (!props.token.portal_url) {
    console.log('No portal_url, returning')
    return
  }

  const token = extractTokenFromPortalUrl(props.token.portal_url)
  console.log('Extracted token:', token ? 'found' : 'not found')
  if (!token) return

  // 优先显示本地存储的Portal信息
  if (!forceRefresh && props.token.portal_info) {
    console.log('Using cached portal info')
    portalInfo.value = {
      data: {
        credits_balance: props.token.portal_info.credits_balance,
        expiry_date: props.token.portal_info.expiry_date,
        is_active: props.token.portal_info.is_active
      },
      error: null
    }
  } else if (!props.token.portal_info) {
    // 如果没有本地数据，先清空错误状态
    console.log('No cached data, clearing error state')
    portalInfo.value = { data: null, error: null }
  }

  // 在后台获取最新信息
  console.log('Starting background fetch')
  isLoadingPortalInfo.value = true

  try {
    // 首先获取customer信息
    console.log('Calling get_customer_info...')
    const customerResponse = await invoke('get_customer_info', { token })
    console.log('Customer response received:', customerResponse)
    const customerData = JSON.parse(customerResponse)
    console.log('Customer data parsed:', customerData)

    if (customerData.customer && customerData.customer.ledger_pricing_units && customerData.customer.ledger_pricing_units.length > 0) {
      const customerId = customerData.customer.id
      const pricingUnitId = customerData.customer.ledger_pricing_units[0].id
      console.log('Customer ID:', customerId, 'Pricing Unit ID:', pricingUnitId)

      // 获取ledger summary
      console.log('Calling get_ledger_summary...')
      const ledgerResponse = await invoke('get_ledger_summary', {
        customerId,
        pricingUnitId,
        token
      })
      console.log('Ledger response received:', ledgerResponse)
      const ledgerData = JSON.parse(ledgerResponse)
      console.log('Ledger data parsed:', ledgerData)

      if (ledgerData.credit_blocks && ledgerData.credit_blocks.length > 0) {
        console.log('Credit blocks found:', ledgerData.credit_blocks.length)
        const newPortalData = {
          credits_balance: parseInt(ledgerData.credits_balance) || 0,
          expiry_date: ledgerData.credit_blocks[0].expiry_date,
          is_active: ledgerData.credit_blocks[0].is_active
        }
        console.log('New portal data:', newPortalData)

        // 更新UI显示
        portalInfo.value = {
          data: newPortalData,
          error: null
        }
        console.log('UI updated with portal data')

        // 保存到本地存储
        try {
          const saveResult = await invoke('update_token_portal_info', {
            id: props.token.id,
            creditsBalance: parseInt(ledgerData.credits_balance) || 0,
            expiryDate: ledgerData.credit_blocks[0].expiry_date,
            isActive: ledgerData.credit_blocks[0].is_active
          })
          console.log('Portal info saved successfully:', saveResult)
        } catch (saveError) {
          console.error('Failed to save portal info:', saveError)
        }

        // 更新本地token对象
        props.token.portal_info = {
          credits_balance: parseInt(ledgerData.credits_balance) || 0,
          expiry_date: ledgerData.credit_blocks[0].expiry_date,
          is_active: ledgerData.credit_blocks[0].is_active,
          last_updated: new Date().toISOString()
        }
        console.log('Updated token portal_info:', props.token.portal_info)
      } else {
        // 如果没有本地数据，静默处理，不显示错误信息
        if (!props.token.portal_info) {
          portalInfo.value = { data: null, error: null }
        }
      }
    } else {
      // 如果没有本地数据，静默处理，不显示错误信息
      if (!props.token.portal_info) {
        portalInfo.value = { data: null, error: null }
      }
    }
  } catch (error) {
    console.error('Failed to load portal info:', error)
    // 无论是否有本地数据，都不显示错误信息，静默处理
    if (!props.token.portal_info) {
      portalInfo.value = { data: null, error: null }
    }
  } finally {
    isLoadingPortalInfo.value = false
  }
}

const formatExpiryDate = (dateString) => {
  try {
    const date = new Date(dateString)
    return date.toLocaleString('zh-CN', {
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit'
    })
  } catch {
    return dateString
  }
}

// 检测账号状态
const checkAccountStatus = async () => {
  console.log('checkAccountStatus called')
  if (isCheckingStatus.value) return

  isCheckingStatus.value = true

  try {
    // 并行执行两个操作：账号状态检测和Portal信息获取
    const promises = []

    // 1. 账号状态检测
    console.log('Adding account status check promise')
    const statusCheckPromise = invoke('check_account_status', {
      token: props.token.access_token,
      tenantUrl: props.token.tenant_url
    })
    promises.push(statusCheckPromise)

    // 2. Portal信息获取（如果有portal_url）
    let portalInfoPromise = null
    if (props.token.portal_url) {
      console.log('Adding portal info promise')
      portalInfoPromise = loadPortalInfo(true) // 强制刷新
      promises.push(portalInfoPromise)
    } else {
      console.log('No portal_url, skipping portal info fetch')
    }

    // 等待所有操作完成
    const results = await Promise.allSettled(promises)

    // 处理账号状态检测结果
    const statusResult = results[0]
    let statusMessage = ''
    let statusType = 'info'

    if (statusResult.status === 'fulfilled') {
      const result = statusResult.value
      // 更新token的ban_status
      const banStatus = result.is_banned ? 'SUSPENDED' : 'ACTIVE'
      await invoke('update_token_ban_status', {
        id: props.token.id,
        banStatus: banStatus
      })

      // 更新本地token对象
      props.token.ban_status = banStatus

      statusMessage = result.is_banned ? '账号已封禁' : '账号状态正常'
      statusType = result.is_banned ? 'error' : 'success'
    } else {
      console.error('Account status check failed:', statusResult.reason)
      statusMessage = `状态检测失败: ${statusResult.reason}`
      statusType = 'error'
    }

    // 处理Portal信息获取结果（静默更新，不在通知中显示）
    if (portalInfoPromise && results.length > 1) {
      const portalResult = results[1]
      if (portalResult.status === 'rejected') {
        console.error('Portal info fetch failed:', portalResult.reason)
        // 如果有本地数据，继续显示本地数据，不显示错误
      }
      // loadPortalInfo方法已经处理了成功和失败的情况
    }

    // 发送账号状态消息（不包含次数信息）
    const finalMessage = `检测完成：${statusMessage}`
    emit('copy-success', finalMessage, statusType)

  } catch (error) {
    console.error('Account status check failed:', error)
    emit('copy-success', `检测失败: ${error}`, 'error')
  } finally {
    isCheckingStatus.value = false
    isLoadingPortalInfo.value = false
  }
}

// 暴露刷新Portal信息的方法
const refreshPortalInfo = () => {
  if (props.token.portal_url) {
    loadPortalInfo(true) // 强制刷新
  }
}

// 组件挂载时加载Portal信息
onMounted(() => {
  if (props.token.portal_url) {
    // 如果有本地数据，立即显示
    if (props.token.portal_info) {
      portalInfo.value = {
        data: {
          credits_balance: props.token.portal_info.credits_balance,
          expiry_date: props.token.portal_info.expiry_date,
          is_active: props.token.portal_info.is_active
        },
        error: null
      }
    }
    // 然后在后台刷新数据
    loadPortalInfo(false)
  }
})

// 暴露方法给父组件
defineExpose({
  refreshPortalInfo
})
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
  position: relative; /* 为状态指示器定位 */
}

.status-indicator {
  position: absolute;
  top: 8px;
  right: 8px;
  z-index: 10;
}

.status-badge {
  font-size: 10px;
  font-weight: 600;
  padding: 3px 8px;
  border-radius: 12px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.status-badge.active {
  background: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
}

.status-badge.inactive {
  background: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}

.status-badge.banned {
  background: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
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
  justify-content: flex-start;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap; /* 允许换行 */
}

.created-date {
  font-size: 12px;
  color: #666;
}

.portal-meta {
  font-size: 12px;
  font-weight: 500;
  padding: 2px 6px;
  border-radius: 4px;
}

.portal-meta.loading {
  color: #6c757d;
  font-style: italic;
}

.portal-meta.error {
  color: #dc3545;
  background: #f8d7da;
}

.portal-meta.expiry {
  color: #856404;
  background: #fff3cd;
}

.portal-meta.balance {
  color: #155724;
  background: #d4edda;
  font-weight: 600;
}





.actions {
  display: flex;
  flex-direction: row;
  gap: 6px;
  justify-content: flex-end;
  margin-top: auto;
  flex-wrap: wrap;
}

.btn-action {
  background: #f8f9fa;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  padding: 8px;
  cursor: pointer;
  color: #495057;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 36px;
  min-height: 36px;
  flex-shrink: 0;
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

.btn-action.portal {
  color: #007bff;
}

.btn-action.portal:hover {
  background: #e3f2fd;
  border-color: #90caf9;
}

.btn-action.edit {
  color: #28a745;
}

.btn-action.edit:hover {
  background: #d4edda;
  border-color: #c3e6cb;
}

.btn-action.status-check {
  color: #ffc107;
}

.btn-action.status-check:hover {
  background: #fff3cd;
  border-color: #ffeaa7;
}

.btn-action.status-check.loading {
  opacity: 0.7;
  cursor: not-allowed;
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
    gap: 4px;
  }

  .btn-action {
    padding: 6px;
    min-width: 32px;
    min-height: 32px;
  }

  .btn-action svg {
    width: 16px;
    height: 16px;
  }
}

@media (max-width: 480px) {
  .token-card {
    padding: 12px;
  }

  .actions {
    gap: 3px;
    justify-content: center;
  }

  .btn-action {
    padding: 6px;
    min-width: 30px;
    min-height: 30px;
  }

  .btn-action svg {
    width: 14px;
    height: 14px;
  }

  .token-meta {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }
}


</style>

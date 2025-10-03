<template>
  <div class="token-card">
    <!-- 状态指示器 -->
    <div v-if="(token.portal_url && portalInfo.data) || token.ban_status" class="status-indicator">
      <span :class="['status-badge', getStatusClass(token.ban_status)]">
        {{ getStatusText(token.ban_status) }}
      </span>
    </div>

    <div class="card-main">
      <div class="token-info">
        <h3 class="tenant-name">{{ displayUrl }}</h3>
        <div class="token-meta">
          <!-- 第一行：创建日期和邮箱备注 -->
          <div class="meta-row">
            <span class="created-date">{{ formatDate(token.created_at) }}</span>
            <div v-if="token.email_note" class="email-note-container">
              <span
                class="email-note"
                @mouseenter="handleEmailMouseEnter"
                @mouseleave="handleEmailMouseLeave"
                :title="isEmailHovered ? '' : token.email_note"
              >
                <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor" class="email-icon">
                  <path d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.89 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"/>
                </svg>
                {{ isEmailHovered ? token.email_note : maskedEmail }}
              </span>
              <button @click="copyEmailNote" class="copy-email-btn" :title="$t('tokenCard.copyEmailNote')">
                <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
                </svg>
              </button>
            </div>
          </div>
          <!-- 第二行：Portal信息 -->
          <template v-if="token.portal_url">
            <div class="meta-row portal-row">
              <template v-if="portalInfo.data">
                <span v-if="portalInfo.data.expiry_date" class="portal-meta expiry">{{ $t('tokenCard.expiry') }}: {{ formatExpiryDate(portalInfo.data.expiry_date) }}</span>
                <span :class="balanceClasses">
                  {{ balanceDisplay }}
                </span>
              </template>
            </div>
          </template>
        </div>
      </div>

      <div class="actions">
        <button @click="openEditorModal" class="btn-action vscode" :title="$t('tokenCard.selectEditor')">
          <img :src="editorIcons.vscode" :alt="$t('tokenCard.selectEditor')" width="18" height="18" />
        </button>
        <button @click="copyToken" class="btn-action" :title="$t('tokenCard.copyToken')">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
          </svg>
        </button>
        <button @click="copyTenantUrl" class="btn-action" :title="$t('tokenCard.copyTenantUrl')">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M3.9 12c0-1.71 1.39-3.1 3.1-3.1h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-1.9H7c-1.71 0-3.1-1.39-3.1-3.1zM8 13h8v-2H8v2zm9-6h-4v1.9h4c1.71 0 3.1 1.39 3.1 3.1s-1.39 3.1-3.1 3.1h-4V17h4c2.76 0 5-2.24 5-5s-2.24-5-5-5z"/>
          </svg>
        </button>
        <button @click="checkAccountStatus" :class="['btn-action', 'status-check', { loading: isCheckingStatus || isBatchChecking }]" :disabled="isCheckingStatus || isBatchChecking" :title="$t('tokenCard.checkAccountStatus')">
          <svg v-if="!isCheckingStatus && !isBatchChecking" width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
          </svg>
          <div v-else-if="isCheckingStatus || isBatchChecking" class="loading-spinner"></div>
        </button>
        <button v-if="token.portal_url" @click="showPortalDialog = true" class="btn-action portal" :title="$t('tokenCard.openPortal')">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
          </svg>
        </button>
        <button @click="$emit('edit', token)" class="btn-action edit" :title="$t('tokenCard.editToken')">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
          </svg>
        </button>
        <button @click="deleteToken" class="btn-action delete" :title="$t('tokenCard.deleteToken')">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
          </svg>
        </button>
      </div>
    </div>
  </div>

  <Teleport to="body">
    <Transition name="modal" appear>
      <div v-if="showEditorModal" class="editor-modal-overlay">
        <div class="editor-modal" @click.stop>
          <div class="modal-header">
            <h3>{{ $t('tokenCard.selectEditor') }}</h3>
            <button @click.stop="showEditorModal = false" class="modal-close">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
              </svg>
            </button>
          </div>
          <div class="modal-content">
            <!-- VSCode 系编辑器区域 -->
            <div class="editor-section">
              <div class="editor-options jetbrains-grid">
                <button @click="handleEditorClick('vscode')" class="editor-option vscode-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.vscode" alt="VS Code" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">VS Code</span>
                  </div>
                </button>
                <button @click="handleEditorClick('cursor')" class="editor-option cursor-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.cursor" alt="Cursor" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">Cursor</span>
                  </div>
                </button>
                <button @click="handleEditorClick('kiro')" class="editor-option kiro-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.kiro" alt="Kiro" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">Kiro</span>
                  </div>
                </button>
                <button @click="handleEditorClick('trae')" class="editor-option trae-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.trae" alt="Trae" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">Trae</span>
                  </div>
                </button>
                <button @click="handleEditorClick('windsurf')" class="editor-option windsurf-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.windsurf" alt="Windsurf" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">Windsurf</span>
                  </div>
                </button>
                <button @click="handleEditorClick('qoder')" class="editor-option qoder-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.qoder" alt="Qoder" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">Qoder</span>
                  </div>
                </button>
                <button @click="handleEditorClick('vscodium')" class="editor-option vscodium-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.vscodium" alt="VSCodium" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">VSCodium</span>
                  </div>
                </button>
                <button @click="handleEditorClick('codebuddy')" class="editor-option codebuddy-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.codebuddy" alt="CodeBuddy" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">CodeBuddy</span>
                  </div>
                </button>
              </div>
            </div>

            <!-- JetBrains 系编辑器区域 -->
            <div class="editor-section">
              <div class="editor-options jetbrains-grid">
                <button @click="handleEditorClick('idea')" class="editor-option idea-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.idea" alt="IntelliJ IDEA" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">IntelliJ IDEA</span>
                  </div>
                </button>
                <button @click="handleEditorClick('pycharm')" class="editor-option pycharm-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.pycharm" alt="PyCharm" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">PyCharm</span>
                  </div>
                </button>
                <button @click="handleEditorClick('goland')" class="editor-option goland-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.goland" alt="GoLand" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">GoLand</span>
                  </div>
                </button>
                <button @click="handleEditorClick('rustrover')" class="editor-option rustrover-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.rustrover" alt="RustRover" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">RustRover</span>
                  </div>
                </button>
                <button @click="handleEditorClick('webstorm')" class="editor-option webstorm-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.webstorm" alt="WebStorm" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">WebStorm</span>
                  </div>
                </button>
                <button @click="handleEditorClick('phpstorm')" class="editor-option phpstorm-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.phpstorm" alt="PhpStorm" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">PhpStorm</span>
                  </div>
                </button>
                <button @click="handleEditorClick('androidstudio')" class="editor-option androidstudio-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.androidstudio" alt="Android Studio" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">Android Studio</span>
                  </div>
                </button>
                <button @click="handleEditorClick('clion')" class="editor-option clion-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.clion" alt="CLion" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">CLion</span>
                  </div>
                </button>
                <button @click="handleEditorClick('datagrip')" class="editor-option datagrip-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.datagrip" alt="DataGrip" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">DataGrip</span>
                  </div>
                </button>
                <button @click="handleEditorClick('rider')" class="editor-option rider-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.rider" alt="Rider" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">Rider</span>
                  </div>
                </button>
                <button @click="handleEditorClick('rubymine')" class="editor-option rubymine-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.rubymine" alt="RubyMine" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">RubyMine</span>
                  </div>
                </button>
                <button @click="handleEditorClick('aqua')" class="editor-option aqua-option">
                  <div class="editor-icon">
                    <img :src="editorIcons.aqua" alt="Aqua" width="32" height="32" />
                  </div>
                  <div class="editor-info">
                    <span class="editor-name">Aqua</span>
                  </div>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>

  <ExternalLinkDialog
    :show="showPortalDialog"
    :title="$t('dialogs.selectOpenMethod')"
    :url="token.portal_url || ''"
    :browser-title="getPortalBrowserTitle()"
    @close="showPortalDialog = false"
  />
</template>

<script setup>
import { computed, ref, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import ExternalLinkDialog from './ExternalLinkDialog.vue'

const { t } = useI18n()


// Props
const props = defineProps({
  token: {
    type: Object,
    required: true
  },
  isBatchChecking: {
    type: Boolean,
    default: false
  }
})

// Emits
const emit = defineEmits(['delete', 'edit', 'token-updated'])

// Reactive data
const isLoadingPortalInfo = ref(false)
const portalInfo = ref({ data: null, error: null })
const isCheckingStatus = ref(false)
const isEmailHovered = ref(false)
const showEditorModal = ref(false)
const isModalClosing = ref(false)
const canStillUse = ref(false)
const showPortalDialog = ref(false)

// 图标映射
const editorIcons = {
  vscode: '/icons/vscode.svg',
  cursor: '/icons/cursor.svg',
  kiro: '/icons/kiro.svg',
  trae: '/icons/trae.svg',
  windsurf: '/icons/windsurf.svg',
  qoder: '/icons/qoder.svg',
  vscodium: '/icons/vscodium.svg',
  codebuddy: '/icons/codebuddy.svg',
  idea: '/icons/idea.svg',
  pycharm: '/icons/pycharm.svg',
  goland: '/icons/goland.svg',
  rustrover: '/icons/rustrover.svg',
  webstorm: '/icons/webstorm.svg',
  phpstorm: '/icons/phpstorm.svg',
  clion: '/icons/clion.svg',
  datagrip: '/icons/datagrip.svg',
  rider: '/icons/rider.svg',
  rubymine: '/icons/rubymine.svg',
  aqua: '/icons/aqua.svg',
  androidstudio: '/icons/androidstudio.svg'
}

// Computed properties
const displayUrl = computed(() => {
  try {
    const url = new URL(props.token.tenant_url)
    return url.hostname
  } catch {
    return props.token.tenant_url
  }
})


const maskedEmail = computed(() => {
  const email = props.token.email_note
  if (!email || !email.includes('@')) return email

  const [username, domain] = email.split('@')

  // 如果用户名太短，直接返回原邮箱
  if (username.length <= 3) {
    return email
  }

  let maskedUsername
  if (username.length <= 6) {
    // 短邮箱：保留前1-2个字符，其余用星号替换
    const keepChars = username.length <= 4 ? 1 : 2
    const hiddenCount = username.length - keepChars
    maskedUsername = username.substring(0, keepChars) + '*'.repeat(hiddenCount)
  } else {
    // 长邮箱：保留前后各2-3个字符，中间用4个星号替换
    const frontKeep = username.length >= 8 ? 3 : 2
    const backKeep = 2
    maskedUsername = username.substring(0, frontKeep) + '****' + username.substring(username.length - backKeep)
  }

  return maskedUsername + '@' + domain
})

// Portal余额显示相关计算属性
const balanceClasses = computed(() => {
  if (!portalInfo.value || !portalInfo.value.data) {
    return ['portal-meta', 'balance']
  }
  const exhausted = (
    props.token.ban_status === 'EXPIRED' ||
    props.token.ban_status === 'SUSPENDED' ||
    (portalInfo.value.data.credits_balance === 0 && !canStillUse.value)
  )
  return ['portal-meta', 'balance', { exhausted }]
})

const balanceDisplay = computed(() => {
  if (!portalInfo.value || !portalInfo.value.data) return ''
  const status = props.token.ban_status
  if (status === 'EXPIRED') return t('tokenCard.expired')
  if (status === 'SUSPENDED') return t('tokenCard.banned')
  const credits = portalInfo.value.data.credits_balance
  if (credits === 0) {
    return canStillUse.value ? t('tokenCard.canUse') : t('tokenCard.exhausted')
  }
  return `${t('tokenCard.balance')}: ${credits}`
})

// 获取状态样式类
const getStatusClass = (status) => {
  switch (status) {
    case 'SUSPENDED':
      return 'banned'
    case 'EXPIRED':
      return 'inactive'
    case 'INVALID_TOKEN':
      return 'invalid'
    case 'ACTIVE':
      return 'active'
    default:
      return 'active'
  }
}

// 获取状态显示文本
const getStatusText = (status) => {
  switch (status) {
    case 'SUSPENDED':
      return t('tokenCard.banned')
    case 'EXPIRED':
      return t('tokenCard.expired')
    case 'INVALID_TOKEN':
      return t('tokenCard.tokenInvalid')
    case 'ACTIVE':
      return t('tokenCard.active')
    default:
      return t('tokenCard.active')
  }
}


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
  // 直接发出删除事件，让父组件处理确认逻辑
  emit('delete', props.token.id)
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

// 通用复制方法
const copyWithNotification = async (text, successMessage, errorMessage) => {
  const success = await copyToClipboard(text)
  if (success) {
    window.$notify.success(t(successMessage))
  } else {
    window.$notify.error(t(errorMessage))
  }
}

// 复制Token
const copyToken = () => copyWithNotification(
  props.token.access_token,
  'messages.tokenCopied',
  'messages.copyTokenFailed'
)

// 复制租户URL
const copyTenantUrl = () => copyWithNotification(
  props.token.tenant_url,
  'messages.tenantUrlCopied',
  'messages.copyTenantUrlFailed'
)

// 复制邮箱备注
const copyEmailNote = () => copyWithNotification(
  props.token.email_note,
  'messages.emailNoteCopied',
  'messages.copyEmailNoteFailed'
)

// 键盘事件处理
const handleKeydown = (event) => {
  if (event.key === 'Escape' && showEditorModal.value) {
    showEditorModal.value = false
  }
}

// 打开编辑器模态框
const openEditorModal = () => {
  if (showEditorModal.value || isModalClosing.value) return
  showEditorModal.value = true
}


const editorNames = {
  'cursor': 'Cursor',
  'vscode': 'VS Code',
  'kiro': 'Kiro',
  'trae': 'Trae',
  'windsurf': 'Windsurf',
  'qoder': 'Qoder',
  'vscodium': 'VSCodium',
  'codebuddy': 'CodeBuddy',
  'idea': 'IntelliJ IDEA',
  'pycharm': 'PyCharm',
  'goland': 'GoLand',
  'rustrover': 'RustRover',
  'webstorm': 'WebStorm',
  'phpstorm': 'PhpStorm',
  'androidstudio': 'Android Studio',
  'clion': 'CLion',
  'datagrip': 'DataGrip',
  'rider': 'Rider',
  'rubymine': 'RubyMine',
  'aqua': 'Aqua'
}

const vscodeSchemes = {
  'cursor': 'cursor',
  'vscode': 'vscode',
  'kiro': 'kiro',
  'trae': 'trae',
  'windsurf': 'windsurf',
  'qoder': 'qoder',
  'vscodium': 'vscodium',
  'codebuddy': 'codebuddy'
}

const createVSCodeProtocolUrl = (scheme, label) => {
  try {
    const token = encodeURIComponent(props.token.access_token)
    const url = encodeURIComponent(props.token.tenant_url)
    const portalUrl = encodeURIComponent(props.token.portal_url || "")
    return `${scheme}://Augment.vscode-augment/autoAuth?token=${token}&url=${url}&portal=${portalUrl}`
  } catch (error) {
    return '#'
  }
}


const jetbrainsEditors = new Set([
  'idea', 'pycharm', 'goland', 'rustrover', 'webstorm',
  'phpstorm', 'androidstudio', 'clion', 'datagrip', 'rider', 'rubymine', 'aqua'
])

const vscodeProtocolResolvers = Object.fromEntries(
  Object.entries(vscodeSchemes).map(([type, scheme]) => [
    type,
    () => createVSCodeProtocolUrl(scheme, editorNames[type] || type)
  ])
)

// 为 JetBrains 编辑器创建 JSON 文件
const createJetBrainsTokenFile = async (editorType) => {
  try {
    // 创建 JSON 数据
    const tokenData = {
      url: props.token.tenant_url,
      token: props.token.access_token,
      timestamp: Date.now(),
      ide: editorType
    }

    // 调用 Tauri 后端命令创建文件
    const result = await invoke('create_jetbrains_token_file', {
      editorType,
      tokenData: JSON.stringify(tokenData, null, 2)
    })

    return { success: true, filePath: result }
  } catch (error) {
    return { success: false, error: error.toString() }
  }
}

// 处理编辑器链接点击事件
const handleEditorClick = async (editorType) => {
  try {
    const editorName = editorNames[editorType] || editorType

    if (jetbrainsEditors.has(editorType)) {
      const result = await createJetBrainsTokenFile(editorType)

      if (result.success) {
        emit('copy-success', t('messages.editorTokenFileCreated', { editor: editorName }), 'success')
      } else {
        emit('copy-success', t('messages.createEditorTokenFileFailed', { editor: editorName, error: result.error }), 'error')
      }
    } else {
      const resolver = vscodeProtocolResolvers[editorType]

      if (!resolver) {
        throw new Error(`Unknown VSCode editor type: ${editorType}`)
      }

      const protocolUrl = resolver()

      await invoke('open_editor_with_protocol', { protocolUrl })
      window.$notify.success(t('messages.openingEditor', { editor: editorName }))
    }
  } catch (error) {
    window.$notify.error(t('messages.operationFailed'))
  } finally {
    showEditorModal.value = false
    isModalClosing.value = false
  }
}

// 邮箱悬浮事件处理
const handleEmailMouseEnter = () => {
  isEmailHovered.value = true
}

const handleEmailMouseLeave = () => {
  isEmailHovered.value = false
}

// Portal相关方法
const getPortalBrowserTitle = () => {
  if (!props.token) return 'Portal'
  const displayUrl = props.token.tenant_url.replace(/^https?:\/\//, '').replace(/\/$/, '')
  return `Portal - ${displayUrl}`
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
const checkAccountStatus = async (showNotification = true) => {
  if (isCheckingStatus.value || props.isBatchChecking) return

  isCheckingStatus.value = true

  try {
    // 单次API调用同时获取账号状态和Portal信息
    const batchResults = await invoke('batch_check_tokens_status', {
      tokens: [{
        id: props.token.id,
        access_token: props.token.access_token,
        tenant_url: props.token.tenant_url,
        portal_url: props.token.portal_url || null
      }]
    })

    // 处理结果
    let statusMessage = ''
    let statusType = 'info'

    if (batchResults && batchResults.length > 0) {
      const result = batchResults[0] // 取第一个结果对象
      const statusResult = result.status_result // 账号状态结果
      
      // 使用后端返回的具体状态
      const banStatus = statusResult.status

      // 更新本地token对象 - 账号状态
      props.token.ban_status = banStatus

      // 更新 suspensions 信息（如果有）
      if (result.suspensions) {
        props.token.suspensions = result.suspensions
        console.log(`Updated suspensions for token ${props.token.id}:`, result.suspensions)
      }

      // 更新Portal信息（如果有）
      if (result.portal_info) {
        props.token.portal_info = {
          credits_balance: result.portal_info.credits_balance,
          expiry_date: result.portal_info.expiry_date,
          can_still_use: result.portal_info.can_still_use
        }

        // 更新UI显示
        portalInfo.value = {
          data: props.token.portal_info,
          error: null
        }

        // 直接使用后端返回的can_still_use字段
        canStillUse.value = result.portal_info.can_still_use
      } else if (result.portal_error) {
        portalInfo.value = {
          data: null,
          error: result.portal_error
        }
      }
      
      // 更新时间戳以确保双向同步时选择正确版本
      props.token.updated_at = new Date().toISOString()

      // 根据具体状态设置消息
      switch (banStatus) {
        case 'SUSPENDED':
          statusMessage = t('messages.accountBanned')
          statusType = 'error'
          break
        case 'EXPIRED':
          statusMessage = t('tokenCard.expired')
          statusType = 'warning'
          break
        case 'INVALID_TOKEN':
          statusMessage = t('messages.tokenInvalid')
          statusType = 'warning'
          break
        case 'ACTIVE':
          statusMessage = t('messages.accountStatusNormal')
          statusType = 'success'
          break
        case 'ERROR':
          statusMessage = `${t('messages.statusCheckFailed')}: ${statusResult.error_message || 'Unknown error'}`
          statusType = 'error'
          break
        default:
          statusMessage = `${t('messages.accountStatus')}: ${banStatus}`
          statusType = 'info'
      }
    } else {
      statusMessage = t('messages.statusCheckFailed') + ': No results returned'
      statusType = 'error'
    }

    // Portal信息现在已经包含在批量API结果中，无需单独处理

    // 发送账号状态消息（不包含次数信息）
    if (showNotification) {
      const finalMessage = `${t('messages.checkComplete')}: ${statusMessage}`
      window.$notify[statusType](finalMessage)
    }

  } catch (error) {
    if (showNotification) {
      window.$notify.error(`${t('messages.checkFailed')}: ${error}`)
    }
  } finally {
    isCheckingStatus.value = false
    isLoadingPortalInfo.value = false
  }
}




// 监听token变化，同步更新Portal信息显示
watch(() => props.token.portal_info, (newPortalInfo) => {
  if (newPortalInfo && props.token.portal_url) {
    portalInfo.value = {
      data: {
        credits_balance: newPortalInfo.credits_balance,
        expiry_date: newPortalInfo.expiry_date,
        can_still_use: newPortalInfo.can_still_use
      },
      error: null
    }
    // 同步can_still_use状态
    if (newPortalInfo.can_still_use !== undefined) {
      canStillUse.value = newPortalInfo.can_still_use
    }
  }
}, { deep: true })

// 组件挂载时加载Portal信息
onMounted(async () => {
  if (props.token.portal_url) {
    // 如果有本地数据，立即显示
    if (props.token.portal_info) {
      portalInfo.value = {
        data: {
          credits_balance: props.token.portal_info.credits_balance,
          expiry_date: props.token.portal_info.expiry_date,
          can_still_use: props.token.portal_info.can_still_use
        },
        error: null
      }
      // 设置本地的can_still_use状态
      if (props.token.portal_info.can_still_use !== undefined) {
        canStillUse.value = props.token.portal_info.can_still_use
      }
    }
    // 然后在后台刷新数据（静默模式，不显示通知）
    checkAccountStatus(false)
  }

  // 添加键盘事件监听器
  document.addEventListener('keydown', handleKeydown)
})

// 组件卸载时清理事件监听器
onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})

// 暴露检查账号状态的方法
const refreshAccountStatus = async () => {
  return await checkAccountStatus()
}

// 暴露方法给父组件
defineExpose({
  refreshAccountStatus
})
</script>

<style scoped>
.token-card {
  background: var(--color-surface, #ffffff);
  border: 1px solid var(--color-divider, #e1e5e9);
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
  background: var(--color-success-surface, #d4edda);
  color: var(--color-success-text, #155724);
  border: 1px solid var(--color-success-border, #c3e6cb);
}

.status-badge.inactive {
  background: var(--color-danger-surface, #f8d7da);
  color: var(--color-danger-text, #721c24);
  border: 1px solid var(--color-danger-border, #f5c6cb);
}

.status-badge.banned {
  background: var(--color-danger-surface, #f8d7da);
  color: var(--color-danger-text, #721c24);
  border: 1px solid var(--color-danger-border, #f5c6cb);
}

.status-badge.invalid {
  background: var(--color-warning-surface, #fff3cd);
  color: var(--color-warning-text, #856404);
  border: 1px solid var(--color-warning-border, #ffeaa7);
}

.token-card:hover {
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.15);
  border-color: var(--color-accent, #3b82f6);
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
  color: var(--color-text-heading, #333);
  word-break: break-all;
  line-height: 1.3;
}

.token-meta {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.meta-row {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.portal-row {
  margin-top: 2px;
}

.created-date {
  font-size: 12px;
  color: var(--color-text-muted, #666);
}

.email-note-container {
  display: flex;
  align-items: center;
  gap: 6px;
}

.email-note {
  font-size: 12px;
  color: var(--color-link-visited, #4f46e5);
  display: flex;
  align-items: center;
  gap: 4px;
  background: var(--color-info-surface, #f0f9ff);
  padding: 2px 6px;
  border-radius: 4px;
  border: 1px solid var(--color-info-surface, #e0f2fe);
  cursor: pointer;
  transition: all 0.2s ease;
  user-select: none;
}

.email-note:hover {
  background: var(--color-info-surface, #e0f2fe);
  border-color: var(--color-info-border, #bae6fd);
  transform: translateY(-1px);
}

/* 黑暗模式下的邮箱样式优化 */
[data-theme='dark'] .email-note {
  background: rgba(56, 189, 248, 0.2);
  color: #93c5fd;
  border-color: rgba(56, 189, 248, 0.4);
}

[data-theme='dark'] .email-note:hover {
  background: rgba(56, 189, 248, 0.3);
  border-color: rgba(56, 189, 248, 0.6);
  color: #bfdbfe;
}

/* 黑暗模式下的按钮样式优化 */
[data-theme='dark'] .btn-action {
  background: rgba(51, 65, 85, 0.5);
  border-color: rgba(71, 85, 105, 0.6);
  color: #cbd5e1;
}

[data-theme='dark'] .btn-action:hover {
  background: rgba(71, 85, 105, 0.6);
  border-color: rgba(100, 116, 139, 0.7);
}

[data-theme='dark'] .btn-action.delete {
  color: #fca5a5;
}

[data-theme='dark'] .btn-action.delete:hover {
  background: rgba(220, 38, 38, 0.2);
  border-color: rgba(220, 38, 38, 0.4);
}

[data-theme='dark'] .btn-action.portal {
  color: #93c5fd;
}

[data-theme='dark'] .btn-action.portal:hover {
  background: rgba(59, 130, 246, 0.2);
  border-color: rgba(59, 130, 246, 0.4);
}

[data-theme='dark'] .btn-action.edit {
  color: #86efac;
}

[data-theme='dark'] .btn-action.edit:hover {
  background: rgba(34, 197, 94, 0.2);
  border-color: rgba(34, 197, 94, 0.4);
}

[data-theme='dark'] .btn-action.vscode {
  color: #7dd3fc;
}

[data-theme='dark'] .btn-action.vscode:hover {
  background: rgba(14, 165, 233, 0.2);
  border-color: rgba(14, 165, 233, 0.4);
}

[data-theme='dark'] .btn-action.status-check {
  color: #fcd34d;
}

[data-theme='dark'] .btn-action.status-check:hover {
  background: rgba(245, 158, 11, 0.2);
  border-color: rgba(245, 158, 11, 0.4);
}

.email-icon {
  flex-shrink: 0;
  opacity: 0.7;
}

.copy-email-btn {
  background: none;
  border: none;
  padding: 2px;
  cursor: pointer;
  color: var(--color-text-muted, #6b7280);
  border-radius: 3px;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.copy-email-btn:hover {
  background: var(--color-surface-hover, #f3f4f6);
  color: var(--color-link-visited, #4f46e5);
}

.copy-email-btn:active {
  transform: scale(0.95);
}

.portal-meta {
  font-size: 12px;
  font-weight: 500;
  padding: 2px 6px;
  border-radius: 4px;
}

.portal-meta.loading {
  color: var(--color-text-muted, #6c757d);
  font-style: italic;
}

.portal-meta.error {
  color: var(--color-danger-bg, #dc3545);
  background: var(--color-danger-surface, #f8d7da);
}

.portal-meta.expiry {
  color: var(--color-warning-text, #856404);
  background: var(--color-warning-surface, #fff3cd);
}

.portal-meta.balance {
  color: var(--color-success-text, #155724);
  background: var(--color-success-surface, #d4edda);
  font-weight: 600;
}

.portal-meta.balance.exhausted {
  color: var(--color-danger-text, #721c24);
  background: var(--color-danger-surface, #f8d7da);
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
  background: rgba(148, 163, 184, 0.15);
  border: 1px solid rgba(148, 163, 184, 0.3);
  border-radius: 8px;
  padding: 8px;
  cursor: pointer;
  color: #64748b;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 36px;
  min-height: 36px;
  flex-shrink: 0;
}

/* 防止按钮内的 SVG 图标在 hover 时抖动 */
.btn-action svg,
.btn-action img {
  will-change: transform;
  backface-visibility: hidden;
  -webkit-font-smoothing: subpixel-antialiased;
}

.btn-action:hover {
  background: rgba(148, 163, 184, 0.25);
  border-color: rgba(148, 163, 184, 0.5);
  transform: translateY(-1px);
}

.btn-action.delete {
  color: #dc2626;
}

.btn-action.delete:hover {
  background: rgba(220, 38, 38, 0.15);
  border-color: rgba(220, 38, 38, 0.3);
}

.btn-action.portal {
  color: #2563eb;
}

.btn-action.portal:hover {
  background: rgba(37, 99, 235, 0.15);
  border-color: rgba(37, 99, 235, 0.3);
}

.btn-action.edit {
  color: #16a34a;
}

.btn-action.edit:hover {
  background: rgba(22, 163, 74, 0.15);
  border-color: rgba(22, 163, 74, 0.3);
}

.btn-action.vscode {
  color: #0284c7;
}

.btn-action.vscode:hover {
  background: rgba(2, 132, 199, 0.15);
  border-color: rgba(2, 132, 199, 0.3);
}

.btn-action.status-check {
  color: #ca8a04;
}

.btn-action.status-check:hover {
  background: rgba(202, 138, 4, 0.15);
  border-color: rgba(202, 138, 4, 0.3);
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

/* Vue 过渡动画 */
.modal-enter-active,
.modal-leave-active {
  transition: all 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .editor-modal,
.modal-leave-to .editor-modal {
  transform: translateY(-20px) scale(0.95);
}

.modal-enter-to .editor-modal,
.modal-leave-from .editor-modal {
  transform: translateY(0) scale(1);
}

/* 编辑器选择模态框样式 */
.editor-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2100;
  backdrop-filter: blur(2px);
  pointer-events: auto;
}

.editor-modal {
  background: var(--color-surface, #ffffff);
  border-radius: 12px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  max-width: 700px;
  width: 90%;
  max-height: 95vh;
  overflow: hidden;
  transition: transform 0.3s ease;
  position: relative;
  pointer-events: auto;
  margin: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px 16px;
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
}

.modal-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-heading, #333);
}

.modal-close {
  background: none;
  border: none;
  padding: 4px;
  cursor: pointer;
  color: var(--color-text-muted, #666);
  border-radius: 4px;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal-close:hover {
  background: var(--color-surface-hover, #f3f4f6);
  color: var(--color-text-heading, #333);
}

.modal-content {
  padding: 20px 24px 24px;
  max-height: calc(95vh - 80px);
  overflow-y: auto;
}



.editor-section {
  margin-bottom: 24px;
  padding-bottom: 24px;
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
}

.editor-section:last-child {
  margin-bottom: 0;
  padding-bottom: 0;
  border-bottom: none;
}

.editor-options {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.jetbrains-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 12px;
}

.editor-option {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  border: 2px solid var(--color-divider, #e1e5e9);
  border-radius: 8px;
  background: var(--color-surface, #ffffff);
  cursor: pointer;
  transition: all 0.15s ease;
  text-align: left;
  width: 100%;
  position: relative;
  /* 移除按钮默认样式 */
  font-family: inherit;
  font-size: inherit;
  text-decoration: none;
  color: inherit;
  box-sizing: border-box;
}

.editor-option:hover {
  border-color: var(--color-accent, #3b82f6);
  background: var(--color-surface-soft, #f8fafc);
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.12);
}

.editor-option:active {
  background: var(--color-surface-soft, #f1f5f9);
  box-shadow: 0 1px 4px rgba(59, 130, 246, 0.08);
}

/* 确保链接在所有状态下都保持正确的样式 */
.editor-option:visited {
  color: inherit;
  text-decoration: none;
}

.editor-option:focus {
  outline: 2px solid var(--color-accent, #3b82f6);
  outline-offset: 2px;
}

.editor-icon {
  flex-shrink: 0;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  background: var(--color-surface-muted, #f8f9fa);
  border: 1px solid var(--color-surface-muted, #e9ecef);
}

.editor-icon img {
  width: 32px;
  height: 32px;
  object-fit: contain;
}

.cursor-option .editor-icon {
  background: var(--color-info-surface, #f0f9ff);
  border-color: var(--color-info-surface, #e0f2fe);
}

.vscode-option .editor-icon {
  background: var(--color-info-surface, #f0f9ff);
  border-color: var(--color-info-surface, #e0f2fe);
}

.kiro-option .editor-icon,
.trae-option .editor-icon,
.windsurf-option .editor-icon,
.qoder-option .editor-icon,
.vscodium-option .editor-icon,
.codebuddy-option .editor-icon {
  background: var(--color-info-surface, #f0f9ff);
  border-color: var(--color-info-surface, #e0f2fe);
}

.idea-option .editor-icon,
.pycharm-option .editor-icon,
.goland-option .editor-icon,
.rustrover-option .editor-icon,
.webstorm-option .editor-icon,
.phpstorm-option .editor-icon,
.androidstudio-option .editor-icon,
.clion-option .editor-icon,
.datagrip-option .editor-icon,
.rider-option .editor-icon,
.rubymine-option .editor-icon,
.aqua-option .editor-icon {
  background: var(--color-info-surface, #f0f9ff);
  border-color: var(--color-info-surface, #e0f2fe);
}

.editor-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.editor-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-heading, #333);
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

  .btn-action svg,
  .btn-action img {
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

  .btn-action svg,
  .btn-action img {
    width: 14px;
    height: 14px;
  }

  .token-meta {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }

  /* 模态框响应式样式 */
  .editor-modal {
    width: 95%;
    margin: 16px;
    max-height: 90vh;
  }

  .modal-header {
    padding: 16px 20px 12px;
  }

  .modal-header h3 {
    font-size: 16px;
  }

  .modal-content {
    padding: 16px 20px 20px;
  }

  .editor-section {
    margin-bottom: 20px;
    padding-bottom: 20px;
  }

  .jetbrains-grid {
    grid-template-columns: 1fr;
  }

  .editor-option {
    padding: 12px;
    gap: 12px;
  }

  .editor-icon {
    width: 40px;
    height: 40px;
  }

  .editor-icon img {
    width: 28px;
    height: 28px;
  }

  .editor-name {
    font-size: 15px;
  }
}


</style>

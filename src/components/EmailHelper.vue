<template>
  <div class="email-helper">
    <div class="header">
      <h3>{{ $t('emailHelper.title') }}</h3>
      <div class="header-actions">
        <button
          v-if="canGetToken && emailsWithoutToken.length > 0"
          @click="batchGetAllTokens"
          :disabled="isBatchGettingTokens || isBatchRegistering"
          :class="['btn', 'primary', 'small', { loading: isBatchGettingTokens }]"
        >
          {{ isBatchGettingTokens ? `获取中 (${currentBatchIndex + 1}/${emailsWithoutToken.length})` : '一键获取所有Token' }}
        </button>
        <button
          @click="showBatchRegisterDialog"
          :disabled="isBatchGettingTokens || isBatchRegistering"
          :class="['btn', 'success', 'small', { loading: isBatchRegistering }]"
        >
          {{ isBatchRegistering ? `注册中 (${batchRegisterProgress.current}/${batchRegisterProgress.total})` : '批量注册' }}
        </button>
        <button @click="openCardSettings" class="btn secondary small">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="1" y="4" width="22" height="16" rx="2" ry="2"/>
            <line x1="1" y1="10" x2="23" y2="10"/>
          </svg>
          {{ $t('emailHelper.cardSettings') }}
        </button>
        <button @click="openSettings" class="btn secondary small">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <circle cx="12" cy="12" r="3"/>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
          </svg>
          {{ $t('emailHelper.settings') }}
        </button>
      </div>
    </div>

    <div class="body">
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
              <!-- Token标签放在右上角 -->
              <span v-if="checkEmailHasToken(emailInfo.email)" class="token-badge-corner" :title="$t('emailHelper.tokenObtained')">
                ✅ Token
              </span>

              <div class="email-info">
                <!-- 邮箱名称单独一行 -->
                <div class="email-address">
                  {{ emailInfo.email }}
                </div>

                <!-- 密码、时间和按钮在同一行 -->
                <div class="email-meta-actions">
                  <div class="email-meta">
                    <div class="email-created">{{ formatDate(emailInfo.created_at) }}</div>
                  </div>
                  <!-- 按钮放在密码时间后面 -->
                  <div class="email-actions-inline">
                    <button
                      @click="copyEmail(emailInfo.email)"
                      class="btn secondary small"
                    >
                      {{ $t('emailHelper.copyEmail') }}
                    </button>
                    <button
                      @click="startMonitoring(emailInfo.email)"
                      :disabled="isMonitoring && monitoringEmail !== emailInfo.email"
                      :class="['btn', monitoringEmail === emailInfo.email ? 'warning' : 'primary', 'small']"
                    >
                      {{ monitoringEmail === emailInfo.email ? $t('emailHelper.monitoring') : $t('emailHelper.startMonitor') }}
                    </button>
                    <button
                      v-if="!isGettingAugmentToken || currentQuickGetEmail !== emailInfo.email"
                      @click="quickGetAugmentToken(emailInfo.email)"
                      :disabled="isGettingAugmentToken || isQuickRegistering"
                      :class="['btn', 'success', 'small']"
                      :title="$t('emailHelper.quickGetTokenHint')"
                    >
                      {{ $t('emailHelper.quickGetToken') }}
                    </button>
                    <button
                      v-else
                      @click="stopQuickGet"
                      :class="['btn', 'danger', 'small']"
                    >
                      {{ $t('emailHelper.stopQuickGet') }}
                    </button>
                    <button
                      v-if="!isQuickRegistering || currentQuickRegisterEmail !== emailInfo.email"
                      @click="quickRegister(emailInfo.email)"
                      :disabled="isGettingAugmentToken || isQuickRegistering"
                      :class="['btn', 'warning', 'small']"
                      :title="$t('emailHelper.quickRegisterHint')"
                    >
                      {{ $t('emailHelper.quickRegister') }}
                    </button>
                    <button
                      v-else
                      @click="stopQuickRegister"
                      :class="['btn', 'danger', 'small']"
                    >
                      {{ $t('emailHelper.stopQuickRegister') }}
                    </button>
                    <button
                      @click="viewEmails(emailInfo.email)"
                      class="btn primary small"
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

    <!-- 设置模态框 - 使用 Teleport 渲染到 body -->
    <Teleport to="body">
      <Transition name="modal">
        <div v-if="showSettings" class="settings-overlay" @click="closeSettings">
          <div class="settings-content" @click.stop>
        <div class="settings-header">
          <h3>{{ $t('emailHelper.settingsTitle') }}</h3>
          <button @click="closeSettings" class="close-btn">×</button>
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
                <option v-for="domain in savedDomains" :key="domain" :value="domain">{{ domain }}</option>
                <option value="custom">{{ $t('emailHelper.customDomain') }}</option>
              </select>
              <button
                v-if="selectedDomain !== 'custom' && savedDomains.includes(selectedDomain)"
                @click="deleteSavedDomain"
                class="btn-delete-domain"
                :title="$t('emailHelper.deleteDomain')"
              >
                ×
              </button>
              <input
                v-if="selectedDomain === 'custom'"
                v-model="config.emailDomain"
                type="text"
                class="form-input domain-input"
                :placeholder="$t('emailHelper.emailDomainPlaceholder')"
                @blur="saveConfig"
                @input="saveConfig"
              >
            </div>
          </div>

          <div class="form-group">
            <label>{{ $t('emailHelper.emailPrefix') }}:</label>
            <input
              v-model="config.emailPrefix"
              type="text"
              class="form-input"
              :placeholder="$t('emailHelper.emailPrefixPlaceholder')"
              @blur="saveConfig"
              @input="saveConfig"
            >
          </div>

          <div class="settings-actions">
            <button @click="saveSettingsWithToken" :disabled="!canGetToken || isGettingToken" class="btn primary">
              {{ isGettingToken ? $t('emailHelper.gettingToken') : $t('emailHelper.saveSettings') }}
            </button>
            <button @click="exportEmails" :disabled="emails.length === 0" class="btn secondary">
              {{ $t('emailHelper.exportEmails') }}
            </button>
            <button @click="importEmails" class="btn secondary">
              {{ $t('emailHelper.importEmails') }}
            </button>
            <button @click="clearAllEmails" :disabled="emails.length === 0" class="btn danger">
              {{ $t('emailHelper.clearAll') }}
            </button>
          </div>

          <!-- Token状态显示(不显示具体值) -->
          <div v-if="config.token" class="token-status">
            <span class="status-icon">✅</span>
            <span>{{ $t('emailHelper.tokenConfigured') }}</span>
          </div>
        </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- 信用卡设置对话框 - 使用 Teleport 渲染到 body -->
    <Teleport to="body">
      <Transition name="modal">
        <div v-if="showCardSettings" class="modal-overlay" @click="closeCardSettings">
          <div class="modal-content card-settings-modal" @click.stop>
            <div class="modal-header">
              <h3>{{ $t('emailHelper.cardSettings') }}</h3>
              <button @click="closeCardSettings" class="close-btn">×</button>
            </div>

            <div class="modal-body">
              <div class="form-group">
                <label>{{ $t('emailHelper.cardBin') }} <span class="required">*</span>:</label>
                <input
                  v-model="cardConfig.bin"
                  type="text"
                  class="form-input"
                  :placeholder="$t('emailHelper.cardBinPlaceholder')"
                  maxlength="12"
                >
                <small class="form-hint">{{ $t('emailHelper.cardBinHint') }}</small>
              </div>

              <div class="form-section-title">{{ $t('emailHelper.addressSettings') }}</div>
              <small class="form-hint">{{ $t('emailHelper.addressHint') }}</small>

              <div class="form-group">
                <label>{{ $t('emailHelper.country') }}:</label>
                <input
                  v-model="cardConfig.country"
                  type="text"
                  class="form-input"
                  :placeholder="$t('emailHelper.countryPlaceholder')"
                  maxlength="2"
                >
                <small class="form-hint">{{ $t('emailHelper.countryHint') }}</small>
              </div>

              <div class="form-group">
                <label>{{ $t('emailHelper.province') }}:</label>
                <input
                  v-model="cardConfig.province"
                  type="text"
                  class="form-input"
                  :placeholder="$t('emailHelper.provincePlaceholder')"
                >
              </div>

              <div class="form-group">
                <label>{{ $t('emailHelper.city') }}:</label>
                <input
                  v-model="cardConfig.city"
                  type="text"
                  class="form-input"
                  :placeholder="$t('emailHelper.cityPlaceholder')"
                >
              </div>

              <div class="form-group">
                <label>{{ $t('emailHelper.street') }}:</label>
                <input
                  v-model="cardConfig.street"
                  type="text"
                  class="form-input"
                  :placeholder="$t('emailHelper.streetPlaceholder')"
                >
              </div>

              <div class="form-group">
                <label>{{ $t('emailHelper.postalCode') }}:</label>
                <input
                  v-model="cardConfig.postalCode"
                  type="text"
                  class="form-input"
                  :placeholder="$t('emailHelper.postalCodePlaceholder')"
                  maxlength="10"
                >
              </div>

              <div class="modal-footer">
                <button @click="closeCardSettings" class="btn secondary">
                  {{ $t('emailHelper.cancel') }}
                </button>
                <button @click="saveCardSettings" class="btn primary">
                  {{ $t('emailHelper.save') }}
                </button>
              </div>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- 批量注册对话框 - 使用 Teleport 渲染到 body -->
    <Teleport to="body">
      <Transition name="modal">
        <div v-if="showBatchRegisterDialogVisible" class="modal-overlay" @click="closeBatchRegisterDialog">
          <div class="modal-content batch-register-modal" @click.stop>
            <div class="modal-header">
              <h3>批量注册账号</h3>
              <button @click="closeBatchRegisterDialog" class="close-btn">×</button>
            </div>

            <div class="modal-body">
              <div class="form-group">
                <label>注册数量 <span class="required">*</span>:</label>
                <input
                  v-model.number="batchRegisterCount"
                  type="number"
                  class="form-input"
                  placeholder="请输入注册数量"
                  min="1"
                  max="20"
                >
                <small class="form-hint">建议每次注册 1-10 个账号,注册过程可能失败,系统会自动跳过失败的账号</small>
              </div>

              <div v-if="isBatchRegistering" class="batch-progress">
                <div class="progress-info">
                  <span>正在注册第 {{ batchRegisterProgress.current }}/{{ batchRegisterProgress.total }} 个账号</span>
                </div>
                <div class="progress-bar">
                  <div
                    class="progress-fill"
                    :style="{ width: `${(batchRegisterProgress.current / batchRegisterProgress.total) * 100}%` }"
                  ></div>
                </div>
              </div>

              <div class="modal-footer">
                <button @click="closeBatchRegisterDialog" class="btn secondary" :disabled="isBatchRegistering">
                  取消
                </button>
                <button @click="startBatchRegister" class="btn primary" :disabled="isBatchRegistering || batchRegisterCount < 1 || batchRegisterCount > 20">
                  {{ isBatchRegistering ? '注册中...' : '开始注册' }}
                </button>
              </div>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

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

    <!-- 清空邮箱确认对话框 -->
    <div v-if="showClearAllDialog" class="clear-all-overlay" @click="cancelClearAll">
    <div class="clear-all-dialog" @click.stop>
      <div class="clear-all-header">
        <h3>{{ $t('emailHelper.clearAllTitle') }}</h3>
        <button @click="cancelClearAll" class="close-btn">×</button>
      </div>

      <div class="clear-all-body">
        <p class="clear-all-message">{{ $t('emailHelper.clearAllMessage') }}</p>
        <div class="clear-all-warning">
          ⚠️ {{ $t('emailHelper.confirmClearAll') }}
        </div>
      </div>

      <div class="clear-all-footer">
        <button @click="executeClearAll(false)" class="btn secondary">
          {{ $t('emailHelper.clearLocalOnly') }}
        </button>
        <button
          v-if="config.token"
          @click="executeClearAll(true)"
          class="btn danger"
        >
          {{ $t('emailHelper.clearLocalAndCloud') }}
        </button>
        <button @click="cancelClearAll" class="btn">
          {{ $t('common.cancel') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

const emit = defineEmits(['show-status'])

// i18n
const { t } = useI18n()

// 响应式数据
const emails = ref([])
const config = ref({
  serverUrl: '',
  adminEmail: '',
  adminPassword: '',
  token: '',
  emailDomain: '',
  emailPrefix: 'augment'
})



const isCreating = ref(false)
const isGettingToken = ref(false)
const isMonitoring = ref(false)
const monitoringEmail = ref('')
const verificationCode = ref('')
const showSettings = ref(false)
const monitorInterval = ref(null)
const monitorStartTime = ref(null) // 监控开始时间
const isGettingAugmentToken = ref(false) // 一键获取Token状态
const currentQuickGetEmail = ref('') // 当前正在获取Token的邮箱
let quickGetCleanup = null // 清理函数

// 一键注册相关
const isQuickRegistering = ref(false) // 一键注册状态
const currentQuickRegisterEmail = ref('') // 当前正在注册的邮箱
let quickRegisterCleanup = null // 清理函数

// 批量获取Token相关
const isBatchGettingTokens = ref(false) // 批量获取Token状态
const currentBatchIndex = ref(0) // 当前批量获取的索引

// 批量注册相关
const isBatchRegistering = ref(false) // 批量注册状态
const batchRegisterProgress = ref({ current: 0, total: 0 }) // 批量注册进度
const showBatchRegisterDialogVisible = ref(false) // 批量注册对话框显示状态
const batchRegisterCount = ref(5) // 批量注册数量

// Token列表(从主窗口获取)
const tokenList = ref([])

// 新增的响应式变量
const createType = ref('random') // 'random' 或 'custom'
const customEmailName = ref('')
const selectedDomain = ref('custom')
const savedDomains = ref([]) // 已保存的自定义域名列表

// 信用卡设置对话框状态
const showCardSettings = ref(false)

// 信用卡配置
const cardConfig = ref({
  bin: '515462002040', // 默认BIN,与 zhifu.js 一致
  country: '', // 国家代码,如 CN, US
  province: '', // 省份
  city: '', // 城市
  street: '', // 街道
  postalCode: '' // 邮编
})

// 邮件查看相关状态
const showEmailViewer = ref(false)
const showEmailContent = ref(false)
const currentViewingEmail = ref('')
const emailList = ref([])
const currentEmailContent = ref(null)
const isLoadingEmails = ref(false)
const isLoadingEmailContent = ref(false)

// 清空邮箱对话框状态
const showClearAllDialog = ref(false)

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

// 没有Token的邮箱列表(按创建时间正序排列,实时从Token列表中检查)
const emailsWithoutToken = computed(() => {
  return [...emails.value]
    .filter(email => !checkEmailHasToken(email.email))
    .sort((a, b) => {
      const timeA = new Date(a.created_at).getTime()
      const timeB = new Date(b.created_at).getTime()
      return timeA - timeB // 正序排列,从旧到新
    })
})

// 方法
const showStatus = (message, type = 'info') => {
  emit('show-status', message, type)
}

// 加载Token列表
const loadTokenList = async () => {
  try {
    const tokensJson = await invoke('load_tokens_json')
    // load_tokens_json 返回的是 JSON 字符串,需要解析
    const tokens = JSON.parse(tokensJson)

    // 根据邮箱备注(email_note)去重,保留最新的
    const uniqueTokens = []
    const emailNoteMap = new Map()

    // 遍历所有 token,使用 Map 去重
    if (Array.isArray(tokens)) {
      tokens.forEach(token => {
        if (token.email_note) {
          // 如果已存在相同 email_note,比较 updated_at 保留最新的
          if (emailNoteMap.has(token.email_note)) {
            const existing = emailNoteMap.get(token.email_note)
            const existingTime = new Date(existing.updated_at || existing.created_at || 0).getTime()
            const currentTime = new Date(token.updated_at || token.created_at || 0).getTime()

            // 保留时间更新的
            if (currentTime > existingTime) {
              emailNoteMap.set(token.email_note, token)
            }
          } else {
            emailNoteMap.set(token.email_note, token)
          }
        } else {
          // 如果没有 email_note,保留该 token
          uniqueTokens.push(token)
        }
      })

      // 将 Map 中的值添加到结果数组
      uniqueTokens.push(...emailNoteMap.values())
    }

    tokenList.value = uniqueTokens
    console.log('[EmailHelper] Loaded token list:', tokenList.value.length, 'tokens (after deduplication)')
  } catch (error) {
    console.error('[EmailHelper] Failed to load token list:', error)
    tokenList.value = []
  }
}

// 检查邮箱是否有Token(实时从Token列表中检查)
const checkEmailHasToken = (email) => {
  return tokenList.value.some(token => token.email_note === email)
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
    const prefix = config.value.emailPrefix || 'augment'
    return `${prefix}${result}@${domain}`
  }
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
    // 选择了已保存的域名
    config.value.emailDomain = selectedDomain.value
    saveConfig()
  } else if (selectedDomain.value === 'custom') {
    // 选择了自定义域名,清空当前值让用户输入
    if (!config.value.emailDomain || savedDomains.value.includes(config.value.emailDomain)) {
      config.value.emailDomain = ''
    }
  }
}

// 删除已保存的域名
const deleteSavedDomain = () => {
  if (selectedDomain.value && savedDomains.value.includes(selectedDomain.value)) {
    const index = savedDomains.value.indexOf(selectedDomain.value)
    savedDomains.value.splice(index, 1)
    saveSavedDomains()

    // 删除后切换到第一个已保存的域名，如果没有则切换到自定义
    if (savedDomains.value.length > 0) {
      selectedDomain.value = savedDomains.value[0]
      config.value.emailDomain = savedDomains.value[0]
    } else {
      selectedDomain.value = 'custom'
      config.value.emailDomain = ''
    }
    saveConfig()

    showStatus(t('emailHelper.domainDeleted'), 'success')
  }
}

// 保存信用卡配置
const saveCardConfig = async () => {
  const data = JSON.stringify(cardConfig.value)
  localStorage.setItem('emailHelper_cardConfig', data)

  // 同步到后端
  try {
    await invoke('set_card_bin', { bin: cardConfig.value.bin })
    await invoke('set_card_address', {
      country: cardConfig.value.country,
      province: cardConfig.value.province,
      city: cardConfig.value.city,
      street: cardConfig.value.street,
      postalCode: cardConfig.value.postalCode
    })
  } catch (error) {
    console.error('Failed to sync card config to backend:', error)
  }

  console.log('💾 已保存信用卡配置')
}

// 加载信用卡配置
const loadCardConfig = () => {
  try {
    const saved = localStorage.getItem('emailHelper_cardConfig')
    if (saved) {
      const savedConfig = JSON.parse(saved)
      cardConfig.value = {
        bin: savedConfig.bin || '515462002040',
        country: savedConfig.country || '',
        province: savedConfig.province || '',
        city: savedConfig.city || '',
        street: savedConfig.street || '',
        postalCode: savedConfig.postalCode || ''
      }
    }
  } catch (error) {
    console.error('Failed to load card config:', error)
    cardConfig.value = {
      bin: '515462002040',
      country: '',
      province: '',
      city: '',
      street: '',
      postalCode: ''
    }
  }
}

// 打开信用卡设置对话框
const openCardSettings = () => {
  console.log('💳 打开信用卡设置,当前状态:', showCardSettings.value)
  showCardSettings.value = true
  console.log('💳 信用卡设置状态已更新为:', showCardSettings.value)
}

// 关闭信用卡设置对话框
const closeCardSettings = () => {
  console.log('💳 关闭信用卡设置,当前状态:', showCardSettings.value)
  showCardSettings.value = false
  console.log('💳 信用卡设置状态已更新为:', showCardSettings.value)
}

// 保存信用卡设置
const saveCardSettings = async () => {
  if (!cardConfig.value.bin || cardConfig.value.bin.length < 6) {
    showStatus(t('emailHelper.cardBinRequired'), 'error')
    return
  }

  await saveCardConfig()
  showStatus(t('emailHelper.cardSettingsSaved'), 'success')
  closeCardSettings()
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
  console.log('💾 已保存邮箱列表,当前数量:', emails.value.length)
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
const saveConfig = async () => {
  const data = JSON.stringify(config.value)
  localStorage.setItem('emailHelper_config', data)
  // 触发强制更新
  config.value = JSON.parse(data)
  console.log('💾 已保存配置')
}

// 从本地存储加载配置
const loadConfig = () => {
  try {
    const saved = localStorage.getItem('emailHelper_config')
    if (saved) {
      const savedConfig = JSON.parse(saved)
      // 确保所有必要的字段都存在
      config.value = {
        serverUrl: savedConfig.serverUrl || '',
        adminEmail: savedConfig.adminEmail || '',
        adminPassword: savedConfig.adminPassword || '',
        token: savedConfig.token || '',
        emailDomain: savedConfig.emailDomain || '',
        emailPrefix: savedConfig.emailPrefix || 'augment'
      }
    }
  } catch (error) {
    // 如果配置损坏，重置为默认配置
    localStorage.removeItem('emailHelper_config')
    config.value = {
      serverUrl: '',
      adminEmail: '',
      adminPassword: '',
      token: '',
      emailDomain: '',
      emailPrefix: 'augment'
    }
  }
}

// 保存已保存的域名列表到本地存储
const saveSavedDomains = () => {
  localStorage.setItem('emailHelper_savedDomains', JSON.stringify(savedDomains.value))
}

// 从本地存储加载已保存的域名列表
const loadSavedDomains = () => {
  try {
    const saved = localStorage.getItem('emailHelper_savedDomains')
    if (saved) {
      savedDomains.value = JSON.parse(saved)
    }
  } catch (error) {
    console.error('Failed to load saved domains:', error)
    savedDomains.value = []
  }
}

// 添加域名到已保存列表
const addSavedDomain = (domain) => {
  // 确保域名格式正确（以@开头）
  const formattedDomain = domain.startsWith('@') ? domain : `@${domain}`

  // 检查是否已存在
  if (!savedDomains.value.includes(formattedDomain)) {
    savedDomains.value.push(formattedDomain)
    saveSavedDomains()
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

// 创建邮箱（为当前用户添加邮箱账户）
const createEmail = async () => {
  isCreating.value = true
  try {
    const email = generateEmail()

    showStatus(`${t('emailHelper.creatingEmail')}: ${email}`, 'info')

    // 使用用户API添加邮箱账户（只需要邮箱地址，不需要密码）
    const { result } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/user/account/add`, {
      method: 'POST',
      body: JSON.stringify({
        email: email
      })
    })

    if (result.code === 200) {
      const emailInfo = {
        email: email,
        created_at: new Date().toISOString(),
        verificationCodes: [], // 添加验证码数组
        accountId: result.data.accountId // 保存accountId用于后续删除
      }

      emails.value.push(emailInfo)
      // 强制触发响应式更新
      emails.value = [...emails.value]
      console.log('📧 创建邮箱后,当前邮箱列表:', emails.value.length, emails.value)
      saveEmailsList()
      showStatus(t('emailHelper.createSuccess'), 'success')

      // 如果使用的是自定义域名，保存该域名
      if (selectedDomain.value === 'custom' && config.value.emailDomain) {
        addSavedDomain(config.value.emailDomain)
      }

      // 清空自定义邮箱名称
      if (createType.value === 'custom') {
        customEmailName.value = ''
      }

      // 自动开始监控新创建的邮箱（不需要密码）
      startMonitoring(email, '')
    } else {
      // 创建失败，可能是邮箱已存在或其他原因
      const errorMsg = result.message || 'Create email failed'
      if (errorMsg.includes('exist') || errorMsg.includes('已存在') || errorMsg.includes('duplicate')) {
        throw new Error(`邮箱账户已存在于云端: ${email}`)
      } else {
        throw new Error(errorMsg)
      }
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

    // 如果有Token，先尝试删除云端邮箱账户
    if (config.value.token) {
      try {
        // 查找邮箱的accountId
        let emailInfo = emails.value.find(e => e.email === email)
        let accountId = emailInfo?.accountId

        // 如果没有accountId，先从服务器查询
        if (!accountId) {
          showStatus('正在查询邮箱账户ID...', 'info')
          const { result: listResult } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/user/account/list`, {
            method: 'GET'
          })

          if (listResult.code === 200 && listResult.data) {
            const serverAccount = listResult.data.find(acc => acc.email === email)
            if (serverAccount) {
              accountId = serverAccount.accountId
              // 更新本地记录
              if (emailInfo) {
                emailInfo.accountId = accountId
                saveEmailsList()
              }
            }
          }
        }

        if (!accountId) {
          // 查询后仍然没有accountId，说明云端不存在
          showStatus('云端未找到此邮箱账户，将只删除本地记录', 'warning')
        } else {
          const { result } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/user/account/delete?accountId=${accountId}`, {
            method: 'DELETE'
          })

          if (result.code !== 200) {
            // 云端删除失败，提示但继续删除本地
            showStatus(`云端删除失败: ${result.message}，将继续删除本地记录`, 'warning')
          } else {
            showStatus('云端邮箱账户已删除', 'success')
          }
        }
      } catch (error) {
        // 网络错误或其他异常，提示但继续删除本地
        showStatus(`云端删除出错: ${error.message}，将继续删除本地记录`, 'warning')
      }
    }

    // 云端删除成功或用户确认删除本地记录，从本地列表中移除
    const originalLength = emails.value.length
    console.log('🗑️ 删除前邮箱数量:', originalLength)
    emails.value = emails.value.filter(emailInfo => emailInfo.email !== email)
    // 强制触发响应式更新
    emails.value = [...emails.value]
    console.log('🗑️ 删除后邮箱数量:', emails.value.length, emails.value)

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

// 清空所有邮箱 - 显示选择对话框
const clearAllEmails = () => {
  showClearAllDialog.value = true
}

// 执行清空操作
const executeClearAll = async (clearCloud = false) => {
  // 关闭对话框
  showClearAllDialog.value = false

  try {
    const totalEmails = emails.value.length

    // 停止监控
    stopMonitoring()

    let cloudDeleteErrors = []

    // 如果选择清空云端且有Token，批量删除云端邮箱账户
    if (clearCloud && config.value.token) {
      showStatus(`${t('emailHelper.clearingLocalAndCloud')}`, 'info')

      // 先查询所有账户ID
      let serverAccounts = []
      try {
        const { result: listResult } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/user/account/list`, {
          method: 'GET'
        })
        if (listResult.code === 200 && listResult.data) {
          serverAccounts = listResult.data
        }
      } catch (error) {
        console.log('Failed to fetch account list:', error)
      }

      for (const emailInfo of emails.value) {
        try {
          let accountId = emailInfo.accountId

          // 如果没有accountId，从服务器查询结果中查找
          if (!accountId && serverAccounts.length > 0) {
            const serverAccount = serverAccounts.find(acc => acc.email === emailInfo.email)
            if (serverAccount) {
              accountId = serverAccount.accountId
              // 更新本地记录
              emailInfo.accountId = accountId
            }
          }

          if (!accountId) {
            cloudDeleteErrors.push(`${emailInfo.email}: 云端未找到此邮箱账户`)
            console.log('Account ID not found for email:', emailInfo.email)
            continue
          }

          const { result } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/user/account/delete?accountId=${accountId}`, {
            method: 'DELETE'
          })

          if (result.code !== 200) {
            cloudDeleteErrors.push(`${emailInfo.email}: ${result.message}`)
            console.log('Failed to delete email account from server:', emailInfo.email, result.message)
          }
        } catch (error) {
          cloudDeleteErrors.push(`${emailInfo.email}: ${error.message}`)
          console.log('Failed to delete email account from server:', emailInfo.email, error)
        }
      }

      // 如果有云端删除失败的邮箱，显示提示但继续删除本地
      if (cloudDeleteErrors.length > 0) {
        const errorMessage = `部分云端邮箱删除失败:\n${cloudDeleteErrors.slice(0, 3).join('\n')}${cloudDeleteErrors.length > 3 ? `\n...还有${cloudDeleteErrors.length - 3}个` : ''}\n\n将继续删除本地记录`
        showStatus(errorMessage, 'warning')
      }
    } else {
      // 只清空本地
      showStatus(`${t('emailHelper.clearingLocalOnly')}`, 'info')
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

// 取消清空操作
const cancelClearAll = () => {
  showClearAllDialog.value = false
}

// 开始监控验证码
const startMonitoring = async (email) => {
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

  // 通知主进程当前监控的邮箱
  try {
    await invoke('set_monitoring_email', { email })
    console.log('✅ 已通知主进程监控邮箱:', email)
  } catch (error) {
    console.error('Failed to set monitoring email:', error)
  }

  showStatus(`${t('emailHelper.startMonitoringEmail')}: ${email}`, 'info')

  // 开始定时检查
  monitorInterval.value = setInterval(async () => {
    await checkForVerificationCode(email)
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
const checkForVerificationCode = async (email) => {
  try {
    const { result } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/user/email/list`, {
      method: 'POST',
      body: JSON.stringify({
        toEmail: email,
        sendEmail: 'support@augmentcode.com',
        num: 1,
        size: 10, // 增加查询数量，确保能获取到最新邮件
        timeSort: 'desc',
        type: 0,
        isDel: 0
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
              // 强制触发响应式更新
              emails.value = [...emails.value]
              saveEmailsList() // 保存到本地存储
              console.log('✅ 验证码已添加到邮箱列表')
            } else {
              console.log('⚠️ 验证码已存在，跳过添加')
            }
          }

          verificationCode.value = code

          // 通知主进程验证码
          try {
            await invoke('set_verification_code', { code })
            console.log('✅ 已通知主进程验证码:', code)
          } catch (error) {
            console.error('Failed to set verification code:', error)
          }

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
    console.log('⚙️ 打开设置,当前状态:', showSettings.value)
    showSettings.value = true
    console.log('⚙️ 设置状态已更新为:', showSettings.value)
  } catch (error) {
    console.error('Failed to open settings:', error)
  }
}

// 关闭设置
const closeSettings = () => {
  try {
    console.log('⚙️ 关闭设置,当前状态:', showSettings.value)
    showSettings.value = false
    console.log('⚙️ 设置状态已更新为:', showSettings.value)
  } catch (error) {
    console.error('Failed to close settings:', error)
  }
}

// 保存设置
const saveSettings = () => {
  saveConfig()
  showStatus(t('emailHelper.settingsSaved'), 'success')
}

// 保存设置并获取Token（用户API Token）
const saveSettingsWithToken = async () => {
  isGettingToken.value = true
  try {
    // 获取用户API Token（普通用户权限）
    const response = await fetch(`${config.value.serverUrl}/api/user/token/generate`, {
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
const viewEmails = async (email) => {
  // 检查是否有token
  if (!config.value.token) {
    showStatus(t('emailHelper.tokenRequired'), 'error')
    return
  }

  // 清除该邮箱的旧验证码
  const emailIndex = emails.value.findIndex(e => e.email === email)
  if (emailIndex !== -1) {
    if (!emails.value[emailIndex].verificationCodes) {
      emails.value[emailIndex].verificationCodes = []
    }
    emails.value[emailIndex].verificationCodes = []
    // 强制触发响应式更新
    emails.value = [...emails.value]
    saveEmailsList()
    console.log('[ViewEmails] 已清除邮箱的旧验证码')
  }

  currentViewingEmail.value = email
  showEmailViewer.value = true
  isLoadingEmails.value = true
  emailList.value = []

  try {
    const { result } = await makeAuthenticatedRequest(`${config.value.serverUrl}/api/user/email/list`, {
      method: 'POST',
      body: JSON.stringify({
        toEmail: email,
        sendEmail: '', // 获取所有发件人的邮件
        num: 1,
        size: 50, // 获取更多邮件
        timeSort: 'desc',
        type: 0,
        isDel: 0
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
    // 邮件列表已包含完整内容，直接使用
    currentEmailContent.value = {
      from: email.from,
      subject: email.subject,
      date: email.date,
      content: email.content || '邮件内容为空'
    }
  } catch (error) {
    currentEmailContent.value = {
      from: email.from,
      subject: email.subject,
      date: email.date,
      content: `加载邮件内容失败: ${error.message}`
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

  // 生成导出内容（只导出邮箱地址，不导出密码）
  let exportContent = ''
  emails.value.forEach((emailInfo) => {
    exportContent += `${emailInfo.email}\n`
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

// 停止一键获取
const stopQuickGet = () => {
  if (quickGetCleanup) {
    quickGetCleanup()
  }
  isGettingAugmentToken.value = false
  currentQuickGetEmail.value = ''
  showStatus(t('emailHelper.quickGetStopped'), 'info')
}

// 一键获取Augment Token
const quickGetAugmentToken = async (email, isBatchMode = false) => {
  if (isGettingAugmentToken.value) return

  isGettingAugmentToken.value = true
  currentQuickGetEmail.value = email
  showStatus(t('emailHelper.startingQuickGet'), 'info')

  let timeoutId = null
  let handleSessionAutoImported = null

  // 清理函数
  quickGetCleanup = () => {
    if (timeoutId) clearTimeout(timeoutId)
    if (handleSessionAutoImported) {
      handleSessionAutoImported()
    }
    // 停止监控
    stopMonitoring()
  }

  try {
    // 1. 清除该邮箱的旧验证码
    const emailIndex = emails.value.findIndex(e => e.email === email)
    if (emailIndex !== -1) {
      if (!emails.value[emailIndex].verificationCodes) {
        emails.value[emailIndex].verificationCodes = []
      }
      emails.value[emailIndex].verificationCodes = []
      saveEmailsList()
      console.log('[QuickGet] 已清除邮箱的旧验证码')
    }

    // 2. 先设置监控邮箱(用于自动填充邮箱和验证码)
    await invoke('set_monitoring_email', { email })
    console.log('[QuickGet] Set monitoring email:', email)

    // 3. 开始监控验证码
    monitoringEmail.value = email
    isMonitoring.value = true
    monitorStartTime.value = new Date()
    await startMonitoring(email)
    console.log('[QuickGet] Started monitoring for verification codes')

    // 4. 通知主进程在人机验证通过后不要启动监控（因为我们已经手动启动了）
    await invoke('set_quick_get_mode', { email, password: '', registerOnly: false })

    // 5. 打开登录窗口
    console.log('[QuickGet] Opening login window...')
    await invoke('open_login_window', { shouldClearCache: true, isBatchMode })

    showStatus(t('emailHelper.waitingForLogin'), 'info')

    // 6. 等待 session cookie (通过监听事件)
    const sessionCookie = await new Promise((resolve, reject) => {
      timeoutId = setTimeout(() => {
        reject(new Error('Login timeout'))
      }, 120000) // 2分钟超时

      // 监听 session cookie 接收事件
      listen('session-cookie-received', (event) => {
        clearTimeout(timeoutId)
        if (handleSessionAutoImported) handleSessionAutoImported()
        resolve(event.payload)
      }).then(unlisten => {
        handleSessionAutoImported = unlisten
      })
    })

    console.log('[QuickGet] Session cookie received')
    showStatus(t('emailHelper.generatingToken'), 'info')

    // 7. 使用 session cookie 生成 token
    const tokenResult = await invoke('get_token_from_session_cookie', { sessionCookie })
    console.log('[QuickGet] Token generated:', tokenResult)

    // 8. 保存 token 到主窗口
    await invoke('save_token_from_email_helper', {
      tenantUrl: tokenResult.tenant_url,
      accessToken: tokenResult.access_token,
      portalUrl: tokenResult.portal_url || null,
      emailNote: tokenResult.email || email,
      sessionCookie: sessionCookie
    })

    // 9. 关闭登录窗口
    try {
      await invoke('close_login_window')
      console.log('[QuickGet] Login window closed')
      // 等待窗口完全关闭
      await new Promise(resolve => setTimeout(resolve, 1000))
    } catch (error) {
      console.warn('[QuickGet] Failed to close login window:', error)
    }

    // 10. 重新加载Token列表以更新UI
    await loadTokenList()

    showStatus(t('emailHelper.tokenSavedSuccess') + ' ✅', 'success')

    // 只在非批量获取模式下显示详细信息弹窗
    if (!isBatchGettingTokens.value) {
      setTimeout(() => {
        alert(`${t('emailHelper.tokenSavedSuccessDetail')}\n\n` +
              `${t('emailHelper.email')}: ${email}\n` +
              `Tenant URL: ${tokenResult.tenant_url}\n` +
              `Portal URL: ${tokenResult.portal_url || 'N/A'}`)
      }, 500)
    }
  } catch (error) {
    console.error('[QuickGet] Failed:', error)
    showStatus(t('emailHelper.quickGetFailed') + ': ' + error.message, 'error')
  } finally {
    // 确保关闭登录窗口
    try {
      await invoke('close_login_window')
      console.log('[QuickGet] Login window closed in finally block')
      // 等待窗口完全关闭
      await new Promise(resolve => setTimeout(resolve, 1000))
    } catch (error) {
      console.warn('[QuickGet] Failed to close login window in finally:', error)
    }

    quickGetCleanup()
    isGettingAugmentToken.value = false
    currentQuickGetEmail.value = ''
  }
}

// 一键注册功能（注册完成后自动获取Token）
const quickRegister = async (email) => {
  if (isQuickRegistering.value) return

  isQuickRegistering.value = true
  currentQuickRegisterEmail.value = email
  showStatus(t('emailHelper.startingQuickRegister'), 'info')

  let timeoutId = null
  let handleSessionCookie = null

  // 清理函数
  quickRegisterCleanup = () => {
    if (timeoutId) clearTimeout(timeoutId)
    if (handleSessionCookie) {
      handleSessionCookie()
    }
    // 停止监控
    stopMonitoring()
  }

  try {
    // 1. 清除该邮箱的旧验证码
    const emailIndex = emails.value.findIndex(e => e.email === email)
    if (emailIndex !== -1) {
      if (!emails.value[emailIndex].verificationCodes) {
        emails.value[emailIndex].verificationCodes = []
      }
      emails.value[emailIndex].verificationCodes = []
      saveEmailsList()
      console.log('[QuickRegister] 已清除邮箱的旧验证码')
    }

    // 2. 先设置当前监控邮箱(用于自动填充)
    await invoke('set_monitoring_email', { email })
    console.log('[QuickRegister] Set monitoring email:', email)

    // 3. 开始监控验证码（一键注册也需要监控验证码）
    monitoringEmail.value = email
    isMonitoring.value = true
    monitorStartTime.value = new Date()
    await startMonitoring(email)
    console.log('[QuickRegister] Started monitoring for verification codes')

    // 4. 通知主进程在人机验证通过后不要启动监控（因为我们已经手动启动了）
    // 注意：这里设置为 registerOnly: true，等待 onboarding 完成后再获取 token
    await invoke('set_quick_get_mode', { email, password: '', registerOnly: true })

    // 5. 打开登录窗口
    console.log('[QuickRegister] Opening login window...')
    await invoke('open_login_window', { shouldClearCache: true, isBatchMode: false })

    showStatus(t('emailHelper.waitingForRegistration'), 'info')

    // 6. 等待 session cookie (通过监听事件)
    const sessionCookie = await new Promise((resolve, reject) => {
      timeoutId = setTimeout(() => {
        reject(new Error('Registration timeout'))
      }, 70000) // 1分10秒超时(如果验证失败会卡住,不需要等太久)

      // 监听 session cookie 接收事件
      listen('session-cookie-received', (event) => {
        clearTimeout(timeoutId)
        if (handleSessionCookie) handleSessionCookie()
        resolve(event.payload)
      }).then(unlisten => {
        handleSessionCookie = unlisten
      })
    })

    console.log('[QuickRegister] Session cookie received')
    showStatus(t('emailHelper.generatingToken'), 'info')

    // 7. 使用 session cookie 生成 token
    const tokenResult = await invoke('get_token_from_session_cookie', { sessionCookie })
    console.log('[QuickRegister] Token generated:', tokenResult)

    // 8. 保存 token 到主窗口
    await invoke('save_token_from_email_helper', {
      tenantUrl: tokenResult.tenant_url,
      accessToken: tokenResult.access_token,
      portalUrl: tokenResult.portal_url || null,
      emailNote: tokenResult.email || email,
      sessionCookie: sessionCookie
    })

    // 9. 关闭登录窗口
    try {
      await invoke('close_login_window')
      console.log('[QuickRegister] Login window closed')
      // 等待窗口完全关闭
      await new Promise(resolve => setTimeout(resolve, 1000))
    } catch (error) {
      console.warn('[QuickRegister] Failed to close login window:', error)
    }

    // 10. 重新加载Token列表以更新UI
    await loadTokenList()

    // 只在非批量注册模式下显示详细信息
    if (!isBatchRegistering.value) {
      showStatus(t('emailHelper.registrationCompleted') + ' ✅', 'success')

      // 显示详细信息弹窗
      setTimeout(() => {
        alert(`${t('emailHelper.registrationCompletedDetail')}\n\n` +
              `${t('emailHelper.email')}: ${email}\n` +
              `Tenant URL: ${tokenResult.tenant_url}\n` +
              `Portal URL: ${tokenResult.portal_url || 'N/A'}`)
      }, 500)
    }

  } catch (error) {
    console.error('[QuickRegister] Failed:', error)

    // 如果是批量注册模式,抛出错误让批量注册函数处理
    if (isBatchRegistering.value) {
      // 添加邮箱信息到错误对象
      error.email = email
      throw error
    }

    showStatus(t('emailHelper.quickRegisterFailed') + ': ' + error.message, 'error')
  } finally {
    // 确保关闭登录窗口
    try {
      await invoke('close_login_window')
      console.log('[QuickRegister] Login window closed in finally block')
      // 等待窗口完全关闭
      await new Promise(resolve => setTimeout(resolve, 1000))
    } catch (error) {
      console.warn('[QuickRegister] Failed to close login window in finally:', error)
    }

    quickRegisterCleanup()
    isQuickRegistering.value = false
    currentQuickRegisterEmail.value = ''
  }
}

// 停止一键注册
const stopQuickRegister = () => {
  if (quickRegisterCleanup) {
    quickRegisterCleanup()
  }
  isQuickRegistering.value = false
  currentQuickRegisterEmail.value = ''
  showStatus(t('emailHelper.quickRegisterStopped'), 'info')
}

// 显示批量注册对话框
const showBatchRegisterDialog = () => {
  showBatchRegisterDialogVisible.value = true
}

// 关闭批量注册对话框
const closeBatchRegisterDialog = () => {
  if (!isBatchRegistering.value) {
    showBatchRegisterDialogVisible.value = false
  }
}

// 开始批量注册
const startBatchRegister = async () => {
  if (isBatchRegistering.value) return

  const count = batchRegisterCount.value
  if (count < 1 || count > 20) {
    showStatus('注册数量必须在 1-20 之间', 'error')
    return
  }

  isBatchRegistering.value = true
  batchRegisterProgress.value = { current: 0, total: count }

  const results = {
    total: count,
    success: 0,
    failed: 0,
    failedEmails: []
  }

  try {
    for (let i = 0; i < count; i++) {
      batchRegisterProgress.value.current = i + 1

      try {
        console.log(`[BatchRegister] Starting registration ${i + 1}/${count}`)

        // 创建临时邮箱
        const emailCountBefore = emails.value.length
        await createEmail()

        // 获取最新创建的邮箱
        if (emails.value.length <= emailCountBefore) {
          throw new Error('创建邮箱失败')
        }

        const email = emails.value[emails.value.length - 1].email
        console.log(`[BatchRegister] Created email: ${email}`)
        showStatus(`正在注册第 ${i + 1}/${count} 个账号: ${email}`, 'info')

        // 执行一键注册
        await quickRegister(email)

        // 成功
        results.success++
        console.log(`[BatchRegister] Registration ${i + 1}/${count} succeeded: ${email}`)

        // 等待窗口关闭
        await new Promise(resolve => setTimeout(resolve, 2000))

      } catch (error) {
        // 失败
        results.failed++
        const failedEmail = error.email || '未知邮箱'
        results.failedEmails.push({
          email: failedEmail,
          error: error.message
        })

        console.error(`[BatchRegister] Registration ${i + 1}/${count} failed:`, error)
        showStatus(`第 ${i + 1}/${count} 个账号注册失败: ${error.message}`, 'error')

        // 删除失败的邮箱
        if (failedEmail !== '未知邮箱') {
          try {
            console.log(`[BatchRegister] Deleting failed email: ${failedEmail}`)
            await deleteEmail(failedEmail)
            console.log(`[BatchRegister] Failed email deleted: ${failedEmail}`)
          } catch (deleteError) {
            console.error(`[BatchRegister] Failed to delete email ${failedEmail}:`, deleteError)
          }
        }

        // 确保窗口关闭
        try {
          await invoke('close_login_window')
          console.log('[BatchRegister] Login window closed after failure')
        } catch (e) {
          console.error('[BatchRegister] Failed to close login window:', e)
        }

        // 等待后继续下一个
        await new Promise(resolve => setTimeout(resolve, 2000))
      }
    }

    // 显示最终统计
    let message = `批量注册完成!\n成功: ${results.success}, 失败: ${results.failed}, 总计: ${results.total}`

    if (results.failedEmails.length > 0) {
      message += '\n\n失败的邮箱:\n'
      results.failedEmails.forEach(item => {
        message += `- ${item.email}: ${item.error}\n`
      })
    }

    showStatus(`批量注册完成! 成功: ${results.success}, 失败: ${results.failed}`, results.failed > 0 ? 'warning' : 'success')

    // 显示详细结果
    setTimeout(() => {
      alert(message)
    }, 500)

    // 重新加载邮箱列表
    loadEmailsList()

  } catch (error) {
    console.error('[BatchRegister] Batch register failed:', error)
    showStatus('批量注册失败: ' + error.message, 'error')
  } finally {
    isBatchRegistering.value = false
    batchRegisterProgress.value = { current: 0, total: 0 }
    showBatchRegisterDialogVisible.value = false
  }
}

// 批量获取所有Token
const batchGetAllTokens = async () => {
  if (isBatchGettingTokens.value) return

  const emailsToGet = emailsWithoutToken.value
  if (emailsToGet.length === 0) {
    showStatus('没有需要获取Token的邮箱', 'info')
    return
  }

  isBatchGettingTokens.value = true
  currentBatchIndex.value = 0
  let successCount = 0
  let failCount = 0

  try {
    for (let i = 0; i < emailsToGet.length; i++) {
      currentBatchIndex.value = i
      const emailInfo = emailsToGet[i]

      showStatus(`正在获取第 ${i + 1}/${emailsToGet.length} 个邮箱的Token: ${emailInfo.email}`, 'info')

      try {
        // 调用单个邮箱的快速获取Token方法,传递批量模式标志
        await quickGetAugmentToken(emailInfo.email, true)
        successCount++

        // 等待3秒再处理下一个邮箱
        if (i < emailsToGet.length - 1) {
          showStatus(`等待3秒后继续下一个邮箱...`, 'info')
          await new Promise(resolve => setTimeout(resolve, 3000))
        }
      } catch (error) {
        failCount++
        console.error(`[BatchGet] Failed to get token for ${emailInfo.email}:`, error)
        showStatus(`获取 ${emailInfo.email} 的Token失败: ${error.message},自动继续下一个`, 'error')

        // 等待2秒后继续下一个
        if (i < emailsToGet.length - 1) {
          await new Promise(resolve => setTimeout(resolve, 2000))
        }
      }
    }

    showStatus(`批量获取完成! 成功: ${successCount}, 失败: ${failCount}, 总计: ${emailsToGet.length}`, 'success')
  } catch (error) {
    console.error('[BatchGet] Batch get failed:', error)
    showStatus('批量获取失败: ' + error.message, 'error')
  } finally {
    isBatchGettingTokens.value = false
    currentBatchIndex.value = 0
  }
}

// 导入邮箱列表
const importEmails = () => {
  // 创建文件输入元素
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.txt'

  input.onchange = async (e) => {
    const file = e.target.files[0]
    if (!file) return

    try {
      const text = await file.text()
      const lines = text.split('\n').filter(line => line.trim())

      let importedCount = 0
      let skippedCount = 0

      for (const line of lines) {
        const email = line.trim()

        // 跳过空行
        if (!email) continue

        // 检查是否是有效的邮箱格式
        if (!email.includes('@')) {
          console.log('Invalid email format:', email)
          continue
        }

        // 检查是否已存在
        const exists = emails.value.some(e => e.email === email)
        if (!exists) {
          emails.value.push({
            email: email,
            created_at: new Date().toISOString(),
            verificationCodes: []
          })
          importedCount++
        } else {
          skippedCount++
        }
      }

      if (importedCount > 0) {
        saveEmailsList()
        const message = t('emailHelper.importSuccess').replace('{count}', importedCount).replace('{skipped}', skippedCount)
        showStatus(message, 'success')
      } else {
        showStatus(t('emailHelper.noEmailsImported'), 'warning')
      }
    } catch (error) {
      showStatus(t('emailHelper.importFailed') + ': ' + error.message, 'error')
    }
  }

  input.click()
}

// 重新加载所有数据
const reloadAllData = async () => {
  // 如果设置对话框或信用卡设置对话框或批量注册对话框打开,则不重新加载
  if (showSettings.value || showCardSettings.value || showBatchRegisterDialogVisible.value) {
    console.log('⏸️ 对话框打开中,跳过重新加载')
    return
  }

  loadConfig()
  loadEmailsList()
  loadSavedDomains()
  await loadTokenList()
  console.log('🔄 已重新加载所有数据')
}

// 生命周期
onMounted(async () => {
  console.log('🚀🚀🚀 EmailHelper 组件已挂载! 版本: 2025-10-25-v2')
  console.log('📍 当前时间:', new Date().toISOString())

  loadConfig()
  loadEmailsList()
  loadSavedDomains()
  loadCardConfig()

  // 加载Token列表
  await loadTokenList()

  // 清除所有邮箱的验证码
  emails.value.forEach(email => {
    if (!email.verificationCodes) {
      email.verificationCodes = []
    } else {
      email.verificationCodes = []
    }
  })
  saveEmailsList()
  console.log('🗑️ 已清除所有邮箱的旧验证码')

  // 初始化域名选择器
  if (config.value.emailDomain && savedDomains.value.includes(config.value.emailDomain)) {
    // 如果当前域名在已保存列表中，直接选中
    selectedDomain.value = config.value.emailDomain
  } else if (config.value.emailDomain) {
    // 如果有域名但不在已保存列表中，选择自定义
    selectedDomain.value = 'custom'
  } else if (savedDomains.value.length > 0) {
    // 如果没有配置域名但有已保存的域名，选择第一个
    selectedDomain.value = savedDomains.value[0]
    config.value.emailDomain = savedDomains.value[0]
    saveConfig()
  } else {
    // 否则默认选择自定义
    selectedDomain.value = 'custom'
  }

  // 监听一键获取模式下的开始监控事件
  listen('start-monitoring-from-quick-get', (event) => {
    const data = event.payload
    console.log('[QuickGet] Received start monitoring signal:', data)
    // 开始监控验证码
    monitoringEmail.value = data.email
    isMonitoring.value = true
    monitorStartTime.value = new Date()

    // 开始定时检查验证码
    monitorInterval.value = setInterval(async () => {
      await checkForVerificationCode(data.email)
    }, 5000)

    showStatus(t('emailHelper.startMonitoringEmail') + ': ' + data.email, 'info')
  })

  // 监听窗口焦点事件,重新加载数据
  // 注意:只在窗口真正失去焦点后再获得焦点时才重新加载
  let windowBlurred = false
  window.addEventListener('blur', () => {
    windowBlurred = true
  })
  window.addEventListener('focus', () => {
    if (windowBlurred) {
      windowBlurred = false
      reloadAllData()
    }
  })

  // 监听 storage 事件,当其他窗口修改数据时同步更新
  window.addEventListener('storage', (e) => {
    if (e.key === 'emailHelperEmails' || e.key === 'emailHelperConfig') {
      console.log('🔄 检测到数据变化,重新加载:', e.key)
      reloadAllData()
    }
  })
})

onBeforeUnmount(() => {
  if (monitorInterval.value) {
    clearInterval(monitorInterval.value)
  }
  // 移除事件监听器
  window.removeEventListener('focus', reloadAllData)
})
</script>

<style scoped>
/* 邮箱助手容器 - 铺满整个窗口 */
.email-helper {
  width: 100vw;
  height: 100vh;
  background: var(--color-surface, #ffffff);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* 头部 */
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-border, #e5e7eb);
  background: var(--color-surface-alt, #f9fafb);
  flex-shrink: 0;
}

.header h3 {
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

/* 主体内容区域 */
.body {
  flex: 1;
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
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.email-address {
  font-weight: 500;
  color: var(--color-text-primary, #374151);
  font-size: 14px;
  display: flex;
  align-items: center;
  gap: 8px;
}

/* 旧的token-badge样式已移除,现在使用token-badge-corner */

.email-address {
  font-weight: 500;
  color: var(--color-text-primary, #1f2937);
  margin-bottom: 8px;
}

.email-meta-actions {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.email-actions {
  display: flex;
  flex-direction: column;
  gap: 4px;
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

.btn.success {
  background: linear-gradient(135deg, #10b981, #059669);
  color: var(--color-text-inverse, #ffffff);
}

.btn.success:hover:not(:disabled) {
  background: linear-gradient(135deg, #059669, #047857);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(16, 185, 129, 0.3);
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
  z-index: 10006;
  display: flex;
  justify-content: center;
  align-items: center;
  backdrop-filter: blur(4px);
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

.token-status {
  margin-top: 16px;
  padding: 12px;
  background: #f0fdf4;
  border: 1px solid #22c55e;
  border-radius: 6px;
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: #166534;
}

.token-status .status-icon {
  font-size: 16px;
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

/* 域名输入组样式 */
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

/* 删除域名按钮 */
.btn-delete-domain {
  width: 32px;
  height: 32px;
  padding: 0;
  border: 1px solid var(--color-divider, #e2e8f0);
  background: var(--color-surface, #ffffff);
  color: var(--color-error, #ef4444);
  border-radius: 6px;
  cursor: pointer;
  font-size: 20px;
  font-weight: bold;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.btn-delete-domain:hover {
  background: var(--color-error, #ef4444);
  color: white;
  border-color: var(--color-error, #ef4444);
  transform: scale(1.05);
}

.btn-delete-domain:active {
  transform: scale(0.95);
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
  position: relative; /* 为右上角Token标签提供定位基准 */
}

.email-card:hover {
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1);
}

/* Token标签固定在右上角 */
.token-badge-corner {
  position: absolute;
  top: 8px;
  right: 8px;
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  background: #10b981;
  color: white;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  white-space: nowrap;
  z-index: 1;
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

/* 暗黑模式样式 */
[data-theme='dark'] .email-helper {
  background: var(--color-surface, #1e293b);
}

[data-theme='dark'] .header {
  background: var(--color-surface, #1e293b);
  border-bottom-color: var(--color-divider, #334155);
}

[data-theme='dark'] .header h3 {
  color: var(--color-text-primary, #cbd5e1);
}

[data-theme='dark'] .body {
  background: var(--color-surface, #1e293b);
}

[data-theme='dark'] .create-section,
[data-theme='dark'] .emails-section {
  background: var(--color-surface-muted, #0f172a);
  border-color: var(--color-divider, #334155);
}

[data-theme='dark'] .config-notice {
  background: rgba(251, 191, 36, 0.1);
  color: var(--color-warning, #fbbf24);
  border-color: rgba(251, 191, 36, 0.3);
}

[data-theme='dark'] .form-select,
[data-theme='dark'] .form-input,
[data-theme='dark'] .domain-select,
[data-theme='dark'] .domain-input {
  background: var(--color-surface, #1e293b);
  color: var(--color-text-primary, #cbd5e1);
  border-color: var(--color-divider, #334155);
}

[data-theme='dark'] .form-select:focus,
[data-theme='dark'] .form-input:focus,
[data-theme='dark'] .domain-select:focus,
[data-theme='dark'] .domain-input:focus {
  border-color: var(--color-accent, #3b82f6);
  background: var(--color-surface-hover, #334155);
}

[data-theme='dark'] .btn-delete-domain {
  background: var(--color-surface, #1e293b);
  border-color: var(--color-divider, #334155);
  color: var(--color-error, #f87171);
}

[data-theme='dark'] .btn-delete-domain:hover {
  background: var(--color-error, #ef4444);
  color: white;
  border-color: var(--color-error, #ef4444);
}

[data-theme='dark'] .section-header h4 {
  color: var(--color-text-primary, #cbd5e1);
}

[data-theme='dark'] .empty-state,
[data-theme='dark'] .loading-state {
  color: var(--color-text-muted, #64748b);
}

[data-theme='dark'] .empty-hint {
  color: var(--color-text-muted, #64748b);
}

[data-theme='dark'] .email-card {
  background: var(--color-surface, #1e293b);
  border-color: var(--color-divider, #334155);
}

[data-theme='dark'] .email-card:hover {
  background: var(--color-surface-hover, #334155);
  border-color: var(--color-accent, #3b82f6);
}

[data-theme='dark'] .email-address {
  color: var(--color-text-primary, #cbd5e1);
}

[data-theme='dark'] .email-password {
  color: var(--color-text-muted, #94a3b8);
}

[data-theme='dark'] .email-meta {
  color: var(--color-text-muted, #64748b);
}

[data-theme='dark'] .email-count {
  background: rgba(59, 130, 246, 0.2);
  color: var(--color-accent, #60a5fa);
}

[data-theme='dark'] .btn.secondary {
  background: var(--color-surface-hover, #334155);
  color: var(--color-text-primary, #cbd5e1);
  border-color: var(--color-divider, #334155);
}

[data-theme='dark'] .btn.secondary:hover {
  background: var(--color-surface-hover, #475569);
  border-color: var(--color-accent, #3b82f6);
}

[data-theme='dark'] .btn.warning {
  background: var(--color-warning, #f59e0b);
  color: var(--color-text-inverse, #ffffff);
}

[data-theme='dark'] .btn.warning:hover {
  background: var(--color-warning-hover, #d97706);
}

[data-theme='dark'] .settings-overlay {
  background: rgba(0, 0, 0, 0.7);
}

[data-theme='dark'] .settings-content {
  background: var(--color-surface, #1e293b);
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
}

[data-theme='dark'] .settings-modal-content {
  background: var(--color-surface, #1e293b);
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
}

[data-theme='dark'] .settings-header {
  background: var(--color-surface, #1e293b);
  border-bottom-color: var(--color-divider, #334155);
}

[data-theme='dark'] .settings-header h3 {
  color: var(--color-text-primary, #cbd5e1);
}

[data-theme='dark'] .close-btn {
  color: var(--color-text-muted, #94a3b8);
}

[data-theme='dark'] .close-btn:hover {
  background: var(--color-surface-muted, #334155);
  color: var(--color-text-heading, #e2e8f0);
}

[data-theme='dark'] .settings-body {
  background: var(--color-surface, #1e293b);
}

[data-theme='dark'] .settings-actions {
  background: var(--color-surface, #1e293b);
  border-top-color: var(--color-divider, #334155);
}

[data-theme='dark'] .token-display {
  background: var(--color-surface-muted, #0f172a);
  border-color: var(--color-divider, #334155);
}

[data-theme='dark'] .token-display label {
  color: var(--color-text-primary, #cbd5e1);
}

[data-theme='dark'] .token-value {
  background: var(--color-surface, #1e293b);
  color: var(--color-text-muted, #94a3b8);
}

[data-theme='dark'] .form-group label {
  color: var(--color-text-primary, #cbd5e1);
}

[data-theme='dark'] .help-text {
  color: var(--color-text-muted, #64748b);
}

[data-theme='dark'] .email-viewer-content,
[data-theme='dark'] .email-content-modal {
  background: var(--color-surface, #1e293b);
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
}

[data-theme='dark'] .email-viewer-header,
[data-theme='dark'] .email-content-header {
  background: var(--color-surface, #1e293b);
  border-bottom-color: var(--color-divider, #334155);
}

[data-theme='dark'] .email-viewer-header h3,
[data-theme='dark'] .email-content-header h3 {
  color: var(--color-text-primary, #cbd5e1);
}

[data-theme='dark'] .email-viewer-body,
[data-theme='dark'] .email-content-body {
  background: var(--color-surface, #1e293b);
}

[data-theme='dark'] .email-item {
  background: var(--color-surface-muted, #0f172a);
  border-color: var(--color-divider, #334155);
}

[data-theme='dark'] .email-item:hover {
  background: var(--color-surface-hover, #334155);
  border-color: var(--color-accent, #3b82f6);
}

[data-theme='dark'] .email-from {
  color: var(--color-text-primary, #cbd5e1);
}

[data-theme='dark'] .email-time,
[data-theme='dark'] .email-subject {
  color: var(--color-text-muted, #64748b);
}

[data-theme='dark'] .email-meta-info {
  background: var(--color-surface-muted, #0f172a);
  border-color: var(--color-divider, #334155);
}

[data-theme='dark'] .email-content-html {
  background: var(--color-surface-muted, #0f172a);
  border-color: var(--color-divider, #334155);
  color: var(--color-text-primary, #cbd5e1);
}

[data-theme='dark'] .email-content-footer {
  background: var(--color-surface, #1e293b);
  border-top-color: var(--color-divider, #334155);
}

/* 清空邮箱对话框样式 */
.clear-all-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10004;
  backdrop-filter: blur(4px);
}

.clear-all-dialog {
  background: var(--color-bg, #ffffff);
  border-radius: 12px;
  width: 90%;
  max-width: 500px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  animation: slideIn 0.3s ease-out;
}

.clear-all-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-divider, #e2e8f0);
}

.clear-all-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary, #1e293b);
}

.clear-all-body {
  padding: 24px;
}

.clear-all-message {
  margin: 0 0 16px 0;
  font-size: 15px;
  color: var(--color-text-primary, #1e293b);
  line-height: 1.6;
}

.clear-all-warning {
  padding: 12px 16px;
  background: #fef3c7;
  border: 1px solid #fbbf24;
  border-radius: 8px;
  color: #92400e;
  font-size: 14px;
  line-height: 1.5;
}

.clear-all-footer {
  display: flex;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid var(--color-divider, #e2e8f0);
  justify-content: flex-end;
}

.clear-all-footer .btn {
  min-width: 100px;
}

/* 暗色主题 */
[data-theme='dark'] .clear-all-overlay {
  background: rgba(0, 0, 0, 0.7);
}

[data-theme='dark'] .clear-all-dialog {
  background: var(--color-surface, #1e293b);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
}

[data-theme='dark'] .clear-all-header {
  border-bottom-color: var(--color-divider, #334155);
}

[data-theme='dark'] .clear-all-header h3 {
  color: var(--color-text-primary, #cbd5e1);
}

[data-theme='dark'] .clear-all-message {
  color: var(--color-text-primary, #cbd5e1);
}

[data-theme='dark'] .clear-all-warning {
  background: rgba(251, 191, 36, 0.1);
  border-color: rgba(251, 191, 36, 0.3);
  color: #fbbf24;
}

[data-theme='dark'] .clear-all-footer {
  border-top-color: var(--color-divider, #334155);
}

/* 信用卡设置对话框样式 */
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
  z-index: 10005;
  backdrop-filter: blur(4px);
}

.modal-content {
  background: var(--color-bg, #ffffff);
  border-radius: 12px;
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  animation: slideIn 0.3s ease-out;
}

.card-settings-modal {
  max-width: 500px;
}

.batch-register-modal {
  max-width: 500px;
}

.batch-progress {
  margin-top: 20px;
  padding: 16px;
  background: var(--color-bg-secondary, #f8fafc);
  border-radius: 8px;
}

.progress-info {
  margin-bottom: 12px;
  font-size: 14px;
  color: var(--color-text-primary, #1e293b);
  font-weight: 500;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: var(--color-divider, #e2e8f0);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #10b981, #059669);
  border-radius: 4px;
  transition: width 0.3s ease;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-divider, #e2e8f0);
  position: sticky;
  top: 0;
  background: var(--color-bg, #ffffff);
  z-index: 1;
  border-radius: 12px 12px 0 0;
}

.modal-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary, #1e293b);
}

.modal-body {
  padding: 24px;
}

.modal-footer {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  margin-top: 24px;
  padding-top: 16px;
  border-top: 1px solid var(--color-divider, #e2e8f0);
}

.form-section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary, #1e293b);
  margin: 24px 0 12px 0;
  padding-bottom: 8px;
  border-bottom: 2px solid var(--color-primary, #3b82f6);
}

.form-hint {
  display: block;
  margin-top: 4px;
  font-size: 12px;
  color: var(--color-text-muted, #64748b);
  line-height: 1.4;
}

.required {
  color: #ef4444;
  margin-left: 2px;
}

/* Modal 动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-active .modal-content,
.modal-leave-active .modal-content {
  transition: transform 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-content,
.modal-leave-to .modal-content {
  transform: scale(0.9) translateY(-20px);
}

@keyframes slideIn {
  from {
    transform: scale(0.9) translateY(-20px);
    opacity: 0;
  }
  to {
    transform: scale(1) translateY(0);
    opacity: 1;
  }
}

/* 暗色主题 */
[data-theme='dark'] .modal-overlay {
  background: rgba(0, 0, 0, 0.7);
}

[data-theme='dark'] .modal-content {
  background: var(--color-surface, #1e293b);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
}

[data-theme='dark'] .modal-header {
  background: var(--color-surface, #1e293b);
  border-bottom-color: var(--color-divider, #334155);
}

[data-theme='dark'] .modal-header h3 {
  color: var(--color-text-primary, #cbd5e1);
}

[data-theme='dark'] .modal-footer {
  border-top-color: var(--color-divider, #334155);
}

[data-theme='dark'] .form-section-title {
  color: var(--color-text-primary, #cbd5e1);
  border-bottom-color: var(--color-primary, #60a5fa);
}

[data-theme='dark'] .form-hint {
  color: var(--color-text-muted, #64748b);
}
</style>



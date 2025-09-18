<template>
  <div class="app">
    <!-- Header -->
    <header class="app-header">
      <div class="header-left">
        <h1>Augment Token Manager</h1>
        <!-- External Link buttons -->
        <div class="external-links-group">
          <button @click="showAppHomeDialog = true" class="btn app-home-btn">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
              <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"/>
            </svg>
            {{ $t('app.appHome') }}
          </button>
          <button @click="showPluginHomeDialog = true" class="btn plugin-home-btn">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
              <path d="M20.5 11H19V7c0-1.1-.9-2-2-2h-4V3.5C13 2.12 11.88 1 10.5 1S8 2.12 8 3.5V5H4c-1.1 0-2 .9-2 2v3.8H3.5c1.49 0 2.7 1.21 2.7 2.7s-1.21 2.7-2.7 2.7H2V20c0 1.1.9 2 2 2h3.8v-1.5c0-1.49 1.21-2.7 2.7-2.7 1.49 0 2.7 1.21 2.7 2.7V22H17c1.1 0 2-.9 2-2v-4h1.5c1.38 0 2.5-1.12 2.5-2.5S21.88 11 20.5 11z"/>
            </svg>
            {{ $t('app.pluginHome') }}
          </button>
        </div>
      </div>
      <div class="header-buttons">
        <!-- Feature buttons -->
        <button @click="showBookmarkManager = true" class="btn secondary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M17 3H7c-1.1 0-1.99.9-1.99 2L5 21l7-3 7 3V5c0-1.1-.9-2-2-2z"/>
          </svg>
          {{ $t('app.bookmarkManager') }}
        </button>
        <button @click="showOutlookManager = true" class="btn warning">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"/>
          </svg>
          {{ $t('app.outlookManager') }}
        </button>

        <button @click="showTokenList = true" class="btn primary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z"/>
          </svg>
          {{ $t('app.viewTokens') }}
        </button>

      </div>
    </header>

    <!-- Main Content -->
    <main class="main-content">
      <div class="token-generator-main">
        <div class="generator-header">
          <div class="header-content">
            <div class="title-section">
              <h2>{{ $t('tokenGenerator.title') }}</h2>
              <p>{{ $t('tokenGenerator.description') }}</p>
            </div>

          </div>
        </div>

        <div class="generator-body">
          <!-- Step 1: Generate Authorization URL -->
          <div class="section">
            <h3>{{ $t('tokenGenerator.step1') }}</h3>
            <button
              @click="generateAuthUrl"
              :class="['btn', 'primary', { loading: isGenerating }]"
              :disabled="isGenerating"
            >
              {{ $t('tokenGenerator.generateUrl') }}
            </button>

            <div v-if="authUrl" class="url-section">
              <label>{{ $t('tokenGenerator.authUrlLabel') }}</label>
              <div class="input-with-copy">
                <input
                  type="text"
                  :value="authUrl"
                  readonly
                  ref="authUrlInput"
                  :placeholder="$t('tokenGenerator.authUrlPlaceholder')"
                >
                <button @click="copyAuthUrl" class="copy-icon-btn" :title="$t('tokenGenerator.copyUrl')">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
                  </svg>
                </button>
              </div>
              <div class="button-container">
                <button @click="showAuthUrlDialog = true" class="btn secondary">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
                  </svg>
                  {{ $t('tokenGenerator.openAuthUrl') }}
                </button>
              </div>
            </div>
          </div>

          <!-- Step 2: Enter Authorization Code -->
          <div class="section">
            <h3>{{ $t('tokenGenerator.step2') }}</h3>
            <textarea
              v-model="authCode"
              :placeholder="$t('tokenGenerator.authCodePlaceholder')"
              rows="4"
            ></textarea>
            <div class="button-container">
              <button
                @click="getAccessToken"
                :class="['btn', 'primary', { loading: isGettingToken }]"
                :disabled="!canGetToken || isGettingToken"
              >
                {{ $t('tokenGenerator.getToken') }}
              </button>
            </div>
          </div>

          <!-- Step 3: Access Token -->
          <div class="section" v-if="tokenResult">
            <h3>{{ $t('tokenGenerator.step3') }}</h3>
            <div class="token-section">
              <div class="result-container">
                <label>{{ $t('tokenGenerator.accessTokenLabel') }}</label>
                <div class="token-container">
                  <input
                    type="text"
                    :value="tokenResult.access_token"
                    readonly
                    ref="accessTokenInput"
                  >
                  <button @click="copyAccessToken" class="btn secondary">{{ $t('tokenGenerator.copy') }}</button>
                </div>
              </div>
              <div class="result-container">
                <label>{{ $t('tokenGenerator.tenantUrlLabel') }}</label>
                <div class="token-container">
                  <input
                    type="text"
                    :value="tokenResult.tenant_url"
                    readonly
                    ref="tenantUrlInput"
                  >
                  <button @click="copyTenantUrl" class="btn secondary">{{ $t('tokenGenerator.copy') }}</button>
                </div>
              </div>

              <!-- Additional Fields -->
              <div class="additional-fields">
                <div class="field-container">
                  <label>{{ $t('tokenGenerator.portalUrl') }}:</label>
                  <input
                    type="text"
                    v-model="portalUrl"
                    :placeholder="$t('tokenGenerator.portalUrlPlaceholder')"
                    class="field-input"
                  >
                </div>
                <div class="field-container">
                  <label>{{ $t('tokenGenerator.emailNote') }}:</label>
                  <input
                    type="text"
                    v-model="emailNote"
                    :placeholder="$t('tokenGenerator.emailNotePlaceholder')"
                    class="field-input"
                  >
                </div>
              </div>

              <div class="button-container">
                <button @click="saveToken" class="btn success">{{ $t('tokenGenerator.saveToken') }}</button>
              </div>
            </div>
          </div>
        </div>


      </div>
    </main>

    <!-- Token List Modal -->
    <TokenList
      v-if="showTokenList"
      :tokens="tokens"
      :isLoading="isLoading"
      :hasUnsavedChanges="hasUnsavedChanges"
      :isDatabaseAvailable="isDatabaseAvailable"
      @close="showTokenList = false"
      @delete="deleteToken"
      @copy-success="showStatus"
      @add-token="showTokenForm"
      @refresh="(showMessage) => loadTokens(showMessage)"
      @open-portal="handleOpenPortal"
      @edit="handleEditToken"
      @save="saveTokensToFile"
      @token-updated="hasUnsavedChanges = true"
      @storage-config-changed="handleStorageConfigChanged"
    />

    <!-- Token Form Modal -->
    <TokenForm
      v-if="showTokenFormModal"
      :token="editingToken"
      @close="closeTokenForm"
      @success="handleTokenFormSuccess"
      @show-status="showStatus"
      @update-token="handleUpdateToken"
      @add-token="handleAddTokenFromForm"
    />

    <!-- Portal打开方式选择对话框 -->
    <div v-if="showPortalDialog" class="portal-dialog-overlay" @click="showPortalDialog = false">
      <div class="portal-dialog" @click.stop>
        <h3>{{ $t('dialogs.selectOpenMethod') }}</h3>
        <div class="dialog-buttons">
          <button @click="copyPortalUrl" class="dialog-btn copy">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
            </svg>
            {{ $t('dialogs.copyLink') }}
          </button>
          <button @click="openPortalExternal" class="dialog-btn external">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
            </svg>
            {{ $t('dialogs.openExternal') }}
          </button>
          <button @click="openPortalInternal" class="dialog-btn internal">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
            </svg>
            {{ $t('dialogs.openInternal') }}
          </button>
          <button @click="showPortalDialog = false" class="dialog-btn cancel">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>
            {{ $t('dialogs.dontOpen') }}
          </button>
        </div>
      </div>
    </div>
    <!-- 删除确认对话框 -->
    <div v-if="showDeleteConfirm" class="portal-dialog-overlay" @click="cancelDelete">
      <div class="portal-dialog delete-confirm" @click.stop>
        <h3>{{ $t('dialogs.confirmDelete') }}</h3>
        <p>{{ $t('dialogs.confirmDeleteMessage') }}</p>
        <div class="dialog-buttons">
          <button @click="cancelDelete" class="dialog-btn cancel">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>
            {{ $t('dialogs.cancel') }}
          </button>
          <button @click="confirmDelete" class="dialog-btn delete">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
            </svg>
            {{ $t('dialogs.delete') }}
          </button>
        </div>
      </div>
    </div>





    <!-- Bookmark Manager Modal -->
    <BookmarkManager
      v-if="showBookmarkManager"
      @close="showBookmarkManager = false"
    />

    <!-- Outlook Manager Modal -->
    <OutlookManager
      v-if="showOutlookManager"
      @close="showOutlookManager = false"
      @show-status="showStatus"
    />



    <!-- Status Messages -->
    <div
      v-if="statusMessage"
      :class="['status-toast', statusType]"
    >
      {{ statusMessage }}
    </div>

    <!-- 授权URL打开方式选择对话框 -->
    <div v-if="showAuthUrlDialog" class="portal-dialog-overlay" @click="showAuthUrlDialog = false">
      <div class="portal-dialog" @click.stop>
        <h3>{{ $t('dialogs.selectOpenMethod') }}</h3>
        <div class="dialog-buttons">
          <button @click="openAuthUrlExternal" class="dialog-btn external">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
            </svg>
            {{ $t('dialogs.openExternal') }}
          </button>
          <button @click="openAuthUrlInternal" class="dialog-btn internal">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
            </svg>
            {{ $t('dialogs.openInternal') }}
          </button>
          <button @click="showAuthUrlDialog = false" class="dialog-btn cancel">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>
            {{ $t('dialogs.cancel') }}
          </button>
        </div>
      </div>
    </div>

    <!-- App主页打开方式选择对话框 -->
    <div v-if="showAppHomeDialog" class="portal-dialog-overlay" @click="showAppHomeDialog = false">
      <div class="portal-dialog" @click.stop>
        <h3>{{ $t('dialogs.appHomeTitle') }}</h3>
        <div class="dialog-buttons">
          <button @click="openAppHomeExternal" class="dialog-btn external">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
            </svg>
            {{ $t('dialogs.openExternal') }}
          </button>
          <button @click="openAppHomeInternal" class="dialog-btn internal">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
            </svg>
            {{ $t('dialogs.openInternal') }}
          </button>
          <button @click="showAppHomeDialog = false" class="dialog-btn cancel">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>
            {{ $t('dialogs.cancel') }}
          </button>
        </div>
      </div>
    </div>

    <!-- 插件主页打开方式选择对话框 -->
    <div v-if="showPluginHomeDialog" class="portal-dialog-overlay" @click="showPluginHomeDialog = false">
      <div class="portal-dialog" @click.stop>
        <h3>{{ $t('dialogs.pluginHomeTitle') }}</h3>
        <div class="dialog-buttons">
          <button @click="openPluginHomeExternal" class="dialog-btn external">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
            </svg>
            {{ $t('dialogs.openExternal') }}
          </button>
          <button @click="openPluginHomeInternal" class="dialog-btn internal">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
            </svg>
            {{ $t('dialogs.openInternal') }}
          </button>
          <button @click="showPluginHomeDialog = false" class="dialog-btn cancel">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
            </svg>
            {{ $t('dialogs.cancel') }}
          </button>
        </div>
      </div>
    </div>

    <!-- 固定在右下角的控制按钮 -->
    <div class="fixed-controls">
      <!-- 弹出的设置选项 -->
      <div v-if="showSettingsMenu" class="settings-menu">
        <!-- 语言切换按钮 -->
        <button
          type="button"
          class="control-btn language-toggle"
          @click="toggleLanguage"
          :aria-label="languageToggleLabel"
          :title="languageToggleLabel"
        >
          {{ currentLocale === 'zh-CN' ? 'EN' : '中' }}
        </button>
        <!-- 主题切换按钮 -->
        <button
          type="button"
          class="control-btn theme-toggle"
          @click="toggleTheme"
          :aria-pressed="isDarkTheme"
          :aria-label="themeToggleLabel"
          :title="themeToggleLabel"
        >
          <svg v-if="isDarkTheme" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
          </svg>
          <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="5"/>
            <path d="m12 1 0 2"/>
            <path d="m12 21 0 2"/>
            <path d="m4.22 4.22 1.42 1.42"/>
            <path d="m18.36 18.36 1.42 1.42"/>
            <path d="m1 12 2 0"/>
            <path d="m21 12 2 0"/>
            <path d="m4.22 19.78 1.42-1.42"/>
            <path d="m18.36 5.64 1.42-1.42"/>
          </svg>
        </button>
      </div>

      <!-- 设置按钮 -->
      <button
        type="button"
        class="control-btn settings-toggle"
        @click="toggleSettingsMenu"
        :aria-label="$t('app.settings')"
        :title="$t('app.settings')"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed, inject, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import TokenCard from './components/TokenCard.vue'
import TokenList from './components/TokenList.vue'
import TokenForm from './components/TokenForm.vue'
import BookmarkManager from './components/BookmarkManager.vue'
import OutlookManager from './components/OutlookManager.vue'

const { t, locale } = useI18n()

// 当前语言
const currentLocale = ref(locale.value)

// 切换语言
const changeLanguage = () => {
  locale.value = currentLocale.value
  // 可以在这里添加保存语言偏好到本地存储的逻辑
  localStorage.setItem('preferred-language', currentLocale.value)
}

// 语言切换按钮
const toggleLanguage = () => {
  currentLocale.value = currentLocale.value === 'zh-CN' ? 'en-US' : 'zh-CN'
  changeLanguage()
}

const languageToggleLabel = computed(() => (currentLocale.value === 'zh-CN' ? t('app.switchToEnglish') : t('app.switchToChinese')))

// 设置菜单切换
const toggleSettingsMenu = () => {
  showSettingsMenu.value = !showSettingsMenu.value
}

// 点击外部区域关闭设置菜单
const handleClickOutside = (event) => {
  const fixedControls = document.querySelector('.fixed-controls')
  if (fixedControls && !fixedControls.contains(event.target)) {
    showSettingsMenu.value = false
  }
}

// 简化的状态管理
const tokens = ref([])
const isLoading = ref(false)
const showTokenList = ref(false)
const showBookmarkManager = ref(false)
const showOutlookManager = ref(false)
const statusMessage = ref('')
const statusType = ref('info')
const hasUnsavedChanges = ref(false)

// 存储状态管理
const isDatabaseAvailable = ref(false)

// 简化的工具函数
const createNewToken = (tenantUrl, accessToken, portalUrl = null, emailNote = null) => {
  return {
    id: crypto.randomUUID(),
    tenant_url: tenantUrl,
    access_token: accessToken,
    created_at: new Date().toISOString(),
    portal_url: portalUrl,
    ban_status: null,
    portal_info: null,
    email_note: emailNote
  }
}

// 移除了复杂的自动保存、错误处理和回滚机制，保持简单

// Token generator data
const authUrl = ref('')
const authCode = ref('')
const tokenResult = ref(null)
const isGenerating = ref(false)
const isGettingToken = ref(false)
const portalUrl = ref('')
const emailNote = ref('')

// Template refs
const authUrlInput = ref(null)
const accessTokenInput = ref(null)
const tenantUrlInput = ref(null)



// Portal dialog
const showPortalDialog = ref(false)
const currentPortalToken = ref(null)

const themeManager = inject('themeManager', null)
const themeStorageKey = themeManager?.storageKey ?? 'atm-theme'

let storedThemePreference = null
try {
  storedThemePreference = themeManager?.getStoredTheme?.() ?? localStorage.getItem(themeStorageKey) ?? null
} catch (error) {
  console.warn('Failed to read stored theme preference inside App.vue', error)
}

const hasManualThemePreference = ref(storedThemePreference === 'dark' || storedThemePreference === 'light')
const currentTheme = ref(themeManager?.initialTheme ?? document.documentElement.dataset.theme ?? 'light')
const isDarkTheme = computed(() => currentTheme.value === 'dark')

const fallbackApplyTheme = (theme) => {
  const normalized = theme === 'dark' ? 'dark' : 'light'
  const root = document.documentElement
  root.dataset.theme = normalized
  root.style.colorScheme = normalized
}

const setTheme = (nextTheme, options = {}) => {
  const normalized = nextTheme === 'dark' ? 'dark' : 'light'
  currentTheme.value = normalized

  if (themeManager?.applyTheme) {
    themeManager.applyTheme(normalized)
  } else {
    fallbackApplyTheme(normalized)
  }

  if (options.persist === false) {
    return
  }

  try {
    localStorage.setItem(themeStorageKey, normalized)
    hasManualThemePreference.value = true
  } catch (error) {
    console.warn('Failed to persist theme preference', error)
  }
}

const toggleTheme = () => {
  setTheme(isDarkTheme.value ? 'light' : 'dark')
}

const themeToggleLabel = computed(() => (isDarkTheme.value ? t('app.switchToLight') : t('app.switchToDark')))

let cleanupSystemThemeListener

if (themeManager?.mediaQuery) {
  const mediaQuery = themeManager.mediaQuery
  const handleSystemThemeChange = (event) => {
    if (hasManualThemePreference.value) {
      return
    }
    setTheme(event.matches ? 'dark' : 'light', { persist: false })
  }

  if (typeof mediaQuery.addEventListener === 'function') {
    mediaQuery.addEventListener('change', handleSystemThemeChange)
    cleanupSystemThemeListener = () => mediaQuery.removeEventListener('change', handleSystemThemeChange)
  } else if (typeof mediaQuery.addListener === 'function') {
    mediaQuery.addListener(handleSystemThemeChange)
    cleanupSystemThemeListener = () => mediaQuery.removeListener(handleSystemThemeChange)
  }
}

onMounted(() => {
  setTheme(currentTheme.value, { persist: hasManualThemePreference.value })
})



// Delete confirmation dialog
const showDeleteConfirm = ref(false)
const tokenToDelete = ref(null)

// Auth URL dialog
const showAuthUrlDialog = ref(false)

// External links dialogs
const showAppHomeDialog = ref(false)
const showPluginHomeDialog = ref(false)

// Settings menu
const showSettingsMenu = ref(false)

// Token form dialog
const showTokenFormModal = ref(false)
const editingToken = ref(null)

// Computed properties

const canGetToken = computed(() => {
  return authUrl.value && authCode.value.trim().length > 0
})

// Methods
const showStatus = (message, type = 'info') => {
  statusMessage.value = message
  statusType.value = type

  setTimeout(() => {
    statusMessage.value = ''
  }, 2000)
}

const loadTokens = async (showSuccessMessage = false) => {
  isLoading.value = true
  try {
    const jsonString = await invoke('load_tokens_json')
    tokens.value = JSON.parse(jsonString)
    hasUnsavedChanges.value = false
    if (showSuccessMessage) {
      showStatus(t('messages.tokenLoadSuccess'), 'success')
    }
  } catch (error) {
    showStatus(`${t('messages.tokenLoadFailed')}: ${error}`, 'error')
    tokens.value = []
    hasUnsavedChanges.value = false
  } finally {
    isLoading.value = false
  }
}


const saveTokensToFile = async () => {
  try {
    const jsonString = JSON.stringify(tokens.value, null, 2)
    await invoke('save_tokens_json', { jsonString })
    hasUnsavedChanges.value = false
    showStatus(t('messages.tokenSaved'), 'success')
  } catch (error) {
    showStatus(`${t('messages.tokenSaveFailed')}: ${error}`, 'error')
    throw error
  }
}


const deleteToken = (tokenId) => {
  // 显示删除确认对话框
  tokenToDelete.value = tokenId
  showDeleteConfirm.value = true
}

const confirmDelete = async () => {
  if (!tokenToDelete.value) return

  const tokenIdToDelete = tokenToDelete.value

  // 立即关闭对话框，提升用户体验
  showDeleteConfirm.value = false
  tokenToDelete.value = null

  try {
    // 先从内存中删除 token
    const tokenIndex = tokens.value.findIndex(token => token.id === tokenIdToDelete)
    if (tokenIndex === -1) {
      showStatus(t('messages.tokenNotFound'), 'error')
      return
    }

    // 从内存中删除
    tokens.value = tokens.value.filter(token => token.id !== tokenIdToDelete)

    // 立即显示删除成功消息
    showStatus(t('messages.tokenDeleted'), 'success')
    hasUnsavedChanges.value = true // 标记有未保存的更改

    // 异步执行后端删除操作（不阻塞UI）
    invoke('delete_token', { tokenId: tokenIdToDelete }).catch(error => {
      // 如果后端删除失败，记录错误但不影响前端删除操作
      console.log('Backend delete failed (token may not be saved):', error)
    })

  } catch (error) {
    showStatus(`${t('messages.deleteFailed')}: ${error}`, 'error')
  }
}


const cancelDelete = () => {
  showDeleteConfirm.value = false
  tokenToDelete.value = null
}



const onTokenSaved = () => {
  loadTokens()
  showStatus(t('messages.newTokenSaved'), 'success')
}

// Token generator methods
const copyToClipboard = async (text) => {
  try {
    await navigator.clipboard.writeText(text)
    return true
  } catch (error) {
    console.error('Failed to copy to clipboard:', error)
    return false
  }
}

const generateAuthUrl = async () => {
  isGenerating.value = true

  try {
    const url = await invoke('generate_augment_auth_url')
    authUrl.value = url
  } catch (error) {
    showStatus(`${t('messages.error')}: ${error}`, 'error')
  } finally {
    isGenerating.value = false
  }
}

const copyAuthUrl = async () => {
  const success = await copyToClipboard(authUrl.value)
  showStatus(
    success ? t('messages.copySuccess') : t('messages.copyFailed'),
    success ? 'success' : 'error'
  )
}



const getAccessToken = async () => {
  isGettingToken.value = true
  showStatus(t('messages.gettingToken'), 'info')

  try {
    const result = await invoke('get_augment_token', { code: authCode.value })
    tokenResult.value = result
    showStatus(t('messages.tokenGetSuccess'), 'success')
  } catch (error) {
    showStatus(`${t('messages.error')}: ${error}`, 'error')
  } finally {
    isGettingToken.value = false
  }
}

const copyAccessToken = async () => {
  const success = await copyToClipboard(tokenResult.value.access_token)
  showStatus(
    success ? t('messages.accessTokenCopied') : t('messages.copyFailed'),
    success ? 'success' : 'error'
  )
}

const copyTenantUrl = async () => {
  const success = await copyToClipboard(tokenResult.value.tenant_url)
  showStatus(
    success ? t('messages.tenantUrlCopied') : t('messages.copyFailed'),
    success ? 'success' : 'error'
  )
}


const saveToken = async () => {
  try {
    // 创建新的 token 对象
    const newToken = createNewToken(
      tokenResult.value.tenant_url,
      tokenResult.value.access_token,
      portalUrl.value.trim() || null,
      emailNote.value.trim() || null
    )

    // 添加到内存中的 tokens 数组
    tokens.value.push(newToken)
    hasUnsavedChanges.value = true

    showStatus(t('messages.tokenAddedToMemory'), 'success')

    // Reset form
    authUrl.value = ''
    authCode.value = ''
    tokenResult.value = null
    portalUrl.value = ''
    emailNote.value = ''
  } catch (error) {
    showStatus(`${t('messages.tokenSaveFailed')}: ${error}`, 'error')
  }
}

// Token form methods
const showTokenForm = () => {
  editingToken.value = null
  showTokenFormModal.value = true
}

const handleEditToken = (token) => {
  editingToken.value = token
  showTokenFormModal.value = true
}

const closeTokenForm = () => {
  showTokenFormModal.value = false
  editingToken.value = null
}

const handleTokenFormSuccess = () => {
  // TokenForm 现在只更新内存，不需要重新加载
  hasUnsavedChanges.value = true
}

const handleUpdateToken = (updatedTokenData) => {
  // 在内存中更新 token
  const index = tokens.value.findIndex(t => t.id === updatedTokenData.id)
  if (index !== -1) {
    tokens.value[index] = {
      ...tokens.value[index],
      tenant_url: updatedTokenData.tenantUrl,
      access_token: updatedTokenData.accessToken,
      portal_url: updatedTokenData.portalUrl,
      email_note: updatedTokenData.emailNote,
      updated_at: new Date().toISOString() // 更新时间戳
    }
    hasUnsavedChanges.value = true
  }
}

const handleAddTokenFromForm = (tokenData) => {
  // 从表单添加新 token 到内存
  const newToken = createNewToken(
    tokenData.tenantUrl,
    tokenData.accessToken,
    tokenData.portalUrl,
    tokenData.emailNote
  )
  tokens.value.push(newToken)
  hasUnsavedChanges.value = true
}

const saveTokenManually = async (tenantUrl, accessToken, portalUrl, emailNote) => {
  try {
    // 创建新的 token 对象
    const newToken = createNewToken(
      tenantUrl,
      accessToken,
      portalUrl || null,
      emailNote || null
    )

    // 添加到内存中的 tokens 数组
    tokens.value.push(newToken)
    hasUnsavedChanges.value = true

    showStatus(t('messages.tokenAddedToMemory'), 'success')
    return { success: true }
  } catch (error) {
    showStatus(`${t('messages.tokenSaveFailed')}: ${error}`, 'error')
    return { success: false, error }
  }
}

// Portal dialog methods
const handleOpenPortal = (token) => {
  currentPortalToken.value = token
  showPortalDialog.value = true
}

const copyPortalUrl = async () => {
  showPortalDialog.value = false
  if (!currentPortalToken.value?.portal_url) return

  const success = await copyToClipboard(currentPortalToken.value.portal_url)
  showStatus(
    success ? t('messages.portalLinkCopied') : t('messages.copyPortalLinkFailed'),
    success ? 'success' : 'error'
  )
}

const openPortalExternal = async () => {
  showPortalDialog.value = false
  if (!currentPortalToken.value?.portal_url) return

  try {
    await invoke('open_url', { url: currentPortalToken.value.portal_url })
  } catch (error) {
    console.error('Failed to open portal externally:', error)
    showStatus(t('messages.openPortalFailed'), 'error')
  }
}

const openPortalInternal = async () => {
  showPortalDialog.value = false
  if (!currentPortalToken.value?.portal_url) return

  try {
    const displayUrl = currentPortalToken.value.tenant_url.replace(/^https?:\/\//, '').replace(/\/$/, '')
    await invoke('open_internal_browser', {
      url: currentPortalToken.value.portal_url,
      title: 'Portal - ' + displayUrl
    })
  } catch (error) {
    console.error('Failed to open portal internally:', error)
    showStatus(t('messages.openPortalFailed'), 'error')
  }
}



// Auth URL dialog methods
const openAuthUrlExternal = async () => {
  showAuthUrlDialog.value = false
  if (!authUrl.value) return

  try {
    await invoke('open_url', { url: authUrl.value })
  } catch (error) {
    console.error('Failed to open auth URL externally:', error)
    showStatus(t('messages.openAuthUrlFailed'), 'error')
  }
}

const openAuthUrlInternal = async () => {
  showAuthUrlDialog.value = false
  if (!authUrl.value) return

  try {
    await invoke('open_internal_browser', {
      url: authUrl.value,
      title: t('messages.oauthTitle')
    })
  } catch (error) {
    console.error('Failed to open auth URL internally:', error)
    showStatus(t('messages.openAuthUrlFailed'), 'error')
  }
}

// External links dialog methods
const openAppHomeExternal = async () => {
  showAppHomeDialog.value = false
  const url = 'https://github.com/zhaochengcube/augment-token-mng'

  try {
    await invoke('open_url', { url })
  } catch (error) {
    console.error('Failed to open App主页 externally:', error)
    showStatus(t('messages.openAppHomeFailed'), 'error')
  }
}

const openAppHomeInternal = async () => {
  showAppHomeDialog.value = false
  const url = 'https://github.com/zhaochengcube/augment-token-mng'

  try {
    await invoke('open_internal_browser', {
      url,
      title: t('messages.appHomeTitle')
    })
  } catch (error) {
    console.error('Failed to open App主页 internally:', error)
    showStatus(t('messages.openAppHomeFailed'), 'error')
  }
}

const openPluginHomeExternal = async () => {
  showPluginHomeDialog.value = false
  const url = 'https://github.com/zhaochengcube/augment-code-auto'

  try {
    await invoke('open_url', { url })
  } catch (error) {
    console.error('Failed to open 插件主页 externally:', error)
    showStatus(t('messages.openPluginHomeFailed'), 'error')
  }
}

const openPluginHomeInternal = async () => {
  showPluginHomeDialog.value = false
  const url = 'https://github.com/zhaochengcube/augment-code-auto'

  try {
    await invoke('open_internal_browser', {
      url,
      title: t('messages.pluginHomeTitle')
    })
  } catch (error) {
    console.error('Failed to open 插件主页 internally:', error)
    showStatus(t('messages.openPluginHomeFailed'), 'error')
  }
}



// 获取初始存储状态
const getInitialStorageStatus = async () => {
  try {
    const status = await invoke('get_storage_status')
    isDatabaseAvailable.value = status?.is_database_available || false
  } catch (error) {
    console.error('Failed to get initial storage status:', error)
    isDatabaseAvailable.value = false
  }
}

// 处理存储配置变更
const handleStorageConfigChanged = async () => {
  await getInitialStorageStatus()
}

// Initialize
onMounted(async () => {
  // 读取保存的语言偏好
  const savedLanguage = localStorage.getItem('preferred-language')
  if (savedLanguage && (savedLanguage === 'zh-CN' || savedLanguage === 'en-US')) {
    currentLocale.value = savedLanguage
    locale.value = savedLanguage
  }

  // 首先获取存储状态
  await getInitialStorageStatus()
  // 然后加载tokens
  // await loadTokens()

  // 添加点击外部区域关闭设置菜单的事件监听器
  document.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
  if (typeof cleanupSystemThemeListener === 'function') {
    cleanupSystemThemeListener()
  }
  // 移除事件监听器
  document.removeEventListener('click', handleClickOutside)
})


</script>

<style scoped>
.app {
  height: 100vh;
  background: var(--color-surface-muted, #f8f9fa);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* 隐藏所有滚动条 */
* {
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
}

*::-webkit-scrollbar {
  width: 0px;
  background: transparent;
}

/* 确保body和html不产生滚动条 */
html, body {
  overflow: hidden;
  height: 100%;
  margin: 0;
  padding: 0;
}

.app-header {
  background: var(--color-surface, #ffffff);
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
  padding: 12px 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  flex-shrink: 0;
  min-height: 60px;
  flex-wrap: wrap;
  gap: 8px;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex: 1;
  min-width: 0;
  align-items: flex-start;
}

.app-header h1 {
  margin: 0;
  color: var(--color-text-heading, #333);
  font-size: 20px;
  font-weight: 600;
  white-space: nowrap;
}

/* 固定控制按钮组 */
.fixed-controls {
  position: fixed;
  bottom: 20px;
  right: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  z-index: 1000;
}

.settings-menu {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 8px;
  animation: slideUp 0.3s ease;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.control-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  border: none;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 3px 10px rgba(0, 0, 0, 0.15);
  backdrop-filter: blur(10px);
  background: var(--color-surface, #ffffff);
  color: var(--color-text-primary, #374151);
  border: 1px solid var(--color-border, #e5e7eb);
}

.control-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2);
}

.control-btn:active {
  transform: translateY(0);
}

/* 语言切换按钮样式 */
.control-btn.language-toggle {
  font-weight: 700;
  font-size: 12px;
}

/* 设置按钮样式 */
.control-btn.settings-toggle {
  background: var(--color-accent, #3b82f6);
  color: var(--color-text-inverse, #ffffff);
  border-color: var(--color-accent, #3b82f6);
}

.control-btn.settings-toggle:hover {
  background: var(--color-accent-hover, #2563eb);
  border-color: var(--color-accent-hover, #2563eb);
}

.control-btn svg {
  transition: all 0.3s ease;
}

.control-btn:hover svg {
  transform: scale(1.1);
}

/* 黑暗模式下的固定控制按钮样式 */
[data-theme='dark'] .control-btn {
  background: var(--color-surface, #1e293b);
  color: var(--color-text-primary, #cbd5e1);
  border-color: rgba(148, 163, 184, 0.35);
  box-shadow: 0 3px 10px rgba(0, 0, 0, 0.3);
}

[data-theme='dark'] .control-btn:hover {
  background: rgba(148, 163, 184, 0.16);
  border-color: rgba(148, 163, 184, 0.55);
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.4);
}

[data-theme='dark'] .control-btn.settings-toggle {
  background: var(--color-accent, #3b82f6);
  color: var(--color-text-inverse, #ffffff);
  border-color: var(--color-accent, #3b82f6);
}

[data-theme='dark'] .control-btn.settings-toggle:hover {
  background: var(--color-accent-hover, #2563eb);
  border-color: var(--color-accent-hover, #2563eb);
}







.header-buttons {
  display: flex;
  gap: 8px;
  align-items: center;
  flex-wrap: wrap;
  justify-content: flex-end;
}



.external-links-group {
  display: flex;
  flex-direction: row;
  gap: 8px;
  align-items: center;
  align-self: flex-start;
}

.external-links-group .btn {
  padding: 6px 12px;
  font-size: 12px;
  font-weight: 500;
  min-width: 80px;
  justify-content: center;
  border: none;
}

.btn.app-home-btn {
  background: var(--color-blue-primary, #007bff);
  color: var(--color-text-inverse, #ffffff);
}

.btn.app-home-btn:hover {
  background: var(--color-blue-primary-hover, #0056b3);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 123, 255, 0.3);
}

.btn.plugin-home-btn {
  background: var(--color-success-bg, #28a745);
  color: var(--color-text-inverse, #ffffff);
}

.btn.plugin-home-btn:hover {
  background: var(--color-success-bg-hover, #1e7e34);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(40, 167, 69, 0.3);
}



.main-content {
  padding: 20px 16px;
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}



.generator-header {
  margin-bottom: 32px;
}

.header-content {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  gap: 20px;
}

.title-section {
  text-align: center;
}

.title-section h2 {
  margin: 0 0 8px 0;
  font-size: 28px;
  font-weight: 700;
  color: var(--color-text-strong, #1f2937);
  line-height: 1.2;
}

.title-section p {
  margin: 0;
  font-size: 16px;
  color: var(--color-text-muted, #6b7280);
  line-height: 1.5;
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
  background: var(--color-blue-primary, #007bff);
  color: var(--color-text-inverse, #ffffff);
}

.btn.primary:hover {
  background: var(--color-blue-primary-hover, #0056b3);
}

.btn.secondary {
  background: var(--color-text-muted, #6c757d);
  color: var(--color-text-inverse, #ffffff);
}

.btn.secondary:hover {
  background: var(--color-btn-secondary-bg-active, #545b62);
}

.btn.warning {
  background: var(--color-warning-bg, #f59e0b);
  color: var(--color-text-inverse, #ffffff);
}

.btn.warning:hover {
  background: var(--color-warning-bg-hover, #d97706);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(245, 158, 11, 0.3);
}

.btn.info {
  background: var(--color-info-bg, #0ea5e9);
  color: var(--color-text-inverse, #ffffff);
}

.btn.info:hover {
  background: var(--color-info-bg-hover, #0284c7);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(14, 165, 233, 0.3);
}

.btn.small {
  padding: 6px 12px;
  font-size: 12px;
}

.btn.disabled,
.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

.login-buttons {
  display: flex;
  gap: 6px;
  align-items: center;
}

/* 输入框样式 */
input[type="text"] {
  padding: 10px 12px;
  border: 1px solid var(--color-btn-secondary-border, #ddd);
  border-radius: 4px;
  font-size: 14px;
  transition: border-color 0.2s;
  width: 100%;
  box-sizing: border-box;
}

input[type="text"]:focus {
  outline: none;
  border-color: var(--color-blue-primary, #007bff);
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.1);
}

input[type="text"]:read-only {
  background-color: var(--color-surface-muted, #f8f9fa);
  color: var(--color-text-muted, #6c757d);
}

/* 带复制图标的输入框 */
.input-with-copy {
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;
}

.input-with-copy input {
  padding-right: 45px;
  flex: 1;
}

.copy-icon-btn {
  position: absolute;
  right: 8px;
  background: none;
  border: none;
  cursor: pointer;
  padding: 6px;
  border-radius: 4px;
  color: var(--color-text-muted, #6c757d);
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.copy-icon-btn:hover {
  background: var(--color-surface-muted, #e9ecef);
  color: var(--color-text-secondary, #495057);
}

.copy-icon-btn:active {
  transform: scale(0.95);
}



/* 响应式设计 */
@media (max-width: 768px) {
  .app-header {
    padding: 8px 12px;
    min-height: 56px;
  }

  .app-header h1 {
    font-size: 18px;
  }

  .header-buttons {
    gap: 6px;
  }

  .btn {
    padding: 8px 12px;
    font-size: 13px;
  }

  .btn.small {
    padding: 6px 10px;
    font-size: 12px;
  }

  .fixed-controls {
    bottom: 16px;
    right: 16px;
  }

  .control-btn {
    width: 36px;
    height: 36px;
  }
}



/* Modal styles */
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

@media (max-width: 480px) {
  .app-header {
    flex-direction: column;
    align-items: stretch;
    gap: 8px;
    padding: 8px;
  }

  .header-left {
    justify-content: space-between;
    width: 100%;
  }

  .header-buttons {
    justify-content: space-between;
    width: 100%;
  }

  .fixed-controls {
    bottom: 12px;
    right: 12px;
    gap: 6px;
  }

  .control-btn {
    width: 32px;
    height: 32px;
  }

  .control-btn.language-toggle {
    font-size: 10px;
  }
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: var(--color-text-muted, #666);
}

.empty-icon {
  margin-bottom: 24px;
  color: var(--color-btn-primary-disabled-bg, #ccc);
}

.empty-state h2 {
  margin: 0 0 12px 0;
  color: var(--color-text-heading, #333);
  font-size: 24px;
  font-weight: 600;
}

.empty-state p {
  margin: 0;
  font-size: 16px;
  line-height: 1.5;
}

.loading-state {
  text-align: center;
  padding: 40px 20px;
  color: var(--color-text-muted, #666);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--color-surface-hover, #f3f3f3);
  border-top: 4px solid var(--color-blue-primary, #007bff);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.token-list {
  width: 100%;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.list-header h2 {
  margin: 0;
  color: var(--color-text-heading, #333);
  font-size: 20px;
  font-weight: 600;
}

.status-toast {
  position: fixed;
  top: 20px;
  left: 20px;
  padding: 12px 20px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 2500;
  animation: slideIn 0.3s ease;
}

[data-theme='dark'] .status-toast {
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.5), 0 0 0 2px rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
}

[data-theme='dark'] .status-toast.info {
  background: var(--color-info-surface, rgba(56, 189, 248, 0.8));
  color: var(--color-info-text, #f0f9ff);
  border: 2px solid var(--color-info-border, rgba(56, 189, 248, 0.9));
}

[data-theme='dark'] .status-toast.success {
  background: var(--color-success-surface, rgba(20, 184, 166, 0.8));
  color: var(--color-success-text, #ecfdf5);
  border: 2px solid var(--color-success-border, rgba(45, 212, 191, 0.9));
}

[data-theme='dark'] .status-toast.error {
  background: var(--color-danger-surface, rgba(239, 68, 68, 0.8));
  color: var(--color-danger-text, #fef2f2);
  border: 2px solid var(--color-danger-border, rgba(239, 68, 68, 0.9));
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

.status-toast.info {
  background: var(--color-info-surface, #d1ecf1);
  color: var(--color-info-text, #0c5460);
  border: 1px solid var(--color-info-border, #bee5eb);
}

.status-toast.success {
  background: var(--color-success-surface, #d4edda);
  color: var(--color-success-text, #155724);
  border: 1px solid var(--color-success-border, #c3e6cb);
}

.status-toast.error {
  background: var(--color-danger-surface, #f8d7da);
  color: var(--color-danger-text, #721c24);
  border: 1px solid var(--color-danger-border, #f5c6cb);
}

/* Portal对话框样式 */
.portal-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 3000; /* 确保在所有其他元素之上 */
}

.portal-dialog {
  background: var(--color-surface, #ffffff);
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
  min-width: 300px;
  max-width: 400px;
}

.portal-dialog h3 {
  margin: 0 0 20px 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-heading, #333);
  text-align: center;
}

.dialog-buttons {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.dialog-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border: 2px solid transparent;
  border-radius: 8px;
  background: var(--color-surface-muted, #f8f9fa);
  color: var(--color-text-secondary, #495057);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  justify-content: center;
}

.dialog-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.dialog-btn.external {
  background: var(--color-blue-soft-bg, #e3f2fd);
  color: var(--color-blue-soft-text, #1976d2);
  border-color: var(--color-blue-soft-border, #90caf9);
}

.dialog-btn.external:hover {
  background: var(--color-blue-soft-bg, #bbdefb);
  border-color: var(--color-blue-soft-hover, #64b5f6);
}

.dialog-btn.internal {
  background: var(--color-success-surface, #e8f5e8);
  color: var(--color-success-text, #2e7d32);
  border-color: var(--color-success-border, #a5d6a7);
}

.dialog-btn.internal:hover {
  background: var(--color-success-surface, #c8e6c9);
  border-color: var(--color-success-border, #81c784);
}

.dialog-btn.cancel {
  background: var(--color-rose-surface, #fce4ec);
  color: var(--color-rose-text, #c2185b);
  border-color: var(--color-rose-border, #f8bbd9);
}

.dialog-btn.cancel:hover {
  background: var(--color-rose-border, #f8bbd9);
  border-color: var(--color-rose-hover, #f48fb1);
}

.dialog-btn.delete {
  background: var(--color-danger-soft-surface, #ffebee);
  color: var(--color-danger-text, #d32f2f);
  border-color: var(--color-danger-soft-border, #ffcdd2);
}

.dialog-btn.delete:hover {
  background: var(--color-danger-soft-border, #ffcdd2);
  border-color: var(--color-danger-soft-border, #ef9a9a);
}

/* 删除确认对话框特定样式 */
.portal-dialog.delete-confirm p {
  margin: 0 0 20px 0;
  color: var(--color-text-muted, #666);
  text-align: center;
  line-height: 1.5;
}

.delete-confirm .dialog-buttons {
  flex-direction: row;
  gap: 12px;
}

.delete-confirm .dialog-btn {
  flex: 1;
}

.additional-fields {
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid var(--color-divider, #e1e5e9);
}

.field-container {
  margin-bottom: 15px;
}

.field-container label {
  display: block;
  margin-bottom: 5px;
  font-weight: 500;
  color: var(--color-text-primary, #374151);
  font-size: 14px;
}

.field-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--color-border-strong, #d1d5db);
  border-radius: 6px;
  font-size: 14px;
  transition: border-color 0.2s ease;
}

.field-input:focus {
  outline: none;
  border-color: var(--color-accent, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.field-input::placeholder {
  color: var(--color-text-soft, #9ca3af);
}

/* 移除了重复的状态指示器样式，现在在 TokenList.vue 中 */


</style>


console.log('🎯🎯🎯 email-helper.js 开始加载! 版本: 2025-10-25-v2')
console.log('📍 当前时间:', new Date().toISOString())

import { createApp } from 'vue'
import { createI18n } from 'vue-i18n'
import EmailHelper from './components/EmailHelper.vue'
import './style.css'

// 导入语言文件
import zhCN from './locales/zh-CN.js'
import enUS from './locales/en-US.js'

// 创建 i18n 实例
const i18n = createI18n({
  legacy: false,
  locale: 'zh-CN', // 默认使用中文
  fallbackLocale: 'en-US',
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS
  }
})

// 创建 Vue 应用
const app = createApp(EmailHelper)

// 使用 i18n
app.use(i18n)

// 挂载应用
console.log('🔧 准备挂载 EmailHelper 应用...')
app.mount('#app')
console.log('✅ EmailHelper 应用已挂载完成!')


<template>
  <div class="email-manager">
    <div class="modal-overlay" @click="$emit('close')">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>邮箱获取</h2>
          <div class="header-actions">
            <button @click="openDataFolder" class="btn-icon info" title="打开数据存储文件夹">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                <path d="M10 4H4c-1.11 0-2 .89-2 2v12c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2h-8l-2-2z"/>
              </svg>
            </button>
            <button class="close-btn" @click="$emit('close')">×</button>
          </div>
        </div>
        
        <div class="modal-body">
          <!-- Tab Navigation -->
          <div class="tab-nav">
            <button 
              :class="['tab-btn', { active: activeTab === 'temp' }]"
              @click="activeTab = 'temp'"
            >
              临时邮箱
            </button>
            <button 
              :class="['tab-btn', { active: activeTab === 'service' }]"
              @click="activeTab = 'service'"
            >
              邮箱服务
            </button>
          </div>

          <!-- Tab Content -->
          <div class="tab-content">
            <!-- Temporary Email Tab -->
            <div v-if="activeTab === 'temp'" class="tab-panel">
              <div class="panel-header">
                <h3>临时邮箱</h3>
                <button @click="showAddForm('temp')" class="btn primary small">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
                  </svg>
                  添加书签
                </button>
              </div>
              
              <div class="bookmarks-grid">
                <div
                  v-for="bookmark in tempBookmarks"
                  :key="bookmark.id"
                  class="bookmark-card"
                >
                  <div class="bookmark-actions">
                    <button @click="editBookmark(bookmark)" class="btn-icon edit" title="编辑">
                      <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
                      </svg>
                    </button>
                    <button @click="deleteBookmark(bookmark.id)" class="btn-icon delete" title="删除">
                      <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
                      </svg>
                    </button>
                  </div>
                  <div class="bookmark-content" @click="openBookmark(bookmark.url)">
                    <div class="bookmark-icon">
                      <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
                      </svg>
                    </div>
                    <div class="bookmark-name">{{ bookmark.name }}</div>
                    <div v-if="bookmark.description" class="bookmark-desc">{{ bookmark.description }}</div>
                  </div>
                </div>
              </div>

              <div v-if="tempBookmarks.length === 0" class="empty-state">
                <p>还没有添加临时邮箱书签</p>
                <p>点击"添加书签"来添加你常用的临时邮箱网站</p>
              </div>
            </div>

            <!-- Email Service Tab -->
            <div v-if="activeTab === 'service'" class="tab-panel">
              <div class="panel-header">
                <h3>邮箱服务</h3>
                <button @click="showAddForm('service')" class="btn primary small">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
                  </svg>
                  添加书签
                </button>
              </div>
              
              <div class="bookmarks-grid">
                <div
                  v-for="bookmark in serviceBookmarks"
                  :key="bookmark.id"
                  class="bookmark-card"
                >
                  <div class="bookmark-actions">
                    <button @click="editBookmark(bookmark)" class="btn-icon edit" title="编辑">
                      <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
                      </svg>
                    </button>
                    <button @click="deleteBookmark(bookmark.id)" class="btn-icon delete" title="删除">
                      <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
                      </svg>
                    </button>
                  </div>
                  <div class="bookmark-content" @click="openBookmark(bookmark.url)">
                    <div class="bookmark-icon">
                      <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z"/>
                      </svg>
                    </div>
                    <div class="bookmark-name">{{ bookmark.name }}</div>
                    <div v-if="bookmark.description" class="bookmark-desc">{{ bookmark.description }}</div>
                  </div>
                </div>
              </div>

              <div v-if="serviceBookmarks.length === 0" class="empty-state">
                <p>还没有添加邮箱服务书签</p>
                <p>点击"添加书签"来添加邮箱购买和接码网站</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Add/Edit Form Modal -->
        <div v-if="showForm" class="form-overlay" @click="hideForm">
          <div class="form-content" @click.stop>
            <div class="form-header">
              <h3>{{ editingBookmark ? '编辑书签' : '添加书签' }}</h3>
              <button class="close-btn" @click="hideForm">×</button>
            </div>
            
            <div class="form-body">
              <div class="form-group">
                <label>名称 *</label>
                <input 
                  v-model="formData.name" 
                  type="text" 
                  placeholder="输入书签名称"
                  required
                >
              </div>
              
              <div class="form-group">
                <label>网址 *</label>
                <input 
                  v-model="formData.url" 
                  type="url" 
                  placeholder="https://example.com"
                  required
                >
              </div>
              
              <div class="form-group">
                <label>描述</label>
                <textarea 
                  v-model="formData.description" 
                  placeholder="可选的描述信息"
                  rows="2"
                ></textarea>
              </div>
              
              <div class="form-actions">
                <button @click="hideForm" class="btn secondary">取消</button>
                <button @click="saveBookmark" class="btn primary" :disabled="!canSave">
                  {{ editingBookmark ? '更新' : '添加' }}
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Status Messages -->
        <div 
          v-if="statusMessage" 
          :class="['status', statusType]"
        >
          {{ statusMessage }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Emits
const emit = defineEmits(['close'])

// Reactive data
const activeTab = ref('temp')
const tempBookmarks = ref([])
const serviceBookmarks = ref([])
const showForm = ref(false)
const editingBookmark = ref(null)
const currentCategory = ref('')
const statusMessage = ref('')
const statusType = ref('info')

const formData = ref({
  name: '',
  url: '',
  description: ''
})

// Computed properties
const canSave = computed(() => {
  return formData.value.name.trim() && formData.value.url.trim()
})

// Methods
const showStatus = (message, type = 'info') => {
  statusMessage.value = message
  statusType.value = type
  
  setTimeout(() => {
    statusMessage.value = ''
  }, 3000)
}

const loadBookmarks = async () => {
  try {
    const tempResult = await invoke('get_bookmarks', { category: 'temp' })
    const serviceResult = await invoke('get_bookmarks', { category: 'service' })
    
    tempBookmarks.value = tempResult || []
    serviceBookmarks.value = serviceResult || []
  } catch (error) {
    showStatus(`加载书签失败: ${error}`, 'error')
  }
}

const showAddForm = (category) => {
  currentCategory.value = category
  editingBookmark.value = null
  formData.value = {
    name: '',
    url: '',
    description: ''
  }
  showForm.value = true
}

const editBookmark = (bookmark) => {
  currentCategory.value = bookmark.category
  editingBookmark.value = bookmark
  formData.value = {
    name: bookmark.name,
    url: bookmark.url,
    description: bookmark.description || ''
  }
  showForm.value = true
}

const hideForm = () => {
  showForm.value = false
  editingBookmark.value = null
  formData.value = {
    name: '',
    url: '',
    description: ''
  }
}

const saveBookmark = async () => {
  if (!canSave.value) return
  
  try {
    const bookmarkData = {
      name: formData.value.name.trim(),
      url: formData.value.url.trim(),
      description: formData.value.description.trim(),
      category: currentCategory.value
    }
    
    if (editingBookmark.value) {
      await invoke('update_bookmark', {
        id: editingBookmark.value.id,
        ...bookmarkData
      })
      showStatus('书签更新成功!', 'success')
    } else {
      await invoke('add_bookmark', bookmarkData)
      showStatus('书签添加成功!', 'success')
    }
    
    await loadBookmarks()
    hideForm()
  } catch (error) {
    showStatus(`保存书签失败: ${error}`, 'error')
  }
}

const deleteBookmark = async (id) => {
  if (!confirm('确定要删除这个书签吗？')) return
  
  try {
    await invoke('delete_bookmark', { id })
    await loadBookmarks()
    showStatus('书签删除成功!', 'success')
  } catch (error) {
    showStatus(`删除书签失败: ${error}`, 'error')
  }
}

const openBookmark = async (url) => {
  try {
    await invoke('open_url', { url })
    showStatus('正在浏览器中打开...', 'info')
  } catch (error) {
    showStatus(`打开网址失败: ${error}`, 'error')
  }
}

const openDataFolder = async () => {
  try {
    await invoke('open_data_folder')
    // 静默执行，不显示状态提示
  } catch (error) {
    showStatus(`打开文件夹失败: ${error}`, 'error')
  }
}

// Initialize
onMounted(() => {
  loadBookmarks()
})
</script>

<style scoped>
/* 隐藏表单弹窗的滚动条 */
.form-overlay * {
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
}

.form-overlay *::-webkit-scrollbar {
  display: none;
}

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
  z-index: 1000;
}

.modal-content {
  background: white;
  border-radius: 8px;
  width: 90%;
  max-width: 800px;
  max-height: 90vh;
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid #eee;
  flex-shrink: 0;
}

.modal-header h2 {
  margin: 0;
  color: #333;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: #666;
  padding: 0;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-btn:hover {
  color: #333;
}

.btn-icon.info {
  background: #f8f9fa;
  color: #007bff;
}

.btn-icon.info:hover {
  background: #e9ecef;
  color: #0056b3;
}

.modal-body {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.tab-nav {
  display: flex;
  border-bottom: 1px solid #eee;
  flex-shrink: 0;
}

.tab-btn {
  flex: 1;
  padding: 16px 20px;
  border: none;
  background: none;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  color: #666;
  transition: all 0.2s;
}

.tab-btn.active {
  color: #007bff;
  border-bottom: 2px solid #007bff;
  background: #f8f9fa;
}

.tab-btn:hover:not(.active) {
  background: #f8f9fa;
  color: #333;
}

.tab-content {
  flex: 1;
  overflow: hidden;
}

.tab-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid #f0f0f0;
  flex-shrink: 0;
}

.panel-header h3 {
  margin: 0;
  color: #333;
  font-size: 18px;
}

.bookmarks-grid {
  flex: 1;
  overflow-y: auto;
  padding: 20px 20px 20px;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 16px;
}

.bookmark-card {
  position: relative;
  aspect-ratio: 1;
  border: 1px solid #e1e5e9;
  border-radius: 12px;
  background: white;
  transition: all 0.2s;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.bookmark-card:hover {
  border-color: #007bff;
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.15);
  transform: translateY(-2px);
}

.bookmark-actions {
  position: absolute;
  top: 8px;
  right: 8px;
  display: flex;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.2s;
  z-index: 2;
}

.bookmark-card:hover .bookmark-actions {
  opacity: 1;
}

.bookmark-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 16px 12px;
  cursor: pointer;
  text-align: center;
}

.bookmark-icon {
  margin-bottom: 8px;
  color: #007bff;
  opacity: 0.8;
}

.bookmark-name {
  font-weight: 600;
  color: #333;
  font-size: 13px;
  line-height: 1.3;
  margin-bottom: 4px;
  word-break: break-word;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.bookmark-desc {
  color: #666;
  font-size: 11px;
  line-height: 1.3;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.btn-icon {
  width: 24px;
  height: 24px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  backdrop-filter: blur(10px);
}

.btn-icon.edit {
  background: rgba(255, 255, 255, 0.9);
  color: #6c757d;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.btn-icon.edit:hover {
  background: rgba(233, 236, 239, 0.95);
  color: #495057;
  transform: scale(1.1);
}

.btn-icon.delete {
  background: rgba(255, 255, 255, 0.9);
  color: #dc3545;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.btn-icon.delete:hover {
  background: rgba(245, 198, 203, 0.95);
  color: #721c24;
  transform: scale(1.1);
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: #666;
  grid-column: 1 / -1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 200px;
}

.empty-state p {
  margin: 8px 0;
  line-height: 1.5;
}

.empty-state p:first-child {
  font-size: 18px;
  font-weight: 500;
  color: #333;
}

/* Form Modal Styles */
.form-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1100;
}

.form-content {
  background: white;
  border-radius: 8px;
  width: 90%;
  max-width: 500px;
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  display: flex;
  flex-direction: column;
}

.form-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid #eee;
  flex-shrink: 0;
}

.form-header h3 {
  margin: 0;
  color: #333;
  font-size: 18px;
}

.form-body {
  padding: 20px;
  flex: 1;
  min-height: 0;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: #333;
  font-size: 14px;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  transition: border-color 0.2s;
  box-sizing: border-box;
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.1);
}

.form-group textarea {
  resize: vertical;
  font-family: inherit;
}

.form-actions {
  display: flex;
  gap: 12px;
  margin-top: 24px;
}

.form-actions .btn {
  flex: 1;
  justify-content: center;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.btn.primary {
  background: #007bff;
  color: white;
}

.btn.primary:hover:not(:disabled) {
  background: #0056b3;
}

.btn.secondary {
  background: #6c757d;
  color: white;
}

.btn.secondary:hover {
  background: #545b62;
}

.btn.small {
  padding: 8px 12px;
  font-size: 13px;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.status {
  padding: 12px 20px;
  margin: 0 20px 20px;
  border-radius: 4px;
  font-size: 14px;
}

.status.info {
  background: #d1ecf1;
  color: #0c5460;
  border: 1px solid #bee5eb;
}

.status.success {
  background: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
}

.status.error {
  background: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}

/* Responsive Design */
@media (max-width: 768px) {
  .modal-content {
    width: 95%;
    max-height: 95vh;
  }

  .form-content {
    width: 95%;
  }

  .panel-header {
    flex-direction: column;
    gap: 12px;
    align-items: stretch;
  }

  .bookmarks-grid {
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 12px;
    padding: 16px 16px 16px;
  }

  .bookmark-card {
    border-radius: 8px;
  }

  .bookmark-content {
    padding: 12px 8px;
  }

  .bookmark-name {
    font-size: 12px;
  }

  .bookmark-desc {
    font-size: 10px;
  }

  .form-actions {
    flex-direction: row;
  }

  .form-actions .btn {
    flex: 1;
  }
}
</style>

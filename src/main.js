import { createApp } from "vue"
import "./style.css"
import App from "./App.vue"

const THEME_STORAGE_KEY = "atm-theme"
const prefersDarkMedia = window.matchMedia("(prefers-color-scheme: dark)")

const applyTheme = (theme) => {
  const normalized = theme === "dark" ? "dark" : "light"
  const root = document.documentElement
  root.dataset.theme = normalized
  root.style.colorScheme = normalized
}

const getStoredTheme = () => {
  try {
    return localStorage.getItem(THEME_STORAGE_KEY)
  } catch (error) {
    console.warn("Failed to read stored theme preference", error)
    return null
  }
}

const resolveInitialTheme = () => {
  const stored = getStoredTheme()
  if (stored === "dark" || stored === "light") {
    return stored
  }
  return prefersDarkMedia.matches ? "dark" : "light"
}

const initialTheme = resolveInitialTheme()
applyTheme(initialTheme)

const themeManager = {
  storageKey: THEME_STORAGE_KEY,
  mediaQuery: prefersDarkMedia,
  applyTheme,
  getStoredTheme,
  initialTheme
}

const app = createApp(App)
app.provide("themeManager", themeManager)
app.mount("#app")

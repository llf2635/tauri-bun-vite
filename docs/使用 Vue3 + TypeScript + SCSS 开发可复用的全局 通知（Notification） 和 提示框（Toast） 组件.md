当然可以！下面我们将使用 **Vue3 + TypeScript + SCSS** 创建两个可复用的全局组件：

- **Notification（通知）**：常用于系统级通知，可手动关闭，支持多条堆叠显示。
- **Toast（轻提示）**：短暂显示后自动消失，用于操作反馈（如“保存成功”）。

我们将使用以下 Vue3 内置组件：

| 功能         | 使用的内置组件               |
|--------------|------------------------------|
| 动画效果     | `<Transition>`               |
| 多条堆叠     | `<TransitionGroup>`（可选）  |
| DOM 脱离上下文 | `<Teleport>`                 |
| 状态缓存     | 不需要 `KeepAlive`           |

---

## ✅ 一、全局通知组件 `Notification.vue`

```vue
<!-- components/Notification.vue -->
<template>
  <!-- 使用 Teleport 将通知挂载到 body 下，避免被父容器裁剪 -->
  <Teleport to="body">
    <!-- TransitionGroup 为多个通知项提供进入/离开/移动动画 -->
    <TransitionGroup
      name="notification"
      tag="div"
      class="notification-container"
    >
      <!-- 遍历通知队列 -->
      <div
        v-for="item in notifications"
        :key="item.id"
        class="notification-item"
        :class="`notification--${item.type}`"
        @mouseenter="pauseTimer(item)"
        @mouseleave="resumeTimer(item)"
      >
        <div class="notification-content">
          <!-- 图标（可选） -->
          <span v-if="item.icon" class="notification-icon">{{ item.icon }}</span>
          <div class="notification-text">
            <div class="notification-title">{{ item.title }}</div>
            <div v-if="item.message" class="notification-message">{{ item.message }}</div>
          </div>
        </div>
        <!-- 关闭按钮 -->
        <button
          v-if="item.closable"
          class="notification-close"
          @click="removeNotification(item.id)"
        >
          ×
        </button>
      </div>
    </TransitionGroup>
  </Teleport>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue'

// 定义通知类型
type NotificationType = 'info' | 'success' | 'warning' | 'error'

// 定义通知对象结构
interface NotificationItem {
  id: number
  title: string
  message?: string
  type: NotificationType
  duration?: number // 毫秒，0 表示不自动关闭
  closable?: boolean
  icon?: string
  onClose?: () => void
  timer?: ReturnType<typeof setTimeout> // 用于暂停/恢复定时器
}

// 存储所有通知
const notifications = ref<NotificationItem[]>([])

// 生成唯一 ID
let idCounter = 0
const generateId = () => ++idCounter

// 添加通知的方法（供外部调用）
const addNotification = (options: {
  title: string
  message?: string
  type?: NotificationType
  duration?: number
  closable?: boolean
  icon?: string
  onClose?: () => void
}) => {
  const id = generateId()
  const type = options.type || 'info'
  const duration = options.duration === undefined ? 4500 : options.duration
  const closable = options.closable !== undefined ? options.closable : true

  const item: NotificationItem = {
    id,
    title: options.title,
    message: options.message,
    type,
    duration,
    closable,
    icon: options.icon,
    onClose: options.onClose
  }

  notifications.value.push(item)

  // 如果 duration > 0，设置自动关闭
  if (duration > 0) {
    const timer = setTimeout(() => {
      removeNotification(id)
    }, duration)
    item.timer = timer
  }
}

// 移除通知
const removeNotification = (id: number) => {
  const index = notifications.value.findIndex(n => n.id === id)
  if (index === -1) return

  const item = notifications.value[index]
  // 清除定时器
  if (item.timer) clearTimeout(item.timer)
  // 调用关闭回调
  if (item.onClose) item.onClose()

  // 从数组中移除
  notifications.value.splice(index, 1)
}

// 暂停自动关闭（鼠标悬停）
const pauseTimer = (item: NotificationItem) => {
  if (item.timer) {
    clearTimeout(item.timer)
    item.timer = undefined
  }
}

// 恢复自动关闭
const resumeTimer = (item: NotificationItem) => {
  if (item.duration > 0 && !item.timer && item.id) {
    item.timer = setTimeout(() => {
      removeNotification(item.id)
    }, item.duration)
  }
}

// 暴露方法给外部使用（如通过 app.config.globalProperties 或插件方式）
defineExpose({
  addNotification,
  removeNotification
})
</script>

<style lang="scss" scoped>
.notification-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 9999;
  max-width: 400px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 10px;
}

.notification-item {
  display: flex;
  align-items: flex-start;
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  overflow: hidden;
  opacity: 0.8;
  transform: translateX(100%);
  transition: all 0.3s ease;
  min-width: 300px;
}

.notification-item:hover {
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.2);
  opacity: 1;
}

// 不同类型的颜色
.notification--info { border-left: 4px solid #1890ff; }
.notification--success { border-left: 4px solid #52c41a; }
.notification--warning { border-left: 4px solid #faad14; }
.notification--error { border-left: 4px solid #f5222d; }

.notification-content {
  display: flex;
  flex: 1;
  padding: 12px;
  gap: 10px;
}

.notification-icon {
  font-size: 18px;
  line-height: 1;
}

.notification-text {
  flex: 1;
}

.notification-title {
  font-weight: 600;
  color: #333;
  margin-bottom: 4px;
}

.notification-message {
  font-size: 14px;
  color: #666;
}

.notification-close {
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: none;
  font-size: 18px;
  color: #999;
  cursor: pointer;
  margin-right: 4px;
  border-radius: 4px;
  transition: all 0.2s;

  &:hover {
    background: #f5f5f5;
    color: #333;
  }
}

// 进入/离开动画
.notification-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.notification-leave-to {
  opacity: 0;
  transform: translateY(-100%);
}

.notification-leave-active {
  position: absolute;
  right: 0;
  transition: all 0.3s ease;
}
</style>
```

---

## ✅ 二、轻提示组件 `Toast.vue`

```vue
<!-- components/Toast.vue -->
<template>
  <!-- 使用 Teleport 将 Toast 挂载到 body -->
  <Teleport to="body">
    <Transition name="toast">
      <div
        v-if="isVisible"
        class="toast"
        :class="`toast--${type}`"
        @mouseenter="pauseTimer"
        @mouseleave="resumeTimer"
      >
        <!-- 图标 -->
        <span v-if="icon" class="toast-icon">{{ icon }}</span>
        <span class="toast-message">{{ message }}</span>
      </div>
    </Transition>
  </Teleport>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue'

// Toast 类型
type ToastType = 'info' | 'success' | 'warning' | 'error'

// 定义 props
const props = defineProps<{
  message: string
  type?: ToastType
  duration?: number
  icon?: string
  visible: boolean // 控制显示隐藏
  onClose?: () => void
}>()

// 内部状态
const isVisible = ref(false)
const timer = ref<ReturnType<typeof setTimeout> | null>(null)

// 默认值
const type = props.type || 'info'
const duration = props.duration === undefined ? 3000 : props.duration

// 显示时启动定时器
const startTimer = () => {
  if (duration > 0 && !timer.value) {
    timer.value = setTimeout(() => {
      hideToast()
    }, duration)
  }
}

// 隐藏 Toast
const hideToast = () => {
  if (timer.value) clearTimeout(timer.value)
  timer.value = null
  isVisible.value = false
  if (props.onClose) props.onClose()
}

// 暂停自动关闭
const pauseTimer = () => {
  if (timer.value) clearTimeout(timer.value)
  timer.value = null
}

// 恢复自动关闭
const resumeTimer = () => {
  if (duration > 0 && !timer.value && isVisible.value) {
    timer.value = setTimeout(() => {
      hideToast()
    }, duration)
  }
}

// 监听 visible 变化
watch(
  () => props.visible,
  (newVal) => {
    if (newVal) {
      isVisible.value = true
      // 异步启动定时器，确保动画执行
      setTimeout(startTimer, 50)
    } else {
      hideToast()
    }
  },
  { immediate: true }
)

// 暴露方法
defineExpose({
  show: () => (isVisible.value = true),
  hide: hideToast
})
</script>

<style lang="scss" scoped>
.toast {
  position: fixed;
  left: 50%;
  bottom: 50px;
  transform: translateX(-50%) translateY(100%);
  background: #333;
  color: #fff;
  padding: 12px 20px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  font-size: 14px;
  display: flex;
  align-items: center;
  gap: 8px;
  z-index: 9998;
  min-width: 200px;
  justify-content: center;
}

.toast-icon {
  font-weight: bold;
}

// 不同类型图标（可替换为 emoji 或图标库）
.toast--success::before { content: "✅"; }
.toast--warning::before { content: "⚠️"; }
.toast--error::before { content: "❌"; }
.toast--info::before { content: "ℹ️"; }

// 动画
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(60px);
}

.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}
</style>
```

---

## ✅ 三、如何使用（在 App.vue 或组件中）

### 1. 注册为全局组件（推荐使用插件方式）

```ts
// plugins/notification.ts
import { createApp } from 'vue'
import Notification from '@/components/Notification.vue'
import Toast from '@/components/Toast.vue'

export const NotificationPlugin = {
  install(app: any) {
    const notificationInstance = app
      .component('Notification', Notification)
      .mount(document.createElement('div'))

    const toastInstance = app
      .component('Toast', Toast)
      .mount(document.createElement('div'))

    // 挂载到 body
    document.body.appendChild(notificationInstance.$el)
    document.body.appendChild(toastInstance.$el)

    // 暴露方法
    app.config.globalProperties.$notify = notificationInstance.addNotification
    app.config.globalProperties.$toast = (message: string, opts = {}) => {
      toastInstance.message = message
      Object.assign(toastInstance, { visible: true }, opts)
    }
  }
}
```

### 2. 在 `main.ts` 中使用

```ts
// main.ts
import { createApp } from 'vue'
import App from './App.vue'
import { NotificationPlugin } from './plugins/notification'

const app = createApp(App)
app.use(NotificationPlugin)
app.mount('#app')
```

### 3. 在组件中调用

```vue
<script setup lang="ts">
// 使用全局方法
const notify = () => {
  $notify({
    title: '系统通知',
    message: '这是一条成功消息',
    type: 'success',
    duration: 3000
  })
}

const showToast = () => {
  $toast('操作成功！', { type: 'success' })
}
</script>

<template>
  <div>
    <button @click="notify">显示通知</button>
    <button @click="showToast">显示 Toast</button>
  </div>
</template>
```

---

## ✅ 总结

| 组件       | 使用的内置组件       | 说明 |
|------------|----------------------|------|
| Notification | `Teleport` + `TransitionGroup` | 多条通知堆叠，支持手动关闭和自动消失 |
| Toast       | `Teleport` + `Transition`      | 轻量提示，自动消失，居中显示 |

这两个组件具备：

- ✅ 类型安全（TypeScript）
- ✅ 动画流畅（SCSS + Transition）
- ✅ 可复用、可配置
- ✅ 支持全局调用
- ✅ 鼠标悬停暂停关闭

非常适合集成到中后台管理系统或 UI 组件库中。
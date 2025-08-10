在使用 **Vue3 + TypeScript + TailwindCSS** 开发可复用的全局 **通知（Notification）** 和 **提示框（Toast）** 组件时，我们可以充分利用 Vue3 的内置组件来实现动画、脱离上下文渲染、状态缓存等能力。

---

## ✅ 核心 Vue3 内置组件使用说明

| 功能需求 | 使用的内置组件 | 说明 |
|--------|----------------|------|
| 动画效果（出现/消失） | `<Transition>` | 为通知添加淡入淡出或滑动动画 |
| 渲染到页面顶层（避免被遮挡） | `<Teleport>` | 将 Toast 渲染到 `body` 或特定容器，避免 `overflow: hidden` 截断 |
| 多个 Toast 列表管理（可选） | `<TransitionGroup>` | 如果要同时显示多个 Toast，支持列表动画 |
| 状态缓存（一般不需要） | `<KeepAlive>` | Toast 通常是瞬态组件，**不需要缓存** |

> ✅ 所以我们主要使用：`<Transition>` 和 `<Teleport>`，可选 `<TransitionGroup>`（用于多 Toast）

---

## 🎯 示例一：全局提示框 Toast 组件（带动画 + Teleport）

### 📁 文件结构
```
components/
  Toast.vue
  ToastManager.ts (管理实例)
```

---

### ✅ `Toast.vue` —— 单个 Toast 提示组件

```vue
<!-- components/Toast.vue -->
<script setup lang="ts">
import { computed } from 'vue'

// 定义 Toast 类型
export type ToastType = 'success' | 'error' | 'info' | 'warning'

// 接收外部传参
interface Props {
  id: number // 用于 TransitionGroup 移除
  message: string
  type?: ToastType
  duration?: number // 持续时间
  onClose: (id: number) => void
}

const props = withDefaults(defineProps<Props>(), {
  type: 'info',
  duration: 3000,
})

// 根据类型设置背景色
const bgColor = computed(() => {
  switch (props.type) {
    case 'success':
      return 'bg-green-500'
    case 'error':
      return 'bg-red-500'
    case 'warning':
      return 'bg-yellow-500'
    default:
      return 'bg-blue-500'
  }
})

// 自动关闭逻辑
setTimeout(() => {
  props.onClose(props.id)
}, props.duration)
</script>

<template>
  <!-- 使用 Transition 实现淡入淡出动画 -->
  <Transition
    enter-active-class="transform transition-transform duration-300 ease-in-out"
    leave-active-class="transform transition-transform duration-300 ease-in-out"
    enter-from-class="translate-y-4 opacity-0"
    enter-to-class="translate-y-0 opacity-100"
    leave-from-class="translate-y-0 opacity-100"
    leave-to-class="-translate-y-4 opacity-0"
  >
    <!-- Toast 外层容器 -->
    <div
      class="flex items-center justify-between p-4 mb-2 max-w-md rounded shadow-lg text-white"
      :class="bgColor"
      role="alert"
    >
      <!-- 消息内容 -->
      <span class="flex-1">{{ message }}</span>
      <!-- 关闭按钮 -->
      <button
        @click="$emit('close', id)"
        class="ml-4 text-white hover:text-gray-200 focus:outline-none"
      >
        ✕
      </button>
    </div>
  </Transition>
</template>
```

---

### ✅ `ToastManager.ts` —— Toast 管理器（全局调用）

```ts
// components/ToastManager.ts
import { ref, Ref } from 'vue'

// Toast 数据结构
export interface Toast {
  id: number
  message: string
  type: 'success' | 'error' | 'info' | 'warning'
  duration?: number
}

// 存储所有 Toast 实例
const toasts: Ref<Toast[]> = ref([])

// 自增 ID
let nextId = 1

// 添加 Toast
export function useToast() {
  const addToast = (message: string, type: Toast['type'] = 'info', duration?: number) => {
    const id = nextId++
    toasts.value.push({ id, message, type, duration })

    // 设置自动移除
    setTimeout(() => {
      removeToast(id)
    }, duration || 3000)
  }

  // 移除 Toast
  const removeToast = (id: number) => {
    toasts.value = toasts.value.filter(t => t.id !== id)
  }

  return {
    toasts,
    addToast,
    removeToast,
  }
}
```

---

### ✅ 在 `App.vue` 或 `main.ts` 中注册全局 Toast 容器

```vue
<!-- App.vue -->
<script setup lang="ts">
import { useToast } from './components/ToastManager'
import Toast from './components/Toast.vue'

const { toasts, addToast } = useToast()

// 模拟触发
const triggerToast = () => {
  addToast('操作成功！', 'success', 2000)
}
</script>

<template>
  <div class="p-10">
    <h1 class="text-2xl mb-6">Vue3 + TS + Tailwind Toast 示例</h1>
    <button
      @click="triggerToast"
      class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
    >
      显示成功提示
    </button>
  </div>

  <!-- 使用 Teleport 将 Toast 传送到 body 底部 -->
  <Teleport to="body">
    <!-- 使用 TransitionGroup 管理多个 Toast 动画 -->
    <TransitionGroup
      tag="div"
      name="toast-group"
      class="fixed bottom-4 right-4 flex flex-col items-end space-y-2 z-50"
    >
      <Toast
        v-for="t in toasts"
        :key="t.id"
        :id="t.id"
        :message="t.message"
        :type="t.type"
        :duration="t.duration"
        @close="removeToast"
      />
    </TransitionGroup>
  </Teleport>
</template>
```

---

### ✅ 动画样式补充（可选，增强效果）

```css
/* 可在全局 CSS 中添加 */
.toast-group-move {
  transition: transform 0.3s ease;
}
```

---

## 🎯 示例二：全局通知 Notification（类似 Ant Design 风格）

> 与 Toast 类似，但更正式，常用于系统级通知。

### ✅ `Notification.vue`

```vue
<!-- components/Notification.vue -->
<script setup lang="ts">
import { computed } from 'vue'

export type NotificationType = 'success' | 'error' | 'info' | 'warning'

interface Props {
  id: number
  title: string
  message: string
  type: NotificationType
  duration?: number
  onClose: (id: number) => void
}

const props = withDefaults(defineProps<Props>(), {
  duration: 4000,
})

// 图标映射
const icons = {
  success: '✅',
  error: '❌',
  warning: '⚠️',
  info: 'ℹ️',
}

const icon = computed(() => icons[props.type])
const bgColor = computed(() => {
  switch (props.type) {
    case 'success': return 'border-green-500'
    case 'error': return 'border-red-500'
    case 'warning': return 'border-yellow-500'
    default: return 'border-blue-500'
  }
})
</script>

<template>
  <Transition
    enter-active-class="transition-all duration-300 ease-out"
    leave-active-class="transition-all duration-300 ease-in"
    enter-from-class="opacity-0 translate-y-4"
    enter-to-class="opacity-100 translate-y-0"
    leave-from-class="opacity-100 translate-y-0"
    leave-to-class="opacity-0 -translate-y-4"
  >
    <div
      :class="['border-l-4 p-4 mb-3 max-w-sm rounded shadow-lg bg-white', bgColor]"
      role="alert"
    >
      <div class="flex items-start">
        <span class="text-2xl mr-3">{{ icon }}</span>
        <div class="flex-1">
          <h3 class="font-bold text-gray-800">{{ title }}</h3>
          <p class="text-gray-600 text-sm mt-1">{{ message }}</p>
        </div>
        <button
          @click="$emit('close', id)"
          class="text-gray-400 hover:text-gray-600 ml-2 focus:outline-none"
        >
          ✕
        </button>
      </div>
    </div>
  </Transition>
</template>
```

---

### ✅ `NotificationManager.ts`

```ts
// components/NotificationManager.ts
import { ref, Ref } from 'vue'

export interface Notification {
  id: number
  title: string
  message: string
  type: 'success' | 'error' | 'info' | 'warning'
  duration?: number
}

const notifications: Ref<Notification[]> = ref([])
let nextId = 1

export function useNotification() {
  const add = (title: string, message: string, type: Notification['type'] = 'info', duration = 4000) => {
    const id = nextId++
    notifications.value.push({ id, title, message, type, duration })
    
    setTimeout(() => remove(id), duration)
  }

  const remove = (id: number) => {
    notifications.value = notifications.value.filter(n => n.id !== id)
  }

  return { notifications, add, remove }
}
```

---

### ✅ 在 `App.vue` 中使用 Notification

```vue
<template>
  <!-- 其他内容 -->
  <Teleport to="body">
    <TransitionGroup
      tag="div"
      name="notif-group"
      class="fixed top-4 right-4 z-50"
    >
      <Notification
        v-for="n in notifications"
        :key="n.id"
        :id="n.id"
        :title="n.title"
        :message="n.message"
        :type="n.type"
        :duration="n.duration"
        @close="remove"
      />
    </TransitionGroup>
  </Teleport>
</template>

<script setup lang="ts">
import { useNotification } from './components/NotificationManager'
import Notification from './components/Notification.vue'

const { notifications, add: addNotification, remove } = useNotification()

const showNotification = () => {
  addNotification('新消息', '您有一条未读通知', 'info', 3000)
}
</script>
```

---

## ✅ 总结：关键点

| 功能 | 使用组件 | 说明 |
|------|----------|------|
| 动画效果 | `<Transition>` | 控制单个 Toast/Notification 的出现/消失 |
| 多个提示管理 | `<TransitionGroup>` | 支持列表插入/移除动画，`key` 必须唯一 |
| 脱离父级限制 | `<Teleport to="body">` | 避免被裁剪，确保层级高 |
| 状态保留 | ❌ 不需要 `KeepAlive` | Toast 是瞬态组件，无需缓存 |

---

## ✅ 最佳实践建议

1. **封装 Hook**：使用 `useToast()` 和 `useNotification()` 实现全局调用。
2. **支持 Promise 调用**（进阶）：如 `toast.success('ok').then(...)`。
3. **支持关闭所有**：添加 `clearAll()` 方法。
4. **支持位置配置**：顶部、右上、底部等。
5. **TypeScript 类型安全**：严格定义 props 和事件。

---

这样你就拥有了一个 **高性能、可复用、美观的全局提示系统**，适用于大多数 Vue3 + TS + Tailwind 项目。
当然可以！下面是一个**综合性、生产级的 `useNotification` Hook 封装示例**，适用于 Vue3 + TypeScript 项目，支持全局调用、类型安全、自动关闭、手动关闭、Promise 风格 API、堆叠管理等特性。

同时，我也会在最后总结 **封装其他自定义 Hook 的最佳实践和必备内容**，供你参考。

---

## ✅ 一、`useNotification` Hook 封装（完整示例）

### 📁 文件结构
```
composables/
  useNotification.ts
components/
  Notification.vue
```

---

### ✅ `composables/useNotification.ts`

```ts
// composables/useNotification.ts
import { ref, Ref } from 'vue'

// ==================== 类型定义 ====================

// 通知类型
export type NotificationType = 'success' | 'info' | 'warning' | 'error'

// 通知配置项（可选参数）
export interface NotificationOptions {
  title: string
  message: string
  type?: NotificationType
  duration?: number // 毫秒，默认 3000
  onClose?: () => void // 关闭回调
  onClick?: () => void // 点击通知本体回调
}

// 内部通知对象（带 ID）
interface Notification extends NotificationOptions {
  id: number
  type: NotificationType
  duration: number
}

// 返回的 Hook 类型
export interface UseNotificationReturn {
  notifications: Ref<Notification[]>
  // 添加通知（函数重载）
  add: (options: NotificationOptions) => number
  success: (title: string, message: string, duration?: number) => number
  error: (title: string, message: string, duration?: number) => number
  warning: (title: string, message: string, duration?: number) => number
  info: (title: string, message: string, duration?: number) => number
  // 手动关闭
  close: (id: number) => void
  // 清除所有
  clearAll: () => void
  // 支持 Promise 风格调用（例如 await 后继续）
  show: (options: NotificationOptions) => Promise<void>
}

// ==================== 核心实现 ====================

// 存储所有通知
const notifications: Ref<Notification[]> = ref([])

// 自增 ID
let idCounter = 0

// 默认配置
const DEFAULT_DURATION = 3000

/**
 * 全局通知 Hook
 * 支持链式调用、Promise、快捷方法（success/error 等）
 */
export function useNotification(): UseNotificationReturn {
  /**
   * 添加一条通知
   * @param options 配置项
   * @returns 通知 ID（可用于手动关闭）
   */
  const add = (options: NotificationOptions): number => {
    const id = ++idCounter
    const notification: Notification = {
      id,
      type: options.type || 'info',
      duration: options.duration ?? DEFAULT_DURATION,
      ...options,
    }

    notifications.value.push(notification)

    // 自动关闭
    if (notification.duration > 0) {
      setTimeout(() => {
        close(id)
      }, notification.duration)
    }

    return id
  }

  /**
   * 显示通知并返回 Promise（可用于等待关闭）
   * @example await notify.show({ title: '完成', message: '操作成功' })
   */
  const show = (options: NotificationOptions): Promise<void> => {
    return new Promise((resolve) => {
      const id = add(options)
      const originalOnClose = options.onClose
      // 重写 onClose，触发 resolve
      const onCloseWrapper = () => {
        originalOnClose?.()
        resolve()
      }
      // 临时替换 onClose（需在组件中调用）
      const notif = notifications.value.find(n => n.id === id)
      if (notif) {
        notif.onClose = onCloseWrapper
      }
    })
  }

  /**
   * 手动关闭指定通知
   */
  const close = (id: number): void => {
    const index = notifications.value.findIndex(n => n.id === id)
    if (index > -1) {
      const notification = notifications.value[index]
      notification.onClose?.() // 触发关闭回调
      notifications.value.splice(index, 1)
    }
  }

  /**
   * 清除所有通知
   */
  const clearAll = (): void => {
    notifications.value.forEach(n => n.onClose?.())
    notifications.value = []
  }

  // 快捷方法：success / error / warning / info
  const success = (title: string, message: string, duration?: number): number => {
    return add({ title, message, type: 'success', duration })
  }

  const error = (title: string, message: string, duration?: number): number => {
    return add({ title, message, type: 'error', duration })
  }

  const warning = (title: string, message: string, duration?: number): number => {
    return add({ title, message, type: 'warning', duration })
  }

  const info = (title: string, message: string, duration?: number): number => {
    return add({ title, message, type: 'info', duration })
  }

  return {
    notifications,
    add,
    show,
    close,
    clearAll,
    success,
    error,
    warning,
    info,
  }
}
```

---

### ✅ `components/Notification.vue`（配套组件）

```vue
<!-- components/Notification.vue -->
<script setup lang="ts">
import { Notification } from '../composables/useNotification'

defineProps<{
  notification: Notification
}>()

// 图标映射
const icons = {
  success: '✅',
  error: '❌',
  warning: '⚠️',
  info: 'ℹ️',
}

// 背景颜色
const bgColor = {
  success: 'border-green-500 bg-green-50',
  error: 'border-red-500 bg-red-50',
  warning: 'border-yellow-500 bg-yellow-50',
  info: 'border-blue-500 bg-blue-50',
}[props.notification.type]
</script>

<template>
  <!-- 使用 Transition 实现滑入滑出 -->
  <Transition
    enter-active-class="transform transition-transform duration-300 ease-out"
    leave-active-class="transform transition-transform duration-300 ease-in"
    enter-from-class="-translate-y-4 opacity-0"
    enter-to-class="translate-y-0 opacity-100"
    leave-from-class="translate-y-0 opacity-100"
    leave-to-class="-translate-y-4 opacity-0"
  >
    <div
      :class="['border-l-4 p-4 mb-3 max-w-sm rounded shadow-md', bgColor]"
      role="alert"
      @click="notification.onClick"
    >
      <div class="flex items-start">
        <span class="text-xl mr-3">{{ icons[notification.type] }}</span>
        <div class="flex-1">
          <h3 class="font-bold text-gray-800">{{ notification.title }}</h3>
          <p class="text-gray-600 text-sm mt-1">{{ notification.message }}</p>
        </div>
        <button
          @click.stop="$emit('close', notification.id)"
          class="text-gray-400 hover:text-gray-600 focus:outline-none"
        >
          ✕
        </button>
      </div>
    </div>
  </Transition>
</template>
```

---

### ✅ 在 `App.vue` 中使用

```vue
<!-- App.vue -->
<script setup lang="ts">
import { useNotification } from './composables/useNotification'
import Notification from './components/Notification.vue'

const { notifications, success, error, show, clearAll } = useNotification()

// 示例：触发通知
const testSuccess = () => success('成功', '操作已完成！')
const testError = () => error('错误', '网络连接失败')
const testAsync = async () => {
  await show({ title: '加载中', message: '请稍候...', duration: 2000 })
  alert('通知已关闭！')
}
</script>

<template>
  <div class="p-10 font-sans">
    <h1 class="text-2xl mb-6">Notification 测试</h1>
    <button @click="testSuccess" class="mr-2 px-4 py-2 bg-green-500 text-white rounded">成功</button>
    <button @click="testError" class="mr-2 px-4 py-2 bg-red-500 text-white rounded">错误</button>
    <button @click="testAsync" class="mr-2 px-4 py-2 bg-blue-500 text-white rounded">异步通知</button>
    <button @click="clearAll" class="px-4 py-2 bg-gray-500 text-white rounded">清除所有</button>
  </div>

  <!-- 使用 Teleport 渲染到 body -->
  <Teleport to="body">
    <div class="fixed top-4 right-4 z-50">
      <TransitionGroup tag="div" name="notif" class="space-y-2">
        <Notification
          v-for="n in notifications"
          :key="n.id"
          :notification="n"
          @close="close"
        />
      </TransitionGroup>
    </div>
  </Teleport>
</template>
```

---

## ✅ 二、封装其他自定义 Hook 的注意事项（最佳实践）

### ✅ 1. **必备内容**

| 内容 | 说明 |
|------|------|
| 🔹 **类型定义（TypeScript）** | 所有参数、返回值必须有清晰的类型，提升可维护性 |
| 🔹 **响应式数据（ref / reactive）** | 使用 `ref` 或 `reactive` 管理状态 |
| 🔹 **副作用清理（onUnmounted）** | 如有定时器、事件监听，需在组件销毁时清理 |
| 🔹 **可组合性（Composable）** | 只做一件事，职责单一，可被其他 Hook 调用 |
| 🔹 **默认参数与配置项** | 提供合理默认值，支持可选配置 |
| 🔹 **返回值结构清晰** | 使用对象返回多个方法/状态，命名清晰 |

### ✅ 2. **推荐实践**

| 实践 | 说明 |
|------|------|
| 🌟 **支持 Promise 风格 API** | 如 `await useModal().show()`，增强流程控制 |
| 🌟 **支持事件回调（onClose, onClick）** | 提供扩展能力 |
| 🌟 **支持全局唯一实例（可选）** | 如 `useGlobalState`，避免重复创建 |
| 🌟 **文档化（JSDoc）** | 为每个函数添加注释，说明用途、参数、返回值 |
| 🌟 **错误边界处理** | 对异常输入做容错处理 |
| 🌟 **可测试性** | 逻辑独立，便于单元测试 |

### ✅ 3. **避免的问题**

| 问题 | 建议 |
|------|------|
| ❌ 在 Hook 中直接操作 DOM | 应由组件处理，Hook 只管理状态 |
| ❌ 强依赖特定 UI 框架组件 | 保持逻辑与 UI 解耦 |
| ❌ 返回匿名函数或内联函数过多 | 影响性能和调试 |
| ❌ 不清理副作用 | 导致内存泄漏、定时器堆积 |

---

## ✅ 三、总结

### ✅ `useNotification` 的亮点：
- ✅ 类型安全（TypeScript）
- ✅ 支持快捷方法（success/error）
- ✅ 支持 Promise 风格调用
- ✅ 自动关闭 + 手动关闭 + 回调
- ✅ 可集成到任何 Vue3 项目
- ✅ 配合 Teleport + Transition 实现优雅 UI

### ✅ 封装 Hook 的通用原则：
> **“状态逻辑抽离，UI 交给组件，API 简洁清晰，类型安全优先”**

---

这个 `useNotification` 可直接用于企业级项目，也可作为你封装其他 Hook（如 `useModal`、`useLoading`、`useForm`）的模板参考。希望对你有帮助！
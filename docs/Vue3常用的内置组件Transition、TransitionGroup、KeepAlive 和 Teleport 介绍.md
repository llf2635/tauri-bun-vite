Vue3 提供了多个内置组件，其中 `Transition`、`TransitionGroup`、`KeepAlive` 和 `Teleport` 是最常用且功能强大的四个。它们分别用于处理不同的场景，下面详细说明它们的作用和使用场景：

---

### 1. **`<Transition>` 组件**

#### ✅ 作用：
为单个元素或组件的**进入和离开**添加过渡动画效果。它不会渲染成真实的 DOM 元素，只是在内部包裹的内容发生插入或删除时，自动添加/移除 CSS 类名，从而实现动画。

#### ✅ 动画触发时机：
- 元素被 `v-if`、`v-show`、条件渲染、组件切换等控制显隐时。

#### ✅ 常见类名：
- `v-enter-from` → `v-enter-active` → `v-enter-to`
- `v-leave-from` → `v-leave-active` → `v-leave-to`

> 可通过 `name` 属性自定义前缀（如 `name="fade"` → `fade-enter-from`）

#### ✅ 使用场景：
- 模态框（Modal）的淡入淡出
- 下拉菜单的展开/收起
- 路由切换时的页面过渡
- 提示框（Toast）的出现与消失

#### ✅ 示例：
```vue
<Transition name="fade">
  <div v-if="show" class="modal">Hello Modal</div>
</Transition>

<style>
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.5s;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
}
</style>
```

---

### 2. **`<TransitionGroup>` 组件**

#### ✅ 作用：
为**多个元素或组件的列表**添加过渡效果，适用于 `v-for` 渲染的列表项的**插入、移动、删除**动画。

#### ✅ 特点：
- 默认会渲染一个 `<span>` 标签（可用 `tag` 属性修改）
- 每个子元素必须有唯一的 `key`
- 支持对**列表排序、新增、删除**进行动画

#### ✅ 使用场景：
- 聊天消息列表的动态添加/删除
- 待办事项（TodoList）的增删动画
- 拖拽排序时的平滑移动动画
- 图片墙的动态加载与移除

#### ✅ 示例：
```vue
<TransitionGroup name="list" tag="ul">
  <li v-for="item in items" :key="item.id">
    {{ item.text }}
  </li>
</TransitionGroup>

<style>
.list-move {
  transition: transform 0.5s;
}
</style>
```

> `list-move` 类用于实现**重排动画**（如拖拽后自动滑动归位）

---

### 3. **`<KeepAlive>` 组件**

#### ✅ 作用：
**缓存动态组件或路由组件的实例**，避免重复创建和销毁，保留组件状态（如表单输入、滚动位置等）。

#### ✅ 特性：
- 包裹的组件在切换时不被销毁，而是被缓存
- 支持 `include`（白名单）、`exclude`（黑名单）、`max`（最大缓存数量）
- 触发 `activated` 和 `deactivated` 生命周期钩子

#### ✅ 使用场景：
- 多标签页（Tab）切换时保留状态
- 路由切换时缓存页面（如从列表页进入详情页再返回，保持滚动位置）
- 表单向导中来回切换步骤时保留已填内容

#### ✅ 示例：
```vue
<KeepAlive include="UserList,UserProfile">
  <component :is="currentView" />
</KeepAlive>
```

或在路由中：
```vue
<router-view v-slot="{ Component }">
  <KeepAlive>
    <component :is="Component" />
  </KeepAlive>
</router-view>
```

---

### 4. **`<Teleport>` 组件**

#### ✅ 作用：
将组件的 DOM 结构**“传送”到页面的其他位置**（如 `body` 或某个固定容器），但逻辑上仍属于当前组件。

#### ✅ 用途：
解决**样式隔离、层级问题（z-index）、布局限制**。

#### ✅ 使用场景：
- 模态框（Modal）、弹窗、Tooltip 需要脱离父级 `overflow: hidden` 的限制
- 全局通知（Notification）挂载到 `body`
- 全屏遮罩层
- 固定在页面顶部/底部的 UI 元素

#### ✅ 示例：
```vue
<Teleport to="body">
  <div v-if="showModal" class="modal">
    <p>这是一个模态框</p>
    <button @click="showModal = false">关闭</button>
  </div>
</Teleport>
```

> 这样模态框会插入到 `<body>` 下，避免被父容器裁剪。

---

### 总结对比表：

| 组件 | 作用 | 典型使用场景 |
|------|------|---------------|
| `<Transition>` | 单个元素/组件的进入/离开动画 | 模态框、提示框、路由切换动画 |
| `<TransitionGroup>` | 列表项的插入/移动/删除动画 | 聊天列表、TodoList、拖拽排序 |
| `<KeepAlive>` | 缓存组件实例，保留状态 | 多标签页、路由缓存、表单向导 |
| `<Teleport>` | 将 DOM 渲染到指定位置 | 弹窗、全屏组件、脱离父级限制 |

---

### 小贴士：
- 这些组件**无需导入**，直接使用即可。
- 可组合使用，例如：`<Teleport>` + `<Transition>` 实现带动画的全局模态框。
- `KeepAlive` 对性能有帮助，但注意控制缓存数量（`max`）防止内存泄漏。

---

掌握这四个内置组件，能极大提升 Vue3 应用的用户体验和开发效率。
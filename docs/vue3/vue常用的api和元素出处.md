Vue 所有的 API 都可以参考官方文档。它包括了以下所有内容：  https://cn.vuejs.org/api/

1、Vue 常用的全局 API 参考 https://cn.vuejs.org/api/application.html

2、Vue 常用的组合式 API 参考 https://cn.vuejs.org/api/composition-api-setup.html
包含以下 API：
响应式 API：核心
    - ref
    - reactive
    - computed
    - readonly
    - watch
    - watchEffect
    - watchPostEffect
    - watchSyncEffect
响应式 API：工具函数
    - isRef
    - unref
    - toRef
    - toRefs
    - isReactive
    - isReadonly
    - isProxy
组合式 API：生命周期
    - onBeforeMount
    - onMounted
    - onBeforeUpdate
    - onUpdated
    - onBeforeUnmount
    - onUnmounted
    - onErrorCaptured
    - onRenderTracked
    - onRenderTriggered

3、Vue 常用的内置指令参考 https://cn.vuejs.org/api/built-in-directives.html
包含以下指令：
- v-text
- v-html
- v-show
- v-if
- v-else
- v-else-if
- v-for
- v-on
- v-bind
- v-model
- v-slot
- v-pre
- v-once
- v-memo
- v-cloak
- v-custom-directive

4、Vue 常用的内置组件参考 https://cn.vuejs.org/api/built-in-components.html
包含以下组件：
- transition
- transition-group
- keep-alive
- teleport
内置组件无需注册便可以直接在模板中使用。

5、Vue 常用的内置特殊元素参考 https://cn.vuejs.org/api/built-in-special-elements.html
包含以下元素：
- component
- slot
- template

不是组件， <component>、<slot> 和 <template> 具有类似组件的特性，也是模板语法的一部分。
但它们并非真正的组件，同时在模板编译期间会被编译掉。因此，它们通常在模板中用小写字母书写。

6、Vue 常用的内置的特殊 Attributes 参考 https://cn.vuejs.org/api/built-in-special-attributes.html
包含以下 Attributes：
- key
- ref
- is

7、Vue 单文件组件参考 https://cn.vuejs.org/api/sfc-spec.html#overview
包含以下内容：
单文件组件语法定义:
    - template
    - script
    - styles
使用 <script setup> 语法：
    - defineProps
    - defineEmits
    - defineExpose
单文件组件 CSS 功能：
    - 组件作用域 CSS  ( scoped )
    - 深度选择器 ( :deep )
    - 全局样式 ( :global )
    - 预处理器 (Sass/Less/Stylus)

8、Vue 进阶 API 参考 https://cn.vuejs.org/api/advanced.html
包含以下内容：
- TypeScript 工具类型 
    - PropType<T> 

9、Vue 开箱即用的后台系统 https://cn.vuejs.org/ecosystem/themes.html
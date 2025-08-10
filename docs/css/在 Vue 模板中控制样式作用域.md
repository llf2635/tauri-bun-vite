`:global` 和 `:deep` 是在 Vue 单文件组件（SFC）中使用的 Scoped CSS 特性，主要用于控制样式的作用范围。以下是它们的主要区别：

### 1. `:global`

- **作用**：用于将某个样式标记为全局样式，覆盖组件的 Scoped CSS 限制。
- **用法**：当需要让某些特定的样式适用于整个应用，而不仅局限于当前组件时，可以使用 `:global`。

```css
<style scoped>
.container {
  background-color: blue;
}

:global(.global-class) {
  color: white;
}
</style>
```

在上面的例子中，`.global-class` 不受当前组件的 Scoped CSS 限制，会被视为全局样式。

### 2. `:deep`

- **作用**：用于选择深层嵌套的子组件，通过 Scoped CSS 选择器选择子组件的某些内部元素。
- **用法**：当你需要在 Scoped CSS 中, 选择子组件的某个特定元素时，可以使用 `:deep`。

```css
<style scoped>
.parent {
  color: red;
}

:deep(.child-class) {
  font-weight: bold;
}
</style>
```

在这个例子中，所有子组件中带有 `.child-class` 的元素都会变为加粗字体，而这个样式不受 Scoped CSS 的限制。

### 总结

- `:global`：用于将某个样式设置为全局样式，适用于所有组件，不受当前组件的作用域限制。
- `:deep`：用于选择深层嵌套的子组件中的元素，使得 Scoped CSS 也可以选中这些深层元素。

这两个指令在组件化开发中非常有用，可以帮助开发者更好地控制样式的作用范围和层级关系。
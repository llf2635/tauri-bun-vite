CSS 中有多种伪类选择器，它们用于选中元素的特定状态或特定条件下的元素。以下是一些常用的伪类选择器及其作用：

### 1. `:hover`
- **作用**：选中鼠标悬停在元素上的状态。
- **示例**：
  ```css
  button:hover {
    background-color: blue;
  }
  ```

### 2. `:focus`
- **作用**：选中当前获得焦点的元素，通常用于输入框和可交互元素。
- **示例**：
  ```css
  input:focus {
    outline: 2px solid blue;
  }
  ```

### 3. `:active`
- **作用**：选中当前按下（激活）状态的元素，通常用于按钮。
- **示例**：
  ```css
  button:active {
    transform: scale(0.95);
  }
  ```

### 4. `:first-child`
- **作用**：选中父元素的第一个子元素。
- **示例**：
  ```css
  p:first-child {
    font-weight: bold;
  }
  ```

### 5. `:last-child`
- **作用**：选中父元素的最后一个子元素。
- **示例**：
  ```css
  li:last-child {
    color: red;
  }
  ```

### 6. `:nth-child()`
- **作用**：根据给定的模式选中父元素的某个子元素。
- **示例**：
  ```css
  li:nth-child(2n) {
    background-color: lightgray;
  }
  ```

### 7. `:nth-of-type()`
- **作用**：根据类型选中父元素的某个子元素。
- **示例**：
  ```css
  div:nth-of-type(2) {
    border: 1px solid black;
  }
  ```

### 8. `:not()`
- **作用**：选中不匹配给定选择器的元素。
- **示例**：
  ```css
  p:not(.special) {
    color: gray;
  }
  ```

### 9. `:checked`
- **作用**：选中当前被选中的复选框或单选框。
- **示例**：
  ```css
  input[type="checkbox"]:checked {
    background-color: green;
  }
  ```

### 10. `:disabled`
- **作用**：选中当前禁用的表单元素。
- **示例**：
  ```css
  input:disabled {
    background-color: lightgray;
  }
  ```

### 11. `:enabled`
- **作用**：选中当前启用的表单元素。
- **示例**：
  ```css
  input:enabled {
    background-color: white;
  }
  ```

### 14. `:before` 和 `:after`
- **作用**：在元素内容之前或之后插入内容，通常与 `content` 属性联用。
- **示例**：
  ```css
  p:before {
    content: "Note: ";
    font-weight: bold;
  }
  ```

### 15. `:empty`
- **作用**：选中没有子元素（包括文本节点）的元素。
- **示例**：
  ```css
  div:empty {
    display: none;
  }
  ```

### 16. `:first-of-type` 和 `:last-of-type`
- **作用**：选中父元素中指定类型的第一个或最后一个子元素。
- **示例**：
  ```css
  p:first-of-type {
    color: blue;
  }
  ```

### 17. `:nth-last-child()`
- **作用**：从后往前选中父元素的某个指定子元素。
- **示例**：
  ```css
  li:nth-last-child(2) {
    color: orange;
  }
  ```

### 18. `:nth-last-of-type()`
- **作用**：从后往前选中父元素中指定类型的某个子元素。
- **示例**：
  ```css
  div:nth-last-of-type(1) {
    border: 1px solid blue;
  }
  ```

### 20. `:not()`
- **作用**：用来排除某些元素，符合特定条件的元素会被选中。
- **示例**：
  ```css
  ul li:not(.special) {
    color: gray;
  }
  ```

### 21. `:visited`
- **作用**：选中用户已经访问过的链接。
- **示例**：
  ```css
  a:visited {
    color: purple;
  }
  ```

### 总结
这些是一些常用及次常用的 CSS 伪类选择器。它们在样式的选择和应用上提供了更灵活的方式，使得开发者能够根据状态、结构和特定条件来控制样式。深入理解伪类选择器的使用，可以大大增强 CSS 的表现力和功能性。
  ```
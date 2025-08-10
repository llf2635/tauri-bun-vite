VueUse 提供了一系列强大的组合式 API 函数，这些函数可以帮助你快速实现常见的功能和逻辑。以下是一些常用的组合式 API 函数及其作用：

### 1. **useMouse**
- **作用**：获取鼠标位置的响应式对象 (`x` 和 `y` 坐标)。
- **用法**：
  ```javascript
  const mouse = useMouse();
  console.log(mouse.x, mouse.y);
  ```

### 2. **useLocalStorage**
- **作用**：创建一个与浏览器的 Local Storage 同步的响应式变量。
- **用法**：
  ```javascript
  const name = useLocalStorage('name', '未命名');
  ```

### 3. **useSessionStorage**
- **作用**：类似于 `useLocalStorage`，但与 Session Storage 同步。
- **用法**：
  ```javascript
  const theme = useSessionStorage('theme', 'light');
  ```

### 4. **useFetch**
- **作用**：用于发起 HTTP 请求并管理响应的状态（加载中、错误等）。
- **用法**：
  ```javascript
  const { data, error, isLoading } = useFetch('https://api.example.com/data');
  ```

### 5. **useDebounce**
- **作用**：创建一个防抖效果的响应式变量，常用于处理输入框的变化。
- **用法**：
  ```javascript
  const debounceValue = useDebounce(value, 300); // 300ms 的防抖
  ```

### 6. **useThrottle**
- **作用**：创建一个节流效果的响应式变量，控制函数调用的频率。
- **用法**：
  ```javascript
  const throttleValue = useThrottle(value, 1000); // 每1000ms调用一次
  ```

### 7. **useEventListener**
- **作用**：简化添加和移除事件监听器的过程。
- **用法**：
  ```javascript
  useEventListener('resize', () => {
    console.log(window.innerWidth);
  });
  ```

### 8. **useTimeout**
- **作用**：简化定时器操作，创建一个可以控制的响应式定时器。
- **用法**：
  ```javascript
  const { start, stop } = useTimeout(() => {
    console.log('Timeout!');
  }, 1000);
  ```

### 9. **useInterval**
- **作用**：创建一个可以控制的响应式间隔定时器。
- **用法**：
  ```javascript
  const { start, stop } = useInterval(() => {
    console.log('Interval!');
  }, 1000);
  ```

### 10. **useWindowSize**
- **作用**：获取浏览器窗口的宽度和高度的响应式对象。
- **用法**：
  ```javascript
  const { width, height } = useWindowSize();
  ```

### 14. **useDark**
- **作用**：根据用户的系统主题自动创建响应式的深色模式切换。
- **用法**：
  ```javascript
  const isDark = useDark();
  ```

### 16. **useMediaQuery**
- **作用**：用于检测屏幕尺寸或媒体特性的响应式挂钩。
- **用法**：
  ```javascript
  const isSmallScreen = useMediaQuery('(max-width: 600px)');
  ```

### 17. **useCssVar**
- **作用**：创建与 CSS 自定义属性（CSS 变量）相绑定的响应式变量。
- **用法**：
  ```javascript
  const themeColor = useCssVar('--theme-color', '#42b983');
  ```

### 18. **useClipboard**
- **作用**：实现剪贴板操作的功能。
- **用法**：
  ```javascript
  const { copy, isSupported } = useClipboard();
  copy('Hello, World!');
  ```

### 19. **useFocus**
- **作用**：管理元素的聚焦状态。
- **用法**：
  ```javascript
  const { isFocused, focus } = useFocus(ref);
  ```

### 20. **useCounter**
- **作用**：创建一个简单的计数器。
- **用法**：
  ```javascript
  const { count, inc, dec, reset } = useCounter();
  ```

### 21. **usePromise**
- **作用**：简化处理 Promise 的状态。
- **用法**：
  ```javascript
  const { result, isLoading, isError } = usePromise(fetchData);
  ```

### 22. **useState**
- **作用**：简化创建响应式状态的过程，类似 Vue 的 `ref`。
- **用法**：
  ```javascript
  const state = useState(() => ({ count: 0 }));
  ```

### 13. **useCopyToClipboard**
- **作用**：将文本复制到剪贴板，并支持操作成功和失败的回调。
- **用法**：
  ```javascript
  const { copy, isSuccess } = useCopyToClipboard();
  copy('Hello World!');
  ```

### 18. **useDraggable**
- **作用**：轻松实现拖放功能。
- **用法**：
  ```javascript
  const { isDragging, drag } = useDraggable();
  ```

### 22. **useToggle**
- **作用**：创建一个简单的布尔切换功能。
- **用法**：
  ```javascript
  const { state, toggle } = useToggle(false);
  ```

### 23. **useForm**
- **作用**：管理表单状态和验证。
- **用法**：
  ```javascript
  const { values, errors, handleSubmit } = useForm({
    initialValues: { name: '', email: '' },
    onSubmit: (values) => { console.log(values); },
    validate: (values) => {
      const errors = {};
      // Validation logic
      return errors;
    },
  });
  ```

### 25. **useNotifications**
- **作用**：管理通知的显示和消失。
- **用法**：
  ```javascript
  const { add, remove } = useNotifications();
  add({ title: 'Notification Title', message: 'Message content' });
  ```

### 42. **useFavicon**
- **作用**：动态设置网页图标。
- **用法**：
  ```javascript
  useFavicon('path/to/favicon.ico');
  ```

### 43. **useTitle**
- **作用**：动态设置网页标题。
- **用法**：
  ```javascript
  useTitle('My New Title');
  ```

### 100. **useFullscreen**
- **作用**：管理元素的全屏状态。
- **用法**：
  ```javascript
  const { isFullscreen, toggle } = useFullscreen(targetRef);
  ```

### 小结

这些仅仅是 VueUse 中一部分常用的组合式 API 函数，它们可以极大地简化 Vue 应用的开发，处理常见的需求和逻辑。有关更多函数和详细用法，可以参考 [VueUse 官方文档](https://vueuse.org/) 进行深入了解。
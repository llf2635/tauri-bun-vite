<template>
  <el-drawer
    size="300px"
    v-model="showDrawer"
    :lock-scroll="false"
    :with-header="false"
    :before-close="closeDrawer"
    @open="toggleDrawer(true)"
    @close="toggleDrawer(false)"
    modal-class="setting-modal"
  >
    <div class="drawer-con">
      <div class="close-wrap">
        <i class="iconfont" @click="closeDrawer">&#xe7a3;</i>
      </div>
      <!-- 系统设置主题风格 -->
<!--      <p class="title">{{ $t('setting.theme.title') }}</p>-->
      <el-divider>
        <span>主题风格</span>
      </el-divider>
      <div class="theme-wrap">
        <div
          class="item"
          v-for="(item, index) in settingThemeList"
          :key="item.theme"
          @click="switchTheme(item.theme)"
        >
          <div class="box" :class="{ 'is-active': item.theme === systemThemeMode }">
            <div :style="{ background: item.color[0] + '!important' }">
              <div
                v-for="(cItem, index) in 3"
                :key="index"
                :class="'line' + index"
                :style="{ background: item.leftLineColor }"
              ></div>
            </div>
            <div
              :style="{ background: index === 2 ? item.color[1] : item.color[0] + '!important' }"
            >
              <div
                v-for="(cItem, index) in 3"
                :key="index"
                :class="'line' + index"
                :style="{ background: item.rightLineColor }"
              ></div>
            </div>
          </div>
          <p class="name">{{ $t(`setting.theme.list[${index}]`) }}</p>
          <div class="active" v-show="item.theme === systemThemeMode"></div>
        </div>
      </div>
      <!-- 系统设置菜单风格 -->
      <p class="title" style="margin-top: 30px">{{ $t('setting.menu.title') }}</p>
      <div class="menu-theme-wrap">
        <div>
          <div
            class="item"
            v-for="item in menuThemeList"
            :key="item.theme"
            @click="setMenuTheme(item.theme)"
          >
            <div class="box" :class="{ 'is-active': item.theme === currentMenuTheme }">
              <div class="top" :style="{ background: item.tabBarBackground + '!important' }"></div>
              <div class="left" :style="{ background: item.background + '!important' }">
                <div
                  v-for="(cItem, index) in 3"
                  :key="index"
                  :class="'line' + index"
                  :style="{ background: item.leftLineColor }"
                />
              </div>
              <div class="right">
                <div
                  v-for="(cItem, index) in 3"
                  :key="index"
                  :class="'line' + index"
                  :style="{ background: item.rightLineColor }"
                />
              </div>
            </div>
            <div class="active" v-if="item.theme === currentMenuTheme"></div>
          </div>
        </div>
      </div>
      <!-- 系统设置主题色 -->
      <p class="title" style="margin-top: 30px">{{ $t('setting.color.title') }}</p>
      <div class="main-color-wrap">
        <div class="offset">
          <div
            v-for="color in mainColor"
            :key="color"
            :style="{ background: `${color} !important` }"
            @click="setElementTheme(color)"
          >
            <i class="iconfont-sys" v-show="color == systemThemeColor">&#xe616;</i>
          </div>
        </div>
      </div>
      <!-- 系统设置盒子样式 -->
      <p class="title" style="margin-top: 40px">{{ $t('setting.box.title') }}</p>
      <div class="box-style">
        <div v-if="false">{{ boxBorderMode }}</div>
        <div
          class="button"
          :class="{ 'is-active': !boxBorderMode }"
          @click="switchBoxMode(false, 'shadow-mode')"
        >
          {{ $t('setting.box.list[0]') }}
        </div>
        <div
          class="button"
          :class="{ 'is-active': boxBorderMode }"
          @click="switchBoxMode(false, 'border-mode')"
        >
          {{ $t('setting.box.list[1]') }}
        </div>
      </div>
      <!-- 系统设置项切换开关 -->
      <p class="title" style="margin-top: 50px">{{ $t('setting.basics.title') }}</p>
      <div class="basic-box">
        <div class="item" style="display: flex">
          <span>{{ $t('setting.basics.list[0]') }}</span>
          <el-switch v-model="uniqueOpened" @change="setUniqueOpened" />
        </div>
        <div class="item">
          <span>{{ $t('setting.basics.list[1]') }}</span>
          <el-switch v-model="showMenuButton" @change="setButton" />
        </div>
        <div class="item">
          <span>{{ $t('setting.basics.list[2]') }}</span>
          <el-switch v-model="showRefreshButton" @change="setShowRefreshButton" />
        </div>
        <div class="item mobile-hide">
          <span>{{ $t('setting.basics.list[3]') }}</span>
          <el-switch v-model="showCrumbs" @change="setCrumbs" />
        </div>
        <div class="item">
          <span>{{ $t('setting.basics.list[4]') }}</span>
          <el-switch v-model="showWorkTab" @change="showWorkTabFunc" />
        </div>
        <div class="item">
          <span>{{ $t('setting.basics.list[5]') }}</span>
          <el-switch v-model="showLanguage" @change="setLanguage" />
        </div>
        <div class="item">
          <span>{{ $t('setting.basics.list[6]') }}</span>
          <el-switch v-model="showNprogress" @change="setNprogress" />
        </div>
        <div class="item">
          <span>{{ $t('setting.basics.list[7]') }}</span>
          <el-switch v-model="colorWeak" @change="setColorWeak()" />
        </div>
        <div class="item">
          <span>{{ $t('setting.basics.list[8]') }}</span>
          <el-switch v-model="autoClose" @change="setAutoClose" />
        </div>
      </div>
    </div>
  </el-drawer>
</template>

<script setup lang="ts">
  import { useSettingStore } from '@/store/modules/setting'
  import { SettingThemeList, ThemeList, SystemMainColor, SystemThemeStyles } from '@/config/setting'
  import { SystemThemeEnum, MenuThemeEnum } from '@/enums/appEnum'
  import { getDarkColor, getLightColor } from '@/utils/color'
  import { SystemThemeTypes } from '@/types/store'
  import mittBus from '@/utils/mittBus'

  const store = useSettingStore()

  const props = defineProps(['open'])

  const showDrawer = ref(false)

  watch(
    () => props.open,
    (val) => (showDrawer.value = val)
  )

  const settingThemeList = SettingThemeList
  const menuThemeList = ThemeList
  const mainColor = SystemMainColor
  const currentGlopTheme = computed(() => store.systemThemeType)
  const systemThemeMode = computed(() => store.systemThemeMode)
  const currentMenuTheme = computed(() => store.menuThemeType)
  const systemThemeColor = computed(() => store.systemThemeColor)
  const boxBorderMode = computed(() => store.boxBorderMode)
  const uniqueOpened = ref(true)
  const showMenuButton = ref(true)
  const autoClose = ref(true)
  const showRefreshButton = ref(true)
  const showCrumbs = ref(true)
  let showWorkTab = ref(true)
  const showLanguage = ref(true)
  const showNprogress = ref(true)
  const colorWeak = ref(false)

  watch(
    () => store.showWorkTab,
    (e) => {
      showWorkTab.value = e
    }
  )

  onMounted(() => {
    mittBus.on('openSetting', openSetting)
    listenerSystemTheme()
    initUserSetting()
    initSystemTheme()
  })

  onUnmounted(() => {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    mediaQuery.removeEventListener('change', initSystemTheme)
  })

  const initUserSetting = () => {
    uniqueOpened.value = store.uniqueOpened
    showMenuButton.value = store.showMenuButton
    autoClose.value = store.autoClose
    showRefreshButton.value = store.showRefreshButton
    showCrumbs.value = store.showCrumbs
    showWorkTab.value = store.showWorkTab
    showLanguage.value = store.showLanguage
    showNprogress.value = store.showNprogress
    colorWeak.value = store.colorWeak
    initColorWeak()
    setBoxMode(true, store.boxBorderMode ? 'border-mode' : 'shadow-mode')
  }

  const setMenuTheme = (theme: MenuThemeEnum) => {
    store.setMenuTheme(theme)
    isAutoClose()
  }

  // 监听系统主题变化
  const listenerSystemTheme = () => {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    mediaQuery.addEventListener('change', initSystemTheme)
  }

  // 初始化系统主题
  const initSystemTheme = () => {
    if (systemThemeMode.value === SystemThemeEnum.AUTO) {
      setSystemAutoTheme()
    } else {
      setSystemTheme(currentGlopTheme.value)
    }
  }

  // 主题跟随系统
  const setSystemAutoTheme = () => {
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      setSystemTheme(SystemThemeEnum.DARK, SystemThemeEnum.AUTO)
    } else {
      setSystemTheme(SystemThemeEnum.LIGHT, SystemThemeEnum.AUTO)
    }
  }

  // 切换系统主题
  const switchTheme = (theme: SystemThemeEnum) => {
    if (theme === SystemThemeEnum.AUTO) {
      setSystemAutoTheme()
    } else {
      setSystemTheme(theme)
    }
  }

  // 设置系统主题
  const setSystemTheme = (theme: SystemThemeEnum, themeMode?: SystemThemeEnum) => {
    let el = document.getElementsByTagName('html')[0]
    let isDark = theme === SystemThemeEnum.DARK

    if (!themeMode) {
      themeMode = theme
    }

    const currentTheme = SystemThemeStyles[theme as keyof SystemThemeTypes]

    if (currentTheme) {
      el.setAttribute('class', currentTheme.className)
    }

    // 设置按钮颜色加深或变浅
    let primary = systemThemeColor.value

    for (let i = 1; i <= 9; i++) {
      document.documentElement.style.setProperty(
        `--el-color-primary-light-${i}`,
        isDark ? `${getDarkColor(primary, i / 10)}` : `${getLightColor(primary, i / 10)}`
      )
    }

    setSystemThemeModel(theme, themeMode)
    isAutoClose()
  }

  const showWorkTabFunc = () => {
    store.setWorkTab(!store.showWorkTab)
    isAutoClose()
  }

  // 系统主题变量存储到 vuex 里面
  const setSystemThemeModel = (theme: SystemThemeEnum, themeMode: SystemThemeEnum) => {
    store.setGlopTheme(theme, themeMode)
    isAutoClose()
    store.reload()
  }

  // 自动关闭
  const isAutoClose = () => {
    if (autoClose.value) {
      closeDrawer()
    }
  }

  // 打开或关闭抽屉
  const toggleDrawer = (open: boolean) => {
    let el = document.getElementsByTagName('body')[0]
    if (open) {
      setTimeout(() => {
        el.setAttribute('class', 'theme-change')
      }, 500)
    } else {
      el.removeAttribute('class')
    }
  }

  const openSetting = () => {
    showDrawer.value = true
  }

  const closeDrawer = () => {
    showDrawer.value = false
  }

  const switchBoxMode = (isInit: boolean = false, type: string) => {
    if (
      (type === 'shadow-mode' && boxBorderMode.value === false) ||
      (type === 'border-mode' && boxBorderMode.value === true)
    ) {
      return
    }
    setBoxMode(isInit, type)
  }

  // 设置盒子边框 ｜ 阴影 样式
  const setBoxMode = (isInit: boolean = false, type: string) => {
    setTimeout(() => {
      const el = document.documentElement
      el.setAttribute('data-box-mode', type)

      if (!isInit) {
        store.setBorderMode()
      }
    }, 50)
  }

  const setUniqueOpened = () => {
    store['setUniqueOpened']()
    isAutoClose()
  }

  const setButton = () => {
    store['setButton']()
    isAutoClose()
  }

  const setShowRefreshButton = () => {
    store['setShowRefreshButton']()
    isAutoClose()
  }

  const setCrumbs = () => {
    store['setCrumbs']()
    isAutoClose()
  }

  const setLanguage = () => {
    store['setLanguage']()
    isAutoClose()
  }

  const setNprogress = () => {
    store['setNprogress']()
    isAutoClose()
  }

  const setAutoClose = () => {
    store['setAutoClose']()
    isAutoClose()
  }

  const setElementTheme = (theme: string) => {
    store['setElementTheme'](theme)
    store.reload()
    isAutoClose()
  }

  const setColorWeak = () => {
    let el = document.getElementsByTagName('html')[0]

    if (colorWeak.value) {
      el.setAttribute('class', 'color-weak')
    } else {
      el.removeAttribute('class')
      setSystemTheme(SystemThemeEnum.LIGHT)
    }
    store.setColorWeak()
    isAutoClose()
  }

  const initColorWeak = () => {
    if (colorWeak.value) {
      let el = document.getElementsByTagName('html')[0]
      setTimeout(() => {
        el.setAttribute('class', 'color-weak')
      }, 100)
    }
  }
</script>

<style lang="scss">
  .setting-modal {
    background: transparent !important;
  }

  .el-drawer {
    // 背景滤镜
    background: rgba($color: #fff, $alpha: 50%) !important;
    box-shadow: 0 0 30px rgb(0 0 0 / 10%) !important;

    @include backdropBlur();
  }

  .dark {
    .el-drawer {
      background: rgba($color: #000, $alpha: 50%) !important;
    }
  }

  // 去除滚动条
  .el-drawer__body::-webkit-scrollbar {
    width: 0 !important;
  }

  // .el-switch.is-checked .el-switch__core {
  //   border-color: transparent !important;
  //   background-image: linear-gradient(310deg,#18D6FF,#1890FF) !important;
  // }

  // .el-switch .el-switch__core {
  //   border-color: transparent !important;
  //   background: #E8E9E9 !important;
  // }
</style>

<style lang="scss" scoped>
.drawer-con {
  $box-shadow: 0 2px 8px 0 rgba(0, 0, 0, 0.2);
  padding: 0 5px 30px;

  .close-wrap {
    display: flex;
    justify-content: flex-end;
    margin-top: 15px;

    i {
      font-weight: bold;
      color: var(--art-gray-600);
      cursor: pointer;
    }
  }

  .title {
    position: relative;
    font-size: 14px;
    color: var(--art-text-gray-800);
    text-align: center;

    &:first-of-type {
      margin-top: 20px;
    }

    &::before,
    &::after {
      position: absolute;
      top: 10px;
      border-bottom: 1px solid rgba(var(--art-gray-300-rgb), 0.8);
      width: 50px;
      margin: auto;
      content: '';
    }

    &::before {
      left: 0;
    }

    &::after {
      right: 0;
    }
  }



  .theme-wrap {
    display: flex;
    flex-wrap: wrap;
    margin-top: 25px;

    .item {
      box-sizing: border-box;
      width: 70px;
      margin-right: 15px;

      &:last-of-type {
        margin-right: 0;
      }

      .box {
        position: relative;
        box-sizing: border-box;
        display: flex;
        height: 50px;
        overflow: hidden;
        cursor: pointer;
        border: 1px solid transparent;
        border-radius: 6px;
        box-shadow: $box-shadow;
        transition: box-shadow 0.1s;

        &.is-active {
          border: 1px solid var(--main-color);
        }

        > div {
          position: relative;
          width: 50%;
          height: 100%;

          &:first-of-type {

            > div {
              width: 15px;
              height: 2px;
              margin: 5px 0 0 10px;

              &.line0 {
                margin-top: 13px;
              }

              &.line1 {
                width: 10px;
              }

              &.line2 {
                width: 13px;
              }
            }
          }

          &:last-of-type {
            > div {
              height: 5px;
              margin: 6px 0 0 5px;

              &.line0 {
                width: calc(100% - 15px);
                margin-top: 12px;
              }

              &.line1 {
                width: calc(50% - 5px);
              }

              &.line2 {
                width: calc(52%);
              }
            }
          }
        }
      }

      .name {
        margin-top: 6px;
        font-size: 14px;
        text-align: center;
      }

      .active {
        position: relative;
        right: 0;
        bottom: -5px;
        left: 0;
        width: 6px;
        height: 6px;
        margin: auto;
        background: var(--art-success) !important;
        border-radius: 50%;
      }
    }
  }

  .menu-theme-wrap {
    margin-top: 20px;

    > div {
      display: flex;
      flex-wrap: wrap;
      width: calc(100% + 15px);

      .item {
        width: 70px;
        margin-right: 15px;
        margin-bottom: 15px;

        &:last-of-type {
          margin-right: 0;
        }

        .box {
          position: relative;
          box-sizing: border-box;
          height: 50px;
          overflow: hidden;
          cursor: pointer;
          background: #f5f7f9 !important;
          border: 1px solid transparent;
          border-radius: 6px;
          box-shadow: $box-shadow;
          transition: box-shadow 0.1s;

          &.is-active {
            border: 1px solid var(--main-color);
          }

          &:nth-child(even) {
            margin-right: 0;
          }

          .top {
            width: 100%;
            height: 8px;
          }

          .left {
            position: absolute;
            top: 0;
            left: 0;
            width: 22px;
            height: 100%;

            > div {
              width: 15px;
              height: 2px;
              margin: 5px 0 0 4px;

              &.line0 {
                margin-top: 15px;
              }

              &.line1 {
                width: 10px;
              }

              &.line2 {
                width: 13px;
              }
            }
          }

          .right {
            position: absolute;
            top: 0;
            right: 0;
            width: 46px;
            height: 100%;

            > div {
              height: 6px;
              margin: 5px 0 0 5px;

              &.line0 {
                width: calc(100% - 15px);
                margin-top: 12px;
              }

              &.line1 {
                width: calc(50% - 5px);
              }

              &.line2 {
                width: calc(52%);
              }
            }
          }
        }

        .active {
          width: 6px;
          height: 6px;
          margin: auto;
          margin-top: 8px;
          background: var(--art-success) !important;
          border-radius: 50%;
        }
      }
    }
  }

  .main-color-wrap {
    padding-top: 20px;

    .offset {
      display: flex;
      flex-wrap: wrap;
      width: calc(100% + 12.5px);

      $size: 23px;

      > div {
        display: flex;
        align-items: center;
        justify-content: center;
        width: $size;
        height: $size;
        margin: 0 13px 10px 0;
        cursor: pointer;
        border-radius: $size;

        &:last-of-type {
          margin-right: 0;
        }

        i {
          font-size: 14px;
          color: #fff !important;
        }
      }
    }
  }

  .box-style {
    padding: 4px;
    box-sizing: border-box;
    border-radius: 7px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 20px;
    background-color: var(--art-gray-200);

    .button {
      width: calc(50% - 3px);
      height: 34px;
      line-height: 34px;
      border-radius: 6px;
      font-size: 14px;
      text-align: center;
      cursor: pointer;
      user-select: none;
      transition: all .2s !important;

      &.is-active {
        color: var(--art-gray-800);
        background-color: var(--art-main-bg-color);
      }

      &:hover:not(.is-active) {
        color: var(--art-gray-800);
        background-color: rgba($color: #000000, $alpha: 0.04);
      }
    }
  }

  .basic-box {
    position: relative;
    z-index: 10;
    background: transparent !important;

    .item {
      position: relative;
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-top: 35px;
      background: transparent !important;

      span {
        font-size: 14px;
        background: transparent !important;
      }
    }
  }

  &::-webkit-scrollbar {
    width: 0 !important;
  }
}

.dark {
  .drawer-con {
    .box-style {

      .button {
        &.is-active {
          color: #fff !important;
          background-color: rgba(var(--art-gray-400-rgb), 0.7);
        }

        &:hover:not(.is-active) {
          background-color: rgba($color: #000000, $alpha: 0.2);
        }
      }
    }
  }
}

@media screen and (max-width: $device-ipad) {
  .mobile-hide {
    display: none !important;
  }
}
</style>

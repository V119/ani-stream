<template>
  <div class="container">
    <main>
      <router-view></router-view>
    </main>
  </div>
</template>

<script setup>
import { ElConfigProvider } from 'element-plus'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
</script>

<style>
:root {
  --primary-color: #6366f1;      /* 靛蓝色主色调 */
  --primary-light: #818cf8;      /* 较浅的靛蓝色 */
  --background: #f8fafc;         /* 浅灰背景色 */
  --text-primary: #1e293b;       /* 主要文字颜色 */
  --text-secondary: #64748b;     /* 次要文字颜色 */
  --surface: #ffffff;            /* 卡片背景色 */
  --border: #e2e8f0;            /* 边框颜色 */
  --hover: #f1f5f9;             /* 悬停效果色 */
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 1rem;
}

main {
  padding-top: 1rem;
}
</style>
<template>
  <div class="container">
    <main>
      <h1>动漫列表</h1>
      <AnimeList />
    </main>
  </div>
</template>

<script setup>
import { ElConfigProvider } from 'element-plus'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import AnimeList from './components/AnimeList.vue'
</script>

<style>
.el-header {
  background-color: #fff;
  color: #333;
  text-align: center;
  line-height: 60px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, .12);
}

.el-main {
  background-color: #f5f7fa;
  min-height: calc(100vh - 60px);
}

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

h1 {
  margin: 2rem 0;
  padding: 0 1.5rem;
  font-size: 2rem;
  font-weight: 600;
  color: var(--text-primary);
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
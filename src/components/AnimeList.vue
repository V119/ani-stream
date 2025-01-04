<template>
  <div class="anime-list">
    <el-row :gutter="20">
      <el-col :xs="12" :sm="8" :md="6" :lg="4" v-for="anime in animeList" :key="anime.id">
        <el-card :body-style="{ padding: '0px' }" class="anime-card">
          <el-image 
            :src="anime.imageUrl" 
            fit="cover"
            class="anime-image"
            :lazy="true">
            <template #error>
              <div class="image-error">
                <el-icon><Picture /></el-icon>
              </div>
            </template>
          </el-image>
          
          <div class="anime-info">
            <h3 class="anime-title">{{ anime.name }}</h3>
            <el-tag :type="getStatusType(anime.status)">
              {{ getStatusText(anime.status) }}
            </el-tag>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <div class="pagination-container">
      <el-pagination
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :total="total"
        :page-sizes="[12, 24, 36, 48]"
        layout="total, sizes, prev, pager, next"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Picture } from '@element-plus/icons-vue'
import { mockAnimeList } from '../mock/animeData'
import type { Anime } from '../types/anime'

// 数据状态
const animeList = ref<Anime[]>([])
const currentPage = ref(1)
const pageSize = ref(24)
const total = ref(0)
const loading = ref(false)

// 模拟 API 调用
const fetchAnimeList = async () => {
  try {
    loading.value = true
    // 模拟网络延迟
    await new Promise(resolve => setTimeout(resolve, 500))

    // 计算分页数据
    const start = (currentPage.value - 1) * pageSize.value
    const end = start + pageSize.value
    const paginatedData = mockAnimeList.slice(start, end)
    
    animeList.value = paginatedData
    total.value = mockAnimeList.length
  } catch (error) {
    ElMessage.error('获取数据失败，请稍后重试')
  } finally {
    loading.value = false
  }
}
// 状态映射
const getStatusType = (status: string): 'success' | 'warning' | 'info' | 'primary' | 'danger' => {
  const statusMap: Record<string, 'success' | 'warning' | 'info'> = {
    ongoing: 'success',
    completed: 'info',
    upcoming: 'warning'
  }
  return statusMap[status as keyof typeof statusMap] || 'info'
}

const getStatusText = (status: string): string => {
  const statusMap: Record<string, string> = {
    ongoing: '连载中',
    completed: '已完结',
    upcoming: '即将上映'
  }
  return statusMap[status as keyof typeof statusMap] || '未知'
}

// 分页处理
const handleSizeChange = (val: number) => {
  pageSize.value = val
  currentPage.value = 1
  fetchAnimeList()
}

const handleCurrentChange = (val: number) => {
  currentPage.value = val
  fetchAnimeList()
}

// 组件挂载时获取数据
onMounted(() => {
  fetchAnimeList()
})
</script>

<style scoped>
.anime-list {
  background-color: var(--background);
  padding: 1.5rem;
}

.anime-card {
  background-color: var(--surface);
  border: 1px solid var(--border);
  border-radius: 0.75rem;
  transition: all 0.2s ease;
}

.anime-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

.anime-image {
  height: 200px;
  width: 100%;
  display: block;
  border-bottom: 3px solid #95e1d3;
}

.image-error {
  height: 200px;
  background: #fce38a;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #f38181;
}

.anime-info {
  padding: 14px;
}

.anime-title {
  font-size: 14px;
  margin: 0 0 10px;
  height: 40px;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  color: var(--text-primary);
  font-weight: 600;
}

.pagination-container {
  margin-top: 20px;
  display: flex;
  justify-content: center;
}

:deep(.el-pagination.is-background .el-pager li.is-active) {
  background-color: #95e1d3;
}

:deep(.el-tag--success) {
  background-color: #95e1d3;
  border-color: #95e1d3;
}

:deep(.el-tag--warning) {
  background-color: #fce38a;
  border-color: #fce38a;
  color: #333;
}

:deep(.el-tag--info) {
  background-color: #f38181;
  border-color: #f38181;
}

.status-tag {
  background-color: var(--primary-color);
  color: white;
  padding: 0.25rem 0.75rem;
  border-radius: 9999px;
  font-size: 0.875rem;
}

.anime-info {
  color: var(--text-secondary);
}

/* 可以为不同状态设置不同的颜色 */
.status-ongoing {
  background-color: #6366f1;  /* 靛蓝色 */
}

.status-completed {
  background-color: #22c55e;  /* 绿色 */
}

.status-upcoming {
  background-color: #f59e0b;  /* 橙色 */
}
</style> 
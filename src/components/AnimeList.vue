<template>
  <div class="anime-list">
    <el-row :gutter="24">
      <el-col 
        :xs="24"
        :sm="12"
        :md="8"
        :lg="6"
        :xl="6"
        v-for="item in animeList?.items" 
        :key="item.id"
        class="anime-col"
      >
        <el-card :body-style="{ padding: '0px' }" class="anime-card" @click="handleAnimeClick(item)">
          <el-image 
            :src="item.image_url" 
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
            <h3 class="anime-title">{{ item.name }}</h3>
            <p class="anime-desc">{{ item.desc }}</p>
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
import { useRouter } from 'vue-router'
import { ItemPage } from '../bindings/ItemPage'
import { invoke } from "@tauri-apps/api/core"

// 数据状态
const animeList = ref<ItemPage>()
const currentPage = ref(1)
const pageSize = ref(12)
const total = ref(0)
const loading = ref(false)
const router = useRouter()

// 获取数据函数
const fetchAnimeList = async () => {

    loading.value = true
    await invoke("main_page", {}).then((response) => {
      animeList.value = response as ItemPage
      total.value = animeList.value?.items.length || 0
    }).catch((e) => {
      ElMessage.error('获取数据失败: ' + e + '，请稍后重试')
    }).finally(() => {
      loading.value = false
    })
}

// 分页处理
const handleSizeChange = (val: number) => {
  pageSize.value = val
  currentPage.value = 1
  fetchAnimeList()
}

const handleCurrentChange = async (val: number) => {
  currentPage.value = val
  if (val > currentPage.value) {
    try {
      const response = await invoke('spider:next_page')
      animeList.value = response as ItemPage
    } catch (error) {
      ElMessage.error('获取下一页失败')
    }
  } else {
    try {
      const response = await invoke('spider:pre_page')
      animeList.value = response as ItemPage
    } catch (error) {
      ElMessage.error('获取上一页失败')
    }
  }
}

// 点击处理函数
const handleAnimeClick = (item: any) => {
  router.push({
    name: 'player',
    params: { id: item.id },
    query: { url: item.page_url }
  })
}

// 组件挂载时获取数据
onMounted(() => {
  fetchAnimeList()
})
</script>

<style scoped>
.anime-list {
  background-color: var(--background);
  padding: 1rem;
}

.anime-col {
  margin-bottom: 24px;
  display: flex;
  justify-content: center;
}

.anime-card {
  width: 100%;
  max-width: 280px;
  background-color: var(--surface);
  border: 1px solid var(--border);
  border-radius: 0.75rem;
  transition: all 0.2s ease;
  cursor: pointer;
  display: flex;
  flex-direction: column;
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
  object-fit: cover;
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
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.anime-title {
  font-size: 14px;
  margin: 0 0 10px;
  line-height: 1.4;
  height: 40px;
  overflow: hidden;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  color: var(--text-primary);
  font-weight: 600;
}

:deep(.el-tag) {
  margin-top: auto;
}

.pagination-container {
  margin-top: 32px;
  display: flex;
  justify-content: center;
  padding: 0 1rem;
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

/* 响应式调整 */
@media (max-width: 768px) {
  .anime-card {
    max-width: 100%; /* 在小屏幕上取消最大宽度限制 */
  }
  
  .anime-image {
    height: 180px; /* 在小屏幕上稍微降低图片高度 */
  }
}

.anime-desc {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 8px 0;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}
</style>
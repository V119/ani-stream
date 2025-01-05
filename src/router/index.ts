import HomeView from '../views/HomeView.vue'
import { createRouter, createWebHistory } from 'vue-router'


const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: HomeView
        },
        {
            path: '/player/:id',
            name: 'player',
            component: () => import('../views/PlayerView.vue')
        }
        // 其他路由...
    ]
})

export default router 
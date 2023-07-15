import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/random/csv'
    },
    {
      path: '/random/csv',
      name: 'csvRand',
      component: () => import('../components/random/CsvRand.vue') //延迟加载
    },
    {
      path: '/random/csv/setting',
      name: 'csvRand_setting',
      component: () => import('../components/random/CsvRandSetting.vue')
    }
  ]
})

export default router

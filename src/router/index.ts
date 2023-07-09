import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/rand/csv'
    },
    {
      path: '/rand/csv',
      name: 'csvRand',
      component: () => import('../components/csv_rand/main.vue') //延迟加载
    },
    {
      path: '/rand/csv/setting',
      name: 'csvRand_setting',
      component: () => import('../components/csv_rand/Setting_Page.vue')
    }
  ]
})

export default router

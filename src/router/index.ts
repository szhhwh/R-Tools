import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/rand/csv'
      // name: 'home',
      // component: HomeView
    },
    {
      path: '/rand/csv',
      name: 'csvRand',
      component: () => import('../views/csvRandView.vue') //延迟加载
    }
  ]
})

export default router

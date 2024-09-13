import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/calculators',
      name: 'calculators',
      component: () => import('../views/CalculatorView.vue'),
      children: [
        {
          path: '/calculators/timeLapsephoto',
          name: 'timeLapsephoto',
          component: () => import('../components/calculators/TimeLapsephoto.vue')
        },
      ]
    },
    {
      path: '/random',
      name: 'random',
      component: () => import('../views/RandomView.vue'),
      children: [
        {
          path: '/random/calarand',
          name: 'calarand',
          component: () => import('../components/random/CalaRand.vue')
        }
      ]
    },
    {
      path: '/app',
      name: 'app',
      children: [
        {
          path: '/app/update_dialog',
          name: 'update_dialog',
          component: () => import('../views/UpdateDialog.vue')
        },
        {
          path: '/app/setting_center',
          name: 'setting_center',
          component: () => import('../views/SettingView.vue')
        }
      ]
    }
  ]
})

export default router

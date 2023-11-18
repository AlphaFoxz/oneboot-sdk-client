import { RouteRecordRaw, createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('@/views/Home.vue'),
  },
  {
    path: '/genRestfulApi',
    name: 'GenRestfulApi',
    component: () => import('@/views/GenRestfulApi.vue'),
  },
  {
    path: '/genCrudService',
    name: 'GenCrudService',
    component: () => import('@/views/GenCrudService.vue'),
  },
  {
    path: '/helpApisUtil',
    name: 'HelpApisUtil',
    component: () => import('@/views/HelpApisUtil.vue'),
  },
  {
    path: '/helpDatabase',
    name: 'HelpDatabase',
    component: () => import('@/views/HelpDatabase.vue'),
  },
  {
    path: '/options',
    name: 'Options',
    component: () => import('@/views/Options.vue'),
  },
] as RouteRecordRaw[]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router

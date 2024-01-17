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
    path: '/genApisUtil',
    name: 'GenApisUtil',
    component: () => import('@/views/GenApisUtil.vue'),
  },
  {
    path: '/genApiDocs',
    name: 'GenApiDocs',
    component: () => import('@/views/GenApiDocs.vue'),
  },
  {
    path: '/rulesOfDatabase',
    name: 'RulesOfDatabase',
    component: () => import('@/views/RulesOfDatabase.vue'),
  },
  {
    path: '/rulesOfJava',
    name: 'RulesOfJava',
    component: () => import('@/views/RulesOfJava.vue'),
  },
  {
    path: '/rulesOfRestfulLanguage',
    name: 'RulesOfRestfulLanguage',
    component: () => import('@/views/RulesOfRestfulLanguage.vue'),
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('@/views/Settings.vue'),
  },
  {
    path: '/checkClientCode',
    name: 'CheckClientCode',
    component: () => import('@/views/CheckClientCode.vue'),
  },
  {
    path: '/checkServerCode',
    name: 'CheckServerCode',
    component: () => import('@/views/CheckServerCode.vue'),
  },
  {
    path: '/checkServiceStatus',
    name: 'CheckServiceStatus',
    component: () => import('@/views/CheckServiceStatus.vue'),
  },
] as RouteRecordRaw[]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router

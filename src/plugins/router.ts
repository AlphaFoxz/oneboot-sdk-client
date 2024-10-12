import { App } from 'vue'
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
    path: '/checkServerRestfulCode',
    name: 'CheckServerRestfulCode',
    component: () => import('@/views/CheckServerRestfulCode.vue'),
  },
  {
    path: '/checkServiceStatus',
    name: 'CheckServiceStatus',
    component: () => import('@/views/CheckServiceStatus.vue'),
  },
  {
    path: '/genDomain',
    name: 'GenDomain',
    component: () => import('@/views/GenDomain.vue'),
  },
] as RouteRecordRaw[]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

export function applyRouterPlugin(app: App) {
  app.use(router)
}

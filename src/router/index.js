import { createMemoryHistory, createRouter } from 'vue-router'
import HomeView from '../HomeView.vue'
import ImportView from '../ImportView.vue'

const routes = [
  { path: '/', component: HomeView },
  { path: '/import', component: ImportView },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export default router
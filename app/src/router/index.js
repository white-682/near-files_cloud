import Vue from 'vue'
import VueRouter from 'vue-router'
import Home from '../views/Home.vue'
import Vote from '../views/store.vue'
import List from '../views/List.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    redirect: '/list',
    name: 'home',
    component: Home,
    children: [
      {
        path: 'list',
        component: List
      },
      {
        path: 'store',
        redirect: '/list'
      },
      {
        path: 'store/:vid',
        name: 'vote',
        component: Vote
      }
    ]
  }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

export default router

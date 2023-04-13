import {createRouter, createWebHistory, RouteRecordRaw} from 'vue-router';

export const home: RouteRecordRaw = {
  path: '/',
  name: 'Home',
  component: () => import("../pages/Home.vue")
}


const routes: Array<RouteRecordRaw> = [
  home
]

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;

import { createRouter, createWebHistory } from 'vue-router'
import LoginPage from "./components/LoginPage.vue";

const routes = [
    { path: '/', component: LoginPage },
]

export const router = createRouter({
    history: createWebHistory(),
    routes
})
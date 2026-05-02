import { createRouter, createWebHistory } from 'vue-router'
import LoginPage from "./components/LoginPage.vue";
import FoldersPage from "./components/FoldersPage.vue";

const routes = [
    { path: '/', name: "login", component: LoginPage },
    { path: '/files', name: "files", component: FoldersPage },
    { path: '/:pathMatch(.*)*', redirect: '/files' },
]

export const router = createRouter({
    history: createWebHistory(),
    routes
})

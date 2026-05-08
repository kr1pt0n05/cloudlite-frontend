import { createRouter, createWebHistory } from 'vue-router'
import FilesPage from "./features/files/pages/FilesPage.vue";
import LoginPage from "./features/login/pages/LoginPage.vue";

const routes = [
    { path: '/', name: "login", component: LoginPage },
    { path: '/files', name: "files", component: FilesPage },
    { path: '/:pathMatch(.*)*', redirect: '/files' },
]

export const router = createRouter({
    history: createWebHistory(),
    routes
})

<script setup lang="ts">
import {computed, onMounted} from "vue";
import { RouterView, useRoute } from "vue-router";
import NavBar from "./shared/components/navigation/NavBar.vue";
import {router} from "./routes.ts";
import { isAuthenticated as fetchIsAuthenticated } from "./features/login/services/authService";

const route = useRoute();
const showShell = computed(() => route.name !== "login");

onMounted(() => {
  fetchIsAuthenticated()
      .then(isAuth => {
        if (isAuth) {
          router.push({ name: 'files' })
        }
    })
      .catch(error => {
    console.log(error);
  })
})

</script>

<template>
  <RouterView v-if="!showShell" />

  <div v-else class="app-window flex h-screen w-screen flex-col overflow-hidden bg-background text-foreground">

    <div class="flex min-h-0 flex-1 overflow-hidden">
      <NavBar />
      <main class="flex min-w-0 flex-1 flex-col overflow-hidden bg-background">
        <RouterView />
      </main>
    </div>
  </div>
</template>

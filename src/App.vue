<script setup lang="ts">
import {computed, onMounted, ref} from "vue";
import { RouterView, useRoute } from "vue-router";
import NavBar from "./components/NavBar.vue";
import {invoke} from "@tauri-apps/api/core";
import {router} from "./routes.ts";

const route = useRoute();
const showShell = computed(() => route.name !== "login");
const isAuthenticated = ref<boolean>(false);


onMounted(() => {
  invoke<boolean>("is_authenticated")
      .then(isAuth => {
        isAuthenticated.value = isAuth;
        router.push({ name: 'files' })
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

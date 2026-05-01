<script setup lang="ts">
import { onBeforeUnmount, onMounted, watch } from "vue";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { faXmark } from "@fortawesome/free-solid-svg-icons";

const props = withDefaults(
  defineProps<{
    modelValue: boolean;
    title?: string;
    description?: string;
    closeOnBackdrop?: boolean;
  }>(),
  {
    title: "",
    description: "",
    closeOnBackdrop: true,
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: boolean];
  close: [];
}>();

const popupId = Math.random().toString(36).slice(2);
const titleId = `popup-window-title-${popupId}`;
const descriptionId = `popup-window-description-${popupId}`;

function close() {
  emit("update:modelValue", false);
  emit("close");
}

function handleBackdropClick() {
  if (props.closeOnBackdrop) {
    close();
  }
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === "Escape" && props.modelValue) {
    close();
  }
}

watch(
  () => props.modelValue,
  (isOpen) => {
    document.body.style.overflow = isOpen ? "hidden" : "";
  },
);

onMounted(() => {
  window.addEventListener("keydown", handleKeydown);
});

onBeforeUnmount(() => {
  window.removeEventListener("keydown", handleKeydown);
  document.body.style.overflow = "";
});
</script>

<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition-opacity duration-150"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition-opacity duration-150"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="modelValue"
        class="fixed inset-0 z-50 bg-black/70"
        aria-hidden="true"
        @click="handleBackdropClick"
      />
    </Transition>

    <Transition
      enter-active-class="transition duration-150 ease-out"
      enter-from-class="scale-95 opacity-0"
      enter-to-class="scale-100 opacity-100"
      leave-active-class="transition duration-150 ease-in"
      leave-from-class="scale-100 opacity-100"
      leave-to-class="scale-95 opacity-0"
    >
      <section
        v-if="modelValue"
        class="fixed left-1/2 top-1/2 z-50 grid w-[calc(100vw-2rem)] max-w-lg -translate-x-1/2 -translate-y-1/2 gap-5 rounded-lg border border-border bg-surface p-6 text-foreground shadow-lg"
        role="dialog"
        aria-modal="true"
        :aria-labelledby="title ? titleId : undefined"
        :aria-describedby="description ? descriptionId : undefined"
      >
        <button
          class="absolute right-4 top-4 inline-flex h-8 w-8 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-surface-hover hover:text-foreground focus:outline-none focus:ring-2 focus:ring-ring/25"
          type="button"
          aria-label="Close popup"
          @click="close"
        >
          <FontAwesomeIcon class="h-4 w-4" :icon="faXmark" />
        </button>

        <header v-if="title || description" class="pr-8">
          <h2 v-if="title" :id="titleId" class="text-[18px] font-semibold leading-none tracking-tight">
            {{ title }}
          </h2>
          <p v-if="description" :id="descriptionId" class="mt-2 text-[13px] leading-relaxed text-muted-foreground">
            {{ description }}
          </p>
        </header>

        <div class="min-w-0">
          <slot />
        </div>

        <footer v-if="$slots.footer" class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
          <slot name="footer" />
        </footer>
      </section>
    </Transition>
  </Teleport>
</template>

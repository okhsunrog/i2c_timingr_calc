<template>
  <select v-model="currentTheme" class="select select-bordered select-sm">
    <option v-for="theme in themes" :key="theme" :value="theme">
      {{ theme.charAt(0).toUpperCase() + theme.slice(1) }}
    </option>
  </select>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import tailwindConfig from '../../tailwind.config'

const themes = tailwindConfig.daisyui.themes
const currentTheme = ref(localStorage.getItem('theme') || themes[0])

watch(currentTheme, (newTheme) => {
  localStorage.setItem('theme', newTheme)
  document.documentElement.setAttribute('data-theme', newTheme)
})

onMounted(() => {
  document.documentElement.setAttribute('data-theme', currentTheme.value)
})
</script>

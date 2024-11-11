<template>
  <main class="min-h-screen bg-base-200 p-8">
    <div class="container mx-auto max-w-5xl">
      <div class="flex justify-between items-center mb-8">
        <h1 class="text-3xl font-bold">STM32L071 I2C Timing Register Calculator</h1>
        <select data-choose-theme class="select select-bordered select-sm">
          <option v-for="theme in themes" :key="theme" :value="theme">
            {{ theme.charAt(0).toUpperCase() + theme.slice(1) }}
          </option>
        </select>
      </div>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <TimingCalculator
          v-model="registerHex"
          :showFormulas="showFormulas"
          @calculate="
            ({ i2cFreq: newFreq, i2cclk: newClk }) => {
              i2cFreq = newFreq
              i2cclk = newClk
              showFormulas = true
            }
          "
          @showFormulas="showFormulas = true"
          @error="error = $event"
        />

        <TimingFormulas
          v-if="showFormulas"
          :registerHex="registerHex"
          :i2cclk="i2cclk"
          :i2cFreq="i2cFreq"
          @hideFormulas="showFormulas = false"
        />
        <TimingRegister v-model="registerHex" />
      </div>
    </div>
  </main>
  <div class="toast toast-end">
    <div v-if="error" class="alert alert-error">
      <span>{{ error }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { themeChange } from 'theme-change'
import tailwindConfig from '../tailwind.config'
import TimingCalculator from '@/components/TimingCalculator.vue'
import TimingRegister from '@/components/TimingRegister.vue'
import TimingFormulas from '@/components/TimingFormulas.vue'

onMounted(() => {
  themeChange(false)
})

const i2cFreq = ref(400)
const i2cclk = ref(16)
const error = ref<string | null>(null)
const themes = tailwindConfig.daisyui.themes
const registerHex = ref(localStorage.getItem('i2c-register') || '0x00000000')
const showFormulas = ref(false)

watch(registerHex, (newValue) => {
  localStorage.setItem('i2c-register', newValue)
})
</script>

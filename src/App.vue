<template>
  <main class="min-h-screen bg-base-200 p-4">
    <div class="container mx-auto max-w-full">
      <div class="flex justify-between items-center mb-4">
        <h1 class="text-2xl font-bold">STM32L071 I2C Timing Register Calculator</h1>
        <ThemeSelector />
      </div>
      <div class="grid xl:grid-cols-3 grid-cols-1 gap-4">
        <div class="xl:col-span-1 flex flex-col gap-4">
          <TimingCalculator
            class="flex-1"
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
            @timingError="error = $event"
          />
          <TimingFormulas
            v-if="showFormulas"
            class="flex-1"
            :registerHex="registerHex"
            :i2cclk="i2cclk"
            :i2cFreq="i2cFreq"
            @hideFormulas="showFormulas = false"
          />
        </div>
        <div class="xl:col-span-2">
          <TimingRegister v-model="registerHex" />
        </div>
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
import { ref, watch } from 'vue'
import TimingCalculator from '@/components/TimingCalculator.vue'
import TimingRegister from '@/components/TimingRegister.vue'
import TimingFormulas from '@/components/TimingFormulas.vue'
import ThemeSelector from '@/components/ThemeSelector.vue'

const i2cFreq = ref(400)
const i2cclk = ref(16)
const error = ref<string | null>(null)
const registerHex = ref(localStorage.getItem('i2c-register') || '0x00000000')
const showFormulas = ref(false)

// Add a watcher for error that sets a timeout to clear it
watch(error, (newError) => {
  if (newError) {
    setTimeout(() => {
      error.value = null
    }, 3000) // Error will disappear after 3 seconds
  }
})

watch(registerHex, (newValue) => {
  localStorage.setItem('i2c-register', newValue)
})
</script>

<template>
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title">Register Value (32-bit hex)</h2>
      <div class="space-y-2">
        <div class="flex flex-wrap gap-2 items-center">
          <input
            v-model="registerHex"
            type="text"
            class="input input-bordered w-full sm:w-48"
            placeholder="0x00000000"
          />
          <div class="flex gap-2">
            <button class="btn btn-sm btn-accent" @click="resetRegister">Reset</button>
            <button class="btn btn-sm btn-success" @click="setDefaultValue">Default</button>
          </div>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="form-control">
            <label class="label">
              <span class="label-text">I2C Bus Frequency (kHz)</span>
            </label>
            <input
              v-model.number="inputFreq"
              type="number"
              class="input input-bordered"
              placeholder="400"
            />
          </div>
          <div class="form-control">
            <label class="label">
              <span class="label-text">I2CCLK (MHz)</span>
            </label>
            <input
              v-model.number="inputClk"
              type="number"
              class="input input-bordered"
              placeholder="16"
            />
          </div>
          <button class="btn btn-primary" @click="calculate">Calculate</button>
          <button v-if="!showFormulas" class="btn btn-secondary" @click="$emit('showFormulas')">
            Show Formulas
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { TimingResult } from '@/types/timing'
import { registerFromFields } from '@/utils/register'

const emit = defineEmits<{
  (event: 'showFormulas'): void
  (event: 'update:modelValue', value: string): void
  (event: 'timingError', message: string): void
  (event: 'calculate', data: { i2cFreq: number; i2cclk: number }): void
}>()


const props = defineProps<{
  modelValue: string
  showFormulas: boolean
}>()

const registerHex = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
})

const inputFreq = ref(Number(localStorage.getItem('i2c-freq')) || 400)
const inputClk = ref(Number(localStorage.getItem('i2cclk')) || 16)

// Add watchers for persistence
watch(inputFreq, (newValue) => {
  localStorage.setItem('i2c-freq', newValue.toString())
})

watch(inputClk, (newValue) => {
  localStorage.setItem('i2cclk', newValue.toString())
})

function resetRegister(): void {
  registerHex.value = '0x00000000'
}

function setDefaultValue(): void {
  registerHex.value = '0x300619'
}

async function calculate(): Promise<void> {
  try {
    const result = await invoke<TimingResult>('get_timings', {
      i2cclk: inputClk.value * 1000000,
      freq: inputFreq.value * 1000
    })
    console.log('Timing result:', result)
    registerHex.value = registerFromFields(result)
    emit('calculate', { i2cFreq: inputFreq.value, i2cclk: inputClk.value })
  } catch (e) {
    console.error(e)
    emit('timingError', (e as Error).message)
  }
}

</script>

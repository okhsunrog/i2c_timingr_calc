<template>
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title">Register Value (32-bit hex)</h2>
      <div class="space-y-2">
        <div class="flex gap-2 items-center">
          <input
            v-model="registerHex"
            type="text"
            class="input input-bordered w-48"
            placeholder="0x00000000"
          />
          <button class="btn btn-sm btn-accent" @click="resetRegister">Reset</button>
          <button class="btn btn-sm btn-success" @click="setDefaultValue">Default</button>
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
import { ref, computed } from 'vue'
import type { TimingResult } from '@/types/timing'

const emit = defineEmits<{
  calculate: [result: TimingResult]
  showFormulas: []
  'update:modelValue': [value: string]
  error: [message: string]
}>()

const props = defineProps<{
  modelValue: string
  showFormulas: boolean
}>()

const registerHex = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
})

const inputFreq = ref(400)
const inputClk = ref(16)

function resetRegister(): void {
  registerHex.value = '0x00000000'
}

function setDefaultValue(): void {
  registerHex.value = '0x300619'
}

function calculate(): void {
  try {
    const result = calculateTimings(inputClk.value * 1000000, inputFreq.value * 1000)
    emit('calculate', result)
  } catch (e) {
    emit('error', (e as Error).message)
  }
}

function calculateTimings(i2cclk: number, freq: number): TimingResult {
  // Ratio check
  const ratio: number = Math.floor(i2cclk / freq)
  if (ratio < 4) {
    throw new Error('The I2C PCLK must be at least 4 times the bus frequency!')
  }

  let presc_reg: number
  let scll: number
  let sclh: number
  let sdadel: number
  let scldel: number

  if (freq > 100000) {
    // Fast-mode (Fm) or Fast-mode Plus (Fm+)
    presc_reg = Math.floor((ratio - 1) / 384)
    const presc: number = presc_reg + 1

    sclh = Math.floor((ratio / presc - 3) / 3)
    scll = Math.floor(2 * (sclh + 1) - 1)

    if (freq > 400000) {
      // Fast-mode Plus (Fm+)
      if (i2cclk < 17000000) throw new Error('I2CCLK too low for Fm+')

      sdadel = Math.floor(i2cclk / 8000000 / presc)
      scldel = Math.floor(i2cclk / 4000000 / presc) - 1
    } else {
      // Fast-mode (Fm)
      if (i2cclk < 8000000) throw new Error('I2CCLK too low for Fm')

      sdadel = Math.floor(i2cclk / 4000000 / presc)
      scldel = Math.floor(i2cclk / 2000000 / presc) - 1
    }
  } else {
    // Standard-mode (Sm)
    if (i2cclk < 2000000) throw new Error('I2CCLK too low for Sm')

    const presc: number = Math.floor((ratio - 1) / 512)
    presc_reg = Math.min(presc, 15)

    sclh = Math.floor(ratio / (presc_reg + 1) - 2) / 2
    scll = sclh

    if (sclh >= 256) throw new Error('The I2C PCLK is too fast for this bus frequency!')

    sdadel = Math.floor(i2cclk / 2000000 / (presc_reg + 1))
    scldel = Math.floor(i2cclk / 500000 / (presc_reg + 1)) - 1
  }

  // Sanity checks and limits
  if (presc_reg >= 16) throw new Error('Prescaler value too high')

  sdadel = Math.max(sdadel, 2)
  scldel = Math.max(scldel, 4)

  return {
    presc: presc_reg,
    scll: Math.floor(scll),
    sclh: Math.floor(sclh),
    sdadel: Math.floor(sdadel),
    scldel: Math.floor(scldel),
  }
}
</script>

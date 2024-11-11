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
          @calculate="onCalculate"
          @showFormulas="showFormulas = true"
          @error="error = $event"
        />

        <div v-if="showFormulas" class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <div class="flex justify-between items-center">
              <h2 class="card-title">Timing Calculations</h2>
              <button class="btn btn-sm btn-secondary" @click="showFormulas = false">
                Hide Formulas
              </button>
            </div>
            <div class="space-y-4">
              <div
                v-katex:display="
                  '\\text{Ratio} = \\frac{f_{I2CCLK}}{f_{I2C}} = \\frac{' +
                  i2cclk +
                  '\\text{ MHz}}{' +
                  i2cFreq +
                  '\\text{ kHz}} = ' +
                  timingValues.ratio.toFixed(2)
                "
              ></div>

              <div v-katex:display="'\\text{Mode: } ' + timingValues.mode"></div>

              <div
                v-katex:display="
                  't_{I2CCLK} = \\frac{1}{' +
                  i2cclk +
                  '\\text{ MHz}} = ' +
                  timingValues.tI2CCLK.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  '\\text{PRESC} = \\left\\lfloor\\frac{\\text{ratio} - 1}{' +
                  (i2cFreq <= 100 ? '512' : '384') +
                  '}\\right\\rfloor = ' +
                  fields.presc +
                  '\\text{, actual presc = }' +
                  timingValues.presc
                "
              ></div>

              <div
                v-katex:display="
                  't_{PRESC} = (PRESC + 1) \\times t_{I2CCLK} = ' +
                  timingValues.tPRESC.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  't_{SCLDEL} = (SCLDEL + 1) \\times t_{PRESC} = ' +
                  timingValues.tSCLDEL.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  't_{SDADEL} = SDADEL \\times t_{PRESC} = ' +
                  timingValues.tSDADEL.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  't_{SCLH} = (SCLH + 1) \\times t_{PRESC} = ' +
                  timingValues.tSCLH.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  't_{SCLL} = (SCLL + 1) \\times t_{PRESC} = ' +
                  timingValues.tSCLL.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  't_{SYNC1} + t_{SYNC2} > 4 \\times t_{I2CCLK} = ' +
                  timingValues.tSYNC.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  't_{SCL} = t_{SYNC1} + t_{SYNC2} + t_{SCLL} + t_{SCLH} \\approx ' +
                  timingValues.tSCL.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  'f_{I2C} = \\frac{1}{t_{SCL}} \\approx ' +
                  timingValues.actualFreq.toFixed(2) +
                  '\\text{ kHz}'
                "
              ></div>
            </div>
          </div>
        </div>
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
import { onMounted, ref, reactive, watch, computed } from 'vue'
import { themeChange } from 'theme-change'
import tailwindConfig from '../tailwind.config'
import type { TimingResult, TimingFields } from '@/types/timing'
import TimingRegister from '@/components/TimingRegister.vue'
import TimingCalculator from '@/components/TimingCalculator.vue'


onMounted(() => {
  themeChange(false)
})

const timingValues = computed(() => {
  // Clock calculations
  const tI2CCLK = 1000 / i2cclk.value
  const presc = fields.presc + 1
  const tPRESC = presc * tI2CCLK

  // SCL timing calculations
  const tSCLH = (fields.sclh + 1) * tPRESC
  const tSCLL = (fields.scll + 1) * tPRESC
  const tSYNC = 4 * tI2CCLK // t_SYNC1 + t_SYNC2 > 4 * t_I2CCLK
  const tSCL = tSCLH + tSCLL + tSYNC

  return {
    ratio: (i2cclk.value * 1000) / i2cFreq.value,
    mode:
      i2cFreq.value > 400
        ? 'Fast-mode Plus (Fm+)'
        : i2cFreq.value > 100
          ? 'Fast-mode (Fm)'
          : 'Standard-mode (Sm)',
    tI2CCLK,
    presc,
    tPRESC,
    tSCLDEL: (fields.scldel + 1) * tPRESC,
    tSDADEL: fields.sdadel * tPRESC,
    tSCLH,
    tSCLL,
    tSYNC,
    tSCL,
    actualFreq: 1e6 / tSCL,
  }
})

const error = ref<string | null>(null)

const themes = tailwindConfig.daisyui.themes
const registerHex = ref(localStorage.getItem('i2c-register') || '0x00000000')
const showFormulas = ref(false)

// Current values used in calculations
const i2cFreq = ref(Number(localStorage.getItem('i2c-freq')) || 400)
const i2cclk = ref(Number(localStorage.getItem('i2cclk')) || 16)

const fields = reactive<TimingResult>({
  presc: 0,
  scldel: 0,
  sdadel: 0,
  sclh: 0,
  scll: 0,
})

// Add watcher for persistence
watch(registerHex, (newValue) => {
  localStorage.setItem('i2c-register', newValue)
})

watch(i2cFreq, (newValue) => {
  localStorage.setItem('i2c-freq', newValue.toString())
})

watch(i2cclk, (newValue) => {
  localStorage.setItem('i2cclk', newValue.toString())
})

function onCalculate(result: TimingResult): void {
  try {
    registerHex.value = generateHexFromResult(result)
    showFormulas.value = true
    error.value = null
  } catch (e) {
    error.value = (e as Error).message
    setTimeout(() => {
      error.value = null
    }, 3000)
  }
}

function generateHexFromResult(result: TimingResult): string {
  const value: bigint =
    ((BigInt(result.presc) & 0xfn) << 28n) |
    ((BigInt(result.scldel) & 0xfn) << 20n) |
    ((BigInt(result.sdadel) & 0xfn) << 16n) |
    ((BigInt(result.sclh) & 0xffn) << 8n) |
    (BigInt(result.scll) & 0xffn)
  return '0x' + value.toString(16).toUpperCase().padStart(8, '0')
}
</script>

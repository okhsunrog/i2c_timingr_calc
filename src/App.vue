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
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">Register Value (32-bit hex)</h2>
            <div class="space-y-2">
              <div class="flex gap-2 items-center">
                <input
                  v-model="registerHex"
                  type="text"
                  class="input input-bordered w-full max-w-xs"
                  placeholder="0x00000000"
                  @input="updateFieldsFromRegister"
                />
                <button class="btn btn-sm btn-accent" @click="resetRegister">Reset</button>
                <button class="btn btn-sm btn-success" @click="setDefaultValue">Default</button>
              </div>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label">
                    <span class="label-text">I2C Freq (kHz)</span>
                  </label>
                  <input
                    v-model.number="inputFreq"
                    type="number"
                    class="input input-bordered input-sm w-full"
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
                    class="input input-bordered input-sm w-full"
                    placeholder="16"
                  />
                </div>
                <button class="btn btn-primary" @click="calculateFromFreqs">Calculate</button>
                <button v-if="!showFormulas" class="btn btn-secondary" @click="showFormulas = true">
                  Show Formulas
                </button>
              </div>
            </div>
          </div>
        </div>

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

// Input values that change with user input
const inputFreq = ref(i2cFreq.value)
const inputClk = ref(i2cclk.value)

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

function resetRegister(): void {
  registerHex.value = '0x00000000'
}

function setDefaultValue(): void {
  registerHex.value = '0x300619'
}

function calculateFromFreqs(): void {
  try {
    const result = calculateTimings(inputClk.value * 1000000, inputFreq.value * 1000)

    // Update the main values from inputs
    i2cFreq.value = inputFreq.value
    i2cclk.value = inputClk.value

    fields.presc = result.presc
    fields.scldel = result.scldel
    fields.sdadel = result.sdadel
    fields.sclh = result.sclh
    fields.scll = result.scll
    showFormulas.value = true
    error.value = null
  } catch (e) {
    error.value = (e as Error).message
    setTimeout(() => {
      error.value = null
    }, 3000)
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

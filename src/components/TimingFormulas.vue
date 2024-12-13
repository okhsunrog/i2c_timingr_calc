<template>
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <div class="flex flex-wrap gap-2 items-center">
        <h2 class="card-title">Timing Calculations</h2>
        <button class="btn btn-sm btn-secondary" @click="$emit('hideFormulas')">
          Hide Formulas
        </button>
      </div>
      <div class="space-y-4">
        <div
          v-katex="
            '\\text{Ratio} = \\frac{f_{I2CCLK}}{f_{I2C}} = \\frac{' +
            i2cclk +
            '\\text{ MHz}}{' +
            i2cFreq +
            '\\text{ kHz}} = ' +
            timingValues.ratio.toFixed(2)
          "
        ></div>

        <div v-katex="'\\text{Mode: } ' + timingValues.mode"></div>

        <div
          v-katex="
            't_{I2CCLK} = \\frac{1}{' +
            i2cclk +
            '\\text{ MHz}} = ' +
            timingValues.tI2CCLK.toFixed(2) +
            '\\text{ ns}'
          "
        ></div>

        <div
          v-katex="
            '\\text{PRESC} = \\left\\lfloor\\frac{\\text{ratio} - 1}{' +
            (i2cFreq <= 100 ? '512' : '384') +
            '}\\right\\rfloor = ' +
            fields.presc +
            '\\text{, actual presc = }' +
            timingValues.presc
          "
        ></div>

        <div
          v-katex="
            't_{PRESC} = (PRESC + 1) \\times t_{I2CCLK} = ' +
            timingValues.tPRESC.toFixed(2) +
            '\\text{ ns}'
          "
        ></div>

        <div
          v-katex="
            't_{SCLDEL} = (SCLDEL + 1) \\times t_{PRESC} = ' +
            timingValues.tSCLDEL.toFixed(2) +
            '\\text{ ns}'
          "
        ></div>

        <div
          v-katex="
            't_{SDADEL} = SDADEL \\times t_{PRESC} = ' +
            timingValues.tSDADEL.toFixed(2) +
            '\\text{ ns}'
          "
        ></div>

        <div
          v-katex="
            't_{SCLH} = (SCLH + 1) \\times t_{PRESC} = ' +
            timingValues.tSCLH.toFixed(2) +
            '\\text{ ns}'
          "
        ></div>

        <div
          v-katex="
            't_{SCLL} = (SCLL + 1) \\times t_{PRESC} = ' +
            timingValues.tSCLL.toFixed(2) +
            '\\text{ ns}'
          "
        ></div>

        <div
          v-katex="
            't_{SYNC1} + t_{SYNC2} > 4 \\times t_{I2CCLK} = ' +
            timingValues.tSYNC.toFixed(2) +
            '\\text{ ns}'
          "
        ></div>

        <div
          v-katex="
            't_{SCL} = t_{SYNC1} + t_{SYNC2} + t_{SCLL} + t_{SCLH} \\approx ' +
            timingValues.tSCL.toFixed(2) +
            '\\text{ ns}'
          "
        ></div>

        <div
          v-katex="
            'f_{I2C} = \\frac{1}{t_{SCL}} \\approx ' +
            timingValues.actualFreq.toFixed(2) +
            '\\text{ kHz}'
          "
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { fieldsFromRegister } from '@/utils/register'

const props = defineProps<{
  registerHex: string
  i2cclk: number
  i2cFreq: number
}>()

defineEmits<{
  hideFormulas: []
}>()

const fields = computed(() => fieldsFromRegister(props.registerHex))

const timingValues = computed(() => {
  const tI2CCLK = 1000 / props.i2cclk
  const presc = fields.value.presc + 1
  const tPRESC = presc * tI2CCLK

  const tSCLH = (fields.value.sclh + 1) * tPRESC
  const tSCLL = (fields.value.scll + 1) * tPRESC
  const tSYNC = 4 * tI2CCLK
  const tSCL = tSCLH + tSCLL + tSYNC

  return {
    ratio: (props.i2cclk * 1000) / props.i2cFreq,
    mode:
      props.i2cFreq > 400
        ? 'Fast-mode Plus (Fm+)'
        : props.i2cFreq > 100
          ? 'Fast-mode (Fm)'
          : 'Standard-mode (Sm)',
    tI2CCLK,
    presc,
    tPRESC,
    tSCLDEL: (fields.value.scldel + 1) * tPRESC,
    tSDADEL: fields.value.sdadel * tPRESC,
    tSCLH,
    tSCLL,
    tSYNC,
    tSCL,
    actualFreq: 1e6 / tSCL,
  }
})
</script>

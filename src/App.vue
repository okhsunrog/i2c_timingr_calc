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
        <!-- Hex Input -->
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
                  @input="updateFieldsFromRegister"
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
                    v-model.number="i2cFreq"
                    type="number"
                    class="input input-bordered"
                    placeholder="400000"
                  />
                </div>
                <div class="form-control">
                  <label class="label">
                    <span class="label-text">I2CCLK (MHz)</span>
                  </label>
                  <input
                    v-model.number="i2cclk"
                    type="number"
                    class="input input-bordered"
                    placeholder="16000000"
                  />
                </div>
                <button class="btn btn-primary" @click="calculateFromFreqs">Calculate</button>
              </div>
            </div>
          </div>
        </div>

        <div v-if="calculations" class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">Timing Calculations</h2>
            <div class="space-y-4">
              <div
                v-katex:display="
                  '\\text{Ratio} = \\frac{f_{I2CCLK}}{f_{I2C}} = \\frac{' +
                  i2cclk +
                  '\\text{ MHz}}{' +
                  i2cFreq +
                  '\\text{ kHz}} = ' +
                  calculations.ratio.toFixed(2)
                "
              ></div>

              <div v-katex:display="'\\text{Mode: } ' + calculations.mode"></div>

              <div
                v-katex:display="
                  't_{I2CCLK} = \\frac{1}{' +
                  i2cclk +
                  '\\text{ MHz}} = ' +
                  calculations.tI2CCLK.toFixed(2) +
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
                  calculations.presc
                "
              ></div>

              <div
                v-katex:display="
                  't_{PRESC} = (PRESC + 1) \\times t_{I2CCLK} = ' +
                  calculations.tPRESC.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  't_{SCLDEL} = (SCLDEL + 1) \\times t_{PRESC} = ' +
                  calculations.tSCLDEL.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  't_{SDADEL} = SDADEL \\times t_{PRESC} = ' +
                  calculations.tSDADEL.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  't_{SCLH} = (SCLH + 1) \\times t_{PRESC} = ' +
                  calculations.tSCLH.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  't_{SCLL} = (SCLL + 1) \\times t_{PRESC} = ' +
                  calculations.tSCLL.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  't_{SYNC1} + t_{SYNC2} > 4 \\times t_{I2CCLK} = ' +
                  calculations.tSYNC.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  't_{SCL} = t_{SYNC1} + t_{SYNC2} + t_{SCLL} + t_{SCLH} \\approx ' +
                  calculations.tSCL.toFixed(2) +
                  '\\text{ ns}'
                "
              ></div>

              <div
                v-katex:display="
                  'f_{I2C} = \\frac{1}{t_{SCL}} \\approx ' +
                  calculations.actualFreq.toFixed(2) +
                  '\\text{ kHz}'
                "
              ></div>
            </div>
          </div>
        </div>

        <div class="card bg-base-100 md:col-span-2 shadow-xl">
          <div class="card-body overflow-x-auto">
            <h2 class="card-title">Register Visualization</h2>
            <div class="join join-vertical min-w-fit space-y-4">
              <!-- Upper bits table -->
              <table class="join-item table table-zebra text-center">
                <thead>
                  <tr>
                    <th v-for="i in 16" :key="i">{{ 31 - (i - 1) }}</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td
                      v-for="i in 16"
                      :key="i"
                      class="font-mono cursor-pointer hover:bg-base-300"
                      :class="getBitAtPosition(32 - i) === 1 ? 'bg-accent/30' : 'bg-base-200'"
                      @click="toggleBit(32 - i)"
                    >
                      {{ getBitAtPosition(32 - i) }}
                    </td>
                  </tr>
                  <tr>
                    <td colspan="4" class="border-b-4 border-primary">
                      <div class="flex items-center gap-2">
                        <div class="text-left">
                          <div class="font-semibold">PRESC</div>
                          <div class="text-sm opacity-75">Timing prescaler (Bits 31:28)</div>
                        </div>
                        <input
                          type="text"
                          :value="`0x${fields.presc.toString(16).toUpperCase().padStart(1, '0')}`"
                          class="input input-bordered input-sm w-20"
                          @input="(e) => updateFieldFromHex('presc', e, 0xf)"
                        />
                      </div>
                    </td>
                    <td colspan="4" class="border-b-4 border-base-300">
                      <div class="text-left">
                        <div class="font-semibold">Reserved</div>
                        <div class="text-sm opacity-75">(Bits 27:24)</div>
                      </div>
                    </td>
                    <td colspan="4" class="border-b-4 border-secondary">
                      <div class="flex items-center gap-2">
                        <div class="text-left">
                          <div class="font-semibold">SCLDEL</div>
                          <div class="text-sm opacity-75">Data setup time (Bits 23:20)</div>
                        </div>
                        <input
                          type="text"
                          :value="`0x${fields.scldel.toString(16).toUpperCase().padStart(1, '0')}`"
                          class="input input-bordered input-sm w-20"
                          @input="(e) => updateFieldFromHex('scldel', e, 0xf)"
                        />
                      </div>
                    </td>
                    <td colspan="4" class="border-b-4 border-accent">
                      <div class="flex items-center gap-2">
                        <div class="text-left">
                          <div class="font-semibold">SDADEL</div>
                          <div class="text-sm opacity-75">Data hold time (Bits 19:16)</div>
                        </div>
                        <input
                          type="text"
                          :value="`0x${fields.sdadel.toString(16).toUpperCase().padStart(1, '0')}`"
                          class="input input-bordered input-sm w-20"
                          @input="(e) => updateFieldFromHex('sdadel', e, 0xf)"
                        />
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
              <!-- Lower bits table -->
              <table class="join-item table table-zebra text-center">
                <thead>
                  <tr>
                    <th v-for="i in 16" :key="i">{{ 15 - (i - 1) }}</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td
                      v-for="i in 16"
                      :key="i"
                      class="font-mono cursor-pointer hover:bg-base-300"
                      :class="getBitAtPosition(16 - i) === 1 ? 'bg-accent/30' : 'bg-base-200'"
                      @click="toggleBit(16 - i)"
                    >
                      {{ getBitAtPosition(16 - i) }}
                    </td>
                  </tr>
                  <tr>
                    <td colspan="8" class="border-b-4 border-info">
                      <div class="flex items-center gap-2">
                        <div class="text-left">
                          <div class="font-semibold">SCLH</div>
                          <div class="text-sm opacity-75">SCL high period (Bits 15:8)</div>
                        </div>
                        <input
                          type="text"
                          :value="`0x${fields.sclh.toString(16).toUpperCase().padStart(2, '0')}`"
                          class="input input-bordered input-sm w-20"
                          @input="(e) => updateFieldFromHex('sclh', e, 0xff)"
                        />
                      </div>
                    </td>
                    <td colspan="8" class="border-b-4 border-success">
                      <div class="flex items-center gap-2">
                        <div class="text-left">
                          <div class="font-semibold">SCLL</div>
                          <div class="text-sm opacity-75">SCL low period (Bits 7:0)</div>
                        </div>
                        <input
                          type="text"
                          :value="`0x${fields.scll.toString(16).toUpperCase().padStart(2, '0')}`"
                          class="input input-bordered input-sm w-20"
                          @input="(e) => updateFieldFromHex('scll', e, 0xff)"
                        />
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
            <div class="mt-6">
              <div class="collapse collapse-arrow bg-base-200">
                <input type="checkbox" />
                <div class="collapse-title text-xl font-medium">Notes</div>
                <div class="collapse-content text-sm italic opacity-75 space-y-4">
                  <p>
                    <strong>SCLDEL (Data setup time):</strong> Used to generate delay tSCLDEL
                    between SDA edge and SCL rising edge. In master mode and slave mode with
                    NOSTRETCH = 0, SCL line is stretched low during tSCLDEL. <br />
                    tSCLDEL = (SCLDEL+1) × tPRESC <br />
                    Note: tSCLDEL is used to generate tSU:DAT timing.
                  </p>

                  <p>
                    <strong>SDADEL (Data hold time):</strong> Used to generate delay tSDADEL between
                    SCL falling edge and SDA edge. In master mode and slave mode with NOSTRETCH = 0,
                    SCL line is stretched low during tSDADEL. <br />
                    tSDADEL = SDADEL × tPRESC <br />
                    Note: SDADEL is used to generate tHD:DAT timing.
                  </p>

                  <p>
                    <strong>SCLH (SCL high period):</strong> Used to generate the SCL high period in
                    master mode. <br />
                    tSCLH = (SCLH+1) × tPRESC <br />
                    Note: SCLH is also used to generate tSU:STO and tHD:STA timing.
                  </p>

                  <p>
                    <strong>SCLL (SCL low period):</strong> Used to generate the SCL low period in
                    master mode. <br />
                    tSCLL = (SCLL+1) × tPRESC <br />
                    Note: SCLL is also used to generate tBUF and tSU:STA timings.
                  </p>
                </div>
              </div>
            </div>
          </div>
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
import { onMounted, ref, reactive, watch } from 'vue'
import { themeChange } from 'theme-change'
import tailwindConfig from '../tailwind.config'

onMounted(() => {
  themeChange(false)
})

interface TimingResult {
  presc: number
  scldel: number
  sdadel: number
  sclh: number
  scll: number
}

interface TimingCalculation {
  // Original timing parameters
  timings: TimingResult
  // Intermediate calculations
  ratio: number
  mode: 'Standard-mode (Sm)' | 'Fast-mode (Fm)' | 'Fast-mode Plus (Fm+)'
  tI2CCLK: number
  presc: number
  tPRESC: number
  tSCLDEL: number
  tSDADEL: number
  tSCLH: number
  tSCLL: number
  tSYNC: number
  tSCL: number
  actualFreq: number
}

const error = ref<string | null>(null)

const themes = tailwindConfig.daisyui.themes
const i2cFreq = ref(Number(localStorage.getItem('i2c-freq')) || 400)
const i2cclk = ref(Number(localStorage.getItem('i2cclk')) || 16)
const registerHex = ref(localStorage.getItem('i2c-register') || '0x00000000')
const calculations = ref<TimingCalculation | null>(null)

const fields = reactive<TimingResult>({
  presc: 0,
  scldel: 0,
  sdadel: 0,
  sclh: 0,
  scll: 0,
})

// Initialize fields from stored value
updateFieldsFromRegister()

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

function updateFieldsFromRegister(): void {
  const value: bigint = BigInt(`0x${registerHex.value.replace('0x', '')}`)
  fields.presc = Number((value >> 28n) & 0xfn)
  fields.scldel = Number((value >> 20n) & 0xfn)
  fields.sdadel = Number((value >> 16n) & 0xfn)
  fields.sclh = Number((value >> 8n) & 0xffn)
  fields.scll = Number(value & 0xffn)
}

function updateRegisterFromFields(): void {
  const value: bigint =
    ((BigInt(fields.presc) & 0xfn) << 28n) |
    ((BigInt(fields.scldel) & 0xfn) << 20n) |
    ((BigInt(fields.sdadel) & 0xfn) << 16n) |
    ((BigInt(fields.sclh) & 0xffn) << 8n) |
    (BigInt(fields.scll) & 0xffn)
  registerHex.value = '0x' + value.toString(16).toUpperCase().padStart(8, '0')
}

function getBitAtPosition(position: number): number {
  const value: number = parseInt(registerHex.value, 16)
  return (value >> position) & 1
}

function toggleBit(position: number): void {
  const value: bigint = BigInt(`0x${registerHex.value.replace('0x', '')}`)
  const mask: bigint = 1n << BigInt(position)
  const newValue: bigint = value ^ mask
  registerHex.value = '0x' + (newValue & 0xffffffffn).toString(16).toUpperCase().padStart(8, '0')
  updateFieldsFromRegister()
}

function updateFieldFromHex(field: keyof typeof fields, event: Event, mask: number): void {
  const value = (event.target as HTMLInputElement).value
  const numValue = parseInt(value.replace('0x', ''), 16)
  if (!isNaN(numValue)) {
    fields[field] = numValue & mask
    updateRegisterFromFields()
  }
}

function resetRegister(): void {
  registerHex.value = '0x00000000'
  updateFieldsFromRegister()
}

function setDefaultValue(): void {
  registerHex.value = '0x300619'
  updateFieldsFromRegister()
}

function calculateFromFreqs(): void {
  try {
    const result = calculateTimings(i2cclk.value * 1000000, i2cFreq.value * 1000)
    fields.presc = result.timings.presc
    fields.scldel = result.timings.scldel
    fields.sdadel = result.timings.sdadel
    fields.sclh = result.timings.sclh
    fields.scll = result.timings.scll
    calculations.value = result
    updateRegisterFromFields()
    error.value = null
  } catch (e) {
    error.value = (e as Error).message
    setTimeout(() => {
      error.value = null
    }, 3000)
  }
}

function calculateTimings(i2cclk: number, freq: number): TimingCalculation {
  // Ratio check
  const ratio: number = Math.floor(i2cclk / freq)
  if (ratio < 4) {
    throw new Error('The I2C PCLK must be at least 4 times the bus frequency!')
  }

  const tI2CCLK = 1e9 / i2cclk // ns

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

  const timings: TimingResult = {
    presc: presc_reg,
    scll: Math.floor(scll),
    sclh: Math.floor(sclh),
    sdadel: Math.floor(sdadel),
    scldel: Math.floor(scldel),
  }

  const presc = presc_reg + 1
  const tPRESC = presc * tI2CCLK
  const tSCLH = (timings.sclh + 1) * tPRESC
  const tSCLL = (timings.scll + 1) * tPRESC
  const tSYNC = 4 * tI2CCLK // t_SYNC1 + t_SYNC2 > 4 * t_I2CCLK
  const tSCL = tSCLH + tSCLL + tSYNC

  return {
    timings,
    ratio,
    mode:
      freq > 400000
        ? 'Fast-mode Plus (Fm+)'
        : freq > 100000
          ? 'Fast-mode (Fm)'
          : 'Standard-mode (Sm)',
    tI2CCLK,
    presc,
    tPRESC,
    tSCLDEL: (timings.scldel + 1) * tPRESC,
    tSDADEL: timings.sdadel * tPRESC,
    tSCLH,
    tSCLL,
    tSYNC,
    tSCL,
    actualFreq: 1e6 / tSCL,
  }
}
</script>

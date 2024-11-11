<template>
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
              <strong>SCLDEL (Data setup time):</strong> Used to generate delay tSCLDEL between SDA
              edge and SCL rising edge. In master mode and slave mode with NOSTRETCH = 0, SCL line
              is stretched low during tSCLDEL. <br />
              tSCLDEL = (SCLDEL+1) × tPRESC <br />
              Note: tSCLDEL is used to generate tSU:DAT timing.
            </p>

            <p>
              <strong>SDADEL (Data hold time):</strong> Used to generate delay tSDADEL between SCL
              falling edge and SDA edge. In master mode and slave mode with NOSTRETCH = 0, SCL line
              is stretched low during tSDADEL. <br />
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
              <strong>SCLL (SCL low period):</strong> Used to generate the SCL low period in master
              mode. <br />
              tSCLL = (SCLL+1) × tPRESC <br />
              Note: SCLL is also used to generate tBUF and tSU:STA timings.
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive, watch, onMounted } from 'vue'
import type { TimingFields } from '@/types/timing'

const props = defineProps<{
  modelValue: string // v-model convention
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

// Create a computed property for two-way binding
const registerHex = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
})

// Watch for changes in registerHex
watch(registerHex, (newValue) => {
  fields.updateFromRegister(newValue)
})

// Initialize fields with the initial register value
onMounted(() => {
  fields.updateFromRegister(registerHex.value)
})

const fields = reactive<TimingFields>({
  presc: 0,
  scldel: 0,
  sdadel: 0,
  sclh: 0,
  scll: 0,
  updateFromRegister: (value: string) => {
    const bigValue = BigInt(`0x${value.replace('0x', '')}`)
    fields.presc = Number((bigValue >> 28n) & 0xfn)
    fields.scldel = Number((bigValue >> 20n) & 0xfn)
    fields.sdadel = Number((bigValue >> 16n) & 0xfn)
    fields.sclh = Number((bigValue >> 8n) & 0xffn)
    fields.scll = Number(bigValue & 0xffn)
  },
  updateFromFields: () => {
    const value: bigint =
      ((BigInt(fields.presc) & 0xfn) << 28n) |
      ((BigInt(fields.scldel) & 0xfn) << 20n) |
      ((BigInt(fields.sdadel) & 0xfn) << 16n) |
      ((BigInt(fields.sclh) & 0xffn) << 8n) |
      (BigInt(fields.scll) & 0xffn)
    return '0x' + value.toString(16).toUpperCase().padStart(8, '0')
  },
})

function getBitAtPosition(position: number): number {
  const value: number = parseInt(registerHex.value, 16)
  return (value >> position) & 1
}

function toggleBit(position: number): void {
  const value: bigint = BigInt(`0x${registerHex.value.replace('0x', '')}`)
  const mask: bigint = 1n << BigInt(position)
  const newValue: bigint = value ^ mask
  registerHex.value = '0x' + (newValue & 0xffffffffn).toString(16).toUpperCase().padStart(8, '0')
  fields.updateFromRegister(registerHex.value)
}

type NumericFields = Exclude<keyof TimingFields, 'updateFromRegister' | 'updateFromFields'>

function updateFieldFromHex(field: NumericFields, event: Event, mask: number): void {
  const value = (event.target as HTMLInputElement).value
  const numValue = parseInt(value.replace('0x', ''), 16)
  if (!isNaN(numValue)) {
    fields[field] = numValue & mask
    registerHex.value = fields.updateFromFields()
  }
}
</script>

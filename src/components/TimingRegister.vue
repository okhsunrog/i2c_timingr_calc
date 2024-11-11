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
                class="font-mono"
                :class="[
                  getBitAtPosition(32 - i) === 1 ? 'bg-accent/30' : 'bg-base-200',
                  isReservedBit(32 - i)
                    ? 'opacity-50 bg-base-300'
                    : 'cursor-pointer hover:bg-base-300',
                ]"
                @click="isReservedBit(32 - i) ? null : toggleBit(32 - i)"
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
      <TimingNotes />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { TimingResult } from '@/types/timing'
import { fieldsFromRegister, registerFromFields } from '@/utils/register'
import TimingNotes from './TimingNotes.vue'

const props = defineProps<{
  modelValue: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const registerHex = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
})

const fields = computed(() => fieldsFromRegister(registerHex.value))

type NumericFields = keyof TimingResult

function isReservedBit(position: number): boolean {
  return position >= 24 && position <= 27
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
}

function updateFieldFromHex(field: NumericFields, event: Event, mask: number): void {
  const value = (event.target as HTMLInputElement).value
  const numValue = parseInt(value.replace('0x', ''), 16)
  if (!isNaN(numValue)) {
    const newFields = { ...fields.value, [field]: numValue & mask }
    registerHex.value = registerFromFields(newFields)
  }
}
</script>

import type { TimingResult } from '@/types/timing'

export function fieldsFromRegister(registerHex: string): TimingResult {
  const value: bigint = BigInt(`0x${registerHex.replace('0x', '')}`)
  return {
    presc: Number((value >> 28n) & 0xfn),
    scldel: Number((value >> 20n) & 0xfn),
    sdadel: Number((value >> 16n) & 0xfn),
    sclh: Number((value >> 8n) & 0xffn),
    scll: Number(value & 0xffn),
  }
}

export function registerFromFields(fields: TimingResult): string {
  const value: bigint =
    ((BigInt(fields.presc) & 0xfn) << 28n) |
    ((BigInt(fields.scldel) & 0xfn) << 20n) |
    ((BigInt(fields.sdadel) & 0xfn) << 16n) |
    ((BigInt(fields.sclh) & 0xffn) << 8n) |
    (BigInt(fields.scll) & 0xffn)
  return '0x' + value.toString(16).toUpperCase().padStart(8, '0')
}

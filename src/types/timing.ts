export interface TimingResult {
  presc: number
  scldel: number
  sdadel: number
  sclh: number
  scll: number
}

export interface TimingFields extends TimingResult {
  updateFromRegister: (value: string) => void
  updateFromFields: () => string
}

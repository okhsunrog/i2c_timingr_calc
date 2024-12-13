export interface TimingResult {
  presc: number
  scll: number
  sclh: number
  sdadel: number
  scldel: number
}

export interface TimingFields extends TimingResult {
  updateFromRegister: (value: string) => void
  updateFromFields: () => string
}

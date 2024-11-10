import type { Config } from 'tailwindcss'
import daisyui from 'daisyui'

export default {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {},
  },
  plugins: [daisyui],
  daisyui: {
    themes: [
      'cupcake',
      'bumblebee',
      'retro',
      'halloween',
      'fantasy',
      'wireframe',
      'luxury',
      'dracula',
      'autumn',
      'business',
      'lemonade',
      'night',
      'coffee',
      'dim',
      'sunset',
    ],
    defaultTheme: 'lemonade',
  },
} satisfies Config

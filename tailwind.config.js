import daisyui from 'daisyui'
/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {},
  },
  plugins: [daisyui],
  daisyui: {
    themes: [
      'light',
      'cupcake',
      'bumblebee',
      'emerald',
      'corporate',
      'retro',
      'halloween',
      'garden',
      'lofi',
      'pastel',
      'fantasy',
      'wireframe',
      'luxury',
      'dracula',
      'cmyk',
      'autumn',
      'business',
      'acid',
      'lemonade',
      'night',
      'coffee',
      'winter',
      'dim',
      'nord',
      'sunset',
    ],
    defaultTheme: 'coffee',
  },
}

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
      'nord',
      'sunset',
    ],
    defaultTheme: 'coffee',
  },
}

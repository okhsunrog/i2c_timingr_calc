import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'
import Vue3Katex from 'vue3-katex'
import 'katex/dist/katex.min.css'

createApp(App).use(Vue3Katex).mount('#app')

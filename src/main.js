import { createApp } from 'vue'
import './style.css'
import App from './App.vue'

import * as wasm from '../wasm/pkg/rust_wasm_demo';

wasm.greet('vit-app');



createApp(App).mount('#app')

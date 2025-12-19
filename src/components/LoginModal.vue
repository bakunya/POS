<template>
  <div v-if="isOpen" class="fixed inset-0 bg-black/90 flex items-center justify-center z-50">
    <div class="bg-gray-800 p-8 rounded-xl shadow-2xl border border-gray-700 w-96 text-center">
      <h2 class="text-2xl font-bold mb-6 text-white tracking-wider">LOGIN KASIR</h2>
      
      <div class="mb-6">
        <input 
          type="password" 
          v-model="pin" 
          class="w-full bg-gray-900 border border-gray-600 rounded-lg py-4 text-center text-3xl font-mono text-white tracking-widest focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="PIN"
          autofocus
          @keyup.enter="handleLogin"
        />
      </div>

      <div class="grid grid-cols-3 gap-3 mb-6">
        <button v-for="n in 9" :key="n" @click="appendPin(n.toString())" class="h-14 bg-gray-700 hover:bg-gray-600 rounded text-xl font-bold text-white">{{ n }}</button>
        <button @click="clearPin" class="h-14 bg-red-900/50 hover:bg-red-900 rounded text-xl font-bold text-red-200">C</button>
        <button @click="appendPin('0')" class="h-14 bg-gray-700 hover:bg-gray-600 rounded text-xl font-bold text-white">0</button>
        <button @click="handleLogin" class="h-14 bg-blue-600 hover:bg-blue-500 rounded text-xl font-bold text-white">→</button>
      </div>

      <div v-if="error" class="text-red-500 font-bold animate-pulse">
        {{ error }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const isOpen = ref(true);
const pin = ref('');
const error = ref('');

const emit = defineEmits(['login-success']);

const appendPin = (char: string) => {
  if (pin.value.length < 6) pin.value += char;
  error.value = '';
};

const clearPin = () => {
  pin.value = '';
  error.value = '';
};

const handleLogin = async () => {
  if (!pin.value) return;
  
  try {
    const res: any = await invoke('login', { pin: pin.value });
    if (res.success) {
      isOpen.value = false;
      emit('login-success', res);
    } else {
      error.value = res.message || 'Login Failed';
      pin.value = '';
    }
  } catch (e) {
    error.value = 'System Error';
    console.error(e);
  }
};
</script>

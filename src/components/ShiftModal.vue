<template>
  <div v-if="isOpen" class="fixed inset-0 bg-black/90 flex items-center justify-center z-50">
    <div class="bg-gray-800 p-8 rounded-xl shadow-2xl border border-gray-700 w-96 text-center">
      <h2 class="text-xl font-bold mb-2 text-white">MULAI SHIFT</h2>
      <p class="text-gray-400 mb-6">Masukkan Saldo Awal Laci</p>
      
      <div class="mb-6 relative">
        <span class="absolute left-4 top-4 text-gray-500 font-mono text-xl">Rp</span>
        <input 
          type="number" 
          v-model="amount" 
          class="w-full bg-gray-900 border border-gray-600 rounded-lg py-4 pl-12 pr-4 text-right text-3xl font-mono text-white tracking-widest focus:outline-none focus:ring-2 focus:ring-green-500"
          placeholder="0"
          autofocus
          @keyup.enter="confirmShift"
        />
      </div>

      <button 
        @click="confirmShift"
        class="w-full py-4 bg-green-600 hover:bg-green-500 text-white font-bold rounded-lg text-lg uppercase tracking-wider"
      >
        Buka Kasir
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const isOpen = ref(true);
const amount = ref('');

const props = defineProps<{
  cashierName: string
}>();

const emit = defineEmits(['shift-started']);

const confirmShift = async () => {
  if (!amount.value) return;
  
  try {
    await invoke('start_shift', { 
      cashier: props.cashierName, 
      floatAmount: parseFloat(amount.value) 
    });
    isOpen.value = false;
    emit('shift-started', parseFloat(amount.value));
  } catch (e) {
    console.error(e);
  }
};
</script>

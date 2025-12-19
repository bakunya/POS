<template>
  <div class="h-full flex flex-col bg-gray-900 border-r border-gray-700">
    <!-- Header -->
    <div class="p-4 border-b border-gray-700 bg-gray-800">
      <div class="flex justify-between items-center text-gray-400 text-sm mb-1">
        <span>#TX-001</span>
        <span>Umum</span>
      </div>
      <h2 class="text-xl font-bold text-white">Keranjang Belanja</h2>
    </div>

    <!-- The List -->
    <div class="flex-1 overflow-y-auto p-2 space-y-1">
      <div 
        v-for="item in cart.items" 
        :key="item.id"
        :class="[
          'grid grid-cols-12 gap-2 p-3 rounded text-sm items-center font-mono',
          item.id === cart.lastScannedId ? 'bg-blue-900/30 text-blue-200 border border-blue-500/50' : 'bg-gray-800 text-gray-300'
        ]"
      >
        <div class="col-span-6 truncate font-medium" :class="{'font-bold': item.id === cart.lastScannedId}">
          {{ item.name }}
        </div>
        <div class="col-span-2 text-center text-gray-400">
          x{{ item.qty }}
        </div>
        <div class="col-span-4 text-right">
          {{ formatCurrency(item.subtotal) }}
        </div>
      </div>
    </div>

    <!-- Footer -->
    <div class="p-4 bg-gray-800 border-t border-gray-700">
      <div class="flex justify-between items-end">
        <div class="text-gray-400 text-sm">Total Belanja</div>
        <div class="text-4xl font-mono font-bold text-white tracking-tight">
          {{ formatCurrency(cart.total) }}
        </div>
      </div>
      <div class="mt-2 text-right text-yellow-500 font-mono text-sm animate-pulse">
        Waiting for input...
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useCartStore } from '../stores';

const cart = useCartStore();

const formatCurrency = (value: number) => {
  return new Intl.NumberFormat('id-ID', {
    style: 'currency',
    currency: 'IDR',
    minimumFractionDigits: 0
  }).format(value);
};
</script>

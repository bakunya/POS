<template>
  <div class="h-full flex flex-col bg-gray-900">
    <!-- Search Bar -->
    <div class="p-4 bg-gray-800 border-b border-gray-700">
      <div class="relative">
        <input 
          ref="searchInput"
          type="text" 
          v-model="searchQuery"
          @input="handleSearch"
          @keyup.enter="handleEnter"
          placeholder="Cari Barang (Ketik Nama/Barcode)..." 
          class="w-full bg-gray-900 border border-gray-600 rounded-lg py-3 px-4 text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 font-mono text-lg"
          autofocus
        />
        <div class="absolute right-3 top-3.5 text-gray-500 text-sm">
          [F1] Cari
        </div>
      </div>
    </div>

    <!-- Results or Quick Grid -->
    <div class="flex-1 p-4 overflow-y-auto">
      <div v-if="searchResults.length > 0">
        <h3 class="text-gray-400 text-sm font-bold uppercase tracking-wider mb-3">Search Results</h3>
        <div class="space-y-2">
            <button 
                v-for="(item, index) in searchResults" 
                :key="item.id"
                class="w-full flex justify-between items-center p-4 bg-gray-800 hover:bg-gray-700 border border-gray-700 rounded-lg text-left"
                :class="{'ring-2 ring-blue-500': index === 0}"
                @click="addToCart(item)"
            >
                <div>
                    <div class="font-bold text-white">{{ item.name }}</div>
                    <div class="text-xs text-gray-400 font-mono">{{ item.barcode }}</div>
                </div>
                <div class="text-green-400 font-mono font-bold">
                    Rp {{ item.price }}
                </div>
            </button>
        </div>
      </div>

      <div v-else>
        <h3 class="text-gray-400 text-sm font-bold uppercase tracking-wider mb-3">Quick Actions</h3>
        <div class="grid grid-cols-4 gap-3">
            <button 
            v-for="item in quickItems" 
            :key="item.name"
            class="aspect-square bg-gray-800 hover:bg-gray-700 active:bg-gray-600 border border-gray-700 rounded-xl flex flex-col items-center justify-center p-2 transition-colors group"
            @click="addToCart(item)"
            >
            <span class="text-2xl mb-2 group-hover:scale-110 transition-transform">{{ item.icon }}</span>
            <span class="text-sm text-center font-medium text-gray-300">{{ item.name }}</span>
            <span class="text-xs text-gray-500 mt-1 font-mono">Rp {{ item.price / 1000 }}k</span>
            </button>
        </div>
      </div>
    </div>

    <!-- Keypad Hint / Status -->
    <div class="p-2 bg-gray-800 border-t border-gray-700 flex justify-between text-xs text-gray-500 font-mono">
      <span>[F2] Edit Qty</span>
      <span>[Space] BAYAR</span>
      <span>[Esc] Menu</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useCartStore } from '../stores';
import { invoke } from '@tauri-apps/api/core';
import { useMagicKeys, whenever } from '@vueuse/core';

const cart = useCartStore();
const searchQuery = ref('');
const searchResults = ref<any[]>([]);
const searchInput = ref<HTMLInputElement | null>(null);

const { F1 } = useMagicKeys();

whenever(F1, () => {
    searchInput.value?.focus();
});

const quickItems = [
  { name: 'Galon Aqua', price: 20000, icon: '💧', id: 'q1' },
  { name: 'Gas 3kg', price: 22000, icon: '🔥', id: 'q2' },
  { name: 'Es Batu', price: 2000, icon: '🧊', id: 'q3' },
  { name: 'Kresek Besar', price: 500, icon: '🛍️', id: 'q4' },
  { name: 'Rokok Surya', price: 25000, icon: '🚬', id: 'q5' },
  { name: 'Indomie Grg', price: 3500, icon: '🍜', id: 'q6' },
  { name: 'Kopi Hitam', price: 4000, icon: '☕', id: 'q7' },
  { name: 'Telur 1kg', price: 28000, icon: '🥚', id: 'q8' },
];

const handleSearch = async () => {
    if (searchQuery.value.length < 2) {
        searchResults.value = [];
        return;
    }

    try {
        const res: any = await invoke('search_product', { query: searchQuery.value });
        searchResults.value = res;
    } catch (e) {
        console.error(e);
    }
};

const handleEnter = () => {
    if (searchResults.value.length > 0) {
        addToCart(searchResults.value[0]);
        searchQuery.value = '';
        searchResults.value = [];
    }
};

const addToCart = (item: any) => {
  cart.addItem(item);
  searchQuery.value = '';
  searchResults.value = [];
  searchInput.value?.focus();
};
</script>

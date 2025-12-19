import { createPinia } from 'pinia';
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export const setupStore = (app: any) => {
  const pinia = createPinia();
  app.use(pinia);
};

export interface CartItem {
  id: string;
  name: string;
  qty: number;
  price: number;
  subtotal: number;
}

export const useCartStore = defineStore('cart', () => {
  const items = ref<CartItem[]>([]);
  const lastScannedId = ref<string | null>(null);

  const total = computed(() => {
    return items.value.reduce((acc, item) => acc + item.subtotal, 0);
  });

  const addItem = (product: { id: string, name: string, price: number }) => {
    const existing = items.value.find(i => i.id === product.id);
    if (existing) {
      existing.qty++;
      existing.subtotal = existing.qty * existing.price;
    } else {
      items.value.unshift({
        id: product.id,
        name: product.name,
        price: product.price,
        qty: 1,
        subtotal: product.price
      });
    }
    lastScannedId.value = product.id;
  };

  const updateQty = (id: string, qty: number) => {
    const item = items.value.find(i => i.id === id);
    if (item) {
      item.qty = qty;
      item.subtotal = item.qty * item.price;
    }
  };

  const clearCart = () => {
    items.value = [];
    lastScannedId.value = null;
  };

  return { items, total, lastScannedId, addItem, updateQty, clearCart };
});

<script setup lang="ts">
import { ref } from "vue";
import { setupStore } from "./stores";
import LeftPanel from "./components/LeftPanel.vue";
import RightPanel from "./components/RightPanel.vue";
import LoginModal from "./components/LoginModal.vue";
import ShiftModal from "./components/ShiftModal.vue";

// Setup Pinia
import { getCurrentInstance } from 'vue';
const app = getCurrentInstance()?.appContext.app;
if (app) {
  setupStore(app);
}

const isLoggedIn = ref(false);
const isShiftStarted = ref(false);
const currentUser = ref<any>(null);

const onLoginSuccess = (user: any) => {
  console.log("Logged in as:", user);
  currentUser.value = user;
  isLoggedIn.value = true;
};

const onShiftStarted = (float: number) => {
  console.log("Shift started with float:", float);
  isShiftStarted.value = true;
};
</script>

<template>
  <div class="flex h-screen w-screen overflow-hidden bg-gray-900">
    
    <LoginModal v-if="!isLoggedIn" @login-success="onLoginSuccess" />
    <ShiftModal v-if="isLoggedIn && !isShiftStarted" :cashier-name="currentUser?.user || ''" @shift-started="onShiftStarted" />

    <!-- Left Panel: 40% -->
    <div class="w-[40%] h-full">
      <LeftPanel />
    </div>

    <!-- Right Panel: 60% -->
    <div class="w-[60%] h-full">
      <RightPanel />
    </div>
  </div>
</template>

<style>
/* Global resets if needed */
</style>

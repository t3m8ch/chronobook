<template>
  <div>
    <AuthDialog v-model:open="showAuthDialog" @complete="onAuthComplete" />
    <slot v-if="!showAuthDialog" />
  </div>
</template>

<script setup lang="ts">
import { useAuth } from '~/composables/useAuth';
import AuthDialog from '~/components/AuthDialog.vue';

const { isProfileChecked, isAuthOnLoad } = useAuth();

console.log('isAuthOnLoad:', isAuthOnLoad.value);
const showAuthDialog = ref(!(isProfileChecked.value || isAuthOnLoad.value));

watch(isProfileChecked, (newValue) => {
  showAuthDialog.value = !newValue;
});

watch(isAuthOnLoad, (newValue) => {
  showAuthDialog.value = !newValue;
});

const onAuthComplete = () => {
  showAuthDialog.value = false;
};
</script>

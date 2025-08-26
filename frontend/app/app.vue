<template>
  <NuxtLoadingIndicator />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script setup lang="ts">
import { client } from '@/api/client.gen';

const config = useRuntimeConfig();

client.setConfig({ baseURL: config.public.apiBaseUrl });
</script>

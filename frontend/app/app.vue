<template>
  <NuxtLoadingIndicator />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script setup lang="ts">
import { client } from '@/api/client.gen';
import dayjs from 'dayjs';
import 'dayjs/locale/ru';

dayjs.locale('ru');

const config = useRuntimeConfig();

client.setConfig({ baseURL: config.public.apiBaseUrl });
</script>

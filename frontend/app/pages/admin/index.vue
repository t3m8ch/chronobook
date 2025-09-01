<template>
  <NuxtLayout name="admin">
    <AdminAuthGuard>
      <!-- For now, redirect to create organization page -->
      <!-- In the future, this will show a list of user's organizations -->
      <div class="flex items-center justify-center min-h-[60vh]">
        <div class="text-center">
          <h2 class="text-2xl font-semibold mb-4">
            Добро пожаловать в админ-панель
          </h2>
          <p class="text-gray-600 mb-6">
            Создайте вашу первую организацию для начала работы
          </p>
          <Button @click="navigateToCreate" size="lg">
            Создать организацию
          </Button>
        </div>
      </div>
    </AdminAuthGuard>
  </NuxtLayout>
</template>

<script setup lang="ts">
import { Button } from '@/components/ui/button';
import AdminAuthGuard from '~/components/AdminAuthGuard.vue';

const router = useRouter();

const navigateToCreate = () => {
  router.push('/admin/create-organization');
};
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Header -->
    <header class="bg-white shadow-sm border-b">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between items-center h-16">
          <div class="flex items-center">
            <h1 class="text-xl font-semibold">ChronoBook Admin</h1>
          </div>
          <div class="flex items-center gap-4">
            <!-- <span v-if="profileData" class="text-sm text-gray-600">
              {{ profileData.firstName }} {{ profileData.lastName }}
            </span> -->
            <Button variant="outline" size="sm" @click="handleLogout">
              Выйти
            </Button>
          </div>
        </div>
      </div>
    </header>

    <!-- Main content -->
    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { useAuth } from '~/composables/useAuth';
import { logout } from '~/api';

const { createAuthClient, logout: clearToken } = useAuth();
const router = useRouter();

const handleLogout = async () => {
  const result = await logout({
    composable: 'useFetch',
    client: createAuthClient(),
  });

  if (result.error.value) {
    console.error('Logout error:', result.error.value);
  }

  clearToken();
  await router.push('/');
};
</script>

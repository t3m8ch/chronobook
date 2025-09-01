<template>
  <NuxtLayout name="admin">
    <AdminAuthGuard>
      <div class="space-y-6">
        <div class="flex justify-between items-center">
          <h1 class="text-2xl font-bold">Управление филиалами</h1>
          <Button>Добавить филиал</Button>
        </div>
        <Card>
          <CardContent class="py-8">
            <p class="text-center text-gray-500">
              Раздел управления филиалами в разработке
            </p>
          </CardContent>
        </Card>
      </div>
    </AdminAuthGuard>
  </NuxtLayout>
</template>

<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import AdminAuthGuard from '~/components/AdminAuthGuard.vue';
</script>

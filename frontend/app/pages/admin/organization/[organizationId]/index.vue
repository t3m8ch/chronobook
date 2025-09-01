<template>
  <NuxtLayout name="admin">
    <AdminAuthGuard>
      <!-- Loading state -->
      <div v-if="pending" class="space-y-4">
        <Skeleton class="h-32 w-full" />
        <Skeleton class="h-64 w-full" />
      </div>

      <!-- Error state -->
      <div v-else-if="error" class="rounded-md bg-red-50 p-6">
        <div class="flex">
          <div class="flex-shrink-0">
            <svg
              class="h-5 w-5 text-red-400"
              viewBox="0 0 20 20"
              fill="currentColor"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="ml-3">
            <h3 class="text-sm font-medium text-red-800">
              Ошибка загрузки данных организации
            </h3>
            <div class="mt-2 text-sm text-red-700">
              <p>
                {{
                  error.statusCode === 404
                    ? 'Организация не найдена'
                    : 'Произошла ошибка при загрузке данных'
                }}
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- Dashboard content -->
      <OrganizationDashboard
        v-else
        :dashboard="data"
        :organization-id="organizationId"
      />
    </AdminAuthGuard>
  </NuxtLayout>
</template>

<script setup lang="ts">
import { Skeleton } from '@/components/ui/skeleton';
import AdminAuthGuard from '~/components/AdminAuthGuard.vue';
import OrganizationDashboard from '~/components/OrganizationDashboard.vue';
import { getOrganizationDashboard } from '~/api';
import { useAuth } from '~/composables/useAuth';

const route = useRoute();
const { createAuthClient } = useAuth();

const organizationId = route.params.organizationId as string;

const { data, pending, error } = await getOrganizationDashboard({
  composable: 'useFetch',
  client: createAuthClient(),
  path: { organization_id: organizationId },
});

console.dir('load dashboard error', error.value);
</script>

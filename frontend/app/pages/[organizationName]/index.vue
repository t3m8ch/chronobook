<template>
  <div class="max-w-6xl mx-auto mt-10 p-4">
    <!-- Loading state -->
    <div v-if="orgPending || servicesPending" class="space-y-4">
      <!-- Organization skeleton -->
      <div class="space-y-4">
        <Skeleton class="h-12 w-3/4" />
        <Skeleton class="h-6 w-full" />
        <Skeleton class="h-6 w-2/3" />
      </div>

      <!-- Services skeleton -->
      <div class="mt-8 grid grid-cols-1 lg:grid-cols-2 gap-4">
        <div v-for="i in 4" :key="i" class="space-y-2">
          <Skeleton class="h-40 w-full" />
        </div>
      </div>
    </div>

    <!-- Error state -->
    <div v-else-if="orgError || servicesError" class="text-center py-10">
      <div class="text-red-500 space-y-4">
        <svg
          class="mx-auto h-12 w-12"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
          />
        </svg>
        <h2 class="text-2xl font-semibold">
          {{ isNotFound ? 'Организация не найдена' : 'Произошла ошибка' }}
        </h2>
      </div>
    </div>

    <!-- Content -->
    <div v-else-if="orgData">
      <h1 class="text-5xl font-bold">{{ orgData.displayName }}</h1>
      <p class="text-xl mt-4">{{ orgData.description }}</p>

      <div
        v-if="servicesData && servicesData.length > 0"
        class="mt-8 grid grid-cols-1 lg:grid-cols-2 gap-4"
      >
        <ServiceCard
          v-for="service in servicesData"
          :id="service.id"
          :key="service.id"
          :name="service.name"
          :description="service.description"
          :duration-minutes="service.durationMinutes"
          :price="service.price"
          :organization-name="orgData.name"
        />
      </div>
      <div
        v-else-if="servicesData && servicesData.length === 0"
        class="mt-8 text-center text-gray-500"
      >
        <p class="text-lg">Услуги пока не добавлены</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getOrganizationByName, getServices } from '~/api';
import { Skeleton } from '~/components/ui/skeleton';

const route = useRoute();

const organizationName = route.params.organizationName as string;

const {
  data: orgData,
  pending: orgPending,
  error: orgError,
} = await getOrganizationByName({
  composable: 'useFetch',
  path: { organization_name: organizationName },
});

const {
  data: servicesData,
  pending: servicesPending,
  error: servicesError,
} = await getServices({
  composable: 'useFetch',
  query: { organizationName },
});

const isNotFound =
  orgError.value?.statusCode === 404 || servicesError.value?.statusCode === 404;
</script>

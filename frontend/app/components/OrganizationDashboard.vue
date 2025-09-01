<template>
  <div class="space-y-6">
    <!-- Organization header -->
    <div class="bg-white rounded-lg shadow p-6">
      <h1 class="text-2xl font-bold">{{ dashboard?.displayName }}</h1>
      <p v-if="dashboard?.description" class="text-gray-600 mt-2">
        {{ dashboard.description }}
      </p>
      <div class="mt-4 flex items-center gap-2">
        <span
          class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium"
          :class="
            dashboard?.active
              ? 'bg-green-100 text-green-800'
              : 'bg-gray-100 text-gray-800'
          "
        >
          {{ dashboard?.active ? 'Активна' : 'Неактивна' }}
        </span>
      </div>
    </div>

    <!-- Setup checklist -->
    <Card>
      <CardHeader>
        <CardTitle>Настройка организации</CardTitle>
        <CardDescription>
          Выполните следующие шаги, чтобы ваша организация начала работать
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="space-y-4">
          <!-- Branches -->
          <div class="flex items-start space-x-3">
            <div class="flex-shrink-0 mt-1">
              <div
                class="h-5 w-5 rounded-full flex items-center justify-center"
                :class="
                  dashboard?.alLeastOneBranch
                    ? 'bg-green-100 text-green-600'
                    : 'bg-gray-100 text-gray-400'
                "
              >
                <svg
                  v-if="dashboard?.alLeastOneBranch"
                  class="h-3 w-3"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path
                    fill-rule="evenodd"
                    d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                    clip-rule="evenodd"
                  />
                </svg>
                <span v-else class="text-xs font-medium">1</span>
              </div>
            </div>
            <div class="flex-1">
              <h3
                class="font-medium"
                :class="
                  dashboard?.alLeastOneBranch
                    ? 'text-gray-900'
                    : 'text-gray-500'
                "
              >
                Добавить филиал
              </h3>
              <p class="text-sm text-gray-500 mt-1">
                Укажите адрес и часовой пояс вашего филиала
              </p>
              <NuxtLink
                :to="`/admin/organization/${organizationId}/branches`"
                class="inline-flex items-center text-sm font-medium text-blue-600 hover:text-blue-500 mt-2"
              >
                {{
                  dashboard?.alLeastOneBranch
                    ? 'Управление филиалами'
                    : 'Добавить филиал'
                }}
                <svg
                  class="ml-1 h-4 w-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9 5l7 7-7 7"
                  />
                </svg>
              </NuxtLink>
            </div>
          </div>

          <!-- Masters/Employees -->
          <div class="flex items-start space-x-3">
            <div class="flex-shrink-0 mt-1">
              <div
                class="h-5 w-5 rounded-full flex items-center justify-center"
                :class="
                  dashboard?.alLeastOneMaster
                    ? 'bg-green-100 text-green-600'
                    : 'bg-gray-100 text-gray-400'
                "
              >
                <svg
                  v-if="dashboard?.alLeastOneMaster"
                  class="h-3 w-3"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path
                    fill-rule="evenodd"
                    d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                    clip-rule="evenodd"
                  />
                </svg>
                <span v-else class="text-xs font-medium">2</span>
              </div>
            </div>
            <div class="flex-1">
              <h3
                class="font-medium"
                :class="
                  dashboard?.alLeastOneMaster
                    ? 'text-gray-900'
                    : 'text-gray-500'
                "
              >
                Добавить мастеров
              </h3>
              <p class="text-sm text-gray-500 mt-1">
                Добавьте сотрудников, которые будут оказывать услуги
              </p>
              <NuxtLink
                :to="`/admin/organization/${organizationId}/employees`"
                class="inline-flex items-center text-sm font-medium text-blue-600 hover:text-blue-500 mt-2"
              >
                {{
                  dashboard?.alLeastOneMaster
                    ? 'Управление сотрудниками'
                    : 'Добавить мастера'
                }}
                <svg
                  class="ml-1 h-4 w-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9 5l7 7-7 7"
                  />
                </svg>
              </NuxtLink>
            </div>
          </div>

          <!-- Services -->
          <div class="flex items-start space-x-3">
            <div class="flex-shrink-0 mt-1">
              <div
                class="h-5 w-5 rounded-full flex items-center justify-center"
                :class="
                  dashboard?.alLeastOneService
                    ? 'bg-green-100 text-green-600'
                    : 'bg-gray-100 text-gray-400'
                "
              >
                <svg
                  v-if="dashboard?.alLeastOneService"
                  class="h-3 w-3"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path
                    fill-rule="evenodd"
                    d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                    clip-rule="evenodd"
                  />
                </svg>
                <span v-else class="text-xs font-medium">3</span>
              </div>
            </div>
            <div class="flex-1">
              <h3
                class="font-medium"
                :class="
                  dashboard?.alLeastOneService
                    ? 'text-gray-900'
                    : 'text-gray-500'
                "
              >
                Добавить услуги
              </h3>
              <p class="text-sm text-gray-500 mt-1">
                Создайте список услуг с ценами и длительностью
              </p>
              <NuxtLink
                :to="`/admin/organization/${organizationId}/services`"
                class="inline-flex items-center text-sm font-medium text-blue-600 hover:text-blue-500 mt-2"
              >
                {{
                  dashboard?.alLeastOneService
                    ? 'Управление услугами'
                    : 'Добавить услугу'
                }}
                <svg
                  class="ml-1 h-4 w-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9 5l7 7-7 7"
                  />
                </svg>
              </NuxtLink>
            </div>
          </div>

          <!-- Timetables -->
          <div class="flex items-start space-x-3">
            <div class="flex-shrink-0 mt-1">
              <div
                class="h-5 w-5 rounded-full flex items-center justify-center"
                :class="
                  dashboard?.alLeastOneTimetable
                    ? 'bg-green-100 text-green-600'
                    : 'bg-gray-100 text-gray-400'
                "
              >
                <svg
                  v-if="dashboard?.alLeastOneTimetable"
                  class="h-3 w-3"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path
                    fill-rule="evenodd"
                    d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                    clip-rule="evenodd"
                  />
                </svg>
                <span v-else class="text-xs font-medium">4</span>
              </div>
            </div>
            <div class="flex-1">
              <h3
                class="font-medium"
                :class="
                  dashboard?.alLeastOneTimetable
                    ? 'text-gray-900'
                    : 'text-gray-500'
                "
              >
                Настроить расписание
              </h3>
              <p class="text-sm text-gray-500 mt-1">
                Укажите рабочие часы и перерывы для мастеров
              </p>
              <NuxtLink
                :to="`/admin/organization/${organizationId}/timetables`"
                class="inline-flex items-center text-sm font-medium text-blue-600 hover:text-blue-500 mt-2"
              >
                {{
                  dashboard?.alLeastOneTimetable
                    ? 'Управление расписанием'
                    : 'Настроить расписание'
                }}
                <svg
                  class="ml-1 h-4 w-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9 5l7 7-7 7"
                  />
                </svg>
              </NuxtLink>
            </div>
          </div>
        </div>

        <!-- Progress indicator -->
        <div v-if="setupProgress < 100" class="mt-6 pt-6 border-t">
          <div class="flex items-center justify-between mb-2">
            <span class="text-sm font-medium text-gray-700"
              >Прогресс настройки</span
            >
            <span class="text-sm font-medium text-gray-700"
              >{{ setupProgress }}%</span
            >
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2">
            <div
              class="bg-blue-600 h-2 rounded-full transition-all duration-300"
              :style="`width: ${setupProgress}%`"
            />
          </div>
        </div>

        <!-- Success message -->
        <div v-else class="mt-6 pt-6 border-t">
          <div class="rounded-md bg-green-50 p-4">
            <div class="flex">
              <div class="flex-shrink-0">
                <svg
                  class="h-5 w-5 text-green-400"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fill-rule="evenodd"
                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                    clip-rule="evenodd"
                  />
                </svg>
              </div>
              <div class="ml-3">
                <h3 class="text-sm font-medium text-green-800">
                  Организация готова к работе!
                </h3>
                <div class="mt-2 text-sm text-green-700">
                  <p>
                    Все необходимые настройки выполнены. Клиенты могут начать
                    бронировать услуги.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import type { OrganizationDashboardOut } from '~/api/types.gen';

const props = defineProps<{
  dashboard: OrganizationDashboardOut | null;
  organizationId: string;
}>();

const setupProgress = computed(() => {
  if (!props.dashboard) return 0;

  let completed = 0;
  const total = 4;

  if (props.dashboard.alLeastOneBranch) completed++;
  if (props.dashboard.alLeastOneMaster) completed++;
  if (props.dashboard.alLeastOneService) completed++;
  if (props.dashboard.alLeastOneTimetable) completed++;

  return Math.round((completed / total) * 100);
});
</script>

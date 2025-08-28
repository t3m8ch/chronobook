<template>
  <SidebarProvider>
    <div class="w-full flex">
      <Sidebar>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupLabel class="text-xl font-bold">
              Фильтры
            </SidebarGroupLabel>
            <SidebarGroupContent class="px-2 flex flex-col gap-2">
              <div>
                <Label for="select-master" class="text-lg">Мастер:</Label>
                <Select id="select-master" v-model="selectedMasterId">
                  <SelectTrigger class="w-full">
                    <SelectValue placeholder="Любой мастер" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectGroup>
                      <SelectItem
                        v-for="master in masters.data.value"
                        :key="master.id"
                        :value="master.id"
                      >
                        {{ master.firstName }} {{ master.lastName }}
                        {{ master.patronymic || '' }}
                      </SelectItem>
                    </SelectGroup>
                  </SelectContent>
                </Select>
              </div>
              <div>
                <Label for="select-branch" class="text-lg">Филиал:</Label>
                <Select id="select-branch" v-model="selectedBranchId">
                  <SelectTrigger class="w-full">
                    <SelectValue placeholder="Любой филиал" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectGroup>
                      <BranchSelectItem
                        v-for="branch in branches.data.value"
                        :key="branch.id"
                        :value="branch.id"
                        :branch-name="branch.name"
                        :branch-address="address(branch)"
                      />
                    </SelectGroup>
                  </SelectContent>
                </Select>
              </div>
              <a
                href="#"
                class="hover:text-blue-600 transition-colors duration-100"
                @click.prevent="clearSelection"
              >
                Очистить фильтры
              </a>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>
      <div class="w-full">
        <div class="flex lg:items-center lg:flex-row flex-col">
          <div class="lg:w-1/4">
            <SidebarTrigger />
          </div>
          <div class="lg:mx-auto lg:w-2/4 ml-4 lg:ml-0">
            <h1 class="text-2xl font-bold lg:text-center">
              Выберите мастера, место и время
            </h1>
          </div>
          <NuxtLink
            :to="`/${organizationName}`"
            class="hover:text-blue-600 transition-colors duration-100 lg:w-1/4 lg:text-end pr-2 ml-4 lg:ml-0"
          >
            Выбрать другую услугу
          </NuxtLink>
        </div>
        <div class="max-w-3xl mx-auto flex flex-col gap-2">
          <ClientOnly>
            <BookingWindow
              v-for="window in windows.data.value"
              :key="window.id"
              :id="window.id"
              :master="{ ...window.master }"
              :branch="{ address: address(window.branch), ...window.branch }"
              :slots="
                window.slots.map((slot) => ({
                  start: dayjs(slot.start),
                  end: dayjs(slot.end),
                }))
              "
              :service-id="serviceId"
              :organization-name="organizationName"
            />
            <template #fallback>
              <div class="flex justify-center items-center py-8">
                <div class="text-gray-500">Загрузка доступных окон...</div>
              </div>
            </template>
          </ClientOnly>
        </div>
      </div>
    </div>
  </SidebarProvider>
</template>

<script setup lang="ts">
import { getBranches, getMasters, getWindows } from '~/api';
import {
  Sidebar,
  SidebarProvider,
  SidebarContent,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarTrigger,
  SidebarGroupContent,
} from '@/components/ui/sidebar';
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectGroup,
  SelectContent,
  SelectItem,
  BranchSelectItem,
} from '@/components/ui/select';
import { Label } from '@/components/ui/label';
import dayjs from 'dayjs';

const route = useRoute();
const organizationName = route.params.organizationName as string;
const serviceId = route.params.serviceId as string;

const selectedMasterId = ref<string | null>(null);
const selectedBranchId = ref<string | null>(null);

const mastersQuery = computed(() => ({
  organizationName,
  'branches[]': selectedBranchId.value ? selectedBranchId.value : undefined,
}));

const branchesQuery = computed(() => ({
  organizationName,
  'masters[]': selectedMasterId.value ? selectedMasterId.value : undefined,
}));

const windowsQuery = computed(() => ({
  organizationName,
  serviceId,
  'masters[]': selectedMasterId.value ? selectedMasterId.value : undefined,
  'branches[]': selectedBranchId.value ? selectedBranchId.value : undefined,
}));

const masters = await getMasters({
  composable: 'useFetch',
  query: mastersQuery,
});

const branches = await getBranches({
  composable: 'useFetch',
  query: branchesQuery,
});

const windows = await getWindows({
  composable: 'useFetch',
  query: windowsQuery,
});

const clearSelection = () => {
  selectedMasterId.value = null;
  selectedBranchId.value = null;
};

const address = (branch: {
  country: string;
  region: string;
  city: string;
  street: string;
  houseNumber: string;
}) =>
  `${branch.country}, ${branch.region}, ${branch.city}, ${branch.street}, ${branch.houseNumber}`;
</script>

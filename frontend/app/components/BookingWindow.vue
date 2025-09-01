<template>
  <div>
    <Card>
      <CardHeader>
        <CardTitle>
          {{ props.master.firstName }} {{ props.master.lastName }}
          {{ props.master.patronymic || '' }}
        </CardTitle>
        <CardDescription>
          {{ props.branch.name }} | {{ props.branch.address }}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Select v-model="selectedSlot">
          <SelectTrigger>
            <SelectValue placeholder="Выберите время" />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectItem
                v-for="(slot, idx) in props.slots"
                :key="idx"
                :value="slot"
              >
                {{ slot.start.format('D MMMM HH:mm') }} -
                {{ slot.end.format('D MMMM HH:mm') }}
              </SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
      </CardContent>
      <CardFooter class="flex justify-between">
        <Button :disabled="!selectedSlot || loading" @click="handleBooking">
          {{ loading ? 'Бронирование...' : 'Записаться' }}
        </Button>
      </CardFooter>
    </Card>

    <!-- Auth dialog -->
    <AuthDialog
      v-model:open="showAuthDialog"
      :organization-name="organizationName"
      @complete="onAuthComplete"
    />

    <!-- Success dialog -->
    <Dialog v-model:open="showSuccessDialog">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Успешно!</DialogTitle>
          <DialogDescription>
            Вы записались на {{ bookedSlot?.start.format('D MMMM HH:mm') }}
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button @click="showSuccessDialog = false">Закрыть</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>

    <!-- Error dialog -->
    <Dialog v-model:open="showErrorDialog">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Ошибка</DialogTitle>
          <DialogDescription>{{ errorMessage }}</DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button @click="showErrorDialog = false">Закрыть</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import type { Dayjs } from 'dayjs';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
  CardFooter,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
  SelectValue,
  SelectGroup,
} from '@/components/ui/select';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import AuthDialog from '@/components/AuthDialog.vue';
import { createBooking } from '~/api';
import { useAuth } from '~/composables/useAuth';

const props = defineProps<{
  id: string;
  master: {
    id: string;
    firstName: string;
    lastName: string;
    patronymic?: string | null;
    contactPhone?: string | null;
    contactEmail?: string | null;
    contactTelegram?: string | null;
  };
  branch: {
    id: string;
    name: string;
    description: string;
    timezone: string;
    address: string;
  };
  slots: { start: Dayjs; end: Dayjs }[];
  serviceId?: string;
  organizationName?: string;
}>();

const route = useRoute();
const { isAuthenticated, createAuthClient } = useAuth();

const selectedSlot = ref<{ start: Dayjs; end: Dayjs } | null>(null);
const bookedSlot = ref<{ start: Dayjs; end: Dayjs } | null>(null);
const loading = ref(false);
const showAuthDialog = ref(false);
const showSuccessDialog = ref(false);
const showErrorDialog = ref(false);
const errorMessage = ref('');

// Get serviceId and organizationName from route if not provided
const serviceId = computed(
  () => props.serviceId || (route.params.serviceId as string),
);
const organizationName = computed(
  () => props.organizationName || (route.params.organizationName as string),
);

const handleBooking = async () => {
  if (!selectedSlot.value) return;

  loading.value = true;
  errorMessage.value = '';

  try {
    // Check if authenticated
    if (!isAuthenticated.value) {
      showAuthDialog.value = true;
      loading.value = false;
      return;
    }

    // Try to create booking
    const result = await createBooking({
      composable: 'useFetch',
      client: createAuthClient(),
      body: {
        serviceId: serviceId.value,
        masterId: props.master.id,
        branchId: props.branch.id,
        start: selectedSlot.value.start.toISOString(),
        end: selectedSlot.value.end.toISOString(),
        notifyMethods: ['sms'],
        organizationName: props.organizationName,
      },
    });

    if (!result.error.value) {
      showSuccessDialog.value = true;
      bookedSlot.value = selectedSlot.value;
      selectedSlot.value = null;
    } else {
      // Check if profile incomplete
      if (result.error.value?.data?.error === 'PROFILE_INCOMPLETE') {
        showAuthDialog.value = true;
      } else if (result.error.value?.data?.error === 'UNAUTHORIZED') {
        // Token expired, show auth dialog
        showAuthDialog.value = true;
      } else {
        errorMessage.value = 'Не удалось создать бронирование';
        console.dir('booking error', result.error.value);
        showErrorDialog.value = true;
      }
    }
  } catch (error) {
    console.dir('booking error', error);
    errorMessage.value = 'Произошла ошибка при бронировании';
    showErrorDialog.value = true;
  } finally {
    loading.value = false;
  }
};

const onAuthComplete = async () => {
  // After auth is complete, retry booking
  if (selectedSlot.value) {
    await handleBooking();
  }
};
</script>

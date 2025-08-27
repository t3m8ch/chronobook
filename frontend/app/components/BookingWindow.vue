<template>
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
      <Button :disabled="!selectedSlot">Записаться</Button>
    </CardFooter>
  </Card>
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
} from '@/components/ui/select';

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
}>();

const selectedSlot = ref<{ start: Dayjs; end: Dayjs } | null>(null);
</script>

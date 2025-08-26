<template>
  <Card>
    <CardHeader>
      <CardTitle class="text-2xl">
        {{ props.name }}
      </CardTitle>
      <CardDescription class="text-lg">
        {{ props.description }}
      </CardDescription>
    </CardHeader>
    <CardFooter class="flex justify-between">
      <span class="text-2xl text-bold">{{ bottomText }}</span>
      <NuxtLink :to="`${props.organizationName}/bookService/${props.id}`">
        <Button>Заказать</Button>
      </NuxtLink>
    </CardFooter>
  </Card>
</template>

<script setup lang="ts">
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardFooter,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';

const props = defineProps<{
  id: string;
  description: string;
  durationMinutes: number;
  name: string;
  price: string | null | undefined;
  organizationName: string;
}>();

let bottomText = `${props.durationMinutes} минут`;
if (props.price) {
  bottomText += ` | ${props.price}₽`;
}
</script>

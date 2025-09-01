<template>
  <Card class="max-w-2xl mx-auto">
    <CardHeader>
      <CardTitle>Создание организации</CardTitle>
      <CardDescription>
        Заполните информацию о вашей организации для начала работы
      </CardDescription>
    </CardHeader>
    <CardContent>
      <form @submit.prevent="handleSubmit" class="space-y-6">
        <div class="space-y-2">
          <Label for="name"> Уникальное имя организации * </Label>
          <Input
            id="name"
            v-model="form.name"
            placeholder="my-beauty-salon"
            :disabled="loading"
            @input="clearFieldError('name')"
          />
          <p class="text-sm text-gray-500">
            Только латинские буквы, цифры, дефис и подчеркивание (3-50 символов)
          </p>
          <p v-if="fieldErrors.name" class="text-sm text-red-500">
            {{ fieldErrors.name }}
          </p>
        </div>

        <div class="space-y-2">
          <Label for="displayName"> Название организации * </Label>
          <Input
            id="displayName"
            v-model="form.displayName"
            placeholder="Салон красоты Ромашка"
            :disabled="loading"
            @input="clearFieldError('displayName')"
          />
          <p class="text-sm text-gray-500">
            Это название будут видеть ваши клиенты (1-100 символов)
          </p>
          <p v-if="fieldErrors.displayName" class="text-sm text-red-500">
            {{ fieldErrors.displayName }}
          </p>
        </div>

        <div class="space-y-2">
          <Label for="description"> Описание </Label>
          <textarea
            id="description"
            v-model="form.description"
            class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            placeholder="Опишите вашу организацию..."
            :disabled="loading"
            @input="clearFieldError('description')"
          />
          <p class="text-sm text-gray-500">
            Краткое описание вашей организации (до 500 символов)
          </p>
          <p v-if="fieldErrors.description" class="text-sm text-red-500">
            {{ fieldErrors.description }}
          </p>
        </div>

        <div v-if="generalError" class="rounded-md bg-red-50 p-4">
          <p class="text-sm text-red-800">{{ generalError }}</p>
        </div>

        <Button
          type="submit"
          class="w-full"
          :disabled="loading || !isFormValid"
        >
          {{ loading ? 'Создание...' : 'Создать организацию' }}
        </Button>
      </form>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useAuth } from '~/composables/useAuth';
import { createOrganization } from '~/api';
import type { CreateOrganizationRequest } from '~/api/types.gen';

const emit = defineEmits<{
  success: [organizationId: string];
}>();

const { createAuthClient } = useAuth();

const form = ref<CreateOrganizationRequest>({
  name: '',
  displayName: '',
  description: undefined,
});

const fieldErrors = ref<
  Partial<Record<keyof CreateOrganizationRequest, string>>
>({});
const generalError = ref('');
const loading = ref(false);

const isFormValid = computed(() => {
  return form.value.name.length >= 3 && form.value.displayName.length >= 1;
});

const clearFieldError = (field: keyof CreateOrganizationRequest) => {
  delete fieldErrors.value[field];
  generalError.value = '';
};

const validateForm = (): boolean => {
  fieldErrors.value = {};

  // Validate name
  if (!form.value.name) {
    fieldErrors.value.name = 'Имя организации обязательно';
    return false;
  }
  if (form.value.name.length < 3 || form.value.name.length > 50) {
    fieldErrors.value.name = 'Имя должно быть от 3 до 50 символов';
    return false;
  }
  if (!/^[a-z0-9_-]+$/.test(form.value.name)) {
    fieldErrors.value.name =
      'Только латинские буквы в нижнем регистре, цифры, дефис и подчеркивание';
    return false;
  }

  // Validate displayName
  if (!form.value.displayName) {
    fieldErrors.value.displayName = 'Название организации обязательно';
    return false;
  }
  if (
    form.value.displayName.length < 1 ||
    form.value.displayName.length > 100
  ) {
    fieldErrors.value.displayName = 'Название должно быть от 1 до 100 символов';
    return false;
  }

  // Validate description
  if (form.value.description && form.value.description.length > 500) {
    fieldErrors.value.description = 'Описание не должно превышать 500 символов';
    return false;
  }

  return true;
};

const handleSubmit = async () => {
  if (!validateForm()) {
    return;
  }

  loading.value = true;
  generalError.value = '';

  const result = await createOrganization({
    composable: 'useFetch',
    client: createAuthClient(),
    body: {
      name: form.value.name,
      displayName: form.value.displayName,
      description: form.value.description || undefined,
    },
  });

  loading.value = false;

  if (result.data.value) {
    emit('success', result.data.value.id);
  } else if (result.error.value) {
    const error = result.error.value;

    if (error.statusCode === 409) {
      fieldErrors.value.name = 'Организация с таким именем уже существует';
    } else if (error.statusCode === 400) {
      // Parse validation errors if available
      if (error.data?.details) {
        generalError.value = 'Проверьте правильность заполнения формы';
      } else {
        generalError.value = error.data?.message || 'Неверные данные';
      }
    } else {
      generalError.value = 'Произошла ошибка при создании организации';
    }
  }
};
</script>

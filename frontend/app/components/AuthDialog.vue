<template>
  <Dialog v-model:open="isOpen">
    <DialogContent class="sm:max-w-md">
      <DialogHeader>
        <DialogTitle>
          {{
            step === 'phone'
              ? 'Введите номер телефона'
              : step === 'code'
                ? 'Введите код подтверждения'
                : 'Заполните профиль'
          }}
        </DialogTitle>
        <DialogDescription>
          {{
            step === 'phone'
              ? 'Мы отправим вам СМС с кодом подтверждения'
              : step === 'code'
                ? `Код отправлен на ${phoneNumber}`
                : 'Эти данные нужны для бронирования'
          }}
        </DialogDescription>
      </DialogHeader>

      <!-- Phone input step -->
      <div v-if="step === 'phone'" class="space-y-4">
        <div class="space-y-2">
          <Label for="phone">Номер телефона</Label>
          <Input
            id="phone"
            v-model="phoneNumber"
            type="tel"
            placeholder="+7 (999) 123-45-67"
            :disabled="loading"
            @keyup.enter="sendCode"
          />
          <p v-if="phoneError" class="text-sm text-red-500">{{ phoneError }}</p>
        </div>
        <Button
          class="w-full"
          :disabled="loading || !phoneNumber"
          @click="sendCode"
        >
          {{ loading ? 'Отправка...' : 'Получить код' }}
        </Button>
      </div>

      <!-- Code verification step -->
      <div v-else-if="step === 'code'" class="space-y-4">
        <div class="space-y-2">
          <Label>Код подтверждения</Label>
          <PinInput
            v-model="verificationCode"
            class="flex justify-center gap-2"
            :disabled="loading"
            @complete="verifyCode"
          >
            <PinInputGroup>
              <PinInputSlot
                v-for="i in 6"
                :key="i"
                :index="i - 1"
                class="w-12 h-12"
              />
            </PinInputGroup>
          </PinInput>
          <p v-if="codeError" class="text-sm text-red-500">{{ codeError }}</p>
        </div>
        <div class="flex flex-col gap-2">
          <Button
            class="w-full"
            :disabled="loading || verificationCode.length !== 6"
            @click="verifyCode"
          >
            {{ loading ? 'Проверка...' : 'Подтвердить' }}
          </Button>
          <Button
            variant="ghost"
            class="w-full"
            :disabled="loading"
            @click="step = 'phone'"
          >
            Изменить номер
          </Button>
        </div>
      </div>

      <!-- Profile form step -->
      <div v-else-if="step === 'profile'" class="space-y-4">
        <div class="space-y-2">
          <Label for="firstName">Имя *</Label>
          <Input
            id="firstName"
            v-model="profile.firstName"
            placeholder="Иван"
            :disabled="loading"
          />
        </div>
        <div class="space-y-2">
          <Label for="lastName">Фамилия *</Label>
          <Input
            id="lastName"
            v-model="profile.lastName"
            placeholder="Иванов"
            :disabled="loading"
          />
        </div>
        <div class="space-y-2">
          <Label for="patronymic">Отчество</Label>
          <Input
            id="patronymic"
            v-model="profile.patronymic"
            placeholder="Иванович"
            :disabled="loading"
          />
        </div>
        <p v-if="profileError" class="text-sm text-red-500">
          {{ profileError }}
        </p>
        <Button
          class="w-full"
          :disabled="loading || !profile.firstName || !profile.lastName"
          @click="saveProfile"
        >
          {{ loading ? 'Сохранение...' : 'Сохранить' }}
        </Button>
      </div>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  PinInput,
  PinInputGroup,
  PinInputSlot,
} from '@/components/ui/pin-input';
import { loginPhone, verifyPhone, createProfile } from '~/api';
import { useAuth } from '~/composables/useAuth';

type AuthStep = 'phone' | 'code' | 'profile';

const props = defineProps<{
  open: boolean;
  organizationName?: string;
}>();

const emit = defineEmits<{
  'update:open': [value: boolean];
  complete: [];
}>();

const { setAccessToken, createAuthClient, checkProfile } = useAuth();

const isOpen = computed({
  get: () => props.open,
  set: (value) => emit('update:open', value),
});

const step = ref<AuthStep>('phone');
const loading = ref(false);
const phoneNumber = ref('');
const verificationCode = ref<string[]>([]);
const profile = ref({
  firstName: '',
  lastName: '',
  patronymic: '',
});

const phoneError = ref('');
const codeError = ref('');
const profileError = ref('');

// Format phone number for display
const formatPhoneNumber = (phone: string): string => {
  // Remove all non-digits
  const digits = phone.replace(/\D/g, '');

  // Add +7 if not present
  if (!digits.startsWith('7')) {
    return `+7${digits}`;
  }
  return `+${digits}`;
};

// Send SMS code
const sendCode = async () => {
  phoneError.value = '';
  loading.value = true;

  try {
    const formattedPhone = formatPhoneNumber(phoneNumber.value);

    const result = await loginPhone({
      composable: 'useFetch',
      body: { phone: formattedPhone },
    });

    if (result.data) {
      step.value = 'code';
      phoneNumber.value = formattedPhone;
    } else if (result.error) {
      phoneError.value = result.error.message || 'Не удалось отправить код';
    }
  } catch (error) {
    console.error('Error sending SMS code:', error);
    phoneError.value = 'Произошла ошибка при отправке кода';
  } finally {
    loading.value = false;
  }
};

// Verify SMS code
const verifyCode = async () => {
  codeError.value = '';
  loading.value = true;

  try {
    const result = await verifyPhone({
      composable: 'useFetch',
      body: {
        phone: phoneNumber.value,
        code: parseInt(verificationCode.value.join('')),
      },
      credentials: 'include',
    });

    if (result.data.value) {
      setAccessToken(result.data.value.accessToken);

      const profile = await checkProfile();
      if (profile.exists) {
        emit('complete');
        isOpen.value = false;
      } else {
        if (profile.error?.data.error === 'NOT_FOUND') {
          step.value = 'profile';
        } else {
          codeError.value = 'Произошла ошибка при проверке профиля';
          console.error('Check profile error:', profile.error);
        }
      }
    } else {
      if (result.error.value?.data.error === 'INVALID_VERIFICATION_CODE') {
        codeError.value = 'Неверный код';
      } else {
        codeError.value = 'Произошла ошибка при проверке кода';
      }
    }
  } catch (error) {
    console.dir('Error verifying code:', error);
    codeError.value = 'Произошла ошибка при проверке кода';
  } finally {
    loading.value = false;
  }
};

// Save user profile
const saveProfile = async () => {
  profileError.value = '';
  loading.value = true;

  try {
    const result = await createProfile({
      composable: 'useFetch',
      client: createAuthClient(),
      body: {
        firstName: profile.value.firstName,
        lastName: profile.value.lastName,
        patronymic: profile.value.patronymic || undefined,
      },
    });

    if (result.data) {
      emit('complete');
      isOpen.value = false;
    } else if (result.error) {
      profileError.value =
        result.error.message || 'Не удалось сохранить профиль';
    }
  } catch (error) {
    profileError.value = 'Произошла ошибка при сохранении профиля';
  } finally {
    loading.value = false;
  }
};

// Reset state when dialog closes
watch(isOpen, (value) => {
  if (!value) {
    step.value = 'phone';
    phoneNumber.value = '';
    verificationCode.value = '';
    profile.value = {
      firstName: '',
      lastName: '',
      patronymic: '',
    };
    phoneError.value = '';
    codeError.value = '';
    profileError.value = '';
  }
});
</script>

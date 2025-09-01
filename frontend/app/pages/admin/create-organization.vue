<template>
  <NuxtLayout name="admin">
    <AdminAuthGuard>
      <div class="py-6">
        <CreateOrganizationForm @success="onOrganizationCreated" />
      </div>
    </AdminAuthGuard>
  </NuxtLayout>
</template>

<script setup lang="ts">
import AdminAuthGuard from '~/components/AdminAuthGuard.vue';
import CreateOrganizationForm from '~/components/CreateOrganizationForm.vue';

const router = useRouter();

const onOrganizationCreated = (organizationId: string) => {
  // Redirect to the organization dashboard
  router.push(`/admin/organization/${organizationId}`);
};
</script>

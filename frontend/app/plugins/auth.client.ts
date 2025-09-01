export default defineNuxtPlugin(async () => {
  const { initAuth } = useAuth();

  // Initialize auth on app mount
  await initAuth();
});

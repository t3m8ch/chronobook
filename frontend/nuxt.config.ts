import tailwindcss from '@tailwindcss/vite';

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  modules: ['@nuxt/eslint', 'shadcn-nuxt'],
  css: ['~/assets/css/main.css'],
  shadcn: {
    prefix: '',
    componentDir: './components/ui',
  },
  vite: {
    plugins: [tailwindcss()],
  },
});

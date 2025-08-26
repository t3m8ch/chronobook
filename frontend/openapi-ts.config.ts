import { defineConfig } from '@hey-api/openapi-ts';

export default defineConfig({
  input: 'http://localhost:3222/api/v1/openapi.json',
  output: 'app/api',
  plugins: [
    '@hey-api/client-nuxt',
    'valibot',
    { name: '@hey-api/sdk', validator: true },
  ],
});

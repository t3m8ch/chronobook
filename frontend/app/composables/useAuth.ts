import { createClient } from '~/api/client';
import type { Config } from '~/api/client/types.gen';
import { refresh, getProfile } from '~/api';

export const useAuth = () => {
  const accessToken = useState<string | null>('accessToken', () => null);
  const isAuthenticated = computed(() => !!accessToken.value);
  const config = useRuntimeConfig();
  const baseURL = config.public.apiBaseUrl;

  console.log('baseUrl', baseURL);

  // Create API client with auth interceptor
  const createAuthClient = (config?: Partial<Config>) => {
    return createClient({
      baseURL,
      credentials: 'include', // Include cookies for refresh tokens
      ...config,
      headers: {
        ...config?.headers,
        ...(accessToken.value && {
          Authorization: `Bearer ${accessToken.value}`,
        }),
      },
      onRequest: async ({ request }) => {
        // Add auth header if token exists
        if (accessToken.value && request.headers) {
          request.headers.set('Authorization', `Bearer ${accessToken.value}`);
        }
        return request;
      },
      onResponse: async ({ response, request }) => {
        // If we get 401, try to refresh token
        if (response.status === 401 && !request.url.includes('/auth/refresh')) {
          try {
            // Try to refresh token
            const refreshResult = await refresh({
              client: createClient({
                baseURL,
                credentials: 'include',
              }),
            });

            if (refreshResult.data) {
              // Update access token
              accessToken.value = refreshResult.data.accessToken;

              // Retry original request with new token
              const newRequest = new Request(request);
              newRequest.headers.set(
                'Authorization',
                `Bearer ${refreshResult.data.accessToken}`,
              );

              return await fetch(newRequest);
            }
          } catch (error) {
            // Refresh failed, clear token
            accessToken.value = null;
          }
        }

        return response;
      },
    });
  };

  const setAccessToken = (token: string | null) => {
    accessToken.value = token;
  };

  const logout = () => {
    accessToken.value = null;
  };

  // Check if user has profile
  const checkProfile = async (): Promise<boolean> => {
    try {
      const result = await getProfile({
        composable: 'useFetch',
        client: createAuthClient(),
      });
      return !!result.data;
    } catch {
      return false;
    }
  };

  return {
    accessToken: readonly(accessToken),
    isAuthenticated,
    createAuthClient,
    setAccessToken,
    logout,
    checkProfile,
  };
};

import { createClient } from '~/api/client';
import type { Config } from '~/api/client/types.gen';
import { refresh, getProfile, type ApiError } from '~/api';

export const useAuth = () => {
  const accessToken = useState<string | null>('accessToken', () => null);
  const isAuthenticated = computed(() => !!accessToken.value);
  const isProfileChecked = useState<boolean>('isProfileChecked', () => false);
  const isInitialized = useState<boolean>('authInitialized', () => false);
  const isAuthOnLoad = useState<boolean>('authOnLoad', () => false);
  const config = useRuntimeConfig();
  const baseURL = config.public.apiBaseUrl;

  // Initialize auth on first call
  const initAuth = async () => {
    if (!isInitialized.value && !accessToken.value) {
      isInitialized.value = true;
      try {
        // Try to refresh token using cookies
        const refreshResult = await refresh({
          composable: 'useFetch',
          client: createClient({
            baseURL,
            credentials: 'include',
          }),
        });

        if (refreshResult.data.value?.accessToken) {
          accessToken.value = refreshResult.data.value.accessToken;
          isAuthOnLoad.value = true;
        }
      } catch (error) {
        // No valid refresh token, user needs to login
        console.log('No valid refresh token available', error);
      }
    }
  };

  // Call initialization immediately
  if (import.meta.client) {
    initAuth();
  }

  // Create API client with auth interceptor
  const createAuthClient = (config?: Partial<Config>) => {
    console.log('access token in auth client', accessToken.value);

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
      onResponse: async ({ response, request }) => {
        console.log('Response received:', response);

        // If we get 401, try to refresh token
        if (
          response.status === 401 &&
          !request.toString().includes('/auth/refresh')
        ) {
          try {
            // Try to refresh token
            const refreshResult = await refresh({
              composable: 'useFetch',
              client: createClient({
                baseURL,
                credentials: 'include',
              }),
            });

            if (refreshResult.data.value) {
              // Update access token
              accessToken.value = refreshResult.data.value.accessToken;

              // Retry original request with new token
              const newRequest = new Request(request);
              newRequest.headers.set(
                'Authorization',
                `Bearer ${refreshResult.data.value.accessToken}`,
              );

              return await fetch(newRequest);
            }
          } catch (error) {
            // Refresh failed, clear token
            console.error('Failed to refresh token', error);
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
  const checkProfile = async (): Promise<{
    exists: boolean;
    error: ApiError | null;
  }> => {
    const client = createAuthClient();

    console.dir('Checking profile...', client);

    const result = await getProfile({
      composable: 'useFetch',
      client,
    });
    isProfileChecked.value = true;

    return { exists: !!result.data, error: result.error.value ?? null };
  };

  return {
    accessToken: readonly(accessToken),
    isAuthenticated,
    isInitialized: readonly(isInitialized),
    isProfileChecked,
    isAuthOnLoad,
    createAuthClient,
    setAccessToken,
    logout,
    checkProfile,
    initAuth,
  };
};

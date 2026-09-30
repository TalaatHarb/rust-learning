interface PublicEnvironment {
  VITE_API_BASE_URL?: string
  VITE_OIDC_AUTHORITY?: string
  VITE_OIDC_CLIENT_ID?: string
  VITE_OIDC_REDIRECT_URI?: string
}

declare global {
  interface Window {
    _env_?: PublicEnvironment
  }
}

const runtimeEnvironment = typeof window === 'undefined' ? undefined : window._env_

export const environment = {
  apiBaseUrl:
    runtimeEnvironment?.VITE_API_BASE_URL ||
    import.meta.env.VITE_API_BASE_URL ||
    'http://localhost:8080',
  oidcAuthority:
    runtimeEnvironment?.VITE_OIDC_AUTHORITY ||
    import.meta.env.VITE_OIDC_AUTHORITY ||
    'http://localhost:8081/realms/nextechincubator',
  oidcClientId:
    runtimeEnvironment?.VITE_OIDC_CLIENT_ID ||
    import.meta.env.VITE_OIDC_CLIENT_ID ||
    'rust-learning-web',
  oidcRedirectUri:
    runtimeEnvironment?.VITE_OIDC_REDIRECT_URI ||
    import.meta.env.VITE_OIDC_REDIRECT_URI ||
    (typeof window === 'undefined' ? 'http://localhost:5173' : window.location.origin),
}

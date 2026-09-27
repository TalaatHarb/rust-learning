import type { AuthProviderProps } from 'react-oidc-context'

const authority = import.meta.env.VITE_OIDC_AUTHORITY ?? 'http://localhost:8081/realms/rust-learning'
const clientId = import.meta.env.VITE_OIDC_CLIENT_ID ?? 'rust-learning-web'
const redirectUri = import.meta.env.VITE_OIDC_REDIRECT_URI ?? 'http://localhost:5173'

export const oidcConfig: AuthProviderProps = {
  authority,
  client_id: clientId,
  redirect_uri: redirectUri,
  post_logout_redirect_uri: redirectUri,
  response_type: 'code',
  scope: 'openid profile email',
  automaticSilentRenew: true,
}

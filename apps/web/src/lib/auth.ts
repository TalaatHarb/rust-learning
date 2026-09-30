import type { AuthProviderProps } from 'react-oidc-context'
import { environment } from './environment'

export const oidcConfig: AuthProviderProps = {
  authority: environment.oidcAuthority,
  client_id: environment.oidcClientId,
  redirect_uri: environment.oidcRedirectUri,
  post_logout_redirect_uri: environment.oidcRedirectUri,
  response_type: 'code',
  scope: 'openid profile email',
  automaticSilentRenew: true,
  onSigninCallback: () => {
    window.history.replaceState({}, document.title, window.location.pathname)
  },
}

import { useAuth } from 'react-oidc-context'

export function LoginPage() {
  const auth = useAuth()

  if (auth.isLoading) {
    return (
      <section className="panel">
        <h2>Login</h2>
        <p>Loading authentication session...</p>
      </section>
    )
  }

  if (auth.isAuthenticated) {
    return (
      <section className="panel">
        <h2>Login</h2>
        <p>You are signed in.</p>
        <button type="button" onClick={() => void auth.signoutRedirect()}>
          Logout
        </button>
      </section>
    )
  }

  return (
    <section className="panel">
      <h2>Login</h2>
      <p>Sign in with Keycloak to submit exercises and track progress.</p>
      <button type="button" onClick={() => void auth.signinRedirect()}>
        Login with Keycloak
      </button>
    </section>
  )
}

#!/bin/sh
set -eu

config_file=/usr/share/nginx/html/env-config.js
temporary_file="${config_file}.tmp"
env_file=/etc/web-config.env

if [ -f "$env_file" ]; then
  while IFS= read -r line || [ -n "$line" ]; do
    case "$line" in
      ''|\#*) continue ;;
      *=*) name=${line%%=*}; value=${line#*=} ;;
      *) continue ;;
    esac

    case "$name" in
      VITE_API_BASE_URL)
        if [ "${VITE_API_BASE_URL+x}" != x ]; then VITE_API_BASE_URL=$value; fi
        ;;
      VITE_OIDC_AUTHORITY)
        if [ "${VITE_OIDC_AUTHORITY+x}" != x ]; then VITE_OIDC_AUTHORITY=$value; fi
        ;;
      VITE_OIDC_CLIENT_ID)
        if [ "${VITE_OIDC_CLIENT_ID+x}" != x ]; then VITE_OIDC_CLIENT_ID=$value; fi
        ;;
      VITE_OIDC_REDIRECT_URI)
        if [ "${VITE_OIDC_REDIRECT_URI+x}" != x ]; then VITE_OIDC_REDIRECT_URI=$value; fi
        ;;
    esac
  done < "$env_file"
fi

{
  printf 'window._env_ = '
  jq -cn \
    --arg api_base_url "${VITE_API_BASE_URL:-}" \
    --arg oidc_authority "${VITE_OIDC_AUTHORITY:-}" \
    --arg oidc_client_id "${VITE_OIDC_CLIENT_ID:-}" \
    --arg oidc_redirect_uri "${VITE_OIDC_REDIRECT_URI:-}" \
    '{
      VITE_API_BASE_URL: $api_base_url,
      VITE_OIDC_AUTHORITY: $oidc_authority,
      VITE_OIDC_CLIENT_ID: $oidc_client_id,
      VITE_OIDC_REDIRECT_URI: $oidc_redirect_uri
    }'
  printf ';\n'
} > "$temporary_file"

mv "$temporary_file" "$config_file"
exec "$@"

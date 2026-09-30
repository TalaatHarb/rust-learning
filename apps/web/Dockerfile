FROM node:22-alpine AS builder

WORKDIR /app
COPY package.json package-lock.json ./
COPY apps/web/package.json ./apps/web/package.json
COPY apps/web ./apps/web
RUN npm ci --workspace apps/web
RUN npm run build --workspace apps/web

FROM nginx:alpine

RUN apk add --no-cache jq
COPY apps/web/nginx.conf /etc/nginx/conf.d/default.conf
COPY apps/web/docker-entrypoint.sh /usr/local/bin/web-entrypoint
COPY apps/web/.env.example /etc/web-config.env
COPY --from=builder /app/apps/web/dist /usr/share/nginx/html
RUN chmod +x /usr/local/bin/web-entrypoint

EXPOSE 80
ENTRYPOINT ["/usr/local/bin/web-entrypoint"]
CMD ["nginx", "-g", "daemon off;"]

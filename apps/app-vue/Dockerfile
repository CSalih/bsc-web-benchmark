FROM node:20 AS build

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

WORKDIR /app

# Install latest corepack to avoid "Cannot find matching keyid" error (https://vercel.com/guides/corepack-errors-github-actions#quick-debug-steps)
RUN npm install -g corepack@latest
RUN corepack enable

COPY package.json .
COPY pnpm-lock.yaml .
COPY pnpm-workspace.yaml .
COPY apps/app-vue/package.json apps/app-vue/package.json
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

COPY apps/app-vue apps/app-vue
RUN pnpm run build

FROM nginx:1.27.3-alpine
COPY --from=build /app/apps/app-vue/dist /usr/share/nginx/html

EXPOSE 80

STOPSIGNAL SIGTERM

ENTRYPOINT ["nginx", "-g", "daemon off;"]

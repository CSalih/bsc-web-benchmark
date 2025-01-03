FROM node:18 AS build

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

WORKDIR /app

RUN corepack enable

COPY package.json .
COPY pnpm-lock.yaml .
COPY pnpm-workspace.yaml .
COPY apps/app-vue/package.json apps/app-vue/package.json
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

COPY apps/app-vue apps/app-vue
RUN pnpm run build

FROM nginx:1.26.0
COPY --from=build /app/apps/app-vue/dist /usr/share/nginx/html

EXPOSE 80

STOPSIGNAL SIGTERM

ENTRYPOINT ["nginx", "-g", "daemon off;"]

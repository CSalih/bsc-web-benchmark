FROM node:20 AS build

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

WORKDIR /app

RUN corepack enable

COPY package.json .
COPY pnpm-lock.yaml .
COPY pnpm-workspace.yaml .
COPY apps/react-realworld/package.json apps/react-realworld/package.json
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

COPY apps/react-realworld apps/react-realworld
RUN pnpm run build

FROM nginx:1.27.3-alpine
COPY --from=build /app/apps/react-realworld/dist /usr/share/nginx/html

EXPOSE 80

STOPSIGNAL SIGTERM

ENTRYPOINT ["nginx", "-g", "daemon off;"]

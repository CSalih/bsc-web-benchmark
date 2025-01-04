FROM node:18 AS build

WORKDIR /app

RUN corepack enable

COPY package.json .
COPY package-lock.json .
RUN --mount=type=cache,target=/root/.npm npm ci

COPY . .
RUN npm run nx build

FROM nginx:1.26.0
COPY --from=build /app/dist/apps/conduit /usr/share/nginx/html

EXPOSE 80

STOPSIGNAL SIGTERM

ENTRYPOINT ["nginx", "-g", "daemon off;"]

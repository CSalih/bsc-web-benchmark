FROM node:18 as build

WORKDIR /app

COPY package.json .
RUN yarn install

COPY . .
ENV VITE_ANALYTICS_BASE_URL=http://localhost:8000
RUN yarn build

FROM nginx:1.26.0
COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 80

STOPSIGNAL SIGTERM

CMD ["nginx", "-g", "daemon off;"]

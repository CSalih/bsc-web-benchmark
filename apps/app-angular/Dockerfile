FROM node:18 as build

WORKDIR /app

COPY package.json .
RUN yarn install

COPY . .
RUN yarn build

FROM nginx:1.26.0
COPY --from=build /app/dist/browser /usr/share/nginx/html

EXPOSE 80

STOPSIGNAL SIGTERM

CMD ["nginx", "-g", "daemon off;"]

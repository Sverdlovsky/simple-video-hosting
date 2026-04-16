FROM docker.io/library/node:25.8.1-alpine3.23 AS build

WORKDIR /srv/web

COPY root/package-lock.json .
COPY root/package.json .

RUN npm ci

COPY root/svelte.config.js .
COPY root/tsconfig.json .
COPY root/vite.config.ts .
COPY root/static static
COPY root/src src

RUN npm run build


FROM docker.io/library/nginx:1.29.6-alpine3.23

COPY --from=build /srv/web /var/www/html/default

COPY nginx.conf /etc/nginx/templates/default.conf.template


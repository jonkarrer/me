# Dockerize A Vite App

Vite and docker can be used together to build a web app. This is a simple example of how to do it.

## Setup

```bash
yarn create vite my-app
cd my-app
yarn
yarn dev
```

## Dockerfile

```docker
FROM node:lts-alpine as build
WORKDIR /app
COPY . .
RUN yarn install && yarn build

# Production stage
FROM nginx:stable-alpine
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

## Deploy

```bash
docker build -t portfolio .
docker run -d -p 82100:80 portfolio
```

Go to: **<http://localhost:82100>**

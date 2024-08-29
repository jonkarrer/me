# Dockerize A Vite App

## Prerequisites

- [Vite](https://vitejs.dev/)
- [Docker](https://www.docker.com/)

## Setup

```sh
yarn create vite my-app
cd my-app
yarn
yarn dev
```

## Dockerfile

```dockerfile
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

```sh
docker build -t portfolio .
docker run -d -p 82100:80 portfolio
```

Go to: **<http://localhost:82100>**

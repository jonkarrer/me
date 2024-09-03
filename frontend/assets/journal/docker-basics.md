# Docker Basics

## Dockerfile

This is the instructions for docker. A dockerfile is a text file that specifies how to build an image. The instructions in the dockerfile are executed in the container image.

### Example Dockerfile

This will launch an nginx server on port 80 that serves the content in the directory /app/dist, a js application.

```dockerfile
# Build stage
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

## Building an Image

Now we use the dockerfile to build an image. The command for this is:

```shell
docker build -t <image_name> .
```

This will build an image named <image_name>. This is a non running container, it is only awaiting to be started. Inside the image is all the code needed to run the container.

## Running an Image

Now we can run the image. The command for this is:

```shell
docker run -d -p 80:80 <image_name>
```

This will run the image and expose port 80. The -d flag tells docker to run the image in the background. The -p flag tells docker to expose port 80. The <image_name> is the name of the image we just built.

## Stopping a Container

Now we can stop the container. The command for this is:

```shell
docker stop <container_id>
```

## Removing a Container

Now we can remove the container. The command for this is:

```shell
docker rm <container_id>
```

## Removing an Image

Now we can remove the image. The command for this is:

```shell
docker rmi <image_name>
```

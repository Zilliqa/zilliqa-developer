FROM node:20 AS build-stage

ARG VUE_APP_IPFS_NODE=https://gateway.pinata.cloud
ENV VUE_APP_IPFS_NODE=$VUE_APP_IPFS_NODE

# Set to "development" to produce unminified code with source maps for debugging.
ARG BUILD_MODE=production

WORKDIR /app

COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

COPY . .
RUN yarn build --mode $BUILD_MODE

FROM nginx:stable-alpine AS production-stage
COPY --from=build-stage /app/dist /usr/share/nginx/html
EXPOSE 80
ENTRYPOINT ["nginx", "-g", "daemon off;"]

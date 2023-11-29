FROM node:14.17.3 as build-stage

ENV VUE_APP_IPFS_NODE="gateway.pinata.cloud"

WORKDIR app/
COPY . ./
RUN yarn install --frozen-lockfile
RUN yarn build

FROM nginx:stable-alpine as production-stage
COPY --from=build-stage /app/dist /usr/share/nginx/html
EXPOSE 80
ENTRYPOINT ["nginx", "-g", "daemon off;"]

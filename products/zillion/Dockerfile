FROM node:14.20.1 as build-stage


ENV NODE_OPTIONS=--max-old-space-size=4096
WORKDIR /app
COPY ./package.json ./
COPY ./yarn.lock ./
RUN yarn install -E
COPY . ./
RUN yarn build

FROM nginx:stable-alpine as production-stage
COPY --from=build-stage /app/build /usr/share/nginx/html
EXPOSE 80
ENTRYPOINT ["nginx", "-g", "daemon off;"]

FROM node:16 as build-stage

WORKDIR /app
COPY . ./
RUN yarn install
RUN yarn ci
RUN yarn build

FROM nginx:stable-alpine as production-stage
COPY --from=build-stage /app/build /usr/share/nginx/html
COPY nginx/default.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
ENTRYPOINT ["nginx", "-g", "daemon off;"]

FROM node:14.20.0 as build-stage

WORKDIR app/
COPY . ./
RUN yarn install --frozen-lockfile
RUN yarn build

FROM nginx:stable-alpine as production-stage
COPY --from=build-stage /app/dist /usr/share/nginx/html
EXPOSE 80
ENTRYPOINT ["nginx", "-g", "daemon off;"]

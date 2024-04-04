# build stage
FROM node:18.20 as build-stage

ARG DEPLOY_ENV="dev"

WORKDIR /savant-ide
COPY ./package.json ./yarn.lock ./
RUN yarn install -E
COPY . /savant-ide
RUN mv .env_$DEPLOY_ENV .env
RUN yarn build

# production stage
FROM nginx:1.23.1@sha256:186c79dc14ab93e43d315143ee4b0774506dc4fd952388c20e35d3d37058ab8d as production-stage
COPY --from=build-stage /savant-ide/dist /usr/share/nginx/html
EXPOSE 80

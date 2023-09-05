# build stage
FROM node:14.17 as build-stage

ARG DEPLOY_ENV="dev"

COPY . /savant-ide
WORKDIR /savant-ide
RUN mv .env_$DEPLOY_ENV .env && \
    npm install && \
    npm rebuild node-sass && \
    npm run build

# production stage
FROM nginx:1.23.1@sha256:186c79dc14ab93e43d315143ee4b0774506dc4fd952388c20e35d3d37058ab8d as production-stage
COPY --from=build-stage /savant-ide/dist /usr/share/nginx/html
EXPOSE 80

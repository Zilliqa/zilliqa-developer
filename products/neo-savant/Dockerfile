# build stage
FROM node:14.17 as build-stage

ARG DEPLOY_ENV="stg"

COPY . /savant-ide
WORKDIR /savant-ide
RUN mv .env_$DEPLOY_ENV .env && \
    npm install && \
    npm rebuild node-sass && \
    npm run build

# production stage
FROM nginx:stable-alpine as production-stage
COPY --from=build-stage /savant-ide/dist /usr/share/nginx/html
EXPOSE 80
ENTRYPOINT ["nginx", "-g", "daemon off;"]

#ENTRYPOINT NODE_ENV=production SCILLA_VERSION=0 npm run start

#c699770f095f3c38bf1f7e1bba853dfcadcce98d

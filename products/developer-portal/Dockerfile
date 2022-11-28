FROM node:lts-buster as builder
WORKDIR /src/
COPY products/developer-portal/ /src/

# Removing symlinks used for development
RUN rm /src/docs /src/assets

COPY docs /src/docs
COPY assets /src/assets

ENV NPM_CONFIG_LOGLEVEL=warn
ENV NPM_CONFIG_COLOR=false
RUN yarn && \
	yarn build

FROM nginx:latest as documentation
ADD products/developer-portal/nginx/default.conf /etc/nginx/conf.d/default.conf

COPY --from=builder /src/build /usr/share/nginx/html
COPY --from=builder /src/assets /usr/share/nginx/html/assets

EXPOSE 80
ENTRYPOINT ["nginx", "-g", "daemon off;"]
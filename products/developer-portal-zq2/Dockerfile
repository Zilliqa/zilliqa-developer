FROM ubuntu:22.04 AS builder

ENV DEBIAN_FRONTEND=noninteractive
ENV NEEDRESTART_MODE=a

RUN apt-get update && apt-get dist-upgrade -y
RUN apt-get install -y python3 python3-pip --no-install-recommends

COPY .  /build
RUN pip3 install --no-cache-dir -r /build/requirements.txt

ENV DOC_SOURCE=docs
WORKDIR /build
RUN  mkdocs build

FROM nginx:alpine-slim

COPY --from=builder --chown=nginx:nginx /build/site/. /usr/share/nginx/html/.
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

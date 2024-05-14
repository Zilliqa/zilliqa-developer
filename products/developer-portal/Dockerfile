FROM rust:buster AS builder

ENV DEBIAN_FRONTEND=noninteractive
ENV NEEDRESTART_MODE=a

RUN apt-get update && apt-get dist-upgrade -y
RUN apt-get install -y python3 python3-pip python3-setuptools --no-install-recommends

COPY .  /build
RUN pip3 install --no-cache-dir -r /build/requirements.txt


ENV DOC_SOURCE=docs
WORKDIR /build/zq1
RUN  mkdocs build
WORKDIR /build/docgen
RUN cargo run /build
WORKDIR /build/zq2
ARG VERSION
ENV VERSION=$VERSION
RUN mkdocs build

FROM nginx:alpine-slim

RUN mkdir -p /usr/share/nginx/html/zilliqa1
RUN mkdir -p /usr/share/nginx/html/zilliqa2
COPY --from=builder --chown=nginx:nginx /build/zq1/site/. /usr/share/nginx/html/zilliqa1/.
COPY --from=builder --chown=nginx:nginx /build/zq2/site/. /usr/share/nginx/html/zilliqa2/.
COPY default.conf /etc/nginx/conf.d/default.conf
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

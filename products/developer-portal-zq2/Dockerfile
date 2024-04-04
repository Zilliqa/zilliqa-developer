FROM ubuntu:22.04 AS builder

ENV DEBIAN_FRONTEND=noninteractive
ENV NEEDRESTART_MODE=a

RUN apt update && apt dist-upgrade -y
RUN apt install -y python3 python3-pip

COPY .  /build
RUN pip3 install -r /build/requirements.txt

ENV DOC_SOURCE=/build/docs-zq2
RUN (cd /build && mkdocs build)

FROM nginx:alpine-slim

COPY --from=builder --chown=nginx:nginx /build/site/. /usr/share/nginx/html/.
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

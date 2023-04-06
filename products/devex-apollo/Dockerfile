FROM node:12.18.3

RUN apt update && apt install -y --no-install-recommends python=2.7.13-2 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY ./package.json ./
COPY ./yarn.lock ./
RUN yarn install && yarn cache clean

COPY . ./
RUN wget -q https://s3.amazonaws.com/rds-downloads/rds-combined-ca-bundle.pem
EXPOSE 5000

CMD ["bash", "run.sh"]

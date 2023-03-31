FROM node:12.18.3

RUN apt-get update && apt-get install -y python
WORKDIR /app

COPY ./package.json ./
COPY ./yarn.lock ./
RUN yarn install

COPY . ./
RUN wget https://s3.amazonaws.com/rds-downloads/rds-combined-ca-bundle.pem
EXPOSE 5000

CMD ["bash", "run.sh"]

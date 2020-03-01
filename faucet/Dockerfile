FROM node:10

COPY . ./faucet
WORKDIR /faucet

RUN npm install
RUN npm run build

ENTRYPOINT node ./dist/server.js
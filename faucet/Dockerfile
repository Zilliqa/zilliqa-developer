FROM nodejs:10

COPY ./ ./faucet
WORKDIR /faucet

RUN npm install
ENTRYPOINT node faucetGenerator.js
FROM zilliqa/scilla:v0.12.0rc1

ARG DEPLOY_ENV="dev"

# Install node.js
RUN curl -sL https://deb.nodesource.com/setup_10.x  | bash -
RUN apt-get -y install nodejs

COPY . /scilla-server

WORKDIR /scilla-server
RUN npm install
RUN npm run build

EXPOSE 4000

ENV DEPLOY_ENV=${DEPLOY_ENV}
ENTRYPOINT ["sh", "run_server.sh"]
#ENTRYPOINT NODE_ENV=production SCILLA_VERSION=0 npm run start

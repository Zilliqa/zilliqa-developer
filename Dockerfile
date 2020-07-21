FROM zilliqa/zilliqa:v6.3.0

ARG SOURCE_DIR=/zilliqa

WORKDIR ${SOURCE_DIR}

COPY boot.json ./boot.json
COPY constants.xml ./constants.xml
EXPOSE 5555

ENTRYPOINT ["isolatedServer", "-f", "boot.json", "&>> /zilliqa/logs/isolated-server.logs"]

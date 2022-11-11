#FROM 648273915458.dkr.ecr.us-west-2.amazonaws.com/zilliqa:7186a35
FROM zilliqa/zilliqa:v8.2.0rc2
ARG SOURCE_DIR=/zilliqa

WORKDIR ${SOURCE_DIR}

RUN mkdir -p /zilliqa/logs
COPY boot.json ./boot.json
COPY constants.xml ./constants.xml
COPY dsnodes.xml ./dsnodes.xml
COPY validate.sh ./validate.sh
COPY run.sh ./run.sh
EXPOSE 5555

ARG MODE=""
ENV MODE=${MODE}
ENTRYPOINT ["bash", "run.sh"]

FROM 648273915458.dkr.ecr.us-west-2.amazonaws.com/zilliqa:d22e09a
#FROM zilliqa/zilliqa:v6.3.0

ARG SOURCE_DIR=/zilliqa

WORKDIR ${SOURCE_DIR}

RUN mkdir -p /zilliqa/logs
COPY boot.json ./boot.json
COPY constants.xml ./constants.xml
COPY run.sh ./run.sh
EXPOSE 5555

ARG MODE=""
ENV MODE=${MODE}
ENTRYPOINT ["bash", "run.sh"]

docker build --rm -f Dockerfile -t zilliqa_isolatedserver:1 .
docker run --rm -d  -p 5555:5555 --name=zilliqa_isolatedserver zilliqa_isolatedserver:1
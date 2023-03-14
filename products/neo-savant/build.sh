docker build --rm --no-cache -f Dockerfile -t zilliqa_savant-ide:1 .
docker stop zilliqa_savant-ide
docker run --rm -d  -p 80:80 --name=zilliqa_savant-ide zilliqa_savant-ide:1
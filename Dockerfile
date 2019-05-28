FROM ubuntu:latest

WORKDIR /usr/local/bin

RUN set -e; \
  apt-get update; \
  apt-get install -y openssl

COPY target/release/actix_mongo_test .

ENTRYPOINT ["actix_mongo_test"]
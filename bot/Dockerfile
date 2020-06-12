FROM golang:1.12.17-alpine3.11
LABEL maintainer "eto-shingo"

ARG AWS_ACCESS_KEY_ID
ARG AWS_SECRET_ACCESS_KEY

RUN apk update
RUN apk --update-cache \
    add musl musl-dev \
    linux-headers \
    gcc g++ make git curl\
    gfortran \
    openblas-dev libressl-dev pkgconfig \
    python3 python3-dev \
    nodejs npm\
    musl-dev

RUN pip3 install --upgrade pip
RUN pip3 install awscli
RUN pip3 install boto3
RUN npm install -g serverless

RUN sls config credentials --provider aws --key $AWS_ACCESS_KEY_ID --secret $AWS_SECRET_ACCESS_KEY

ENV GO111MODULE=on

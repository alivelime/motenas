.PHONY: all build clean deploy line line_api

all: clean build
	sls deploy --verbose --stage dev

all-prd: clean build
	sls deploy --verbose --stage prd

build: clean line line_api

line:
	cd line && go mod download
	env GOOS=linux CGO_ENABLED=0 go build -ldflags='-d -s -w -extldflags "-static"' -a -tags netgo -installsuffix netgo -o bin/line line/*.go

line_api:
	cd line_api && go mod download
	env GOOS=linux CGO_ENABLED=0 go build -ldflags='-d -s -w -extldflags "-static"' -a -tags netgo -installsuffix netgo -o bin/line_api line_api/*.go

deploy:
	sls deploy --verbose --stage dev

deploy-prd:
	sls deploy --verbose --stage prd

clean:
	rm -rf ./bin/*


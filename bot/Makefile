.PHONY: build clean deploy

build:
	cd linebot-bachan && go get .
	env GOOS=linux CGO_ENABLED=0 go build -ldflags='-d -s -w -extldflags "-static"' -a -tags netgo -installsuffix netgo -o bin/linebot-bachan linebot-bachan/*.go

clean:
	rm -rf ./bin/*

deploy: clean build
	sls deploy --verbose
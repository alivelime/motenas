.PHONY: build clean deploy

build:
	cd linebot && go get .
	env GOOS=linux go build -ldflags="-d -s -w" -a -tags netgo -installsuffix netgo -o bin/linebot linebot/*.go

clean:
	rm -rf ./bin

deploy: clean build
	sls deploy --verbose

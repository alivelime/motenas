.PHONY: build clean deploy

build:
	cd line-bachan && go get .
	env GOOS=linux CGO_ENABLED=0 go build -ldflags='-d -s -w -extldflags "-static"' -a -tags netgo -installsuffix netgo -o bin/line-bachan line-bachan/*.go
	cd line-bachan-admin && go get .
	env GOOS=linux CGO_ENABLED=0 go build -ldflags='-d -s -w -extldflags "-static"' -a -tags netgo -installsuffix netgo -o bin/line-bachan-admin line-bachan-admin/*.go
	cd line-minarai && go get .
	env GOOS=linux CGO_ENABLED=0 go build -ldflags='-d -s -w -extldflags "-static"' -a -tags netgo -installsuffix netgo -o bin/line-minarai line-minarai/*.go
	cd line-minarai-admin && go get .
	env GOOS=linux CGO_ENABLED=0 go build -ldflags='-d -s -w -extldflags "-static"' -a -tags netgo -installsuffix netgo -o bin/line-minarai-admin line-minarai-admin/*.go

clean:
	rm -rf ./bin/*

deploy: clean build
	sls deploy --verbose

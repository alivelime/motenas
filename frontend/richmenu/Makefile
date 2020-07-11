.PHONY: build build-prd deploy deploy-prd

build:
	REACT_APP_ENV=dev npm run build

build-prd:
	REACT_APP_ENV=prd npm run build

deploy: build
	cd ../deploy && npm run build && cdk deploy -c stage=dev --require-approval never

deploy-prd: build-prd
	cd ../deploy && npm run build && cdk deploy -c stage=prd --require-approval never


OPENAPI_SPEC=./api/service.yml
OUTPUT_DIR=./src/apis/

generate:
	openapi-generator-cli generate -i api/service.yml -g rust -o generated --config api/config.json
	cp generated/src/apis/* src/apis/
	cp generated/src/models/* src/apis/models/

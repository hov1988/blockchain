OPENAPI_SPEC=./api/service.yml
OUTPUT_DIR=./src/apis/

generate-models:
	npx @openapitools/openapi-generator-cli generate \
		-i $(OPENAPI_SPEC) \
		-g rust \
		-o $(OUTPUT_DIR) \
		--global-property=models \
		--additional-properties=packageName=apis.models,modelPackage=apis.models

cleanup:
	mv $(OUTPUT_DIR)/src/* $(OUTPUT_DIR)/ || true
	rm -rf $(OUTPUT_DIR)/src || true

models: generate-models cleanup

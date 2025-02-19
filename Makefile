build-js:
	@echo "Building JS"
	@cd node-binding && yarn build

run-js:
	@echo "Running JS"
	@cd node-binding && start

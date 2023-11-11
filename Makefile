.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx generate generator
	dfx deploy generator
	npm run dev

run:
	npm run dev

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx

bk:
	git add .
	git commit -m "update"
	git push
	./semver

genpb:
	rm -rf gen/*
	rm -rf gen-ts/*
	rm -rf gen-js/*
	buf lint
	rm -rf gen/*
	buf generate
	go mod tidy

all: genpb bk
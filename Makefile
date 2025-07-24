bk:
	git add .
	git commit -m "update"
	git push
	./semver

genpb:
	buf lint
	rm -rf gen/*
	buf generate
	go mod tidy
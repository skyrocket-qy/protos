bk:
	git add .
	git commit -m "update"
	git push

genpb:
	buf lint
	rm -rf gen/*
	buf generate
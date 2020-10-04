.PHONY: web release worker

all: web release worker

web:
	docker build -t frenetiq/dolus-web:latest -f ./web/dockerfile ./web

release:
	docker build -t frenetiq/dolus-release:latest -f ./web/dockerfile-release ./web

worker:
	docker build -t frenetiq/dolus-worker:latest -f ./scraper/dockerfile ./scraper

deploy: all
	docker tag frenetiq/dolus-web:latest registry.heroku.com/dolus/web
	docker tag frenetiq/dolus-release:latest registry.heroku.com/dolus/release
	docker tag frenetiq/dolus-worker:latest registry.heroku.com/dolus/worker
	docker push registry.heroku.com/dolus/web
	docker push registry.heroku.com/dolus/release
	docker push registry.heroku.com/dolus/worker
	heroku container:release web release worker -a=dolus

# ./Makefile
new-app:
	cargo leptos new -g leptos-rs/start-axum -n app 
	curl -o Dockerfile https://raw.githubusercontent.com/maximilianou/weekly70/refs/heads/main/web44-docker/Dockerfile
	curl -o Dockerfile.prod https://raw.githubusercontent.com/maximilianou/weekly70/refs/heads/main/web44-docker/Dockerfile.prod
	curl -o compose.yml https://raw.githubusercontent.com/maximilianou/weekly70/refs/heads/main/web44-docker/compose.yml

dev-app:
	docker compose up --remove-orphans

run-app:
	docker build -f Dockerfile.prod -t rust-app-prod .

del-app:
	rm -r app
	rm Dockerfile Dockerfile.prod compose.yml

clean:
	cd app && cargo clean
	docker container prune
	docker-compose up -d --remove-orphans
	# docker image ls
	# docker image rm rustapp01
	# docker image rm web40-docker-server
	# docker image rm web40-docker-app-dev
	# docker image rm rust-app-watch
	
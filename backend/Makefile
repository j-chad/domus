setup:
	cargo run --bin setup-local-config

docker-deps:
	docker-compose up -d

start: docker-deps
	env DOMUS_ENV='dev' cargo run --bin domus
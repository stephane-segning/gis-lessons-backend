THIS_FILE := $(lastword $(MAKEFILE_LIST))

## ==========================================================
##    ___  ___       _              _         ___  _    _ 
##   / __>/ __> ___ | |_  ___  ___ | | ___   |  _>| |  | |
##   \__ \\__ \/ | '| . |/ . \/ . \| |<_-<   | <__| |_ | |
##   <___/<___/\_|_.|_|_|\___/\___/|_|/__/   `___/|___||_|
##   -------------------------
##   Makefile for the project
##   Author: @stephane-segning
## ==========================================================

.PHONY: help init build up start down destroy stop restart logs logs-api ps login-timescale login-api db-shell

init: 				## Initialize the project
	rm -rf packages/gen-server && docker compose -p sschool run --rm openapi-generator-cli $(c)

help:				## Show this help
	@sed -ne '/@sed/!s/## //p' $(MAKEFILE_LIST)

build: init			## Build the project
	docker compose -p sschool -f compose.yaml build $(c)
up: init 			## Start the project
	docker compose -p sschool -f compose.yaml up -d --remove-orphans --build $(c)
up-app: init			## Start app
	docker compose -p sschool -f compose.yaml up -d --remove-orphans --build app $(c)

start: init			## Start the project
	docker compose -p sschool -f compose.yaml start $(c)
down: 				## Stop the project
	docker compose -p sschool -f compose.yaml down $(c)
destroy: 			## Destroy the project
	docker compose -p sschool -f compose.yaml down -v $(c)
stop: 				## Stop the project
	docker compose -p sschool -f compose.yaml stop $(c)
restart: init			## Restart the project
	docker compose -p sschool -f compose.yaml stop $(c)
	docker compose -p sschool -f compose.yaml up -d $(c)

logs: 				## Show logs
	docker compose -p sschool -f compose.yaml logs --tail=100 -f $(c)
logs-app: 			## Show app logs
	docker compose -p sschool -f compose.yaml logs --tail=100 -f app $(c)
ps: 				## Show status
	docker compose -p sschool -f compose.yaml ps $(c)

stats: 				## Show stats
	docker compose -p sschool -f compose.yaml stats $(c)
	
git-pull:			## Git fetch link-frontend
	git submodule update --remote packages/link-frontend

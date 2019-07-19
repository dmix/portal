# ---------------------------------------------------------------------------------
# Portal
# ---------------------------------------------------------------------------------

# -- Options

HOME=/Users/dmix
CONFIG_PATH=/usr/local/var

# -- Main

dev: 
	@cargo watch -x 'run -- dev'

test: 
	@echo 'Testing'

install:
	@mkdir -p $(CONFIG_PATH)}/portal/
	@mkdir -p $(CONFIG_PATH)}/portal/db/
	@cargo build
	@cp -rf ./portal.toml $(CONFIG_PATH)/portal/portal.toml
	@cp -rf ./target/debug/portal /usr/local/bin/portal

reset: 
	@rm -rf /usr/local/lib/portal/db/*

# -- Helpers

tmux:
	@tmuxp load .tmuxp.yaml

clean:
	@cargo clean
	@rm .DS_Store; rm **/*/.DS_Store

# -- Makefile

.PHONY := dev test install tmux clean
.DEFAULT_GOAL := dev

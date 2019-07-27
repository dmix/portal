# ---------------------------------------------------------------------------------
# Portal
# ---------------------------------------------------------------------------------

# -- Options

HOME=/Users/dmix
CONFIG_PATH=/usr/local/var/portal
DATA_PATH=/usr/local/lib/portal
ELVISH_DIR=~/.elvish/lib/portal

# -- Main

dev: initialize
	@cargo watch -x 'run jump portal'

test: 
	@echo 'Testing'

initialize: 
	@mkdir -p $(ELVISH_DIR)
	@mkdir -p $(DATA_PATH)
	@touch $(DATA_PATH)/portal.log
	@mkdir -p $(CONFIG_PATH)
	@mkdir -p $(CONFIG_PATH)/db/

install: initialize
	@cargo build
	@cp -rf ./portal.toml $(CONFIG_PATH)/portal.toml
	@cp -rf ./target/debug/portal /usr/local/bin/portal

reset: 
	@rm -rf $(DATA_PATH)/portal.log
	@rm -rf $(DATA_PATH)/db/*
	@touch $(DATA_PATH)/portal.log

# -- Helpers

tmux:
	@tmuxp load .tmuxp.yaml

clean:
	@cargo clean
	@rm .DS_Store; rm **/*/.DS_Store

# -- Makefile

.PHONY := dev test install tmux clean
.DEFAULT_GOAL := dev

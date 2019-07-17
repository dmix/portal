# ---------------------------------------------------------------------------------
# Portal
# ---------------------------------------------------------------------------------

# -- Options

HOME=/Users/dmix

# -- Main

dev: 
	@cargo watch -x 'run -- dev'

test: 
	@echo 'Testing'

install:
	@mkdir -p /usr/local/lib/portal/
	@mkdir -p /usr/local/lib/portal/db/
	@cargo build
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

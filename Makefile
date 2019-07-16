# ---------------------------------------------------------------------------------
# Portal
# ---------------------------------------------------------------------------------

# -- Options

HOME=/Users/dmix

# -- Main

dev: 
	@cargo run

test: 
	@echo 'Testing'

install:
	@cargo install

# -- Helpers

tmux:
	@tmuxp load .tmuxp.yaml

clean:
	@cargo clean
	@rm .DS_Store; rm **/*/.DS_Store

# -- Makefile

.PHONY := dev test install tmux clean
.DEFAULT_GOAL := dev

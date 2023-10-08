INSTALL_PATH			:=$(HOME)/usr/libexec/
RXNOW_NAME			:=rxnow
RXNOW_VERSION			:=$(shell cargo run -- --version | awk '{ print $$NF }')
RXNOW_DEBUG_EXEC		:=target/debug/$(RXNOW_NAME)
RXNOW_RELEASE_EXEC		:=target/release/$(RXNOW_NAME)
RXNOW_EXEC			:=$(RXNOW_DEBUG_EXEC)
RXNOW_RUN			:=$(RXNOW_DEBUG_EXEC)
RXNOW_RUN			:=cargo run --bin $(RXNOW_NAME) --

all: test debug release

$(INSTALL_PATH):
	mkdir -p $@

$(RXNOW_RELEASE_EXEC): $(INSTALL_PATH)
	cargo build --release

$(RXNOW_DEBUG_EXEC): $(INSTALL_PATH)
	cargo build

release: check fix | $(RXNOW_RELEASE_EXEC)
	install $(RXNOW_RELEASE_EXEC) $(INSTALL_PATH)/$(RXNOW_NAME)-$(RXNOW_VERSION)
	install $(RXNOW_RELEASE_EXEC) $(INSTALL_PATH)

debug: | $(RXNOW_DEBUG_EXEC)
	install $(RXNOW_DEBUG_EXEC) $(INSTALL_PATH)/$(RXNOW_NAME)-$(RXNOW_VERSION)
	install $(RXNOW_DEBUG_EXEC) $(INSTALL_PATH)

clean: cls
	@rm -rf target

cleanx:
	@rm -rf $(RXNOW_DEBUG_EXEC)
	@rm -rf $(RXNOW_RELEASE_EXEC)

cls:
	-@reset || tput reset

fix:
	cargo fix

fmt:
	rustfmt --edition 2021 src/*.rs

check:
	cargo check --all-targets

run:
	$(RXNOW_RUN) --help
	$(RXNOW_RUN) 'rx(now)' Cargo.toml
	git remote show -n origin | $(RXNOW_RUN) '((https://|git@)github.com[/:].*[.]git)'
	git remote show -n origin | $(RXNOW_RUN) '((https://|git@)github.com[/:].*[.]git)' --replace "foobar$$2"
	$(RXNOW_RUN) 'rx(now)' --colorless Cargo.toml
	git remote show -n origin | $(RXNOW_RUN) --colorless '((https://|git@)github.com[/:].*[.]git)'
	git remote show -n origin | $(RXNOW_RUN) --colorless '((https://|git@)github.com[/:].*[.]git)' --replace "foobar$$2"


build test: check
	cargo $@

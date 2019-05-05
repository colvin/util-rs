PROFILE=debug
RS_MKDIR=./target/$(PROFILE)/mkdir

default: release

build:
	cargo build

release:
	cargo build --release

install:
	cargo install --path .

uninstall:
	cargo uninstall

reinstall: uninstall install

test:
	cargo test
	rm -rf test
	mkdir test
	$(RS_MKDIR) -p test/foo test/bar
	test -d test/foo
	test -d test/bar
	# This is supposed to emit an error
	! $(RS_MKDIR) test/blorg/blip
	$(RS_MKDIR) -p test/blorg/blip
	test -d test/blorg
	test -d test/blorg/blip

.PHONY: test

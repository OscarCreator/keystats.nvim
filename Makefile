
UNAME_S := $(shell uname -s)

.PHONY: build
build:
	cargo build --release
	@rm -rf $(PWD)/lua/libkeystats_nvim.so $(PWD)/lua/deps/
    ifeq ($(UNAME_S),Linux)
		cp $(PWD)/target/release/libkeystats_nvim.so $(PWD)/lua/keystats_nvim.so
    endif
    ifeq ($(UNAME_S),Darwin)
		cp $(PWD)/target/release/libkeystats_nvim.dylib $(PWD)/lua/keystats_nvim.so
    endif
	@mkdir -p $(PWD)/lua/deps/
	cp $(PWD)/target/release/deps/*.rlib $(PWD)/lua/deps/

.PHONY: clean
clean: 
	@rm -rf lua/deps
	@rm lua/keystats_nvim.so


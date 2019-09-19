###############################
# Common defaults/definitions #
###############################

comma := ,

# Checks media given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)




######################
# Project parameters #
######################

RUST_VER := 1.37

CURRENT_BRANCH := $(strip $(shell git branch | grep \* | cut -d ' ' -f2))




###########
# Aliases #
###########

fmt: cargo.fmt


lint: cargo.lint

test:
	@make test.unit




##################
# Cargo commands #
##################

# Resolve Cargo project dependencies.
#
# Usage:
#	make cargo [cmd=(fetch|<cargo-cmd>)]

cargo:
	cargo $(if $(call eq,$(cmd),),fetch,$(cmd))


# Format Rust sources with rustfmt.
#
# Usage:
#	make cargo.fmt [check=(no|yes)]

cargo.fmt:
	cargo +nightly fmt --all $(if $(call eq,$(check),yes),-- --check,)


# Lint Rust sources with clippy.
#
# Usage:
#	make cargo.lint

cargo.lint:
	cargo clippy --all -- -D clippy::pedantic -D warnings




####################
# Testing commands #
####################

# Run Rust unit tests of project.
#
# Usage:
#	make test.unit

CHROMEDRIVER_CLIENT_ARGS := $(strip \
	$(shell grep 'CHROMEDRIVER_CLIENT_ARGS=' .env | cut -d '=' -f2))

test.unit:
	CHROMEDRIVER_CLIENT_ARGS="$(CHROMEDRIVER_CLIENT_ARGS)" \
	cargo test --target wasm32-unknown-unknown




##################
# .PHONY section #
##################

.PHONY: cargo cargo.fmt cargo.lint \
        test test.unit

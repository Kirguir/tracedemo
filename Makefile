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




####################
# Running commands #
####################

up:
	npm run start --prefix=demo




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




#################
# Yarn commands #
#################

# Resolve NPM project dependencies with Yarn.
#
# Optional 'cmd' parameter may be used for handy usage of docker-wrapped Yarn,
# for example: make yarn cmd='upgrade'
#
# Usage:
#	make yarn [cmd=('install --pure-lockfile'|<yarn-cmd>)]

yarn-cmd = $(if $(call eq,$(cmd),),install --pure-lockfile,$(cmd))

yarn:
	yarn --cwd=demo $(yarn-cmd)




##################
# .PHONY section #
##################

.PHONY: cargo cargo.fmt cargo.lint \
        up yarn

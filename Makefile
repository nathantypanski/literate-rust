SRCDIR = src
PROGRAM_NAME = lrustc

all: $(PROGRAM_NAME)

.PHONY: clean $(PROGRAM_NAME)

$(PROGRAM_NAME): $(SRCDIR)/$(PROGRAM_NAME).rs
	rustc src/liblrustc/lib.rs
	rustc -L. $(SRCDIR)/$(PROGRAM_NAME).rs

clean :
	$(RM) $(PROGRAM_NAME)

run: ${PROGRAM_NAME}
	./${PROGRAM_NAME}

test: ${PROGRAM_NAME}
	rustc --test --out-dir src/liblrustc src/liblrustc/lib.rs
	RUST_LOG=debug RUST_TEST_TASKS=1 ./src/liblrustc/lrustc
	RUST_LOG=debug RUST_TEST_TASKS=1 ./lrustc

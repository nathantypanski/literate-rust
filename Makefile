SRCDIR = src
PROGRAM_NAME = lrustc

all: $(PROGRAM_NAME)

.PHONY: clean $(PROGRAM_NAME)

$(PROGRAM_NAME): $(SRCDIR)/$(PROGRAM_NAME).rs
	rustc -L. $(SRCDIR)/$(PROGRAM_NAME).rs

clean :
	$(RM) $(PROGRAM_NAME)

run: ${PROGRAM_NAME}
	./${PROGRAM_NAME}

test:
	rustc --test --out-dir . src/$(PROGRAM_NAME).rs
	RUST_LOG=debug RUST_TEST_TASKS=1 ./lrustc

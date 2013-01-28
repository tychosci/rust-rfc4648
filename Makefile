# makefile

E       := examples
LIBDIR  := lib
BINDIR  := bin
TESTDIR := test
DUMMY   := $(LIBDIR)/librfc4648.dummy
CRATE   := rfc4648.rs
SOURCE  := $(shell find . -name '*.rs')

all: $(DUMMY)

$(DUMMY): $(SOURCE) $(CRATE)
	mkdir -p $(LIBDIR)
	rustc -O $(CRATE) -o $@
	touch $(DUMMY)

base64: $(DUMMY)
	mkdir -p $(BINDIR)
	rustc -O $(E)/base64.rs -o $(BINDIR)/$@ -L $(LIBDIR)

base64-stream: $(DUMMY)
	mkdir -p $(BINDIR)
	rustc -O $(E)/base64-stream.rs -o $(BINDIR)/$@ -L $(LIBDIR)

base64-tcp-server: $(DUMMY)
	mkdir -p $(BINDIR)
	rustc -O $(E)/base64-tcp-server.rs -o $(BINDIR)/$@ -L $(LIBDIR)

test: $(CRATE)
	mkdir -p $(TESTDIR)
	rustc -O $(CRATE) --test --out-dir $(TESTDIR)

clean:
	@rm -rf "$(LIBDIR)"
	@rm -rf "$(TESTDIR)"
	@rm -rf "$(BINDIR)"

.PHONY: base64 base64-stream base64-tcp-server test clean

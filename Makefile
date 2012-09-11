# makefile

E       := examples
SRCDIR  := .
LIBDIR  := lib
BINDIR  := bin
TESTDIR := test
SOURCES := $(wildcard $(SRCDIR)/*.rs)

all: libencoding

libencoding: setup-lib encoding.rc $(SOURCES)
	rustc -O encoding.rc --out-dir $(LIBDIR)

b64: setup-bin libencoding
	rustc -O $(E)/b64.rs -o $(BINDIR)/$@ -L $(LIBDIR)

b64-stream: setup-bin libencoding
	rustc -O $(E)/b64-stream.rs -o $(BINDIR)/$@ -L $(LIBDIR)

b64-tcp-server: setup-bin libencoding
	rustc -O $(E)/b64-tcp-server.rs -o $(BINDIR)/$@ -L $(LIBDIR)

setup-lib:
	mkdir -p $(LIBDIR)

setup-bin:
	mkdir -p $(BINDIR)

setup-test:
	mkdir -p $(TESTDIR)

test: setup-test test-encoding

test-encoding: encoding.rc
	rustc -O $< --test --out-dir $(TESTDIR)

clean:
	if [ -d "$(LIBDIR)" ]; then rm -r $(LIBDIR); fi
	if [ -d "$(TESTDIR)" ]; then rm -r $(TESTDIR); fi
	if [ -d "$(BINDIR)" ]; then rm -r $(BINDIR); fi

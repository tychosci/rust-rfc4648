# makefile

E       := examples
SRCDIR  := .
LIBDIR  := .
BINDIR  := bin
TESTDIR := test
SOURCES := $(wildcard $(SRCDIR)/*.rs)

all: libencoding

libencoding: encoding.rc $(SOURCES)
	rustc -O encoding.rc

b64: setup-bin libencoding
	rustc -O $(E)/b64.rs -L $(LIBDIR) -o $(BINDIR)/$@

b64-stream: setup-bin libencoding
	rustc -O $(E)/b64-stream.rs -L $(LIBDIR) -o $(BINDIR)/$@

setup-bin:
	mkdir -p $(BINDIR)

setup-test:
	mkdir -p $(TESTDIR)

test: setup-test test-encoding

test-encoding: encoding.rc
	rustc -O $< --test --out-dir $(TESTDIR)

clean:
	-rm -r libencoding-*.dSYM
	-rm -r libencoding-*.dylib
	if [ -d "$(TESTDIR)" ]; then rm -r $(TESTDIR); fi
	if [ -d "$(BINDIR)" ]; then rm -r $(BINDIR); fi

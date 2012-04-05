# makefile

E       := examples
SRCDIR  := .
LIBDIR  := .
BINDIR  := .
TESTDIR := test
SOURCES := $(wildcard $(SRCDIR)/*.rs)

all: libencoding

libencoding: encoding.rc $(SOURCES)
	rustc -O encoding.rc

b64: libencoding
	rustc -O $(E)/b64.rs -L $(LIBDIR) -o $(BINDIR)/$@

setup-test:
	mkdir -p $(TESTDIR)

test: setup-test test-encoding

test-encoding: encoding.rc
	rustc -O $< --test --out-dir $(TESTDIR)

clean:
	-rm -r libencoding-*.dSYM
	-rm -r libencoding-*.dylib
	if [ -d "$(TESTDIR)" ]; then rm -r $(TESTDIR); fi
	if [ -e "$(BINDIR)/b64" ]; then rm -r $(BINDIR)/b64; fi
	if [ -e "$(BINDIR)/b64.dSYM" ]; then rm -r $(BINDIR)/b64.dSYM; fi

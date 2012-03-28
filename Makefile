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

test-all: setup-test test-base64 test-base32 test-base16

test-base64: base64.rs
	rustc $< --test -o $(TESTDIR)/$@

test-base32: base32.rs
	rustc $< --test -o $(TESTDIR)/$@

test-base16: base16.rs
	rustc $< --test -o $(TESTDIR)/$@

clean:
	rm -r libencoding-*.dSYM
	rm -r libencoding-*.dylib
	if [ -d "$(TESTDIR)" ]; then rm -r $(TESTDIR); fi
	if [ -e "$(BINDIR)/b64" ]; then rm -r $(BINDIR)/b64; fi
	if [ -e "$(BINDIR)/b64.dSYM" ]; then rm -r $(BINDIR)/b64.dSYM; fi

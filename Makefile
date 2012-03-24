# makefile

E       := examples
SRCDIR  := .
LIBDIR  := .
BINDIR  := .
SOURCES := $(wildcard $(SRCDIR)/*.rs)

all: libencoding

libencoding: encoding.rc $(SOURCES)
	rustc -O encoding.rc

b64: libencoding
	rustc -O $(E)/b64.rs -L $(LIBDIR) -o $(BINDIR)/$@

clean:
	rm -r libencoding-*.dSYM
	rm -r libencoding-*.dylib
	if [ -e "$(BINDIR)/b64" ]; then rm -r $(BINDIR)/b64; fi
	if [ -e "$(BINDIR)/b64.dSYM" ]; then rm -r $(BINDIR)/b64.dSYM; fi

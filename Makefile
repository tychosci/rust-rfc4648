# makefile

E       := examples
LIBDIR  := lib
BINDIR  := bin
TESTDIR := test

all: libencoding

libencoding: setup-lib encoding.rc
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
	@rm -rf "$(LIBDIR)"
	@rm -rf "$(TESTDIR)"
	@rm -rf "$(BINDIR)"

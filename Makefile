# makefile

E       := examples
LIBDIR  := lib
BINDIR  := bin
TESTDIR := test

all: libcodec

libcodec: setup-lib codec.rc
	rustc -O codec.rc --out-dir $(LIBDIR)

base64: setup-bin libcodec
	rustc -O $(E)/base64.rs -o $(BINDIR)/$@ -L $(LIBDIR)

base64-stream: setup-bin libcodec
	rustc -O $(E)/base64-stream.rs -o $(BINDIR)/$@ -L $(LIBDIR)

base64-tcp-server: setup-bin libcodec
	rustc -O $(E)/base64-tcp-server.rs -o $(BINDIR)/$@ -L $(LIBDIR)

setup-lib:
	mkdir -p $(LIBDIR)

setup-bin:
	mkdir -p $(BINDIR)

setup-test:
	mkdir -p $(TESTDIR)

test: setup-test test-codec

test-codec: codec.rc
	rustc -O $< --test --out-dir $(TESTDIR)

clean:
	@rm -rf "$(LIBDIR)"
	@rm -rf "$(TESTDIR)"
	@rm -rf "$(BINDIR)"

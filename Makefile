# makefile

E       := examples
LIBDIR  := lib
BINDIR  := bin
TESTDIR := test

all: librfc4648

librfc4648: setup-lib rfc4648.rc
	rustc -O rfc4648.rc --out-dir $(LIBDIR)

base64: setup-bin librfc4648
	rustc -O $(E)/base64.rs -o $(BINDIR)/$@ -L $(LIBDIR)

base64-stream: setup-bin librfc4648
	rustc -O $(E)/base64-stream.rs -o $(BINDIR)/$@ -L $(LIBDIR)

base64-tcp-server: setup-bin librfc4648
	rustc -O $(E)/base64-tcp-server.rs -o $(BINDIR)/$@ -L $(LIBDIR)

setup-lib:
	mkdir -p $(LIBDIR)

setup-bin:
	mkdir -p $(BINDIR)

setup-test:
	mkdir -p $(TESTDIR)

test: setup-test test-rfc4648

test-rfc4648: rfc4648.rc
	rustc -O $< --test --out-dir $(TESTDIR)

clean:
	@rm -rf "$(LIBDIR)"
	@rm -rf "$(TESTDIR)"
	@rm -rf "$(BINDIR)"

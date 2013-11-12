# makefile

LIBDIR   := lib
BINDIR   := bin
BUILDDIR := build

all: rfc4648

test:
	rustpkg test rfc4648

rfc4648:
	rustpkg install -O rfc4648

base64-demo: rfc4648
	rustpkg install -O base64-demo

clean:
	rustpkg clean
	@rm -rf "$(LIBDIR)"
	@rm -rf "$(BUILDDIR)"
	@rm -rf "$(BINDIR)"

.PHONY: rfc4648 base64-demo test

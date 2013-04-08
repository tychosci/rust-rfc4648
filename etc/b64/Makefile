CC = clang
CFLAGS = -O2 -Wall
SOURCES = $(shell find . -type f -name '*.c')

all: b64

b64: $(SOURCES)
	$(CC) $(CFLAGS) $(SOURCES) -o $@

clean:
	rm b64

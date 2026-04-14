PREFIX  = riscv64-unknown-elf
CC      = $(PREFIX)-gcc
OBJCOPY = $(PREFIX)-objcopy

CFLAGS = -march=rv64i -mabi=lp64 -Wl,-Ttext=0x0 -nostdlib
SRCS   = $(wildcard *.s)
BINS   = $(patsubst %.s,bin/%.bin,$(SRCS))

all: $(BINS)

bin/%.bin: %.s | bin
	$(CC) $(CFLAGS) -o /tmp/$*.o $*.s
	$(OBJCOPY) -O binary /tmp/$*.o $@

bin:
	mkdir -p bin

clean:
	rm -rf bin /tmp/*.o

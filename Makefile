KERNEL_NAME = kernel
TARGET = aarch64-unknown-none
IMG = kernel8.img
OBJCOPY = aarch64-linux-gnu-objcopy

.PHONY: all clean build img

all: img

build:
	RUSTFLAGS="-C link-arg=-Tlink.ld" cargo build --release --target $(TARGET)

img: build
	$(OBJCOPY) -O binary target/$(TARGET)/release/$(KERNEL_NAME) $(IMG)
	@echo "Kernel size:"
	@ls -lh $(IMG)

clean:
	cargo clean
	rm -f $(IMG)
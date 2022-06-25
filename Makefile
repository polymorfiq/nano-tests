PORT ?= $(shell (ls /dev/*modem* || ls /dev/*ACM*) | head -n 1)

build:
	cargo build --release

monitor:
	arduino-cli monitor -p ${PORT}

flash: build
	rust-objcopy -O binary target/thumbv6m-none-eabi/release/stonecast target/arduino.bin
	arduino-cli upload -i target/arduino.bin -b arduino:samd:nano_33_iot -p ${PORT}
	sleep 0.01
	arduino-cli monitor -p ${PORT}
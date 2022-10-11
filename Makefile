PORT ?= $(shell (ls /dev/*modem* || ls /dev/*ACM* || ls /dev/ttys002) | head -n 1)

build:
	cargo build --release

monitor:
	arduino-cli monitor -p ${PORT}

flash: build
	rust-objcopy -O binary target/thumbv6m-none-eabi/release/nano-tests target/arduino.bin
	arduino-cli upload -i target/arduino.bin -b arduino:samd:nano_33_iot -p ${PORT}
	sleep 2
	echo "\n\nListening to USB port of Nano..."
	cat < /dev/cu.usbmodem_nano_device_1
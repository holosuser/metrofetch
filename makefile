build:
	@cargo build --release

all: build

install: build
	@mv target/release/metrofetch /usr/bin

clean:
	@cargo clean

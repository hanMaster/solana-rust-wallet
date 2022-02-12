ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: libwallet.$(EXT)
	g++ main.cpp -L "./target/x86_64-apple-darwin/release" -lwallet -o run -arch x86_64

deploy:
	cp ./target/x86_64-apple-darwin/release/libwallet.$(EXT) /usr/local/lib

libwallet.$(EXT): src/lib.rs Cargo.toml
	cargo build --release

clean:
	rm -rf target
	rm -rf run
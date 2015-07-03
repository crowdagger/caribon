# Useful for packaging(?)

BINARYNAME = caribon
BINARY = target/release/$(BINARYNAME)

default: $(BINARYNAME)

clean:
	cargo clean

$(BINARYNAME): src/*.rs Cargo.toml
	cargo build --release
	cp $(BINARY) $(BINARYNAME)
	cargo clean

install: $(BINARY)
	cp $(BINARYNAME) $(DESTDIR)/usr/bin/$(BINARYNAME)

uninstall:
	rm $(DESTDIR)/usr/bin/$(BINARYNAME)

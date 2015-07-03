# Useful for packaging(?)

BINARYNAME = caribon
BINARY = target/release/$(BINARYNAME)

default: $(BINARY)

clean:
	cargo clean

$(BINARY): src/*.rs Cargo.toml
	cargo build --release

install: $(BINARY)
	install -d $(DESTDIR)/usr/bin/
	install $(BINARY) $(DESTDIR)/usr/bin/

uninstall:
	rm $(DESTDIR)/usr/bin/$(BINARYNAME)

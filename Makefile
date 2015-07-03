INSTALL_DIR = /usr/bin/
BINARYNAME = caribon
BINARY = target/release/$(BINARYNAME)

default: $(BINARY)

clean:
	cargo clean

$(BINARY): src/*.rs Cargo.toml
	cargo build --release

install: $(BINARY)
	install -d $(DESTDIR)$(INSTALL_DIR)
	install $(BINARY) $(DESTDIR)$(INSTALL_DIR)

uninstall:
	rm $(DESTDIR)/usr/bin/$(BINARYNAME)

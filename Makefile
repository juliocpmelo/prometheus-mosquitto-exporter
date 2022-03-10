.PHONY: all build strip install clean
BINARY=prometheus-mosquitto-exporter

all:
	#

build:
	cargo build --release

strip: build
	strip --strip-all target/release/$(BINARY)

clean:
	cargo clean

install:
	test -d $(DESTDIR)/etc/prometheus-mosquitto-exporter || mkdir -m 0755 -p $(DESTDIR)/etc/prometheus-mosquitto-exporter
	test -d $(DESTDIR)/usr/sbin || mkdir -m 0755 -p $(DESTDIR)/usr/sbin
	test -d $(DESTDIR)/lib/systemd/system || mkdir -m 0755 -p $(DESTDIR)/lib/systemd/system
	install -m 0644 etc/prometheus-mosquitto-exporter.yaml $(DESTDIR)/etc/prometheus-mosquitto-exporter/config.sample.yaml
	install -m 0644 etc/prometheus-mosquitto-exporter-env $(DESTDIR)/etc/prometheus-mosquitto-exporter/service-env
	install -m 0755 target/release/$(BINARY) $(DESTDIR)/usr/sbin
	install -m 0755 services/prometheus-mosquitto-exporter.service $(DESTDIR)/lib/systemd/system
	install -m 0755 services/prometheus-mosquitto-exporter.initd.sh $(DESTDIR)/etc/init.d/prometheus-mosquitto-exporter

uninstall:
	/bin/rm -f $(DESTDIR)/usr/sbin/$(BINARY) $(DESTDIR)/etc/prometheus-mosquitto-exporter


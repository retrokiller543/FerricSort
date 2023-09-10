BINARY_PATH=./target/release/ferric_sort
PACKAGE_NAME=ferric-sort
PACKAGE_VERSION=$(shell ./increment_version.sh)
PACKAGE_MAINTAINER="Emil Schütt emil.schutt@gmail.com"
PACKAGE_DESCRIPTION="File sorter"

# Ubuntu .deb packaging
package-ubuntu:
	mkdir -p build/ubuntu/$(PACKAGE_NAME)/DEBIAN
	mkdir -p build/ubuntu/$(PACKAGE_NAME)/usr/local/bin
	cp $(BINARY_PATH) build/ubuntu/$(PACKAGE_NAME)/usr/local/bin/
	echo "Package: $(PACKAGE_NAME)" > build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	echo "Version: $(PACKAGE_VERSION)" >> build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	echo "Section: base" >> build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	echo "Priority: optional" >> build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	echo "Architecture: amd64" >> build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	echo "Maintainer: $(PACKAGE_MAINTAINER)" >> build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	echo "Description: $(PACKAGE_DESCRIPTION)" >> build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	dpkg-deb --build build/ubuntu/$(PACKAGE_NAME)

# macOS .pkg packaging
package-macos:
	mkdir -p build/macos/$(PACKAGE_NAME).pkg/usr/local/bin
	cp $(BINARY_PATH) build/macos/$(PACKAGE_NAME).pkg/usr/local/bin/
	pkgbuild --root build/macos/$(PACKAGE_NAME).pkg --identifier com.yourdomain.$(PACKAGE_NAME) --version $(PACKAGE_VERSION) build/macos/$(PACKAGE_NAME).pkg

clean:
	rm -rf build/

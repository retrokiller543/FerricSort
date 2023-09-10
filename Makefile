# Default target to run the program in dev mode
.PHONY: all
all: dev

BINARY_PATH=./target/release/ferric_sort
PACKAGE_NAME=ferric-sort
PACKAGE_VERSION=1.0.0
PACKAGE_MAINTAINER="Emil Schütt emil.schutt@gmail.com"
PACKAGE_DESCRIPTION="File sorter"

# Define the Python code to generate random numbers in a variable
GENERATE_RANDOM_NUMBERS := python3 generate-test-data.py


# Ubuntu .deb packaging
.PHONY: package-ubuntu
package-ubuntu:
	mkdir -p build/ubuntu/$(PACKAGE_NAME)/DEBIAN
	mkdir -p build/ubuntu/$(PACKAGE_NAME)/usr/local/bin
	cp $(BINARY_PATH) build/ubuntu/$(PACKAGE_NAME)/usr/local/bin/$(PACKAGE_NAME)
	echo "Package: $(PACKAGE_NAME)" > build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	echo "Version: $(PACKAGE_VERSION)" >> build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	echo "Section: base" >> build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	echo "Priority: optional" >> build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	echo "Architecture: amd64" >> build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	echo "Maintainer: $(PACKAGE_MAINTAINER)" >> build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	echo "Description: $(PACKAGE_DESCRIPTION)" >> build/ubuntu/$(PACKAGE_NAME)/DEBIAN/control
	dpkg-deb --build build/ubuntu/$(PACKAGE_NAME)

# macOS .pkg packaging
.PHONY: package-macos
package-macos:
	mkdir -p build/macos/$(PACKAGE_NAME).pkg/usr/local/bin
	cp $(BINARY_PATH) build/macos/$(PACKAGE_NAME).pkg/usr/local/bin/$(PACKAGE_NAME)
	pkgbuild --root build/macos/$(PACKAGE_NAME).pkg --identifier com.yourdomain.$(PACKAGE_NAME) --version $(PACKAGE_VERSION) build/macos/$(PACKAGE_NAME).pkg

.PHONY: clean
clean:
	rm -rf build/

# Target to generate the data, test the Rust tool, and clean up
.PHONY: dev
dev:
	# Generate test data
	$(GENERATE_RANDOM_NUMBERS)
	# Test the Rust tool on the generated data
	cargo run -r -- Tests/test_data.txt

.PHONY: clean-test-data
clean-test-data:
	rm Tests/test_data.txt

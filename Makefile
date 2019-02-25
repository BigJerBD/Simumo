.PHONY: build_venv build_dir build_rust build_scripts build_vizualizer build_simumap \
		clean


# setup to make a build

build: build_dir build_venv build_scripts build_rust build_vizualizer build_simumap

build_rust: build_dir
	@echo "Building rust simulator"
	cargo build --release --manifest-path="simulator/Cargo.toml"
	cp -r simulator/target/release build
	if [ -d build/simulator ]; then rm -r build/simulator; fi
	mv build/release build/simulator

build_venv: build_dir
	if [ ! -d build/venv ]; then virtualenv build/venv --no-site-packages; fi

build_simumap: build_dir build_venv
	@echo "Building Simumap"
	scripts/pip_install.sh build/venv/bin/activate -r simumap/requirements.txt
	scripts/pip_install.sh build/venv/bin/activate ./simumap


build_vizualizer: build_dir
	@echo "Building Visualizer"

build_scripts: build_dir
	cp scripts/simumo.sh build

build_dir:
	mkdir -p build


# setup for a proper dev environment
dev : dev_venv

dev_venv:
	@echo "creating dev environment"
	if [ ! -d venv ]; then virtualenv venv --no-site-packages; fi

	scripts/pip_install.sh venv/bin/activate -r simumap/requirements.txt
	scripts/pip_install.sh venv/bin/activate ./simumap

clean: dev_clean build_clean

dev_clean:
	@echo "Cleaning up dev"
	if [ -d venv ]; then  rm -r venv; fi;

build_clean:
	@echo "Cleaning up build"
	if [ -d build ]; then rm -r build; fi;


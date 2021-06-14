# pipi
KOOMPI ISO BUILD TOOL

TODO

1. Build KOOMPI OS Build Tool using PI (Package Manager) using Programming Language Rust.
	1. we can use a file that contain a list of packages for making the ISO.
	2. Example: pipi (file that contain the packages)
	3. install the require dependencies, Example: xorriso
	4. when run pipi:
		1. Go check whether pi is already installed or not. Install it if it is [not](not) yet install

2. Make command line application or maybe GUI


Features:
. Able to generate ISO
. Able to generate Rootfs
. Able to add or remove package from the submodule repo of the packages repo.


# Features
run pipi --build to start building the rootfs by provide a file that contain packages with the file name of packages
run pipi --add (the app name that you want to addd into the repo)
run pipi --remove (the app name that you remove from the repo)

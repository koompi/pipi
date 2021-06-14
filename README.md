# pipi
KOOMPI ISO BUILD TOOL

TODO

1. Build KOOMPI OS Build Tool using PI (Package Manager) using Programming Language Rust.
	- we can use a file that contain a list of packages for making the ISO.
	- Example: pipi (file that contain the packages)
	- install the require dependencies, Example: xorriso
	- when run pipi:
		- Go check whether pi is already installed or not. Install it if it is [not](not) yet install

2. Make command line application or maybe GUI


Features:
- Able to generate ISO
- Able to generate Rootfs
- Able to add or remove package from the submodule repo of the packages repo.


# Features
1. run pipi --build to start building the rootfs by provide a file that contain packages with the file name of packages
2. run pipi --add (the app name that you want to add into the repo)
3. run pipi --remove (the app name that you remove from the repo)

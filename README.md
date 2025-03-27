# mecha-flash-tool
Flash Tool for flashing image to Mecha devices

## Build Instructions

Follow these steps to build the Mecha Flash Tool:

1. Clone the repository with submodules:
   ```bash
   git clone --recurse-submodules https://github.com/mecha-org/mecha-flash-tool
   ```

2. Navigate to the project directory:
   ```bash
   cd mecha-flash-tool
   ```

3. Build the project using Cargo:
   ```bash
   cargo build
   ```

## Usage

Once built, you can run the tool to flash your Mecha device firmware:

1. Locate the compiled binary in the `target/debug` or `target/release` folder.
2. Execute the binary and follow on-screen instructions for flashing operations.

### Command Overview
```bash
Usage: mechaflt <COMMAND>
Commands:
  devices  List connected USB devices
  flash    Flash an image to a device
  script   Run a script
  shell    Interactive shell
  help     Print this message or the help of the given subcommand(s)
Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Example Usage
1. To list connected USB devices:
   ```bash
   mechaflt devices
   ```

2. To flash a firmware image:
   ```bash
   mechaflt flash rootfs.tar.gz
   ```

3. To run a script:
   ```bash
   mechaflt script mecha-gen1-r5.auto
   ```

4. To start an interactive shell:
   ```bash
   mechaflt shell
   ```

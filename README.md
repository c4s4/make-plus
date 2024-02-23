# Make Plus

Tools to pimp your Makefiles.

## Installation

### Unix users (Linux, BSDs and MacOSX)

Unix users may download and install latest *make-plus* release with command:

```bash
sh -c "$(curl https://sweetohm.net/dist/make-plus/install)"
```

If *curl* is not installed on you system, you might run:

```bash
sh -c "$(wget -O - https://sweetohm.net/dist/make-plus/install)"
```

**Note:** Some directories are protected, even as *root*, on **MacOSX** (since *El Capitan* release), thus you can't install *project* in */usr/bin* for instance.

### Binary package

Otherwise, you can download latest binary archive at <https://github.com/c4s4/make-plus/releases>. Unzip the archive, put binaries for your platform somewhere in your *PATH* and rename them without platform suffix.

## Usage

There are three tools in this toolbox:

### make-help

This tool prints help on targets in current makefile and included ones recursively. Dependant targets are printed in brackets. Thus to get help about makefile in current directory, you might type:

```
$ make-help
build   Build binary [clean]
clean   Clean generated files and test cache
fmt     Format Go source code
help    Print help on Makefile
install [build]
run     Run make help [build]
```

You might also print help calling following target *help*:

```
.PHONY: help
help: # Print help on Makefile
	@make-help
```

To get help on targets in root makefile only (without parsing included ones), you can pass `--root` or `-r` option on command line. To skip help on targets without comment, you can pass `--mute` or `-m` on command line.

To get help, type on command line:

```
$ make-help --help
Print help on makefile targets

Usage: make-help [OPTIONS]

Options:
  -f, --file <FILE>  Makefile to parse
  -r, --root         Parse root makefile only
  -m, --mute         Don't print targets without description
  -h, --help         Print help
  -V, --version      Print version
```

### make-targets

This tool lists targets available in current makefile and included ones recursively. This is called to perform Bash completion. For instance, to list targets in current makefile, you might type:

```
$ make-targets
build clean fmt help install run
```

To get help, type on command line:

```
$ make-targets --help
Print list of targets

Usage: make-targets [OPTIONS]

Options:
  -f, --file <FILE>  Makefile to parse
  -r, --root         Parse root makefile only
  -m, --mute         Don't print targets without description
  -h, --help         Print help
  -V, --version      Print version
```

To enable Bash target completion on make, source following file:

```
complete -W "\`make-targets\`" make
```

## make-desc

Describe given target. You might describe *build* target it with:

```
$ make-desc build
Build binary
```

This prints the target description on command line. In a makefile, you can get current target name with *$@* and thus get its description calling `make-desc $@`.

To get help, type on command line:

```
$ make-desc --help
Describe given target

Usage: make-desc [OPTIONS] <TARGET>

Arguments:
  <TARGET>  Target to get description for

Options:
  -f, --file <FILE>  Makefile to parse
  -r, --root         Parse root makefile only
  -h, --help         Print help
  -V, --version      Print version
```

*Enjoy!*

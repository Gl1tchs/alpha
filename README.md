# Alpha | Greek alphabet for command line!

Alpha is a simple command line tool that allows you to quickly access Greek alphabet characters just by using their latina name. This tool is useful for anyone who needs to type Greek letters frequently, such as students, researchers, and linguists.

# Installation:

## Arch Linux:

You can use PKGCONFIG to install it easily on arch linux.
```bash
git clone https://github.com/username/alpha.git

cd alpha

makepkg -si
```

## Other platforms:

To install Alpha on other platforms, you can build it from source. Here are the steps:

- Clone the Alpha repository:
```bash
git clone https://github.com/username/alpha.git
```

- Change into the Alpha directory:

```bash
cd alpha
```

- Build the project:

```bash
cargo build --release
```

Install Alpha:
```bash
cargo install --path .
```

## Usage:

**To use Alpha, simply type alpha followed by the name of the Greek letter you want to type. For example, to type the Greek letter alpha (α), you can type:**

```bash
alpha alpha
# kinda confusing for that letter .-.

alpha gamma

alpha phi
```

**If you want to get available letters as a table simply type this:**
```bash
alpha --table
```

**You can get all commands available by typing:**
```bash
alpha --help
``` 

That's it! Enjoy typing in Greek with ease using Alpha.

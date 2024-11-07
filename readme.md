# 🚀 Untabify

**Untabify** is a command-line tool that removes pesky tab characters from your space-indented files while maintaining the integrity of your text alignment. It's especially handy for structured text files that require consistent spacing.

## 🌟 Features

- **Remove tabs** from space-indented files with ease.
- **Process entire directories** or individual files.
- Customize the **tab-to-space conversion** based on your preferred indentation size.

## 📥 Installation

### Scripted Installation

### Windows

```sh
powershell -c "irm https://raw.githubusercontent.com/simon-curtis/Untabify/refs/heads/main/scripts/install.ps1 | iex"
```

### Linux

```sh
# !! NOT WRITTEN YET
```

### Manual Installation

#### Step 1: Download the Binary

Head to the [releases page](https://github.com/simon-curtis/untabify/releases) and download the latest binary for your operating system.

#### Step 2: Move the Binary

- **On Windows**: Place the binary in a folder like `C:/Users/{YOUR_PROFILE}/bin/untabify[.exe]`.
  - Add this folder to your PATH if it isn’t already (so you can use `untabify` from any terminal).
- **On Linux**: Choose your preferred location for the binary and move it there. If you’re not sure where to put it, try `/usr/local/bin` or `/home/{YOUR_PROFILE}/bin`.
  - Don’t forget to add the chosen folder to your PATH if necessary!

### Step 3: Verify Installation

Once you've added the binary to your PATH, open a terminal and run:

```sh
untabify --help
```

If you see a help message, you’re all set! 🎉

### Examples

Convert tabs to spaces in all files within a directory:

```sh
untabify dir "~/test_files"
```

Convert tabs to spaces in a single file, specifying a tab size of 4 spaces:

```sh
untabify file "~/test_files/test.py" -t 4
```

## 🛠️ Building from Source

Want to tinker with the code or build Untabify from scratch? Here’s how:

Ensure you have Rust installed.

Clone the repository and navigate into the project directory.

Run the following command to build the release version:

```sh
cargo build --release
```

The compiled binary will be located in ./target/release/untabify[.exe].

To see the help message, you can then run:

```sh
./target/release/untabify[.exe] --help
```

## 🤝 Contributing

Contributions are welcome! Feel free to submit a pull request, open an issue, or provide feedback.

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.

Happy Untabifying! 😄

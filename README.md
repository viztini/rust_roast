# rust_roast

A fun command-line application written in Rust that gathers your system specifications (CPU, RAM, GPU, OS) and then delivers humorous "roasts" based on those specs.

## Features

*   **System Info:** Gathers CPU details, RAM usage, and OS information.
*   **Humorous Roasts:** Generates personalized roasts based on your system's hardware.
*   **Colored Output:** Uses ANSI colors for a visually engaging experience.

## Installation

To install `rust_roast`:

1.  **Install Rust:** If you don't have Rust installed, follow the instructions at [https://rustup.rs/](https://rustup.rs/).
2.  **Clone the GitHub repository:**
    ```bash
    git clone https://github.com/yourusername/rust_roast.git # Replace yourusername
    ```
3.  **Change into the project directory:**
    ```bash
    cd rust_roast
    ```
4.  **Run the installation script:**
    ```bash
    ./install.sh
    ```

This script will:

1.  Build the Rust project in release mode.
2.  Copy the compiled `rust_roast` executable to `~/.local/bin/`.
3.  Make the executable runnable.

## Usage

After installation, you can run `rust_roast` from any terminal:

```bash
rust_roast
```

## Contributing

Feel free to contribute to this project by submitting pull requests or opening issues.

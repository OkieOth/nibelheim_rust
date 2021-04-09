# Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# after that $HOME/.cargo/bin is needed in the path
# ... to check the installation use
cargo --version
```

# Create a new Rust project
```
# since projects can't start with a digit
cargo new --name "nbhr_kickoff" 01_kickoff
```

# Debugging in VSCode
https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/

* Install CodeLLDB extension - because we are using linux :D
* 
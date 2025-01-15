# bridi_os_2
 
I'm currently following a tutorial from [Philipp Oppermann](https://github.com/phil-opp) on building a simple OS/Kernel completely in Rust.

The idea is to build an OS somewhat functional while learning what an OS needs. While it's true that most of the current work is done in the tutorial, I'm planning to go beyond that and add my own features.

## Build and run

- You need a nighly version of the Rust toolchain (`rustup default nightly`).
- You need to install bootimage (`cargo install bootimage llvm-tools-preview`)
- Make sure to have a working QEMU installation
- Now you're good to go, just run `cargo run`, so the OS will be build and started in QEMU

[Project on Github](https://github.com/phil-opp/blog_os)

[Blog to follow step-by-step](https://os.phil-opp.com/)

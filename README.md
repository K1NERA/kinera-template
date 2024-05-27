# Kinera Parachain Template
![kinera-banner-new](https://github.com/kinera-server/kinera-parachain/assets/67910335/9172a79f-6cad-4420-b97e-c5951465c877)
Forked From Substrate Cumulus Parachain Template


1. Completely remove rustup (only if already installed):
    ```
    rustup self uninstall
    sudo apt update
    sudo apt upgrade
    ```


2. Install rust dependencies:
    ```    
    sudo apt install build-essential
    sudo apt install clang 
    sudo apt install curl
    sudo apt install git 
    sudo apt install make 
    sudo apt install --assume-yes git clang curl libssl-dev protobuf-compiler
    ```


3. Install Rust (do not "rustup update" afterwards!):
    ```    
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh
    write "2" > press enter > write "none" > press enter > "Y" > press enter
    source $HOME/.cargo/env
    ```


4. Install intended stable/nightly toolchains:
    ```    
    rustup toolchain install 1.74.0
    rustup toolchain install nightly-2023-10-31
    ```


5. Add wasm to the toolchains:
    ```
    rustup target add wasm32-unknown-unknown --toolchain 1.74.0
    rustup target add wasm32-unknown-unknown --toolchain nightly-2023-10-31
    rustup component add rust-src
    ```


6. Change to project directory and force the project to use the installed toolchains:
    ```
    git clone 
    rustup override set 1.74.0
    ```


7. Ensure you have at least 10Gb of swap (*free -h*). If not, create the swap file:
    ```    
    sudo dd if=/dev/zero of=/swap_file bs=10240 count=1M
    sudo chmod 600 /swap_file
    sudo mkswap /swap_file
    sudo swapon /swap_file
    ```    


8. Check installed version:
    ```    
    rustc --version
    ```

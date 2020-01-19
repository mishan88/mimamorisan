# Mimamorisan

## About
* Sensing something and call API
    * This repository is only edge process
* For Raspberry pi Zero WH and Raspberian

## Requirements
* Rust: 1.40+
* Raspberry pi Zero WH and Raspberian

## Build
* Add rustup target
```sh
rustup target add arm-unknown-linux-gnueabi
```

* Install Openssl
```sh
# for ubuntu
sudo apt install libssl-dev
```

* Build for Raspbery pi Zero WH
```sh
cargo build --release --target arm-unknown-linux-gnueabi
```

## Config
* Write your API Gateway URL and apikey at `MimamoriSettings.toml`
* See example `MimamoriSettings.template.toml`

## Run
* move build files to `/opt`
* move setting files (`MimamoriSettings.toml`) to `/opt`
* Add systemd service

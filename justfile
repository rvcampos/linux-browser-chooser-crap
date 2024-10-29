default:
    just -l

install-bin:
    cargo build --release
    sudo cp ./target/release/simple-browser-chooser /usr/bin/

install-desktop:
    sudo cp ./install/simple-browser-chooser.desktop  /usr/share/applications/.

install-example-file:
    mkdir -p $HOME/.config/simple-browser-chooser
    cp -n ./configuration.toml.example $HOME/.config/simple-browser-chooser/configuration.toml

install-all: install-bin install-desktop


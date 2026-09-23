# firehol-linux

`firehol-linux` runs the FireHOL list scheduler as a systemd service.

## Requirements

- Linux with systemd
- Rust toolchain, if building from source
- Network access to the configured FireHOL list URLs

## Build

From the repository root:

```sh
cargo build --release -p linux
```

The release binary is created at `target/release/firehol-differ-delta`.

## Install

To build an Ubuntu/Debian package from the repository on a Linux host:

```sh
./linux/build-deb.sh
sudo apt install ./target/debian/firehol-differ-delta_0.1.0_$(dpkg --print-architecture).deb
```

The package installs the binary and systemd unit, creates the default configuration,
and enables and starts the service. Edit the configuration after package installation
when needed:

```sh
sudoedit /etc/differ/delta/config.toml
sudo systemctl restart firehol-differ-delta.service
```

The package builder uses the host architecture reported by `dpkg`, so build it on the
Ubuntu architecture where it will run. It requires `cargo`, `dpkg-deb`, and `dpkg`.

For a manual installation without a package, run the following commands as root, or
use `sudo`:

```sh
sudo install -Dm755 target/release/firehol-differ-delta /usr/bin/firehol-differ-delta
sudo install -d -m755 /etc/differ/delta
sudo install -Dm644 config.toml /etc/differ/delta/config.toml
sudo install -Dm644 linux/firehol-differ-delta.service \
  /etc/systemd/system/firehol-differ-delta.service
```

The service uses `/etc/differ/delta` as its working directory. The configuration file
must be created before the service starts; the install command above copies the default
`config.toml` from the repository.

The TOML file must include these settings:

```toml
interval = "4h"
path = "."
l1_url = "https://iplists.firehol.org/files/firehol_level1.netset"
l2_url = "https://iplists.firehol.org/files/firehol_level2.netset"
```

Edit the configuration before starting the service if needed:

```sh
sudoedit /etc/differ/delta/config.toml
```

The `path` setting controls where generated data is written. The unit also provides
systemd-managed state and log directories at `/var/lib/firehol-differ-delta` and
`/var/log/firehol-differ-delta`.

## Start the service

```sh
sudo systemctl daemon-reload
sudo systemctl enable --now firehol-differ-delta.service
```

Check its status and logs with:

```sh
systemctl status firehol-differ-delta.service
journalctl -u firehol-differ-delta.service
```

## Stop or uninstall

```sh
sudo systemctl disable --now firehol-differ-delta.service
sudo rm /etc/systemd/system/firehol-differ-delta.service
sudo rm /usr/bin/firehol-differ-delta
sudo systemctl daemon-reload
```

Remove `/etc/differ/delta` and `/var/lib/firehol` separately if their configuration and
state are no longer required.

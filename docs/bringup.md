# Luckfox Pico RV1103

## APL extended workspaces (LIB 2)

Installed from `../../sw-vibe-coding/sw-apl-workspaces/ws` on 2026-09-28:
all nine workspace files and `library.json` are in `/root/sw-apl/ws/lib2`.
Registered as `2 EXTENDED` in `/root/.config/sw-apl/config.toml` using an
absolute library path. All transferred files passed SHA-256 verification.
APL modes 70 and 75 both listed LIB 2; loading COURSE in mode 70 and DRILL in
mode 75 succeeded. Start a new APL process to read the configuration; an
already-running `sw-apl-server` needs a restart to pick it up.

```sh
cd /root/sw-apl
./sw-apl
```

Then use `)LIB 2`, `)LOAD 2 COURSE`, and `START`.

### Mode-specific servers

Installed executable scripts in `/root/sw-apl/serve70.sh` and
`/root/sw-apl/serve75.sh`; source copies are in
[`scripts/luckfox-1103`](../scripts/).
Run either script on the board from any directory. It prints the host-side
connection command, then runs the server in the foreground; Ctrl-C stops it.

| Script | Host connection command |
|---|---|
| `/root/sw-apl/serve70.sh` | `aplterm --connect 172.32.0.93:2770` |
| `/root/sw-apl/serve75.sh` | `aplterm --connect 172.32.0.93:2775` |

Both modes can run concurrently. Each allows two sessions, uses
`/root/sw-apl` for libraries, and explicitly registers LIB 2 as EXTENDED.
Browser listeners remain on board loopback ports 8370 and 8375. Existing
servers on port 2741 are unaffected. Both scripts were launched and verified
over the line protocol: mode 70 reported (A), mode 75 reported (B), and each
listed LIB 2 correctly. Temporary validation servers were stopped after testing.

Inspected and tested on 2026-09-28. Owner identified the PCB as **Pico**, without
Mini or Plus. Linux runs on its ARM Cortex-A7, not the auxiliary RISC-V MCU.

## Verified running system

| Property | Read from board/host |
|---|---|
| OS | Buildroot 2023.02.6, `-g2ae728b52-dirty` |
| Kernel | Linux 5.10.160, #22, built Wed Jun 11 18:43:57 CST 2025 |
| Architecture | `armv7l`, ARMv7 revision 5; CPU part `0xc07` (Cortex-A7) |
| CPU features | NEON, VFPv3/VFPv4, hardware integer division |
| Hostname | `luckfox` |
| CPU/USB serial | `d21ac3e57d35f9cf` |
| Linux-visible RAM | Approximately 33 MiB total, 19 MiB available at inspection |
| Root filesystem | `/dev/root`, 5.8 GiB, about 5.6 GiB free |
| System C library | uClibc 1.0.31 loader |
| Running USB identity | `2207:0019`, `rockchip`, `rk3xxx` |
| USB interfaces | RNDIS networking and ADB |
| Host USB network interface | `enp0s20f0u11` at inspection |
| Board USB IP | `172.32.0.93` |
| Host USB IP | `172.32.0.100/24` |
| File transfer | `/usr/bin/scp` and `/usr/libexec/sftp-server`; SCP tested |

Before an OS card was installed, the board enumerated as `2207:110c` with no
network interface and no LEDs reported by the owner. After SD boot, it changed
to `2207:0019` and provided working SSH. Published physical RAM and memory
reservations were not independently measured; the table reports Linux's view.

## Image and card preparation

Used the official **standard Pico** Buildroot MicroSD package
`Luckfox_Pico_MicroSD_250607.zip`, downloaded from the MicroSD folder linked by
[Luckfox's flashing guide](https://wiki.luckfox.com/Luckfox-Pico-Plus-Mini/Flash-image/).

- Local working directory: `/disk1/tmp/luckfox-pico/`
- Archive SHA-256 (computed locally, not a vendor-published signature):
  `cd862aa10a1e5c3901f3f6739d0c4b4cb42101ed73141abb07e2808d2bd6b4a6`
- ZIP integrity test passed.
- Card capacity: 15,931,539,456 bytes (nominal 16 GB).
- Reader: Super Top SY-T18, USB `14cd:121f`.
- Reader path: `/dev/disk/by-id/usb-Mass_Storage_Device_121F20110712-0:0`.
- Card was `/dev/sdc`; device names are not permanent.

The archive contains separate images, not a raw whole-card `update.img` suitable
for direct `dd`. SocToolKit v1.85 crashed on this Arch host while loading GTK
icons; its executable bundles older GdkPixbuf and Fontconfig libraries. Its SD
dropdown also remained empty. These GUI issues were bypassed with a custom CLI.

The vendor's `programmer_image_tool` prepared the components using its emmc
output mode, matching the GUI's SD-boot conversion path. Offsets and capacities
were checked against the package's `sd_update.txt`; the environment CRC passed.

| Component | Byte offset | Partition capacity |
|---|---:|---:|
| env | 0 | 32,768 |
| idblock | 32,768 | 524,288 |
| uboot | 557,056 | 524,288 |
| boot | 1,081,344 | 33,554,432 |
| oem | 34,635,776 | 536,870,912 |
| userdata | 571,506,688 | 268,435,456 |
| rootfs | 839,942,144 | 6,442,450,944 |

Prepared CLI (temporary local artifact):

```sh
sudo python3 /disk1/tmp/luckfox-pico/flash-card.py
```

It checks reader identity, exact card capacity, mount state, manifest and image
hashes before writing, then flushes and verifies each region with progress output.
This command overwrites the card. `--check` validates local components without
opening a device. Boot and SSH access were verified after the owner moved the
prepared card into the Pico. Do not reuse this image/layout for other Pico variants.

## SSH and file transfer

On the host, if the USB interface has no IPv4 address:

```sh
sudo ip addr add 172.32.0.100/24 dev enp0s20f0u11
sudo ip link set enp0s20f0u11 up
ssh root@172.32.0.93
```

Factory password: `luckfox`. These are the
[official Buildroot login settings](https://wiki.luckfox.com/Luckfox-Pico-Plus-Mini/Login/).
Use `passwd` to change the password. The host IP configuration is temporary and
may need repeating after reconnect/reboot; check the actual interface name with
`ip -brief address`. If the address already exists, do not add it again.

## Rust CLI: built, copied, and run

Source: [hello-luckfox-1103](../hello-luckfox-1103/).

The program uses the Rust target `armv7-unknown-linux-musleabihf` with static
linking and the bundled `rust-lld` linker. Although the board's system library is
uClibc, this pure Rust executable includes its own musl runtime and does not need
the board's dynamic loader. Libraries linked dynamically against vendor APIs
would need separate ABI/toolchain consideration. Target reference:
[Rust ARM Linux support](https://doc.rust-lang.org/rustc/platform-support/arm-linux.html).

Build on this x86-64 host:

```sh
cd /disk1/github/hardwarewrighter/Foxluck-1103/hello-luckfox-1103
rustup target add armv7-unknown-linux-musleabihf
cargo build --release --offline
```

Run Cargo from the project directory so it reads `.cargo/config.toml`.
Rust 1.88.0 was used. The executable is a stripped, statically linked 32-bit ARM
EABI5 ELF, approximately 309 KiB. No external crates or vendor compiler are
required for this example.

Copy and run from the host:

```sh
scp target/armv7-unknown-linux-musleabihf/release/hello-luckfox-1103 root@172.32.0.93:/root/hello-luckfox-1103
ssh root@172.32.0.93 '/root/hello-luckfox-1103'
```

Already deployed to `/root/hello-luckfox-1103` and tested on the board:

```text
Hello, Luckfox 1103!
Rust CLI running on arm Linux
```

`--help` and `--version` succeeded; an invalid argument returned exit code 2.
`cargo fmt --check` passed. `/root` is persistent SD storage; replacing this
executable does not require reflashing the OS. No autostart entry was added.

# picst

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/yamafaktory/picst/ci.yml?branch=main&logo=github&style=flat-square)](https://github.com/yamafaktory/picst/actions/workflows/ci.yml) [![Crates.io](https://img.shields.io/crates/v/picst?style=flat-square)](https://crates.io/crates/picst)

```
██████╗ ██╗ ██████╗███████╗████████╗
██╔══██╗██║██╔════╝██╔════╝╚══██╔══╝
██████╔╝██║██║     ███████╗   ██║
██╔═══╝ ██║██║     ╚════██║   ██║
██║     ██║╚██████╗███████║   ██║
╚═╝     ╚═╝ ╚═════╝╚══════╝   ╚═╝
```

## 📷 Presentation

`picst` is a small cross-platform CLI tool aiming at making the "copy → resize → paste" image workflow super simple and more friendly.

Start `picst`, copy an image, eventually provide new dimensions if not passed via the flags, paste it anywhere - a piece of cake 🍰!

## 🛠️ Installation

### Cargo

```sh
cargo install picst
```

### Binaries

Binaries for new releases are also available [here](https://github.com/yamafaktory/picst/releases).

## ⚡️ Usage

`picst` is able to manage three different kind of unit to resize images: `percent` | `pixels` | `ratio`.

Note: `picst` will keep running and checking for new images copied in the clipboard until the process is stopped.

### Percent

Use the `--height-percent` or the `--width-percent` flags:

```sh
picst --height-percent 30
```

```sh
picst --width-percent 50
```

By default, `picst` preserves the aspect ratio of the image and will adjust the other dimension accordingly.

You can either skip this behavior with the `--ignore-aspect-ratio` flag - the tool will then prompt you for the other dimension - or you can directly set both flags:

```sh
picst --height-percent 30 --width-percent 50
```

### Pixels

Use the `--height` or the `--width` flags:

```sh
picst --height 300
```

```sh
picst --width 500
```

By default, `picst` preserves the aspect ratio of the image and will adjust the other dimension accordingly.

You can either skip this behavior with the `--ignore-aspect-ratio` flag - the tool will then prompt you for the other dimension - or you can directly set both flags:

```sh
picst --height 300 --width 500
```

### Ratio

Use the `--ratio` flag:

```sh
picst --ratio 0.7
```

### Special case: no flags

If no flags are passed to the tool, a complete wizard will be presented to you:

```sh
picst
```

```sh
Pixel --------┐
              |--->  Height* | Width* | Both --->  value(s)
Percentage ---┘

Ratio ------------>  value
```

- With **Height** and **Width**, the aspect ratio will be preserved.

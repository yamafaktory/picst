# picst

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/yamafaktory/picst/ci?style=for-the-badge) ![Crates.io](https://img.shields.io/crates/v/picst?style=for-the-badge)

## 🤔 What?

`picst` is a small CLI tool aiming at making the "copy -> resize -> paste" image workflow super simple and more friendly.

Start `picst`, copy an image, eventually provide new dimensions if not passed via the flags, paste it anywhere - a piece of cake!

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

Use the `height-percent` and `width-percent` flags:

```sh
picst --height-percent 70 --width-percent 70
```

You can also omit either the `height-percent` or the `width-percent` flag, the tool will prompt you for the missing one.

### Pixels

Use the `height` and `width` flags:

```sh
picst --height 300 --width 300
```

You can also omit either the `height` or the `width` flag, the tool will prompt you for the missing one.

### Ratio

Use the `ratio` flag:

```sh
picst --ratio 0.7
```

### Special case: no flags

With no flags, dimensions will get prompted for every new image (first the unit `percent` | `pixels` | `ratio`, then the `height` and the `width` or the `ratio` based on your choice):

```sh
picst
```

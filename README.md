# picst

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/yamafaktory/picst/ci?style=for-the-badge) ![Crates.io](https://img.shields.io/crates/v/picst?style=for-the-badge)

## 🤔 What?

`picst` is a small CLI tool aiming at making the "copy ➡️ resize ➡️ paste" image workflow super simple and more friendly.

Start `picst`, copy an image, eventually provide new dimensions if not passed via the flags, paste it anywhere - a piece of cake!

## 🛠️ Installation

### Cargo

```sh
cargo install picst
```

## ⚡️ Usage

With `height` and `width` dimensions (will be used until the process is stopped):

```sh
picst --height 300 --width 300
```

With no flags, dimensions will get prompted for every new image:

```sh
picst
```

Note: `picst` will keep running and checking for new images copied in the clipboard until the process is stopped.

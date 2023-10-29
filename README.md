# coven 🧙✨🔮

An occult chat app using [Cabal](https://cabal.chat/)

![An AI-generated image of the prompt "Watercolor drawing of a witch hat, glowing over a plain background."](https://i.imgur.com/jPwXOZrm.png)

For now is a desktop app, but with the magic of [Dioxus](https://dioxuslabs.com/) may also become a mobile app, a web app, and a terminal app.

## Develop

### Pre-requisites

https://beta.tauri.app/guides/prerequisites/

#### Linux

```shell
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

### Run

with [`just`](https://github.com/casey/just) installed:

```shell
just dev
```

(If that doesn't work, fallback to `cargo run`.)

## TODO

### features

from https://github.com/nikolaiwarner/react-cabal/issues/1

* [ ]  add detail panel for channel
* [ ]  add detail panel for user
* [ ]  add channel browser
* [ ]  add create cabal screen
* [ ]  add cabal settings screen
* [ ]  make sidebar lists closable
* [ ]  make message list a sectionlist
* [ ]  create drawer opening/closing
* [ ]  create message composer style/ux
* [ ]  can send message
* [ ]  can add channels
* [ ]  can join channels
* [ ]  can favorite channels
* [ ]  can set a channel name locally
* [ ]  can set a channel topic publicly
* [ ]  can toggle local notifications
* [ ]  can remove a cabal
* [ ]  can view and copy cabal key for sharing
* [ ]  can do moderation actions: hide/mod/admin
* [ ]  can leave channels
* [ ]  can archive channels
* [ ]  can switch cabals
* [ ]  can add a cabal
* [ ]  can join cabal with nick
* [ ]  can drag/drop to reorder cabals
* [ ]  can add/modify a custom theme
* [ ]  global keyboard commands
* [ ]  generate qr code for sharing

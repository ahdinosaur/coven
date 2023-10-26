# coven 🧙

An occult chat app using [Cabal](https://cabal.chat/) 🔮✨

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

## dev

### pre-reqs

https://beta.tauri.app/guides/prerequisites/

#### linux

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

### dev

with `just` installed:

```shell
just dev
```

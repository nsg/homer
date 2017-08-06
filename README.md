# Homer
[![Build Status](https://travis-ci.org/nsg/homer.svg?branch=master)](https://travis-ci.org/nsg/homer) [![Snap Status](https://build.snapcraft.io/badge/nsg/homer.svg)](https://build.snapcraft.io/user/nsg/homer)

Last year I started to connect various systems in my home, made a simple POC in the form of a Python based application that was both a API and a web page to control the system. Later on it expanded to a third system in the "cloud" that internet based services pushed messages to.

The code is messy, you can check it out here [autohome](https://github.com/nsg/autohome).

## Rust

Rust has been on my list-of-things-to-learn for some time, and autohome is messy and has been on my need-to-be-rewritten list ... so, why not combine these two?

## Snap

I also took the chance to try out the new [build service provided at snapcraft.io](https://build.snapcraft.io). Snaps from master are automatically built and published to snappy store.

The resulting snap is a daemon that will start automatically.

### Install

Master: `sudo snap install --edge homer-nsg`

### Manage the service

It's a normal systemd unit file called `snap.homer-nsg.homer-nsg.service`

```
# journalctl -fu snap.homer-nsg.homer-nsg.service
# systemctl status snap.homer-nsg.homer-nsg.service
```

Have fun! :)

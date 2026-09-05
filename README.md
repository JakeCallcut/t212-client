# Trading 212 API Client

<p align="center">
    <img src="https://img.shields.io/badge/Version-0.1.0-blue" alt="Version 0.1.0" />
    <img src="https://img.shields.io/badge/Rust-1.83%2B-orange" alt="Rust 1.83+" />
    <img src="https://img.shields.io/badge/Tests-Passing-green" alt="Test status" />
    <br />
</p>

<p align="center">
    <a href="https://www.trading212.com">
        <img alt="Trading212 logo" src="public/t212-tile.png" width="250" />
    </a>
</p>

A lightweight CLI client for the Trading 212 Public API, written in Rust. This is a standalone
Rust crate which implements only GET endpoints, there are no opportunities to trade in the current version of this tooling.
using the `t212` command, you can fetch account cash, positions, order history, and portfolio analytics. for personal use and for use with shell scripting.

> **N.B** This tool is unofficial and in no way affiliated with Trading 212. Only usable with the Stocks and Shares ISA account type

## Commands

- `t212 help` - display usage information 
- `t212 auth` - enter Trading212 credentials
- `t212 deauth` - delete Trading212 credentials
- `t212 status` - display connection status
- `t212 overview` - show overall portfolio information
- `t212 cash` - show cash allocation
- `t212 positions` - show open positions
- (not implemented)`t212 analytics` - show derived metrics from open positions

## Installation
Requires [Rust](https://rustup.rs) (1.83 or newer).

```bash
git clone https://github.com/JakeCallcut/t212-client
cd t212-client
cargo install --path .
```

## Setup

You need a Trading 212 API key. Generate one in the Trading 212 app under
Settings → API (Beta). Demo and live accounts have **separate** keys.

Store your credentials:

```bash
t212 auth
```

This prompts for your key and secret (the secret is hidden as you type) and
saves them to `~/.t212/creds.toml`.

Check the connection:

```bash
t212 status
```
## Project Structure
```
t212-client/
├── public/                 # logos etc.
├── src/                    # source directory
│   ├── auth.rs             # auth and deauth command logic
│   ├── config.rs           # handles environment and credentials
│   ├── http.rs             # http endpoint requests
│   ├── main.rs             # command routing
│   ├── models.rs           # data models for API responses
│   └── utils.rs            # helpers etc.
├── test/                   
├── Cargo.lock              # lockfile for dep versions
├── Cargo.toml              # crate specification
├── .gitignore              
└── README.md       
```


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

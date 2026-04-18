# Addressing Specification

## Overview

IPv8 supports multiple addressing formats for compatibility and extensibility.

## Address Types

* IPv4 (32-bit)
* IPv6 (128-bit)
* IPv8 Extended (internal representation)

## IPv8 Address Format

* Internal: binary structure
* External representation: Base64-encoded string
* Supports compression similar to IPv6 (`::`-style logic adapted)

## Structure (Conceptual)

* Prefix
* Network ID
* Interface ID
* Session Identifier (optional, for per-connection addressing)

## Features

* Dynamic per-connection addresses
* Privacy extensions (rotating identifiers)
* Dual-stack compatibility

## Notes

* Base64 is only for display/encoding, not transmission format

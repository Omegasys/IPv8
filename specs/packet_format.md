# Packet Format Specification

## Overview

IPv8 packets are designed to be modular and efficient, inspired by IPv6.

## Packet Structure

1. Header
2. Optional Extension Headers
3. Payload

## Base Header Fields

* Version (IPv8)
* Source Address
* Destination Address
* Payload Length
* Next Header
* Hop Limit
* Flow Label
* Flags (routing, encryption, checksum)

## Optional Fields

* Checksum (toggleable)
* Fragmentation info (sender-controlled)
* Routing directives

## Features

* Adjustable MTU (IPv4/IPv6 compatibility)
* No router fragmentation
* Extension header chaining

## Design Goals

* Simplicity
* Extensibility
* Performance

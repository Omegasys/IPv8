# NAT Behavior Specification

## Overview

IPv8 maintains optional NAT support for backward compatibility.

## NAT Modes

* Disabled (default for modern operation)
* Enabled (legacy compatibility)

## Behavior

* Maps internal IPv8 addresses to external IPv4/IPv6
* Maintains session tables
* Supports port translation if needed

## Use Cases

* Legacy networks
* Firewalled environments
* Transitional deployments

## Limitations

* Breaks end-to-end connectivity
* Reduces effectiveness of per-connection addressing

## Recommendation

* Prefer NAT-less operation when possible

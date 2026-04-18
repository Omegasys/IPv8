# SLAAC Specification

## Overview

Stateless Address Auto Configuration (SLAAC) enables automatic address assignment.

## Process

1. Node connects to network
2. Receives router advertisement
3. Generates address using:

   * Network prefix
   * Interface identifier
4. Applies privacy extensions if enabled

## Features

* No DHCP required
* Fast configuration
* Supports temporary addresses

## IPv8 Enhancements

* Per-session address generation
* Rotating identifiers
* Privacy-first defaults

## Notes

* Works best without NAT
* Can be combined with DHCP-like systems if needed

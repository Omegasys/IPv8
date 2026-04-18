# Proxy and Tor Integration Specification

## Overview

IPv8 supports proxy-based routing and integration with anonymity networks.

## Proxy Support

* HTTP/SOCKS proxies
* Configurable per connection
* System-wide or application-level routing

## Tor Integration

### Mode

* Optional overlay routing
* Uses external Tor network

### Behavior

* Packets are encapsulated and routed through Tor nodes
* Provides anonymity and path obfuscation

### Limitations

* Increased latency
* Dependent on external Tor infrastructure

## Routing Features

* Path randomization
* Multi-hop enforcement (overlay-controlled)

## Configuration Options

* Enable/disable proxy
* Enable/disable Tor routing
* Select routing strategy

## Notes

* Tor is not embedded into IP layer
* Implemented as overlay or adapter

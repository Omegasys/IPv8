# Encryption Specification

## Overview

IPv8 uses a layered security model combining hashing and encryption.

## Components

### 1. Hashing

* Algorithm: SHA-256
* Purpose: Data integrity and verification

### 2. Encryption

* Symmetric encryption (e.g., AES-256)
* Optional per-packet or per-session encryption

### 3. Key Exchange

* Diffie-Hellman or modern equivalents (e.g., ECDH)

### 4. IPsec Integration

* Provides authentication and encryption at network layer

## Encryption Modes

* Off (no encryption)
* Integrity-only (hashing)
* Full encryption

## Notes

* SHA-256 is not encryption
* Encryption must include secure key exchange

# AES Single Block Implementation in Rust

Supports AES-128, AES-192, and AES-256

> Written for a cyber security class at BYU. Repository created after development for archival purposes.

## Compile and Run

In order to compile, you'll have to have `rustc` installed. This can usually be installed using the default package manager.

To compile, run `make`.

To compile and run with test cases, run `make test`.

## Syntax

Rust binary:

```
./main -[e|d][v] input key type
	flags:	e (encrypt), d (decrypt), v (verbose). 'e' or 'd' must be present.
	input:	32-digit hex of either ciphertext or plaintext (omit 0x prefix)
	key:	hex of key (omit 0x prefix)
	type:	AES type, either 128, 192, or 256. Corresponds to key length in bits

# encrypt AES-128 test vector
./main -e 00112233445566778899aabbccddeeff 000102030405060708090a0b0c0d0e0f 128

# decrypt AES-192 test vector with printouts
./main -dv dda97ca4864cdfe06eaf70a0ec0d7191 000102030405060708090a0b0c0d0e0f1011121314151617 192
```

Bash test wrapper:

```
./aesTest.sh plaintext ciphertext key type
	All arguments in hex (omit 0x prefix) except 'type'
	Type is AES version (128, 192, or 256)

# tests AES-256 test vector both ways and reports success
./aesTest.sh 00112233445566778899aabbccddeeff 8ea2b7ca516745bfeafc49904b496089 000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 256
```

## Statements

I only used the resources listed above and did not use any additional material.

I pass all test cases in Appendix C. This can be verified by running `make test`.

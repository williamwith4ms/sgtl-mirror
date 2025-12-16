# *S*trin*G* *T*oo*L* (SGTL)

SGTL is a program that combines many other text tools into a unified tool. It can allow you to preform operations on strings such as base64 or cesar ciphers, and also compute various hashes such as sha256

## Install
Installation is easy, build the repo and copy the binary to your chosen bin folder.

```
git clone https://gitlab.com/williamwith4ms/sgtl
cd ./sgtl
cargo build --release
cp ./target/release/sgtl /usr/local/bin/
```

## Usage
### Running the program
```bash
# Simple echo
sgtl echo "Hello, world!"
```
### File Handling
```bash
# Read from input.txt and echo to stdout
sgtl --input input.txt echo

# Read from input.txt and write to output.txt
sgtl -i input.txt -o output.txt echo
```
### Base64 encode / decode
```bash
# Encode a string as Base64
sgtl base64 "Hello, world!"
# -> SGVsbG8sIHdvcmxkIQ==

# Decode from Base64
sgtl --decode base64 "SGVsbG8sIHdvcmxkIQ=="
# -> Hello, world!

# Base64-encode file contents
sgtl -i secrets.txt base64

# Base64-decode file contents
sgtl -d -i encoded.txt base64
```

### Hashing
```bash
# SHA-256 hash
sgtl sha256 "Hello, world!"

# SHA-512 hash
sgtl sha512 "Hello, world!"

# ... etcetera
```

> Note: --decode (-d) is not supported for any hashing commands.

```bash
# This will print an error and exit with non-zero status
sgtl --decode sha256 "something"
```

### Ciphers
Many ciphers take a secondary argument, you can see specific usage with `sgtl help <method>`

```bash
# Encipher with shift 3
sgtl caesar 3 "Hello, world!"
# -> Khoor, zruog!

# Encipher contents of a file with shift -5
sgtl -i message.txt caesar -5

# Decipher contents of a file with shift -5 and write to output file
sgtl -d -i encrypted.txt -o decrypted.txt caesar -5
```

### Debugging 
```
# verbose mode can be used to be more verbose
sgtl -v --input data.txt base64
```

## Supported Operations
Currently the following is supported 

- Echo
- Base64
- Cesar Ciphers
- Sha256
- Sha512
- Sha384
- Sha224
- Sha512_256
- Md5
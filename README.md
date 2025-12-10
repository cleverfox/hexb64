# hexb64 / b64hex

A small Rust CLI tool to convert **hex ↔ base64** using a **single binary**.

The binary detects its mode from its own filename:

- When called as **`hexb64`** → convert **hex → base64**
- When called as **`b64hex`** → convert **base64 → hex**

You can create hard links (or copies/symlinks) with these names to use both modes.

---

## Features

- Single compiled binary, multiple behaviors via hardlink name
- **Hex → Base64** (`hexb64`)
  - Accepts hex in **upper/lowercase**
  - Accepts optional **`0x`** prefix
  - Ignores whitespace
  - Outputs **classic base64** by default
  - Supports **URL-safe base64** output via `-url`
- **Base64 → Hex** (`b64hex`)
  - Accepts **classic base64** and **URL-safe base64**
  - Outputs lowercase hex by default
  - Supports **uppercase** output (`-up`)
- Input can come from:
  - A **command-line argument**, or
  - **STDIN** (if no data argument is given)
- Clean error messages to stderr, non-zero exit on failure

---

## Build

```bash
git clone https://github.com/cleverfox/hexb64
cd hexb64

# Build in release mode
cargo build --release
````

The compiled binary will be at:

```text
target/release/hexb64
```

---

## Hardlinks / Symlinks

Create two names pointing to the same binary so it can switch modes by filename.

### Linux / macOS

```bash
cd target/release

# main binary
ls hexb64

# create a hardlink for b64hex mode
ln hexb64 b64hex
```

Now you have:

* `./hexb64`  → hex → base64
* `./b64hex`  → base64 → hex

---

## Usage

### General

```text
# Base64 → hex
b64hex [-low | -up] [data]

# Hex → base64
hexb64 [-url] [data]
```

* If **`data`** is provided:

  * It is used as the input.
* If **`data`** is omitted:

  * Input is read from **stdin** until EOF.
* Whitespace in the input is ignored for parsing.

---

## Mode: `hexb64` (Hex → Base64)

### Syntax

```text
hexb64 [-url] [data]
```

* `-url`
  Output **URL-safe** base64 (using `-` and `_` instead of `+` and `/`).

### Accepted hex input forms

* Lowercase or uppercase:

  * `48656c6c6f`
  * `48656C6C6F`
* With optional `0x` / `0X` prefix:

  * `0x48656c6c6f`
  * `0X48656C6C6F`
* Whitespace inside is ignored:

  * `48 65 6c 6c 6f`

### Examples

#### Hex to classic base64

```bash
echo -n 48656c6c6f | ./hexb64
# Output: SGVsbG8=
```

```bash
echo -n 0x48656C6C6F | ./hexb64
# Output: SGVsbG8=
```

#### Hex to URL-safe base64

```bash
echo -n 48656c6c6f | ./hexb64 -url
# Output: SGVsbG8=
# (For inputs that produce '+' or '/', they will be replaced by '-' and '_'.)
```

#### Using argument instead of stdin

```bash
./hexb64 48656c6c6f
# Output: SGVsbG8=

./hexb64 -url 48656c6c6f
# Output: SGVsbG8=
```

---

## Mode: `b64hex` (Base64 → Hex)

### Syntax

```text
b64hex [-low | -up] [data]
```

* `-low`
  Output hex in **lowercase** (default).
* `-up`
  Output hex in **uppercase**.

### Supported base64 input forms

* Classic base64:

  * `SGVsbG8=`
* URL-safe base64:

  * `SGVsbG8-4_` (example)
* Whitespace in input is ignored.

The tool first attempts to decode as **classic base64**; if that fails, it retries as **URL-safe**.

### Examples

#### Base64 to lowercase hex (default)

```bash
echo -n SGVsbG8= | ./b64hex
# Output: 48656c6c6f
```

#### Base64 to uppercase hex

```bash
echo -n SGVsbG8= | ./b64hex -up
# Output: 48656C6C6F
```

#### URL-safe input

```bash
echo -n AQID-4_-AA | ./b64hex
# Decoded as URL-safe base64, prints hex output.
```

#### Using argument instead of stdin

```bash
./b64hex SGVsbG8=
# Output: 48656c6c6f

./b64hex -up SGVsbG8=
# Output: 48656C6C6F
```

---

## Error Handling

* Invalid hex (odd length, non-hex characters, etc.)
  → prints an error message to **stderr** and exits with code `1`.
* Invalid base64 in both classic and URL-safe forms
  → prints an error message to **stderr** and exits with code `1`.
* Unknown flags or invalid combinations
  → prints a short usage message and exits with code `1`.

---

## Example Workflow

Convert hex key to URL-safe base64 and back:

```bash
# Hex → URL-safe base64
HEX="00112233445566778899aabbccddeeff"
B64=$(echo -n "$HEX" | ./hexb64 -url)

echo "Base64 (URL-safe): $B64"

# Base64 → hex (lowercase)
echo -n "$B64" | ./b64hex
```

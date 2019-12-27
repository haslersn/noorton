# noorton

Maps an input port on a connected GPIO expander to an HTTP endpoint.
(Currently only `POST` requests are supported and the response is dropped.)

## Build

```bash
$ cargo build
```

### With Nix

```bash
$ nix-build
```

With Nix you can also easily cross-compile noorton:

```bash
$ nix-build '<nixpkgs>' \
    --arg crossSystem '{ config = "aarch64-unknown-linux-gnu"; }' \
    --arg overlays '[ (self: super: { noorton = super.callPackage ./. {}; }) ]' \
    -A noorton
```

## Configuration

In the working directory where noorton is executed, there must be a
`noorton.toml` configuration file.
A good start is to copy the `noorton.toml.example` from this repository.

### top-level keys

#### `expander_device =`

Path to the expander device. This is usually: `"/dev/spidev0.0"`

### [client] section

#### `endpoint =`

The HTTP endpoint to send a `POST` request to, any time a falling edge is
detected on one of the mapped input pins.

### [input_pins] section

#### `switches =`

List of input pin numbers of pins to be mapped.

# Anchor Discriminator Generator

`discriminator` is a CLI tool written in Rust that generates anchor discriminators for your instruction or struct's name from a combination of a namespace and a name. It also supports reading Solana Anchor IDL files and calculating instruction hashes.

The most common namespace `"global"` is set as default.

## Usage

The tool supports both legacy command-line usage and new subcommand-based usage.

### Subcommands

#### Generate Discriminator

Generate discriminator from a name:

```
discriminator generate [OPTIONS] <NAME>
```

Options:
- `-n, --namespace <NAMESPACE>`: Sets the namespace to use when generating the hash (default: "global")
- `-e`: Use 'event' namespace

#### Read Anchor IDL

Read Solana Anchor IDL and calculate instruction and event hashes:

```
discriminator idl [OPTIONS] --file <FILE>
```

Options:
- `-f, --file <FILE>`: Path to IDL JSON file
- `-i, --instruction <INSTRUCTION>`: Specific instruction name to calculate hash for
- `-e, --event <EVENT>`: Specific event name to calculate hash for

### Legacy Usage

For backward compatibility, you can still use the original syntax:

```
discriminator [OPTIONS] NAME
```

Options:
- `-n, --namespace <NAMESPACE>`: Sets the namespace to use when generating the hash (default: "global")
- `-e`: Use 'event' namespace

### Example Usage

#### Generate Discriminator Examples

Generate a hash using the default namespace of "global":
```
$ discriminator generate my_name
namespace: global
name: my_name
hash: [181, 16, 140, 34, 85, 113, 210, 20] 0xb5108c225571d214
b64: tRCMIlVx0hQ=
```

Generate a hash using a custom namespace:
```
$ discriminator generate -n my_namespace my_name
namespace: my_namespace
name: my_name
hash: [71, 239, 96, 91, 126, 146, 191, 3] 0x47ef605b7e92bf03
b64: R+9gW36Svw=
```

Generate a hash using namespace of "event":
```
$ discriminator generate -e BuyEvent
namespace: event
name: BuyEvent
hash: [103, 244, 82, 31, 44, 245, 119, 119] 0x67f4521f2cf57777
b64: Z/RSHyz1d3c=
```

#### IDL Processing Examples

Process all instructions and events in an IDL file:
```
$ discriminator idl -f my_program.idl.json
IDL Name: my_program
IDL Version: 0.1.0
Found 4 instructions
Found 2 events

Instruction discriminators:
--------------------------------------------------------------------------------
initialize                     | 0xafaf6d1f0d989bed | r69tHw2Ym+0=
createOrder                    | 0x8d3625cfedd2fad7 | jTYlz+3S+tc=
cancelOrder                    | 0x5f81edf00831df84 | X4Ht8Agx34Q=
swap                           | 0xf8c69e91e17587c8 | +MaekeF1h8g=

Event discriminators:
--------------------------------------------------------------------------------
OrderCreated                   | 0xe001e53ffe3cbe9f | 4AHlP/48vp8=
OrderCancelled                 | 0x6c388044a871a8ef | bDiARKhxqO8=
```

Calculate hash for a specific instruction:
```
$ discriminator idl -f my_program.idl.json -i createOrder
IDL Name: my_program
IDL Version: 0.1.0
Found 4 instructions
Found 2 events

Instruction: createOrder
namespace: global
name: createOrder
hash: [141, 54, 37, 207, 237, 210, 250, 215] 0x8d3625cfedd2fad7
b64: jTYlz+3S+tc=
```

Calculate hash for a specific event:
```
$ discriminator idl -f my_program.idl.json -e OrderCreated
IDL Name: my_program
IDL Version: 0.1.0
Found 4 instructions
Found 2 events

Event: OrderCreated
namespace: event
name: OrderCreated
hash: [224, 1, 229, 63, 254, 60, 190, 159] 0xe001e53ffe3cbe9f
b64: 4AHlP/48vp8=
```

#### Legacy Examples (Backward Compatibility)

```
$ discriminator my_name
namespace: global
name: my_name
hash: [181, 16, 140, 34, 85, 113, 210, 20] 0xb5108c225571d214
b64: tRCMIlVx0hQ=
```

```
$ discriminator -e BuyEvent
namespace: event
name: BuyEvent
hash: [103, 244, 82, 31, 44, 245, 119, 119] 0x67f4521f2cf57777
b64: Z/RSHyz1d3c=
```
## IDL File Format

The tool expects Solana Anchor IDL files in JSON format. The IDL should contain an `instructions` array with instruction objects and an optional `events` array with event objects that have at least a `name` field. 

### Supported IDL Formats

#### Standard Format
```json
{
  "version": "0.1.0",
  "name": "my_program",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [...],
      "args": [...]
    }
  ],
  "events": [
    {
      "name": "OrderCreated",
      "fields": [...]
    }
  ]
}
```

#### Metadata Format
```json
{
  "metadata": {
    "name": "my_program",
    "version": "0.1.0",
    "spec": "0.1.0"
  },
  "instructions": [...],
  "events": [...]
}
```

#### Pre-calculated Discriminators
The tool also supports IDL files with pre-calculated discriminator values:
```json
{
  "instructions": [
    {
      "name": "buy",
      "discriminator": [102, 6, 61, 18, 1, 218, 235, 234],
      "accounts": [...],
      "args": [...]
    }
  ],
  "events": [
    {
      "name": "BuyEvent",
      "discriminator": [103, 244, 82, 31, 44, 245, 119, 119],
      "fields": [...]
    }
  ]
}
```

**Note:** 
- Events use the `"event"` namespace and preserve the original casing of the event name (no snake_case conversion), while instructions use the `"global"` namespace and convert names to snake_case.
- When a `discriminator` field is present, the tool will use that value instead of calculating it from the name.
- If the `discriminator` field is missing or invalid, the tool will fall back to calculating the discriminator from the name.

## Building

To build `discriminator` from source, you'll need to have Rust installed on your machine. Once you've installed Rust, you can build the program by running the following command from the root of the project directory:

```
$ cargo build --release
```

This will compile the program and create an executable file in the `target/release` directory. To run the program, navigate to the `target/release` directory and run the executable with `./discriminator`.

## Dependencies

This tool uses the following dependencies:
- `clap`: For command-line argument parsing
- `serde` and `serde_json`: For JSON parsing
- `sha2`: For SHA-256 hashing
- `base64`: For base64 encoding
- `bs58`: For base58 encoding
- `heck`: For string case conversion

## License

This program is licensed under the [MIT License](LICENSE).

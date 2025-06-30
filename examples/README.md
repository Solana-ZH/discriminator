# Examples

This directory contains example IDL files to demonstrate the capabilities of the discriminator tool.

## Files

### `example.idl.json`
A simple example IDL file that demonstrates basic instruction and event parsing without pre-calculated discriminators. The tool will calculate discriminators from the instruction and event names.

**Usage:**
```bash
# Show all instructions and events
./discriminator idl -f examples/example.idl.json

# Show specific instruction
./discriminator idl -f examples/example.idl.json -i createOrder

# Show specific event  
./discriminator idl -f examples/example.idl.json -e OrderCreated
```

### `pump_amm_sample.json`
A sample IDL file that demonstrates mixed usage - some instructions and events have pre-calculated discriminators while others don't. This shows how the tool handles both cases.

**Features demonstrated:**
- Instructions with pre-calculated discriminators
- Events with pre-calculated discriminators  
- Events without discriminators (calculated by the tool)
- Account types with discriminators

**Usage:**
```bash
# Show all instructions and events
./discriminator idl -f examples/pump_amm_sample.json

# Show specific instruction with pre-calculated discriminator
./discriminator idl -f examples/pump_amm_sample.json -i buy

# Show specific event with pre-calculated discriminator
./discriminator idl -f examples/pump_amm_sample.json -e BuyEvent

# Show specific event without discriminator (calculated)
./discriminator idl -f examples/pump_amm_sample.json -e CreateEvent
```

### `pump_amm.json`
A real-world IDL file from a Solana program that uses the metadata format and includes pre-calculated discriminators. This demonstrates the tool's ability to handle production IDL files.

**Features demonstrated:**
- Metadata format with program info in `metadata` object
- Pre-calculated discriminators for all instructions and events
- Large number of instructions and events
- Real-world Solana program structure

**Usage:**
```bash
# Show all instructions and events
./discriminator idl -f examples/pump_amm.json

# Show specific instruction
./discriminator idl -f examples/pump_amm.json -i buy

# Show specific event
./discriminator idl -f examples/pump_amm.json -e BuyEvent
```

## Expected Output Format

The tool outputs discriminators in three formats:
- **Hex**: `0x66063d1201daebea` - Standard hexadecimal representation
- **Array**: `[102, 6, 61, 18, 1, 218, 235, 234]` - Byte array format
- **Base64**: `ZgY9EgHa6+o=` - Base64 encoded format

## Discriminator Calculation Rules

1. **Instructions**: Use `global` namespace and convert names to snake_case
2. **Events**: Use `event` namespace and preserve original casing
3. **Pre-calculated**: When a `discriminator` field exists, use that value instead of calculating

## Testing Different Scenarios

You can test various scenarios using these examples:

```bash
# Compare calculated vs pre-calculated discriminators
./discriminator generate -e BuyEvent
./discriminator idl -f examples/pump_amm.json -e BuyEvent

# Test instruction name conversion
./discriminator generate createOrder    # snake_case conversion
./discriminator generate create_order   # already snake_case

# Test different IDL formats
./discriminator idl -f examples/example.idl.json        # Standard format
./discriminator idl -f examples/pump_amm.json          # Metadata format
```

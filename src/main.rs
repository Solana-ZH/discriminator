use base64::{engine::general_purpose, Engine as _};
use clap::{Parser, Subcommand};
use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "discriminator")]
#[command(about = "A CLI tool for generating Anchor discriminators")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    // Legacy support for direct invocation
    #[arg(help = "Name to generate discriminator for (legacy mode)")]
    name: Option<String>,

    #[arg(
        short = 'n',
        long = "namespace",
        default_value = "global",
        help = "Namespace to use"
    )]
    namespace: Option<String>,

    #[arg(short = 'e', help = "Use 'event' namespace")]
    event: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate discriminator from name (explicit command)
    Generate {
        /// Name to generate discriminator for
        name: String,

        #[arg(
            short = 'n',
            long = "namespace",
            default_value = "global",
            help = "Namespace to use"
        )]
        namespace: String,

        #[arg(short = 'e', help = "Use 'event' namespace")]
        event: bool,
    },

    /// Read Solana Anchor IDL and calculate instruction hashes
    Idl {
        /// Path to the IDL JSON file
        #[arg(short = 'f', long = "file", help = "Path to IDL JSON file")]
        file: PathBuf,

        #[arg(
            short = 'i',
            long = "instruction",
            help = "Specific instruction name to calculate hash for"
        )]
        instruction: Option<String>,

        #[arg(
            short = 'e',
            long = "event",
            help = "Specific event name to calculate hash for"
        )]
        event: Option<String>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct IdlInstruction {
    name: String,
    #[serde(default)]
    discriminator: Option<Vec<u8>>,
    #[serde(default)]
    accounts: Vec<serde_json::Value>,
    #[serde(default)]
    args: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct IdlEvent {
    name: String,
    #[serde(default)]
    discriminator: Option<Vec<u8>>,
    #[serde(default)]
    fields: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct IdlMetadata {
    name: String,
    version: String,
    #[serde(default)]
    spec: String,
    #[serde(default)]
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Idl {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    metadata: Option<IdlMetadata>,
    instructions: Vec<IdlInstruction>,
    #[serde(default)]
    accounts: Vec<serde_json::Value>,
    #[serde(default)]
    types: Vec<serde_json::Value>,
    #[serde(default)]
    events: Vec<IdlEvent>,
}

struct ByteArray<'a>(&'a [u8; 8]);

impl<'a> fmt::LowerHex for ByteArray<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

fn bytes_to_hex(bytes: &[u8; 8]) -> String {
    format!("{:x}", ByteArray(bytes))
}

pub fn get_hash(namespace: &str, name: &str, event: bool) -> [u8; 8] {
    let namehash = if event {
        name.to_string()
    } else {
        name.to_snake_case()
    };

    let preimage = format!("{}:{}", namespace, namehash);
    let mut hasher = Sha256::new();
    hasher.update(preimage.as_bytes());
    let hash_result = hasher.finalize();

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&hash_result[..8]);
    sighash
}

fn print_discriminator(namespace: &str, name: &str, hash: [u8; 8]) {
    let hex = bytes_to_hex(&hash);
    println!("namespace: {}", namespace);
    println!("name: {}", name);
    println!("hash: {:?} 0x{}", hash, hex);
    println!("b64: {}", general_purpose::STANDARD.encode(hash));
}

fn handle_idl_command(
    file: PathBuf,
    instruction_name: Option<String>,
    event_name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read and parse IDL file
    let idl_content = fs::read_to_string(&file)
        .map_err(|e| format!("Failed to read IDL file '{}': {}", file.display(), e))?;

    let idl: Idl = serde_json::from_str(&idl_content)
        .map_err(|e| format!("Failed to parse IDL JSON: {}", e))?;

    let name = if let Some(ref metadata) = idl.metadata {
        &metadata.name
    } else if let Some(ref name) = idl.name {
        name
    } else {
        "Unknown"
    };

    let version = if let Some(ref metadata) = idl.metadata {
        &metadata.version
    } else if let Some(ref version) = idl.version {
        version
    } else {
        "Unknown"
    };

    println!("IDL Name: {}", name);
    println!("IDL Version: {}", version);
    println!("Found {} instructions", idl.instructions.len());
    println!("Found {} events", idl.events.len());
    println!();

    if let Some(target_instruction) = instruction_name {
        // Calculate hash for specific instruction
        if let Some(instruction) = idl
            .instructions
            .iter()
            .find(|i| i.name == target_instruction)
        {
            let hash = if let Some(ref discriminator) = instruction.discriminator {
                if discriminator.len() >= 8 {
                    let mut hash_array = [0u8; 8];
                    hash_array.copy_from_slice(&discriminator[..8]);
                    hash_array
                } else {
                    get_hash("global", &instruction.name, false)
                }
            } else {
                get_hash("global", &instruction.name, false)
            };
            println!("Instruction: {}", instruction.name);
            print_discriminator("global", &instruction.name, hash);
        } else {
            println!("Instruction '{}' not found in IDL", target_instruction);
            println!("Available instructions:");
            for instruction in &idl.instructions {
                println!("  - {}", instruction.name);
            }
            return Err("Instruction not found".into());
        }
    } else if let Some(target_event) = event_name {
        // Calculate hash for specific event
        if let Some(event) = idl.events.iter().find(|e| e.name == target_event) {
            let hash = if let Some(ref discriminator) = event.discriminator {
                if discriminator.len() >= 8 {
                    let mut hash_array = [0u8; 8];
                    hash_array.copy_from_slice(&discriminator[..8]);
                    hash_array
                } else {
                    get_hash("event", &event.name, true)
                }
            } else {
                get_hash("event", &event.name, true)
            };
            println!("Event: {}", event.name);
            print_discriminator("event", &event.name, hash);
        } else {
            println!("Event '{}' not found in IDL", target_event);
            println!("Available events:");
            for event in &idl.events {
                println!("  - {}", event.name);
            }
            return Err("Event not found".into());
        }
    } else {
        // Calculate hashes for all instructions
        println!("Instruction discriminators:");
        println!("{:-<80}", "");

        for instruction in &idl.instructions {
            let hash = if let Some(ref discriminator) = instruction.discriminator {
                if discriminator.len() >= 8 {
                    let mut hash_array = [0u8; 8];
                    hash_array.copy_from_slice(&discriminator[..8]);
                    hash_array
                } else {
                    get_hash("global", &instruction.name, false)
                }
            } else {
                get_hash("global", &instruction.name, false)
            };
            let hex = bytes_to_hex(&hash);
            println!(
                "{:<30} | 0x{} | {}",
                instruction.name,
                hex,
                general_purpose::STANDARD.encode(hash)
            );
        }

        // Calculate hashes for all events
        if !idl.events.is_empty() {
            println!();
            println!("Event discriminators:");
            println!("{:-<80}", "");

            for event in &idl.events {
                let hash = if let Some(ref discriminator) = event.discriminator {
                    if discriminator.len() >= 8 {
                        let mut hash_array = [0u8; 8];
                        hash_array.copy_from_slice(&discriminator[..8]);
                        hash_array
                    } else {
                        get_hash("event", &event.name, true)
                    }
                } else {
                    get_hash("event", &event.name, true)
                };
                let hex = bytes_to_hex(&hash);
                println!(
                    "{:<30} | 0x{} | {}",
                    event.name,
                    hex,
                    general_purpose::STANDARD.encode(hash)
                );
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Generate {
            name,
            namespace,
            event,
        }) => {
            let actual_namespace = if event { "event" } else { &namespace };
            let hash = get_hash(actual_namespace, &name, event);
            print_discriminator(actual_namespace, &name, hash);
        }

        Some(Commands::Idl {
            file,
            instruction,
            event,
        }) => {
            handle_idl_command(file, instruction, event)?;
        }

        None => {
            // Legacy mode - handle old-style arguments
            if let Some(name) = cli.name {
                let namespace = if cli.event {
                    "event"
                } else {
                    cli.namespace.as_deref().unwrap_or("global")
                };

                let hash = get_hash(namespace, &name, cli.event);
                print_discriminator(namespace, &name, hash);
            } else {
                // No arguments provided, show help
                println!("No arguments provided. Use --help for usage information.");
                return Ok(());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hash_global_namespace() {
        let hash = get_hash("global", "my_name", false);
        assert_eq!(hash, [181, 16, 140, 34, 85, 113, 210, 20]);
    }

    #[test]
    fn test_get_hash_event_namespace() {
        let hash = get_hash("event", "BuyEvent", true);
        assert_eq!(hash, [103, 244, 82, 31, 44, 245, 119, 119]);
    }

    #[test]
    fn test_get_hash_custom_namespace() {
        let hash = get_hash("my_namespace", "my_name", false);
        assert_eq!(hash, [183, 58, 214, 211, 174, 61, 243, 178]);
    }

    #[test]
    fn test_get_hash_snake_case_conversion() {
        let hash1 = get_hash("global", "createOrder", false);
        let hash2 = get_hash("global", "create_order", false);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_bytes_to_hex() {
        let bytes = [181, 16, 140, 34, 85, 113, 210, 20];
        let hex = bytes_to_hex(&bytes);
        assert_eq!(hex, "b5108c225571d214");
    }

    #[test]
    fn test_get_hash_event_order_created() {
        let hash = get_hash("event", "OrderCreated", true);
        assert_eq!(hash, [224, 1, 229, 63, 254, 60, 190, 159]);
    }

    #[test]
    fn test_get_hash_event_order_cancelled() {
        let hash = get_hash("event", "OrderCancelled", true);
        assert_eq!(hash, [108, 56, 128, 68, 168, 113, 168, 239]);
    }

    #[test]
    fn test_event_vs_non_event_namespace() {
        let hash_event = get_hash("event", "MyEvent", true);
        let hash_global = get_hash("global", "MyEvent", false);
        // These should be different because event uses the name as-is,
        // while global converts to snake_case
        assert_ne!(hash_event, hash_global);
    }

    #[test]
    fn test_parse_idl_with_discriminator() {
        let idl_json = r#"{
            "metadata": {
                "name": "test_program",
                "version": "0.1.0"
            },
            "instructions": [
                {
                    "name": "buy",
                    "discriminator": [102, 6, 61, 18, 1, 218, 235, 234]
                }
            ],
            "events": [
                {
                    "name": "BuyEvent",
                    "discriminator": [103, 244, 82, 31, 44, 245, 119, 119]
                }
            ],
            "accounts": [],
            "types": []
        }"#;

        let idl: Idl = serde_json::from_str(idl_json).unwrap();
        assert_eq!(idl.metadata.as_ref().unwrap().name, "test_program");
        assert_eq!(idl.instructions.len(), 1);
        assert_eq!(idl.events.len(), 1);

        let instruction = &idl.instructions[0];
        assert_eq!(instruction.name, "buy");
        assert_eq!(
            instruction.discriminator.as_ref().unwrap(),
            &vec![102, 6, 61, 18, 1, 218, 235, 234]
        );

        let event = &idl.events[0];
        assert_eq!(event.name, "BuyEvent");
        assert_eq!(
            event.discriminator.as_ref().unwrap(),
            &vec![103, 244, 82, 31, 44, 245, 119, 119]
        );
    }

    #[test]
    fn test_parse_idl_without_discriminator() {
        let idl_json = r#"{
            "name": "test_program",
            "version": "0.1.0",
            "instructions": [
                {
                    "name": "initialize"
                }
            ],
            "events": [
                {
                    "name": "CreateEvent"
                }
            ],
            "accounts": [],
            "types": []
        }"#;

        let idl: Idl = serde_json::from_str(idl_json).unwrap();
        assert_eq!(idl.name.as_ref().unwrap(), "test_program");

        let instruction = &idl.instructions[0];
        assert_eq!(instruction.name, "initialize");
        assert!(instruction.discriminator.is_none());

        let event = &idl.events[0];
        assert_eq!(event.name, "CreateEvent");
        assert!(event.discriminator.is_none());
    }
}

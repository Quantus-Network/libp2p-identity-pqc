use libp2p_identity::Keypair;
use std::env;
use std::fs;
use std::process;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <protobuf_file> <output_file>", args[0]);
        eprintln!("Converts a libp2p keypair from protobuf encoding to hex string");
        eprintln!("Writes the bytes to the output file and prints hex to stdout");
        process::exit(1);
    }

    let filename = &args[1];
    let output_filename = &args[2];

    // Read the protobuf file
    let protobuf_bytes = match fs::read(filename) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    // Decode the keypair from protobuf encoding
    let keypair = match Keypair::from_protobuf_encoding(&protobuf_bytes) {
        Ok(keypair) => keypair,
        Err(e) => {
            eprintln!("Error decoding keypair from protobuf: {}", e);
            process::exit(1);
        }
    };

    // Convert to bytes using dilithium_to_bytes
    let keypair_bytes = keypair.dilithium_to_bytes();

    // Write bytes to output file
    if let Err(e) = fs::write(output_filename, &keypair_bytes) {
        eprintln!("Error writing to file '{}': {}", output_filename, e);
        process::exit(1);
    }

    // Convert to hex string
    let hex_string = hex::encode(&keypair_bytes);

    println!("{}", hex_string);
}

#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
//#![no_std] // std support is experimental
use risc0_zkvm::guest::env;
use std::collections::HashMap;

risc0_zkvm::guest::entry!(main);

fn main() {
    let mut groups: HashMap<String, (u32, u32)> = HashMap::new();

    groups.insert("GenderX".to_string(), (1, 1234));
    groups.insert("CompanyX".to_string(), (2, 4567));
    groups.insert("GroupX".to_string(), (3, 7890));

    let input: u32 = env::read();
    let mut output: (String, u32) = ("Not registerd as member of any group".to_string(), 0);

    for (group, values) in &groups {
        if values.1 == input {
            output = (group.clone(), values.0);
            break;
        }
    }

    env::commit(&output);
}

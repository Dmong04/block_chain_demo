/***
 *
 * It demonstrates how tampering with a block in the blockchain invalidates the entire chain.
 * showing that the blockchain is tamper-evident.
 *
 * run with:
 * cargo run --example tamper_demo
 */

use block_chain_demo::Blockchain;

fn main() {
    let mut chain = Blockchain::new(3);
    chain.add_block("Alice paga 10 monedas a Bob").unwrap();
    chain.add_block("Bob paga 3 monedas a Carol").unwrap();

    println!("Cadena original válida: {}", chain.is_valid());

    // update the data of the second block (index 1) to simulate tampering
    chain.blocks_mut()[1].data = "Alice paga 10000 monedas a Bob".to_string();

    println!("Cadena tras manipular el bloque 1: {}", chain.is_valid());

    match chain.validate() {
        Ok(()) => println!("La cadena sigue siendo válida (esto no debería pasar)."),
        Err(e) => println!("Detectado: {e}"),
    }
}

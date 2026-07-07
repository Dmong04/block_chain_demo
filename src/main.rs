use block_chain_demo::Blockchain;

fn main() {
    let difficulty = 4;

    println!("Creando la blockchain (dificultad = {difficulty})...\n");
    let mut chain = Blockchain::new(difficulty);

    for tx in [
        "Alice paga 10 monedas a Bob",
        "Bob paga 3 monedas a Carol",
        "Carol paga 7 monedas a Dave",
    ] {
        println!("Minando bloque {}: \"{tx}\"", chain.len());
        if let Err(e) = chain.add_block(tx) {
            eprintln!("Error al minar el bloque: {e}");
            std::process::exit(1);
        }
    }

    println!("\nCadena resultante:\n");
    print_chain(&chain);

    match chain.validate() {
        Ok(()) => println!("\n✅ La cadena es válida."),
        Err(e) => println!("\n❌ La cadena NO es válida: {e}"),
    }

    println!("\n(Para ver una demo de detección de manipulación, ejecuta:");
    println!(" cargo run --example tamper_demo)");

    // Persistencia: guardamos y recargamos la cadena.
    let path = std::env::temp_dir().join("mini_blockchain_demo.json");
    if let Err(e) = chain.save_to_file(&path) {
        eprintln!("No se pudo guardar la cadena: {e}");
        return;
    }
    println!("\nCadena guardada en: {}", path.display());

    match Blockchain::load_from_file(&path, difficulty) {
        Ok(loaded) => println!(
            "Cadena recargada desde disco: {} bloques, válida = {}",
            loaded.len(),
            loaded.is_valid()
        ),
        Err(e) => eprintln!("No se pudo cargar la cadena: {e}"),
    }
}

fn print_chain(chain: &Blockchain) {
    for block in chain.blocks() {
        println!("-----------------------------------");
        println!("Índice:        {}", block.index);
        println!("Timestamp:     {}", block.timestamp);
        println!("Datos:         {}", block.data);
        println!("Hash anterior: {}", block.previous_hash);
        println!("Hash:          {}", block.hash);
        println!("Nonce:         {}", block.nonce);
    }
    println!("-----------------------------------");
}

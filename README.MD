# mini_blockchain

Implementación educativa de una blockchain minimalista en Rust, con estructura
de proyecto profesional: separación en módulos, manejo de errores propio,
tests unitarios y de integración, ejemplos ejecutables y documentación.

## Características

- **Bloques encadenados por hash SHA-256**, cada uno referenciando el hash del anterior.
- **Proof-of-work simplificado**: minería por búsqueda de nonce hasta cumplir una dificultad configurable.
- **Validación completa de la cadena**: detecta tanto hashes alterados como enlaces rotos entre bloques.
- **Persistencia**: guardar/cargar la cadena como JSON.
- **Manejo de errores tipado** con [`thiserror`], sin `unwrap`/`panic!` en la lógica de negocio.

## Estructura del proyecto

```
mini_blockchain/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Punto de entrada de la biblioteca, re-exporta la API pública
│   ├── block.rs         # Struct Block: creación, minería, hash, validación individual
│   ├── blockchain.rs    # Struct Blockchain: cadena completa, validación, persistencia
│   ├── errors.rs        # Enum BlockchainError con todos los errores posibles
│   └── main.rs          # Binario de demostración (usa la biblioteca)
├── examples/
│   └── tamper_demo.rs   # Demo de detección de manipulación de datos
└── tests/
    └── integration_test.rs  # Tests de integración usando la API pública
```

## Uso como biblioteca

```rust
use mini_blockchain::Blockchain;

fn main() {
    let mut chain = Blockchain::new(2); // dificultad = 2 ceros iniciales

    chain.add_block("Alice paga 10 a Bob").unwrap();
    chain.add_block("Bob paga 3 a Carol").unwrap();

    assert!(chain.is_valid());

    chain.save_to_file("chain.json").unwrap();
}
```

## Comandos

```bash
# Compilar
cargo build --release

# Ejecutar la demo principal
cargo run --release

# Ejecutar el ejemplo de detección de manipulación
cargo run --example tamper_demo

# Correr todos los tests (unitarios + integración)
cargo test

# Generar y abrir la documentación (con los comentarios /// y //!)
cargo doc --open
```

## Diseño y decisiones

- **Errores como valores, no pánico**: toda operación que puede fallar devuelve
  `Result<T, BlockchainError>` en lugar de hacer `panic!`. Esto hace que la
  biblioteca sea segura de usar en contextos donde no se quiere que el proceso
  entero se caiga por un dato inválido.
- **`Block` no conoce a `Blockchain`**: cada bloque puede validarse a sí mismo
  de forma aislada (`Block::validate`), y `Blockchain::validate` añade la
  verificación de los enlaces entre bloques. Esta separación de
  responsabilidades facilita testear cada nivel por separado.
- **Serialización con `serde`**: permite persistir la cadena a JSON sin
  escribir un (de)serializador a mano.

## Posibles extensiones

- Firmas digitales (ed25519) para autenticar quién crea cada transacción.
- Ajuste dinámico de dificultad según el tiempo de minado.
- Una API HTTP (p. ej. con `axum`) para exponer la cadena en red.
- Persistencia incremental (append-only) en vez de reescribir todo el JSON.

## Licencia

MIT

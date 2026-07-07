use thiserror::Error;

/// Errores que pueden ocurrir al operar sobre la blockchain.
#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error("el bloque {index} tiene un hash inválido (no coincide con su contenido)")]
    InvalidHash { index: u64 },

    #[error("el bloque {index} no enlaza correctamente con el bloque anterior")]
    BrokenLink { index: u64 },

    #[error("el hash del bloque {index} no cumple la dificultad requerida ({difficulty} ceros)")]
    DifficultyNotMet { index: u64, difficulty: usize },

    #[error("la cadena está vacía, no se puede validar ni extender")]
    EmptyChain,

    #[error("error de (de)serialización: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("error de entrada/salida: {0}")]
    Io(#[from] std::io::Error),
}

/// Alias de conveniencia para resultados de esta biblioteca.
pub type Result<T> = std::result::Result<T, BlockchainError>;

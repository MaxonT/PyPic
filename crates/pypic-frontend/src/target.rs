//! Perfil de destino visto por el frontend.
//!
//! PyPIC no es solo para PIC de 8 bits: cada familia/MCU tiene su ancho de
//! palabra y capacidades. El frontend solo consulta este perfil; mas adelante
//! lo construira el Device Pack del dispositivo elegido.

use crate::types::Type;

#[derive(Clone, Debug)]
pub struct TargetProfile {
    pub name: String,
    /// Ancho maximo de entero que el destino admite, en bits.
    pub max_int_bits: u32,
}

impl TargetProfile {
    pub fn generic() -> Self {
        TargetProfile { name: "generic".into(), max_int_bits: 32 }
    }

    pub fn supports(&self, ty: Type) -> bool {
        !ty.is_int() || ty.bits() <= self.max_int_bits
    }
}

impl Default for TargetProfile {
    fn default() -> Self {
        Self::generic()
    }
}

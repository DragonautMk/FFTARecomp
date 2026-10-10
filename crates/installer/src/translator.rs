//! Traductor ROM -> .lex integrado en el instalador.
//!
//! PUNTO DE CONEXION: `translate()`. Hoy devuelve un modulo wasm vacio y
//! valido (placeholder). Aqui se llamara al recompilador como libreria
//! (el crate `recomp`) para emitir el wasm real.

use sha1::{Digest, Sha1};

pub const MAGIC: &[u8; 4] = b"LEX1";
pub const FORMAT_VERSION: u16 = 1;

pub struct LexHeader {
    pub region: [u8; 4],
    pub rom_sha1: [u8; 20],
}

/// Traduce la ROM a un modulo WebAssembly.
/// `progress(fraccion 0..1, texto)` se llama para actualizar la UI.
pub fn translate(rom: &[u8], progress: &mut dyn FnMut(f32, &str)) -> Result<Vec<u8>, String> {
    progress(0.15, "Analizando ROM...");
    if rom.len() < 0xC0 {
        return Err("ROM demasiado pequena.".into());
    }

    // TODO: reemplazar por la llamada real al recompilador, p. ej.:
    //   let wasm = recomp::translate_to_wasm(rom, |p, s| progress(0.15 + 0.7 * p, s))?;
    progress(0.5, "Traduciendo codigo...");
    let wasm = placeholder_wasm();

    progress(0.85, "Empaquetando...");
    Ok(wasm)
}

/// Modulo wasm minimo valido: magic "\0asm" + version 1, sin secciones.
fn placeholder_wasm() -> Vec<u8> {
    vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]
}

/// Formato .lex:
///   "LEX1" | version u16 LE | region [4] | sha1 de la ROM [20] | len u64 LE | payload wasm
/// El payload NUNCA incluye la ROM, solo el codigo traducido.
pub fn build_lex(rom: &[u8], payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(payload.len() + 40);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    out.extend_from_slice(&rom[0xAC..0xB0]);
    out.extend_from_slice(&Sha1::digest(rom));
    out.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    out.extend_from_slice(payload);
    out
}

/// Lectura para la app (exe/apk): valida cabecera y devuelve el wasm.
#[allow(dead_code)]
pub fn read_lex(bytes: &[u8]) -> Result<(LexHeader, &[u8]), String> {
    if bytes.len() < 38 || &bytes[0..4] != MAGIC {
        return Err("No es un archivo .lex valido.".into());
    }
    let ver = u16::from_le_bytes([bytes[4], bytes[5]]);
    if ver != FORMAT_VERSION {
        return Err(format!("Version de .lex no soportada: {ver}"));
    }
    let mut region = [0u8; 4];
    region.copy_from_slice(&bytes[6..10]);
    let mut rom_sha1 = [0u8; 20];
    rom_sha1.copy_from_slice(&bytes[10..30]);
    let len = u64::from_le_bytes(bytes[30..38].try_into().unwrap()) as usize;
    let payload = bytes.get(38..38 + len).ok_or("Archivo .lex truncado.")?;
    Ok((LexHeader { region, rom_sha1 }, payload))
}

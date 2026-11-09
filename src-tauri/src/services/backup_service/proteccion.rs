use super::{err, io, reemplazar, R};
use std::{fs, path::Path};
use zeroize::Zeroizing;

pub fn guardar(p: &Path, frase: &str) -> R<()> {
    if !(16..=128).contains(&frase.chars().count()) {
        return Err(err("La frase debe tener entre 16 y 128 caracteres."));
    }
    let cifrado = plataforma::transformar(frase.as_bytes(), true)?;
    reemplazar(p, &cifrado)
}
pub fn leer(p: &Path) -> R<Zeroizing<String>> {
    if io(fs::metadata(p))?.len() > 64 * 1024 {
        return Err(err("Protección local inválida."));
    }
    let blob = io(fs::read(p))?;
    let claro = plataforma::transformar(&blob, false)?;
    Ok(Zeroizing::new(
        std::str::from_utf8(&claro)
            .map_err(|_| err("La frase local no se pudo recuperar."))?
            .to_owned(),
    ))
}

#[cfg(windows)]
mod plataforma {
    use super::*;
    use std::{ffi::c_void, ptr};
    use zeroize::Zeroize;

    #[repr(C)]
    struct Blob {
        len: u32,
        data: *mut u8,
    }
    #[link(name = "crypt32")]
    unsafe extern "system" {
        fn CryptProtectData(
            input: *const Blob,
            desc: *const u16,
            entropy: *const Blob,
            reserved: *mut c_void,
            prompt: *mut c_void,
            flags: u32,
            output: *mut Blob,
        ) -> i32;
        fn CryptUnprotectData(
            input: *const Blob,
            desc: *mut *mut u16,
            entropy: *const Blob,
            reserved: *mut c_void,
            prompt: *mut c_void,
            flags: u32,
            output: *mut Blob,
        ) -> i32;
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn LocalFree(ptr: *mut c_void) -> *mut c_void;
    }
    pub fn transformar(bytes: &[u8], cifrar: bool) -> R<Zeroizing<Vec<u8>>> {
        let entrada = Blob {
            len: bytes.len() as u32,
            data: bytes.as_ptr().cast_mut(),
        };
        let mut salida = Blob {
            len: 0,
            data: ptr::null_mut(),
        };
        // DPAPI ligado al perfil actual. UI_FORBIDDEN evita diálogos del SO en tareas automáticas.
        let ok = unsafe {
            if cifrar {
                CryptProtectData(
                    &entrada,
                    ptr::null(),
                    ptr::null(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    1,
                    &mut salida,
                )
            } else {
                CryptUnprotectData(
                    &entrada,
                    ptr::null_mut(),
                    ptr::null(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    1,
                    &mut salida,
                )
            }
        };
        if ok == 0 || salida.data.is_null() {
            return Err(err(
                "Windows no pudo abrir o proteger la frase en este perfil.",
            ));
        }
        let data = unsafe { std::slice::from_raw_parts_mut(salida.data, salida.len as usize) };
        let result = Zeroizing::new(data.to_vec());
        data.zeroize();
        unsafe {
            LocalFree(salida.data.cast());
        }
        Ok(result)
    }
}
#[cfg(not(windows))]
mod plataforma {
    use super::*;
    pub fn transformar(_bytes: &[u8], _cifrar: bool) -> R<Zeroizing<Vec<u8>>> {
        Err(err(
            "La protección local de la frase requiere DPAPI de Windows.",
        ))
    }
}

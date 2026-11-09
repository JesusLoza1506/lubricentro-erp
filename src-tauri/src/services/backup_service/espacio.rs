use super::{err, io, R};
use std::path::Path;

#[cfg(windows)]
fn libres(path: &Path) -> std::io::Result<u64> {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetDiskFreeSpaceExW(
            dir: *const u16,
            libre: *mut u64,
            total: *mut u64,
            global: *mut u64,
        ) -> i32;
    }
    let dir: Vec<_> = path.as_os_str().encode_wide().chain([0]).collect();
    let mut disponible = 0;
    if unsafe {
        GetDiskFreeSpaceExW(
            dir.as_ptr(),
            &mut disponible,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    } == 0
    {
        return Err(std::io::Error::last_os_error());
    }
    Ok(disponible)
}

#[cfg(unix)]
fn libres(path: &Path) -> std::io::Result<u64> {
    use std::os::unix::ffi::OsStrExt;
    let dir = std::ffi::CString::new(path.as_os_str().as_bytes())?;
    let mut v = std::mem::MaybeUninit::<libc::statvfs>::uninit();
    if unsafe { libc::statvfs(dir.as_ptr(), v.as_mut_ptr()) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    let v = unsafe { v.assume_init() };
    let bytes = u128::from(v.f_bavail) * u128::from(v.f_frsize);
    Ok(u64::try_from(bytes).unwrap_or(u64::MAX))
}

#[cfg(not(any(unix, windows)))]
fn libres(_path: &Path) -> std::io::Result<u64> {
    Err(std::io::ErrorKind::Unsupported.into())
}

pub(super) fn exigir(root: &Path, bytes: u64) -> R<()> {
    // La suma es conservadora: previas, preparación y catálogos pueden coexistir en un volumen.
    for dir in ["temporales", "generaciones", "copias"] {
        comprobar(io(libres(&root.join(dir)))?, bytes)?;
    }
    Ok(())
}

pub(super) fn comprobar(disponible: u64, bytes: u64) -> R<()> {
    if disponible < bytes {
        let mib = 1024 * 1024;
        return Err(err(&format!(
            "Espacio insuficiente para preparar la restauración: se requieren al menos {} MiB y hay {} MiB libres. La base activa no se cambió.",
            bytes.div_ceil(mib), disponible / mib,
        )));
    }
    Ok(())
}

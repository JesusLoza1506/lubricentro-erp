// Native selectors are used only for paths; no shell command or browser URL is accepted.
use super::{err, R};

#[cfg(not(windows))]
pub fn elegir_ruta(_carpeta: bool) -> R<Option<String>> {
    Err(err("El selector de respaldos está disponible en Windows."))
}

#[cfg(windows)]
pub fn elegir_ruta(carpeta: bool) -> R<Option<String>> {
    use std::{ffi::c_void, mem, ptr};
    type Ptr = *mut c_void;
    #[repr(C)]
    struct OpenFileName {
        size: u32,
        owner: Ptr,
        instance: Ptr,
        filter: *const u16,
        custom_filter: *mut u16,
        max_custom: u32,
        index: u32,
        file: *mut u16,
        max_file: u32,
        file_title: *mut u16,
        max_file_title: u32,
        initial_dir: *const u16,
        title: *const u16,
        flags: u32,
        file_offset: u16,
        extension_offset: u16,
        default_extension: *const u16,
        data: isize,
        hook: Ptr,
        template: *const u16,
        reserved: Ptr,
        reserved_int: u32,
        flags_ex: u32,
    }
    #[repr(C)]
    struct BrowseInfo {
        owner: Ptr,
        root: *const c_void,
        display_name: *mut u16,
        title: *const u16,
        flags: u32,
        callback: Ptr,
        data: isize,
        image: i32,
    }
    #[link(name = "comdlg32")]
    unsafe extern "system" {
        fn GetOpenFileNameW(config: *mut OpenFileName) -> i32;
        fn CommDlgExtendedError() -> u32;
    }
    #[link(name = "shell32")]
    unsafe extern "system" {
        fn SHBrowseForFolderW(config: *mut BrowseInfo) -> Ptr;
        fn SHGetPathFromIDListW(item: *const c_void, path: *mut u16) -> i32;
    }
    #[link(name = "ole32")]
    unsafe extern "system" {
        fn CoInitializeEx(reserved: Ptr, flags: u32) -> i32;
        fn CoUninitialize();
        fn CoTaskMemFree(data: Ptr);
    }
    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain([0]).collect()
    }
    let mut path = vec![0u16; 32768];
    if carpeta {
        // A dedicated STA avoids relying on the COM apartment of a Tokio worker.
        return std::thread::spawn(|| {
            let title = wide("Seleccione la carpeta de destino");
            let mut display = [0u16; 260];
            let mut path = [0u16; 260];
            let mut config = BrowseInfo {
                owner: ptr::null_mut(),
                root: ptr::null(),
                display_name: display.as_mut_ptr(),
                title: title.as_ptr(),
                flags: 0x0001 | 0x0040,
                callback: ptr::null_mut(),
                data: 0,
                image: 0,
            };
            unsafe {
                if CoInitializeEx(ptr::null_mut(), 2) < 0 {
                    return Err(err("No se pudo abrir el selector de carpetas."));
                }
                let item = SHBrowseForFolderW(&mut config);
                let result = if item.is_null() {
                    Ok(None)
                } else {
                    let ok = SHGetPathFromIDListW(item, path.as_mut_ptr());
                    CoTaskMemFree(item);
                    if ok == 0 {
                        Err(err("Seleccione una carpeta del sistema de archivos."))
                    } else {
                        Ok(Some(String::from_utf16_lossy(
                            &path[..path.iter().position(|c| *c == 0).unwrap_or(path.len())],
                        )))
                    }
                };
                CoUninitialize();
                result
            }
        })
        .join()
        .map_err(|_| err("No se pudo abrir el selector de carpetas."))?;
    }
    let title = wide("Seleccione un respaldo de Lubricentro");
    let filter = wide("Respaldo de Lubricentro (*.lgbackup)\0*.lgbackup\0");
    // A null hook and zeroed optional fields match the Win32 API contract.
    let mut config: OpenFileName = unsafe { mem::zeroed() };
    config.size = mem::size_of::<OpenFileName>() as u32;
    config.filter = filter.as_ptr();
    config.index = 1;
    config.file = path.as_mut_ptr();
    config.max_file = path.len() as u32;
    config.title = title.as_ptr();
    config.flags = 0x00000008 | 0x00001000 | 0x00000800 | 0x00080000;
    if unsafe { GetOpenFileNameW(&mut config) } == 0 {
        if unsafe { CommDlgExtendedError() } == 0 {
            return Ok(None);
        }
        return Err(err("No se pudo abrir el selector de respaldos."));
    }
    Ok(Some(String::from_utf16_lossy(
        &path[..path.iter().position(|c| *c == 0).unwrap_or(path.len())],
    )))
}

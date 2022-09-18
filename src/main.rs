use std::ptr::null_mut;
use windows::core::HRESULT;
use windows::Win32::Devices::Display::{DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_TOPOLOGY_ID, GetDisplayConfigBufferSizes, QueryDisplayConfig};
use windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS, WIN32_ERROR};
use windows::Win32::Graphics::Gdi::QDC_ONLY_ACTIVE_PATHS;

pub fn main() -> windows::core::Result<()> {
    let flags = QDC_ONLY_ACTIVE_PATHS;
    loop {
        let mut num_path_array_elements = u32::MAX;
        let mut num_mode_info_array_elements = u32::MAX;
        let ret: WIN32_ERROR = unsafe {
            std::mem::transmute(GetDisplayConfigBufferSizes(
                flags,
                &mut num_path_array_elements,
                &mut num_mode_info_array_elements,
            ))
        };
        if ret != ERROR_SUCCESS {
            break Err(HRESULT::from(ret).into());
        }
        let mut paths =
            Vec::<DISPLAYCONFIG_PATH_INFO>::with_capacity(num_path_array_elements as usize);
        let mut modes =
            Vec::<DISPLAYCONFIG_MODE_INFO>::with_capacity(num_mode_info_array_elements as usize);
        let ret: WIN32_ERROR = unsafe {
            std::mem::transmute(QueryDisplayConfig(
                flags,
                &mut num_path_array_elements,
                paths.as_mut_ptr(),
                &mut num_mode_info_array_elements,
                modes.as_mut_ptr(),
                null_mut(),  // this line
            ))
        };
        if ret == ERROR_INSUFFICIENT_BUFFER {
            continue;
        } else if ret != ERROR_SUCCESS {
            break Err(HRESULT::from(ret).into());
        }
        unsafe {
            paths.set_len(num_path_array_elements as usize);
            modes.set_len(num_mode_info_array_elements as usize);
        }
        for path in paths {
            println!("path_info: {:?}", path.flags)
        }
        for mode in modes {
            println!("mode_info: {:?}: {:?}", mode.id, mode.infoType)
        }
        break Ok(());
    }
}

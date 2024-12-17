use rtss_sys::{LPRTSS_SHARED_MEMORY, RTSS_SHARED_MEMORY_LPRTSS_SHARED_MEMORY_APP_ENTRY};
use windows::{
    core::w,
    Win32::{
        System::Memory::{MapViewOfFile, OpenFileMappingW, FILE_MAP_ALL_ACCESS},
        UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
    },
};

/// https://forums.guru3d.com/threads/get-game-framerate.422135
/// This is a crude unsafe but working example of how to get the foreground windows's framerate
/// If someone wants to split this crate into a workspace with rtss and rtss-sys for a safe wrapper library
/// I'm open to issues and pull requests
fn main() {
    unsafe {
        let h_map_file = OpenFileMappingW(FILE_MAP_ALL_ACCESS.0, false, w!("RTSSSharedMemoryV2"))
            .expect("Please make sure RivaTuner Statistics Server is running");
        let p_map_addr = MapViewOfFile(h_map_file, FILE_MAP_ALL_ACCESS, 0, 0, 0);
        let p_mem = p_map_addr.Value as LPRTSS_SHARED_MEMORY;
        /// Double check that struct is parsed correctly and has compatible version
        if ((*p_mem).dwSignature != 0x52545353) || ((*p_mem).dwVersion < 0x00020000) {
            println!(
                "Bad signature {} or version {}",
                (*p_mem).dwSignature,
                (*p_mem).dwVersion
            );
            return;
        }
        loop {
            let hwnd = GetForegroundWindow();
            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            for dw_entry in 0..(*p_mem).dwAppArrSize {
                /// Let's calculate entry offset from known array offset and entry size, for compatibility
                let entry_offset = (*p_mem).dwAppArrOffset + dw_entry * (*p_mem).dwAppEntrySize;
                let p_entry = p_mem.byte_offset(entry_offset as isize)
                    as RTSS_SHARED_MEMORY_LPRTSS_SHARED_MEMORY_APP_ENTRY;
                if (*p_entry).dwProcessID == pid {
                    let framerate = (*p_entry).dwStatFrameTimeBufFramerate as f32 / 10.0;
                    println!("{}", framerate);
                }
            }
        }
    }
}

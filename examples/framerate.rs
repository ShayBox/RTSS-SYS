use std::mem::transmute;

use rtss_sys::{LPRTSS_SHARED_MEMORY, RTSS_SHARED_MEMORY_RTSS_SHARED_MEMORY_APP_ENTRY};
use windows::{
    w,
    Win32::{
        System::Memory::{
            MapViewOfFile,
            OpenFileMappingW,
            FILE_MAP_ALL_ACCESS,
            MEMORYMAPPEDVIEW_HANDLE,
        },
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
        let p_map_addr = MapViewOfFile(h_map_file, FILE_MAP_ALL_ACCESS, 0, 0, 0).unwrap();
        let p_mem = transmute::<MEMORYMAPPEDVIEW_HANDLE, LPRTSS_SHARED_MEMORY>(p_map_addr);

        loop {
            let hwnd = GetForegroundWindow();
            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));

            for dw_entry in 0..(*p_mem).dwAppArrSize {
                let p_entry = ((*p_mem).arrApp.as_ptr()
                    as *const RTSS_SHARED_MEMORY_RTSS_SHARED_MEMORY_APP_ENTRY)
                    .add(dw_entry as usize);

                if (*p_entry).dwProcessID == pid {
                    let framerate = (*p_entry).dwStatFrameTimeBufFramerate as f32 / 10.0;
                    println!("{}", framerate);
                }
            }
        }
    }
}

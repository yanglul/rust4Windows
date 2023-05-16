use windows::{core::*,Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        // EnumWindows(Some(enum_window), 0);
        let thwnd = FindWindowW(None,w!("新建 文本文档.txt - Notepad"));
        println!("window_handle：{:?}",thwnd);
        let ptr:*mut u32 = 0  as * mut u32 ;
        let process_id =GetWindowThreadProcessId(thwnd,Some(ptr));
        println!("process_id:{}",process_id);
        // let process_handle = OpenProcess(PROCESS_ALL_ACCESS, False, process_id);
        

    }
}

 
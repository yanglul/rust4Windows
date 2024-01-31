use windows::{core::*,Win32::Foundation::*, 
    Win32::UI::WindowsAndMessaging::*,
    Win32::System::Threading::*,
    Win32::System::Diagnostics::Debug::*,
    Win32::System::ProcessStatus::*,
     };
 use core::ffi::{c_void,c_int,c_longlong,c_long};
use std::ptr;
use std::ffi::CString;
use std::mem;




 const MAX_PATH:u32 = 260;
 const PROCESS_NAME :&str = "GenshinImpact.exe";
 extern "system" fn get_base_address( hwd: HANDLE ,flag:bool)->u32{
    unsafe{
        let mut hmodule: [HMODULE;1024]=[HMODULE::default();1024];
        // let ressss = &mut  hmodule;
        let mut cbNeeded: u32 = 0;
        let eb = EnumProcessModulesEx(  hwd,  &mut  hmodule[0] ,1024,&mut  cbNeeded as *mut u32,LIST_MODULES_ALL);
        if eb.as_bool(){
            println!("EnumProcessModules调用成功");
           
            for i in 0.. cbNeeded{
                let mut szModName_temp = [0u16;256];
                let mut szModName =szModName_temp.as_mut_slice();
                // println!("hmodule{:?}",i);
                let rerr  = GetModuleFileNameExW(hwd,hmodule[i as usize],szModName);
                println!("module {:?}",szModName);


            }
            // GetModuleFileNameExW
        }else{
            println!("EnumProcessModules调用失败 {:?}",GetLastError());
        }
        0
    }
}



 #[warn(non_snake_case)]
fn main() {
    let g_path = String::from("D:\\tool\\");
    let dll_path = String::from("D:\\tool\\");
    let arg = String::from("");
    inject(g_path,dll_path,arg);
     
}

 
 pub fn inject(exe_path:String,dll_path:String,startupArguments:String){
    let mut si = STARTUPINFOA {
        cb: mem::size_of::<STARTUPINFOA>() as u32,
        ..unsafe { mem::zeroed() }
    };

    let mut pi = PROCESS_INFORMATION {
        ..unsafe { mem::zeroed() }
    };
    let program = CString::new("C:\\Windows\\System32\\cmd.exe").expect("CString::new failed");
    let arguments = CString::new("/c dir").expect("CString::new failed");
    let command_line = exe_path+" "+&startupArguments;
    let p =  PSTR( program.as_ptr() as *mut u8);
    unsafe{
        
        let th = CreateProcessA(
            Option::None,
            p,
            Option::None,
            Option::None,
            false,
            PROCESS_CREATION_FLAGS(0),
            Option::None,
            Option::None,
            &mut si,
            &mut pi

        
        );
        if th==BOOL(0){
            println!("失败{:?}",GetLastError());
        }

    }
 }
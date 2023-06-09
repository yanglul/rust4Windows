use windows::{core::*,Win32::Foundation::*, 
    Win32::UI::WindowsAndMessaging::*,
    Win32::System::Threading::*,
    Win32::System::Diagnostics::Debug::*,
    Win32::System::ProcessStatus::*,
     };
 use core::ffi::{c_void,c_int,c_longlong,c_long};

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
    unsafe {
        // EnumWindows(Some(enum_window), 0);
        println!("[>] entering  \n");
        let thwnd = FindWindowW(None,w!("原神"));
        println!("window_handle：{:?}",thwnd);
        let mut  ptr:  u32  = 0     ;
        let rptr = &mut ptr;
        let process_id =GetWindowThreadProcessId(thwnd,Some(rptr));
        println!("process_id:{},ptr{:?}",process_id,ptr);
        let process_handle: std::result::Result<HANDLE, Error> = OpenProcess(PROCESS_ALL_ACCESS, FALSE, ptr);
        println!("process_handle:{:?}",process_handle);
        let basead: u64  = 0x143D5AAC0    ;
        // let rbuffer: u64 = 0  ;
        let mut ret: u64 = Default::default();
        
       
        let s =process_handle.unwrap();

        let res2: u32 = get_base_address(s, false);
        // let a = ReadProcessMemory( s,  basead as *const c_void,  &mut ret as *mut u64 as *mut c_void, 8, None);
        // println!("a:{:?} rbuffer{:?} ",a, ret);
        // let   aim1 = ret+0x68;
         
        // let b = ReadProcessMemory( s,  aim1 as *const c_void,  &mut ret as *mut u64 as *mut c_void, 8, None);
        // println!("b:{:?} rbuffer{:?} ",b, ret);
        // let   aim2 = ret+0x238;
        // let c = ReadProcessMemory( s,  aim2 as *const c_void,  &mut ret as *mut u64 as *mut c_void, 8, None);
        // println!("a:{:?} rbuffer{:?} ",c, ret);

        // let   aim3 = ret+0x7C;
        // println!("金币地址{:?} ",aim3);
        // let d = ReadProcessMemory( s,  aim3 as *const c_void,  &mut ret as *mut u64 as *mut c_void, 8, None);
        // println!("a:{:?} rbuffer{:?} ",d, ret);


        // let mut money: u64  = 888; 
        // let f = WriteProcessMemory(s,aim3 as *const c_void,  &mut money as *mut u64 as *mut c_void,8,None);
     
        // println!("c:{:?} ",money);
 


        println!("[>] entering  \n");


    }



     
}

 
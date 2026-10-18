use gba_core::capi::RcgBlock;
use libloading::Library;
use recomp_core::{analyze::{Analysis, View}, emit::emit_c_chunked};
use std::{collections::HashMap, ffi::{c_char, c_int, c_void, CString}};
use std::path::Path;
use std::hash::{BuildHasherDefault,Hasher};

/// Keys are verified guest PCs, not arbitrary input strings. Avoid SipHash's
/// string-oriented cost at every native dispatch and interpreter fallback.
#[derive(Default)]
pub struct PcHasher(u64);
impl Hasher for PcHasher {
    fn finish(&self)->u64 {self.0}
    fn write_u32(&mut self,mut pc:u32) {
        pc^=pc>>16;pc=pc.wrapping_mul(0x7feb352d);
        pc^=pc>>15;pc=pc.wrapping_mul(0x846ca68b);pc^=pc>>16;
        self.0=(pc as u64)<<32|pc as u64;
    }
    fn write(&mut self,bytes:&[u8]) {
        for &byte in bytes {self.0=(self.0^byte as u64).wrapping_mul(0x100000001b3);}
    }
}
pub type PcMap=HashMap<u32,BlockFn,BuildHasherDefault<PcHasher>>;

#[cfg(test)]
mod dispatch_tests {
    use super::*;
    #[test]
    fn pc_map_preserves_regions_thumb_bit_replacements_and_misses(){
        let mut actual=HashMap::<u32,u32,BuildHasherDefault<PcHasher>>::default();
        let mut expected=HashMap::new();
        for region in [0x02000000,0x03000000,0x08000000] {
            for offset in 0..12000u32 {
                let key=region+offset;
                actual.insert(key,key.rotate_left(9));expected.insert(key,key.rotate_left(9));
            }
        }
        for key in (0x08000000..0x08003000).step_by(3) {
            actual.insert(key,7);expected.insert(key,7);
        }
        assert_eq!(actual.len(),expected.len());
        for (key,value) in expected {assert_eq!(actual.get(&key),Some(&value));}
        for key in [0,0x03008000,0x08010001,u32::MAX] {assert_eq!(actual.get(&key),None);}
    }
}

type State = *mut c_void;
static COMPILER_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
pub type BlockFn = extern "C" fn(*const gba_core::capi::RtApi, *mut c_void) -> u32;
pub struct NativeCode {
    pub blocks: PcMap,
    state: State,
    delete: unsafe extern "C" fn(State),
    _library: Library,
    errors: Box<String>,
}
// The engine mutex owns the context and all native calls. No concurrent TCC access.
unsafe impl Send for NativeCode {}
impl Drop for NativeCode {
    fn drop(&mut self) {
        if !self.state.is_null() {
            let _lock = COMPILER_LOCK.lock().unwrap_or_else(|e|e.into_inner());
            unsafe { (self.delete)(self.state) }
        }
    }
}
unsafe extern "C" fn error_callback(opaque: *mut c_void, message: *const c_char) {
    if !opaque.is_null() && !message.is_null() {
        let errors = &mut *(opaque as *mut String);
        if errors.len() < 8192 {
            errors.push_str(&std::ffi::CStr::from_ptr(message).to_string_lossy());
            errors.push('\n');
        }
    }
}
pub fn compile(analysis: &Analysis, view: &View, object: Option<&Path>, progress: impl Fn(usize, usize)) -> Result<NativeCode, String> {
    build(Some((analysis,view)),None,object,&progress)
}
pub fn load_object(path: &Path) -> Result<NativeCode,String> {
    build(None,Some(path),None,&|_,_|{})
}
fn build(input: Option<(&Analysis,&View)>, file: Option<&Path>, object: Option<&Path>, progress: &dyn Fn(usize,usize)) -> Result<NativeCode,String> {
    unsafe {
        #[cfg(target_os = "android")]
        let path = std::ffi::OsString::from("libtcc.so");
        #[cfg(not(target_os = "android"))]
        let path = std::env::var_os("FFTA_ANDROID_TCC_LIBRARY").unwrap_or_else(|| "libtcc.so".into());
        let library = Library::new(path).map_err(|e| format!("Load libtcc: {e}"))?;
        let new = *library.get::<unsafe extern "C" fn() -> State>(b"tcc_new\0").map_err(|e| e.to_string())?;
        let delete = *library.get::<unsafe extern "C" fn(State)>(b"tcc_delete\0").map_err(|e| e.to_string())?;
        let output = *library.get::<unsafe extern "C" fn(State, c_int) -> c_int>(b"tcc_set_output_type\0").map_err(|e| e.to_string())?;
        let options = *library.get::<unsafe extern "C" fn(State, *const c_char) -> c_int>(b"tcc_set_options\0").map_err(|e| e.to_string())?;
        let on_error = *library.get::<unsafe extern "C" fn(State, *mut c_void, unsafe extern "C" fn(*mut c_void,*const c_char))>(b"tcc_set_error_func\0").map_err(|e| e.to_string())?;
        let compile = *library.get::<unsafe extern "C" fn(State, *const c_char) -> c_int>(b"tcc_compile_string\0").map_err(|e| e.to_string())?;
        // This pinned TinyCC revision uses the one-argument relocate ABI.
        let relocate = *library.get::<unsafe extern "C" fn(State) -> c_int>(b"tcc_relocate\0").map_err(|e| e.to_string())?;
        let symbol = *library.get::<unsafe extern "C" fn(State,*const c_char)->*mut c_void>(b"tcc_get_symbol\0").map_err(|e| e.to_string())?;
        let mut code = NativeCode { blocks: PcMap::default(), state: std::ptr::null_mut(), delete, _library: library, errors: Box::new(String::new()) };
        // Declared after code so error unwinding releases this lock before code's Drop.
        let _lock = COMPILER_LOCK.lock().map_err(|_|"Compiler mutex poisoned")?;
        let state = new();
        if state.is_null() { return Err("libtcc could not allocate a context".into()); }
        code.state = state;
        on_error(state, &mut *code.errors as *mut _ as *mut _, error_callback);
        options(state, b"-std=c11 -nostdinc -nostdlib\0".as_ptr() as *const _);
        if output(state, if object.is_some(){3}else{1}) < 0 { return Err(format!("TCC output: {}",code.errors)); }
        #[cfg(windows)]
        if object.is_none() {
            // TinyCC's Windows helper also builds the frame (not the OS __chkstk ABI).
            // The host test DLL includes TinyCC's own chkstk.S; Android needs neither.
            let probe = code._library.get::<unsafe extern "C" fn()>(b"__chkstk\0").map_err(|e| e.to_string())?;
            let add = *code._library.get::<unsafe extern "C" fn(State,*const c_char,*const c_void)->c_int>(b"tcc_add_symbol\0").map_err(|e| e.to_string())?;
            add(state,b"__chkstk\0".as_ptr() as *const _,*probe as *const c_void);
        }
        if let Some((analysis,view)) = input {
        let result = emit_c_chunked(analysis, view, 1024 * 1024, |source, done| {
            let source = source.replace("#include <stdint.h>",
                "typedef unsigned char uint8_t; typedef signed char int8_t; typedef unsigned short uint16_t; typedef signed short int16_t; typedef unsigned int uint32_t; typedef unsigned long long uint64_t; typedef signed int int32_t; typedef signed long long int64_t;");
            let source = CString::new(source).map_err(|e| e.to_string())?;
            if compile(state, source.as_ptr()) < 0 { return Err("C compilation failed".into()); }
            progress(done, analysis.blocks.len());
            Ok(())
        });
        if let Err(error) = result { return Err(format!("{error}: {}",code.errors)); }
        }
        if let Some(path) = file {
            let add = *code._library.get::<unsafe extern "C" fn(State,*const c_char)->c_int>(b"tcc_add_file\0").map_err(|e|e.to_string())?;
            let path = CString::new(path.to_string_lossy().as_bytes()).map_err(|e|e.to_string())?;
            if add(state,path.as_ptr())<0 { return Err(format!("Read cached object: {}",code.errors)); }
        }
        if let Some(path) = object {
            let write = *code._library.get::<unsafe extern "C" fn(State,*const c_char)->c_int>(b"tcc_output_file\0").map_err(|e|e.to_string())?;
            let name = CString::new(path.to_string_lossy().as_bytes()).map_err(|e|e.to_string())?;
            if write(state,name.as_ptr())<0 { return Err(format!("Write cached object: {}",code.errors)); }
            drop(_lock);
            drop(code);
            return load_object(path);
        }
        if relocate(state) < 0 { return Err(format!("TCC relocation: {}",code.errors)); }
        let blocks = symbol(state,b"rcg_blocks\0".as_ptr() as *const _) as *const RcgBlock;
        let count = symbol(state,b"rcg_block_count\0".as_ptr() as *const _) as *const u64;
        if blocks.is_null() || count.is_null() || *count>1_000_000
            || input.is_some_and(|(analysis,_)|*count as usize!=analysis.blocks.len()) {
            return Err("Native block table missing or inconsistent".into());
        }
        for block in std::slice::from_raw_parts(blocks, *count as usize) {
            code.blocks.insert(block.key, block.func);
        }
        Ok(code)
    }
}

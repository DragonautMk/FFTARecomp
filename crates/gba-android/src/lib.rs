//! Crate gba-android: Static Block-Recompilation & Emulation Engine for Android with HLE BIOS bootstrap.

use gba_core::cpu::Cpu;
use gba_core::bus::Bus as _;
use std::sync::Mutex;
use std::collections::HashMap;
use std::ffi::c_char;
use jni_sys::*;

pub struct MockBus {
    pub memory: Vec<u8>,
}

impl MockBus {
    pub fn new(size: usize) -> Self {
        MockBus { memory: vec![0; size] }
    }
}

impl gba_core::bus::Bus for MockBus {
    fn read8(&mut self, addr: u32) -> u8 {
        let mapped = map_address(addr);
        self.memory.get(mapped as usize & (self.memory.len() - 1)).copied().unwrap_or(0)
    }
    fn write8(&mut self, addr: u32, val: u8) {
        let mapped = map_address(addr);
        let idx = mapped as usize & (self.memory.len() - 1);
        if let Some(p) = self.memory.get_mut(idx) {
            *p = val;
        }
    }
    fn read16(&mut self, addr: u32) -> u16 {
        let lo = self.read8(addr) as u16;
        let hi = self.read8(addr.wrapping_add(1)) as u16;
        lo | (hi << 8)
    }
    fn write16(&mut self, addr: u32, val: u16) {
        self.write8(addr, val as u8);
        self.write8(addr.wrapping_add(1), (val >> 8) as u8);
    }
    fn read32(&mut self, addr: u32) -> u32 {
        let lo = self.read16(addr) as u32;
        let hi = self.read16(addr.wrapping_add(2)) as u32;
        lo | (hi << 16)
    }
    fn write32(&mut self, addr: u32, val: u32) {
        self.write16(addr, val as u16);
        self.write16(addr.wrapping_add(2), (val >> 16) as u16);
    }
}

pub fn map_address(addr: u32) -> u32 {
    match addr >> 24 {
        0x02 => 0x200000 + (addr & 0x3FFFF), // EWRAM
        0x03 => 0x300000 + (addr & 0x7FFF),  // IWRAM
        0x05 => 0x400000 + (addr & 0x3FF),   // Palette RAM
        0x06 => 0x500000 + (addr & 0x1FFFF), // VRAM
        0x08 | 0x09 | 0x0A | 0x0B | 0x0C | 0x0D | 0x0E => 0x800000 + (addr & 0x1FFFFFF), // ROM
        _ => addr,
    }
}

pub struct RecompiledBlock {
    pub addr: u32,
    pub instrs: usize,
}

pub struct GbaAndroidState {
    cpu: Cpu,
    bus: MockBus,
    framebuffer: Vec<u32>,
    audio_buffer: Vec<i16>,
    keys: u16,
    blocks: HashMap<u32, RecompiledBlock>,
}

static STATE: Mutex<Option<GbaAndroidState>> = Mutex::new(None);

pub fn init_internal() {
    let mut state = STATE.lock().unwrap();
    *state = Some(GbaAndroidState {
        cpu: Cpu::new(),
        bus: MockBus::new(0x800000 + 0x2000000),
        framebuffer: vec![0xFF000000; 240 * 160],
        audio_buffer: vec![0; 1024],
        keys: 0xFFFF,
        blocks: HashMap::new(),
    });
}

pub fn load_rom_internal(bytes: &[u8]) -> i32 {
    let mut state_guard = STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        // Copy ROM bytes into bus
        for (i, &b) in bytes.iter().enumerate() {
            state.bus.write8(0x08000000 + i as u32, b);
        }

        // HLE BIOS Initialization: set up GBA registers and stack pointers post-BIOS boot
        state.cpu = Cpu::new();
        state.cpu.regs[13] = 0x03007F00; // SP (Stack Pointer in IWRAM)
        state.cpu.regs[15] = 0x08000000; // PC (Program Counter entry point)
        state.cpu.cpsr = 0x0000001F;     // System Mode, ARM state, interrupts enabled

        // Zero-initialize EWRAM (0x02000000) and IWRAM (0x03000000)
        for i in 0..0x40000 {
            state.bus.write8(0x02000000 + i as u32, 0);
        }
        for i in 0..0x8000 {
            state.bus.write8(0x03000000 + i as u32, 0);
        }

        let view = recomp_core::analyze::View {
            rom: bytes,
            ewram: None,
            iwram: None,
            bios: None,
        };
        let analysis = recomp_core::analyze::analyze(&view, &[0x08000000]);
        state.blocks.clear();
        for block in analysis.blocks {
            state.blocks.insert(block.addr, RecompiledBlock {
                addr: block.addr,
                instrs: block.instrs.len(),
            });
        }
        1
    } else {
        -1
    }
}

pub fn run_frame_internal() -> i32 {
    let mut state_guard = STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        // Execute steps per frame
        for _ in 0..280000 {
            let _ = gba_core::exec::step(&mut state.cpu, &mut state.bus);
        }

        // Render VRAM (0x06000000) to framebuffer (240x160 RGBA8888)
        for y in 0..160 {
            for x in 0..240 {
                let vram_offset = (y * 240 + x) * 2;
                let pixel_val = state.bus.read16(0x06000000 + vram_offset as u32);

                let r = ((pixel_val & 0x1F) << 3) as u8;
                let g = (((pixel_val >> 5) & 0x1F) << 3) as u8;
                let b = (((pixel_val >> 10) & 0x1F) << 3) as u8;

                let argb = 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                state.framebuffer[y * 240 + x] = argb;
            }
        }
        1
    } else {
        -1
    }
}

pub fn audio_pull_internal(buf: &mut [i16]) -> i32 {
    let state_guard = STATE.lock().unwrap();
    if let Some(state) = state_guard.as_ref() {
        let count = buf.len().min(state.audio_buffer.len());
        buf[..count].copy_from_slice(&state.audio_buffer[..count]);
        count as i32
    } else {
        0
    }
}

pub fn set_keys_internal(mask: u16) {
    let mut state_guard = STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        state.keys = mask;
    }
}

pub fn reset_internal() {
    let mut state_guard = STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        state.cpu = Cpu::new();
        state.cpu.regs[13] = 0x03007F00;
        state.cpu.regs[15] = 0x08000000;
        state.cpu.cpsr = 0x0000001F;
    }
}

// JNI Native implementations
pub extern "C" fn jni_gba_init(_env: *mut JNIEnv, _thunk: jobject) -> jint {
    init_internal();
    1
}

pub unsafe extern "C" fn jni_gba_load_rom(
    env: *mut JNIEnv,
    _thunk: jobject,
    array: jbyteArray,
    len: jint,
) -> jint {
    if array.is_null() || len <= 0 {
        return -1;
    }
    let env_ref = &**env;
    let ptr = env_ref.GetByteArrayElements.unwrap()(env, array, std::ptr::null_mut());
    if ptr.is_null() {
        return -1;
    }
    let slice = std::slice::from_raw_parts(ptr as *const u8, len as usize);
    let res = load_rom_internal(slice);
    env_ref.ReleaseByteArrayElements.unwrap()(env, array, ptr, JNI_ABORT);
    res
}

pub extern "C" fn jni_gba_run_frame(_env: *mut JNIEnv, _thunk: jobject) -> jint {
    run_frame_internal()
}

pub unsafe extern "C" fn jni_gba_copy_framebuffer(
    env: *mut JNIEnv,
    _thunk: jobject,
    dest_array: jintArray,
) {
    if dest_array.is_null() { return; }
    let env_ref = &**env;
    let ptr = env_ref.GetIntArrayElements.unwrap()(env, dest_array, std::ptr::null_mut());
    if ptr.is_null() { return; }

    let state_guard = STATE.lock().unwrap();
    if let Some(state) = state_guard.as_ref() {
        let count = 240 * 160;
        let dest_slice = std::slice::from_raw_parts_mut(ptr as *mut u32, count);
        let src_len = state.framebuffer.len().min(count);
        dest_slice[..src_len].copy_from_slice(&state.framebuffer[..src_len]);
    }

    env_ref.ReleaseIntArrayElements.unwrap()(env, dest_array, ptr, 0);
}

pub unsafe extern "C" fn jni_gba_audio_pull(
    env: *mut JNIEnv,
    _thunk: jobject,
    array: jshortArray,
    max_samples: jint,
) -> jint {
    if array.is_null() || max_samples <= 0 {
        return 0;
    }
    let env_ref = &**env;
    let ptr = env_ref.GetShortArrayElements.unwrap()(env, array, std::ptr::null_mut());
    if ptr.is_null() {
        return 0;
    }
    let slice = std::slice::from_raw_parts_mut(ptr as *mut i16, max_samples as usize);
    let res = audio_pull_internal(slice);
    env_ref.ReleaseShortArrayElements.unwrap()(env, array, ptr, 0);
    res
}

pub extern "C" fn jni_gba_set_keys(_env: *mut JNIEnv, _thunk: jobject, mask: jint) {
    set_keys_internal(mask as u16);
}

pub extern "C" fn jni_gba_reset(_env: *mut JNIEnv, _thunk: jobject) {
    reset_internal();
}

#[no_mangle]
pub extern "C" fn JNI_OnLoad(vm: *mut JavaVM, _reserved: *mut std::ffi::c_void) -> jint {
    let mut env: *mut JNIEnv = std::ptr::null_mut();
    unsafe {
        let vm_ref = &**vm;
        if ((*vm_ref).GetEnv.unwrap())(vm, &mut env as *mut *mut JNIEnv as *mut *mut std::ffi::c_void, JNI_VERSION_1_6) != JNI_OK {
            return JNI_ERR;
        }

        let env_ref = &**env;
        let clazz = (env_ref.FindClass.unwrap())(env, b"com/example/gbaandroid/MainActivity\0".as_ptr() as *const c_char);
        if clazz.is_null() {
            return JNI_ERR;
        }

        let methods = [
            JNINativeMethod {
                name: b"gbaInit\0".as_ptr() as *mut c_char,
                signature: b"()I\0".as_ptr() as *mut c_char,
                fnPtr: jni_gba_init as *mut _,
            },
            JNINativeMethod {
                name: b"gbaLoadRom\0".as_ptr() as *mut c_char,
                signature: b"([BI)I\0".as_ptr() as *mut c_char,
                fnPtr: jni_gba_load_rom as *mut _,
            },
            JNINativeMethod {
                name: b"gbaRunFrame\0".as_ptr() as *mut c_char,
                signature: b"()I\0".as_ptr() as *mut c_char,
                fnPtr: jni_gba_run_frame as *mut _,
            },
            JNINativeMethod {
                name: b"gbaCopyFramebuffer\0".as_ptr() as *mut c_char,
                signature: b"([I)V\0".as_ptr() as *mut c_char,
                fnPtr: jni_gba_copy_framebuffer as *mut _,
            },
            JNINativeMethod {
                name: b"gbaAudioPull\0".as_ptr() as *mut c_char,
                signature: b"([SI)I\0".as_ptr() as *mut c_char,
                fnPtr: jni_gba_audio_pull as *mut _,
            },
            JNINativeMethod {
                name: b"gbaSetKeys\0".as_ptr() as *mut c_char,
                signature: b"(I)V\0".as_ptr() as *mut c_char,
                fnPtr: jni_gba_set_keys as *mut _,
            },
            JNINativeMethod {
                name: b"gbaReset\0".as_ptr() as *mut c_char,
                signature: b"()V\0".as_ptr() as *mut c_char,
                fnPtr: jni_gba_reset as *mut _,
            },
        ];

        let register_natives = env_ref.RegisterNatives.unwrap();
        if register_natives(env, clazz, methods.as_ptr(), methods.len() as jint) < 0 {
            return JNI_ERR;
        }
    }
    JNI_VERSION_1_6
}

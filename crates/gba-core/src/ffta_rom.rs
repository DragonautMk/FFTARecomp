//! Patch eligibility data, shared with the installer. No game code is embedded.
use sha2::{Digest, Sha256};

pub const SHA256: &str = "43fc8204c6dceee58828aebc7af0c72eb807e99f35ad641c8bb0a4fa8b6edc19";
pub const HOOK_TAG: u64 = 0x4646544153505231;
pub const RAM_SPRITE_START: u32 = 0x030065a8;
pub const RAM_SPRITE_END: u32 = 0x03006888;
pub const RAM_SOURCE_START: usize = 0xa3915c;
pub const RAM_SOURCE_END: usize = 0xa39458;
pub const VIS_START:u32=0x030062ec;
pub const VIS_RETURN:u32=0x0300640e;
pub const VIS_END:u32=0x03006414;
pub const VIS_SOURCE_START:usize=0xa38ea0;
pub const VIS_SOURCE_END:usize=0xa38fc8;
pub fn verify_visibility(rom:&[u8])->bool {
    rom.get(VIS_SOURCE_START..VIS_SOURCE_END).is_some_and(|bytes|
        bytes.starts_with(&[0xf0,0xb5]) && digest(bytes)=="1e988998a21067596046e2b303da1ad9b6924c18146f396ce0bf5f6e038433c7")
}
pub const CHECKS: &[(usize, &[u8])] = &[
    (0x14c8, &[0xf0, 0xb5]), (0x159c, &[0x83, 0xe0]),
    (0x15aa, &[0x7c, 0xdc]), (0x15fe, &[0x52, 0xdb]),
    (0x160c, &[0x4b, 0xdc]), (0x169a, &[0x08, 0x35]),
    (0x16d4, &[0xf0, 0xb5]), (0x1778, &[0x70, 0xdb]),
    (0x1788, &[0x68, 0xdc]), (0x17c2, &[0x4b, 0xdb]),
    (0x17d2, &[0x43, 0xdc]), (0x1856, &[0x08, 0x33]),
    (RAM_SOURCE_START, &[0xf0, 0xb5]), (0xa3930c, &[0x18, 0x80]),
];
const FUNCTIONS: &[(usize, usize, &str)] = &[
    (0x14c8, 0x16d4, "6be04b5192e1a3e6776819aa834b155911206bc53444ba3bde476d7b2b14063f"),
    (0x16d4, 0x18a0, "d883a53ce5bc4c3b4dd08bf37fddef575da1c3cae2c6d30886fbbbc9976eaac6"),
    (RAM_SOURCE_START, RAM_SOURCE_END, "ce5416f21676ea64415b11aa832634fddd97e304b2d6ae29bb0a5e42828a1afc"),
];
pub fn digest(data: &[u8]) -> String {
    Sha256::digest(data).iter().map(|b| format!("{b:02x}")).collect()
}
pub fn verify_hooks(rom: &[u8]) -> bool {
    CHECKS.iter().all(|(off, expected)| rom.get(*off..off + expected.len()) == Some(*expected))
        && FUNCTIONS.iter().all(|(start, end, hash)| rom.get(*start..*end).is_some_and(|bytes| digest(bytes) == *hash))
}
pub fn eligible(rom: &[u8]) -> bool { digest(rom) == SHA256 && verify_hooks(rom) }
pub fn sprite_pc(pc: u32) -> bool {
    (0x080014c8..0x080016c8).contains(&pc) || (0x080016d4..0x08001882).contains(&pc)
        || (RAM_SPRITE_START..RAM_SPRITE_END).contains(&pc)
        || (VIS_START..VIS_END).contains(&pc)
}

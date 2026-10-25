//! Versioned decoded base resources. No filesystem I/O or guest state.
//! Only exact, independently verified profiles are eligible for extraction.
use crate::ffta_rom::{self, Version};
use serde::{Deserialize, Serialize};

mod runtime;
pub use runtime::{AlternatePaletteStats, BackgroundStats, GraphicsStats, PacketRead, PacketStats, PaletteRead, PaletteStats, ResourceStats, RuntimeResources, TransferStats, TextStats, TextCatalogStats, FontKeyboardStats, ObjListStats, ObjAnimationStats};
pub mod bg_descriptors;
pub mod bg_packets;
pub mod bg_packet_data;
pub mod bg_palettes;
pub mod bg_alternate_palettes;
pub mod bg_graphics;
pub mod bg_transfers;
pub mod text_short;
pub mod text_catalogs;
pub mod fonts;
pub mod name_keyboard;
pub mod obj_lists;
pub mod obj_animations;
pub mod obj_graphics;
pub mod ui_palettes;
pub mod ui_graphics;
pub mod items;
pub mod item_links;
pub mod metadata16;
pub mod metadata52;
pub mod metadata4;
pub mod metadata2;
pub mod default48;
pub mod constructor_alternates;
pub mod metadata14;
pub mod metadata12;
pub mod global_pair_flags;
pub mod evaluation_parameters;
pub mod progression_rewards;
pub mod progression_gates;
pub mod progression_selection_gates;
pub mod aggregate_flag_keys;
pub mod trade_parameters;
pub mod item_effect_parameters;
pub mod audio_directory;
mod audio_producers;
pub mod audio_tones;
pub mod audio_sequences;
pub mod audio_driver;
pub mod palette_effect_dispatch;
pub mod palette_programs;
pub mod blend_programs;
pub mod task_descriptors;
pub mod effect_event_directories;
pub mod script_commands;
pub mod script_archive;
pub mod script_text;
pub mod script_coordinates;
pub mod auxiliary_scripts;
pub mod map_raster_palettes;
pub mod ui_choice_lists;
pub mod unit_obj_initial_pose;
pub mod unit_obj_equipment;
pub mod unit_obj_normal_battle;
pub mod unit_obj_script_special_normal;
pub mod unit_obj_alternate_battle;
pub mod color_mode_palettes;
mod custom_lz;
pub mod lz77;

pub const SCHEMA: u32 = 1;
pub const EXTRACTOR_REV: u32 = 61;
pub const DECODER_REV: u32 = 60;
pub const METADATA16_ID: &str = "gameplay.linked-metadata16";
pub const METADATA16_FILE: &str = "linked-metadata16.json";
pub const METADATA16_FORMAT: &str = "linked-metadata16-v1-json";
pub const MAX_METADATA16_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const METADATA52_ID: &str = "gameplay.linked-metadata52";
pub const METADATA52_FILE: &str = "linked-metadata52.json";
pub const METADATA52_FORMAT: &str = "linked-metadata52-v1-json";
pub const MAX_METADATA52_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const METADATA4_ID: &str = "gameplay.linked-metadata4";
pub const METADATA4_FILE: &str = "linked-metadata4.json";
pub const METADATA4_FORMAT: &str = "linked-metadata4-v1-json";
pub const MAX_METADATA4_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const METADATA2_ID: &str = "gameplay.linked-metadata2";
pub const METADATA2_FILE: &str = "linked-metadata2.json";
pub const METADATA2_FORMAT: &str = "linked-metadata2-v1-json";
pub const MAX_METADATA2_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const DEFAULT48_ID: &str = "gameplay.default-constructor-template";
pub const DEFAULT48_FILE: &str = "default-constructor-template.json";
pub const DEFAULT48_FORMAT: &str = "default-constructor-template-v1-json";
pub const MAX_DEFAULT48_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const CONSTRUCTOR_ALTERNATES_ID: &str = "gameplay.constructor-alternate-templates";
pub const CONSTRUCTOR_ALTERNATES_FILE: &str = "constructor-alternate-templates.json";
pub const CONSTRUCTOR_ALTERNATES_FORMAT: &str = "constructor-alternate-templates-v1-json";
pub const MAX_CONSTRUCTOR_ALTERNATES_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const METADATA14_ID: &str = "gameplay.linked-metadata14";
pub const METADATA14_FILE: &str = "linked-metadata14.json";
pub const METADATA14_FORMAT: &str = "linked-metadata14-v1-json";
pub const MAX_METADATA14_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const METADATA12_ID: &str = "gameplay.linked-metadata12";
pub const METADATA12_FILE: &str = "linked-metadata12.json";
pub const METADATA12_FORMAT: &str = "linked-metadata12-v1-json";
pub const MAX_METADATA12_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const GLOBAL_PAIR_FLAGS_ID: &str = "gameplay.global-pair-flags";
pub const GLOBAL_PAIR_FLAGS_FILE: &str = "global-pair-flags.json";
pub const GLOBAL_PAIR_FLAGS_FORMAT: &str = "global-pair-flags-v1-json";
pub const MAX_GLOBAL_PAIR_FLAGS_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const EVALUATION_PARAMETERS_ID: &str = "gameplay.evaluation-parameters";
pub const EVALUATION_PARAMETERS_FILE: &str = "evaluation-parameters.json";
pub const EVALUATION_PARAMETERS_FORMAT: &str = "evaluation-parameters-v1-json";
pub const MAX_EVALUATION_PARAMETERS_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const PROGRESSION_REWARDS_ID: &str = "gameplay.progression-rewards";
pub const PROGRESSION_REWARDS_FILE: &str = "progression-rewards.json";
pub const PROGRESSION_REWARDS_FORMAT: &str = "progression-rewards-v1-json";
pub const MAX_PROGRESSION_REWARDS_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const PROGRESSION_GATES_ID: &str = "gameplay.progression-gates";
pub const PROGRESSION_GATES_FILE: &str = "progression-gates.json";
pub const PROGRESSION_GATES_FORMAT: &str = "progression-gates-v1-json";
pub const MAX_PROGRESSION_GATES_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const PROGRESSION_SELECTION_GATES_ID: &str = "gameplay.progression-selection-gates";
pub const PROGRESSION_SELECTION_GATES_FILE: &str = "progression-selection-gates.json";
pub const PROGRESSION_SELECTION_GATES_FORMAT: &str = "progression-selection-gates-v1-json";
pub const MAX_PROGRESSION_SELECTION_GATES_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const AGGREGATE_FLAG_KEYS_ID: &str = "gameplay.aggregate-flag-keys";
pub const AGGREGATE_FLAG_KEYS_FILE: &str = "aggregate-flag-keys.json";
pub const AGGREGATE_FLAG_KEYS_FORMAT: &str = "aggregate-flag-keys-v1-json";
pub const MAX_AGGREGATE_FLAG_KEYS_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const TRADE_PARAMETERS_ID: &str = "gameplay.trade-parameters";
pub const TRADE_PARAMETERS_FILE: &str = "trade-parameters.json";
pub const TRADE_PARAMETERS_FORMAT: &str = "trade-parameters-v1-json";
pub const MAX_TRADE_PARAMETERS_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const ITEM_EFFECT_PARAMETERS_ID: &str = "gameplay.item-effect-parameters";
pub const ITEM_EFFECT_PARAMETERS_FILE: &str = "item-effect-parameters.json";
pub const ITEM_EFFECT_PARAMETERS_FORMAT: &str = "item-effect-parameters-v1-json";
pub const MAX_ITEM_EFFECT_PARAMETERS_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const AUDIO_DIRECTORY_ID: &str = "audio.m4a-directory";
pub const AUDIO_DIRECTORY_FILE: &str = "audio-directory.json";
pub const AUDIO_DIRECTORY_FORMAT: &str = "audio-directory-v1-json";
pub const MAX_AUDIO_DIRECTORY_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const AUDIO_TONES_ID: &str = "audio.m4a-tones";
pub const AUDIO_TONES_FILE: &str = "audio-tones.json";
pub const AUDIO_TONES_FORMAT: &str = "audio-tones-v1-json";
pub const MAX_AUDIO_TONES_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const AUDIO_SEQUENCES_ID: &str = "audio.m4a-sequences";
pub const AUDIO_SEQUENCES_FILE: &str = "audio-sequences.json";
pub const AUDIO_SEQUENCES_FORMAT: &str = "audio-sequences-v1-json";
pub const MAX_AUDIO_SEQUENCES_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const AUDIO_DRIVER_ID: &str = "audio.m4a-driver-tables";
pub const AUDIO_DRIVER_FILE: &str = "audio-driver-tables.json";
pub const AUDIO_DRIVER_FORMAT: &str = "audio-driver-tables-v1-json";
pub const MAX_AUDIO_DRIVER_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const PALETTE_EFFECT_DISPATCH_ID: &str = "graphics.palette-effect-dispatch";
pub const PALETTE_EFFECT_DISPATCH_FILE: &str = "palette-effect-dispatch.json";
pub const PALETTE_EFFECT_DISPATCH_FORMAT: &str = "palette-effect-dispatch-v1-json";
pub const MAX_PALETTE_EFFECT_DISPATCH_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const PALETTE_PROGRAMS_ID: &str = "graphics.palette-programs";
pub const PALETTE_PROGRAMS_FILE: &str = "palette-programs.json";
pub const PALETTE_PROGRAMS_FORMAT: &str = "palette-programs-v1-json";
pub const MAX_PALETTE_PROGRAMS_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const BLEND_PROGRAMS_ID: &str = "graphics.blend-programs";
pub const BLEND_PROGRAMS_FILE: &str = "blend-programs.json";
pub const BLEND_PROGRAMS_FORMAT: &str = "blend-programs-v1-json";
pub const MAX_BLEND_PROGRAMS_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const TASK_DESCRIPTORS_ID: &str = "engine.task-descriptors";
pub const TASK_DESCRIPTORS_FILE: &str = "task-descriptors.json";
pub const TASK_DESCRIPTORS_FORMAT: &str = "task-descriptors-v1-json";
pub const MAX_TASK_DESCRIPTORS_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const EFFECT_EVENT_DIRECTORIES_ID: &str = "engine.effect-event-directories";
pub const EFFECT_EVENT_DIRECTORIES_FILE: &str = "effect-event-directories.json";
pub const EFFECT_EVENT_DIRECTORIES_FORMAT: &str = "effect-event-directories-v1-json";
pub const MAX_EFFECT_EVENT_DIRECTORIES_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const SCRIPT_COMMANDS_ID: &str = "engine.script-command-directory";
pub const SCRIPT_COMMANDS_FILE: &str = "script-command-directory.json";
pub const SCRIPT_COMMANDS_FORMAT: &str = "script-command-directory-v1-json";
pub const MAX_SCRIPT_COMMANDS_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const SCRIPT_ARCHIVE_ID: &str = "engine.script-archive";
pub const SCRIPT_ARCHIVE_FILE: &str = "script-archive.json";
pub const SCRIPT_ARCHIVE_FORMAT: &str = "script-archive-v1-json";
pub const MAX_SCRIPT_ARCHIVE_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const SCRIPT_TEXT_ID: &str = "text.script-archives";
pub const SCRIPT_TEXT_FILE: &str = "script-text.json";
pub const SCRIPT_TEXT_FORMAT: &str = "script-text-v1-json";
pub const MAX_SCRIPT_TEXT_FILE_BYTES: u64 = 512 * 1024 * 1024;
pub const SCRIPT_COORDINATES_ID: &str = "engine.script-coordinate-paths";
pub const SCRIPT_COORDINATES_FILE: &str = "script-coordinate-paths.json";
pub const SCRIPT_COORDINATES_FORMAT: &str = "script-coordinate-paths-v1-json";
pub const MAX_SCRIPT_COORDINATES_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const UI_CHOICE_LISTS_ID: &str = "engine.ui-choice-lists";
pub const UI_CHOICE_LISTS_FILE: &str = "ui-choice-lists.json";
pub const UI_CHOICE_LISTS_FORMAT: &str = "ui-choice-lists-v1-json";
pub const MAX_UI_CHOICE_LISTS_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const UNIT_OBJ_INITIAL_POSE_ID: &str = "graphics.unit-obj-initial-pose";
pub const UNIT_OBJ_INITIAL_POSE_FILE: &str = "unit-obj-initial-pose.json";
pub const UNIT_OBJ_INITIAL_POSE_FORMAT: &str = "unit-obj-initial-pose-v1-json";
pub const MAX_UNIT_OBJ_INITIAL_POSE_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const COLOR_MODE_PALETTES_ID: &str = "graphics.color-mode-palettes";
pub const COLOR_MODE_PALETTES_FILE: &str = "color-mode-palettes.json";
pub const COLOR_MODE_PALETTES_FORMAT: &str = "color-mode-palettes-v1-json";
pub const MAX_COLOR_MODE_PALETTES_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const UNIT_OBJ_EQUIPMENT_ID: &str = "graphics.unit-obj-equipment";
pub const UNIT_OBJ_EQUIPMENT_FILE: &str = "unit-obj-equipment.json";
pub const UNIT_OBJ_EQUIPMENT_FORMAT: &str = "unit-obj-equipment-v1-json";
pub const MAX_UNIT_OBJ_EQUIPMENT_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const UNIT_OBJ_NORMAL_BATTLE_ID: &str = "graphics.unit-obj-normal-battle";
pub const UNIT_OBJ_NORMAL_BATTLE_FILE: &str = "unit-obj-normal-battle.json";
pub const UNIT_OBJ_NORMAL_BATTLE_FORMAT: &str = "unit-obj-normal-battle-v1-json";
pub const MAX_UNIT_OBJ_NORMAL_BATTLE_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const UNIT_OBJ_ALTERNATE_BATTLE_ID: &str = "graphics.unit-obj-alternate-battle";
pub const UNIT_OBJ_ALTERNATE_BATTLE_FILE: &str = "unit-obj-alternate-battle.json";
pub const UNIT_OBJ_ALTERNATE_BATTLE_FORMAT: &str = "unit-obj-alternate-battle-v1-json";
pub const MAX_UNIT_OBJ_ALTERNATE_BATTLE_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const MAP_RASTER_PALETTES_ID: &str = "graphics.map-raster-palettes";
pub const MAP_RASTER_PALETTES_FILE: &str = "map-raster-palettes.json";
pub const MAP_RASTER_PALETTES_FORMAT: &str = "map-raster-palettes-v1-json";
pub const MAX_MAP_RASTER_PALETTES_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const AUXILIARY_COMMANDS_ID: &str = "engine.auxiliary-script-command-directory";
pub const AUXILIARY_COMMANDS_FILE: &str = "auxiliary-script-command-directory.json";
pub const AUXILIARY_COMMANDS_FORMAT: &str = "auxiliary-script-command-directory-v1-json";
pub const MAX_AUXILIARY_COMMANDS_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const AUXILIARY_ARCHIVE_ID: &str = "engine.auxiliary-script-archive";
pub const AUXILIARY_ARCHIVE_FILE: &str = "auxiliary-script-archive.json";
pub const AUXILIARY_ARCHIVE_FORMAT: &str = "auxiliary-script-archive-v1-json";
pub const MAX_AUXILIARY_ARCHIVE_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const ITEM_LINKS_ID: &str = "gameplay.item-links";
pub const ITEM_LINKS_FILE: &str = "item-links.json";
pub const ITEM_LINKS_FORMAT: &str = "item-links-v1-json";
pub const MAX_ITEM_LINKS_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const ITEM_RECORDS_ID: &str = "gameplay.item-records";
pub const ITEM_RECORDS_FILE: &str = "items.json";
pub const ITEM_RECORDS_FORMAT: &str = "item-records-v1-json";
pub const MAX_ITEM_RECORDS_FILE_BYTES: u64 = 2 * 1024 * 1024;
pub const UI_CUSTOM_GRAPHICS_ID: &str = "graphics.ui-custom-banks";
pub const UI_CUSTOM_GRAPHICS_FILE: &str = "ui-graphics.json";
pub const UI_CUSTOM_GRAPHICS_FORMAT: &str = "ui-custom-banks-v1-json";
pub const MAX_UI_CUSTOM_GRAPHICS_FILE_BYTES: u64 = 16 * 1024 * 1024;
pub const UI_RAW_PALETTES_ID: &str = "graphics.ui-raw-palettes";
pub const UI_RAW_PALETTES_FILE: &str = "ui-palettes.json";
pub const UI_RAW_PALETTES_FORMAT: &str = "ui-raw-palettes-v1-json";
pub const MAX_UI_RAW_PALETTES_FILE_BYTES: u64 = 16 * 1024 * 1024;
pub const OBJ_CUSTOM_GRAPHICS_ID: &str = "graphics.obj-custom-banks";
pub const OBJ_CUSTOM_GRAPHICS_FILE: &str = "obj-graphics.json";
pub const OBJ_CUSTOM_GRAPHICS_FORMAT: &str = "obj-custom-banks-v1-json";
pub const MAX_OBJ_CUSTOM_GRAPHICS_FILE_BYTES: u64 = 16 * 1024 * 1024;
pub const OBJ_ANIMATION_BANKS_ID: &str = "graphics.obj-animation-banks";
pub const OBJ_ANIMATION_BANKS_FILE: &str = "obj-animation-banks.json";
pub const OBJ_ANIMATION_BANKS_FORMAT: &str = "obj-animation-banks-v1-json";
pub const MAX_OBJ_ANIMATION_BANKS_FILE_BYTES: u64 = 4 * 1024 * 1024;
pub const OBJ_STATIC_LISTS_ID: &str = "graphics.obj-static-lists";
pub const OBJ_STATIC_LISTS_FILE: &str = "obj-static-lists.json";
pub const OBJ_STATIC_LISTS_FORMAT: &str = "obj-static-lists-v1-json";
pub const MAX_OBJ_STATIC_LISTS_FILE_BYTES: u64 = 4 * 1024 * 1024;
pub const OBJ_DIMENSIONS_ID: &str = "graphics.obj-dimensions";
pub const OBJ_DIMENSIONS_FILE: &str = "obj-dimensions.toml";
pub const BG_DESCRIPTORS_ID: &str = "graphics.bg-descriptors";
pub const BG_DESCRIPTORS_FILE: &str = "bg-descriptors.json";
pub const BG_DESCRIPTORS_FORMAT: &str = "bg-descriptors-json-v2";
pub const BG_BASE_PALETTES_ID: &str = "graphics.bg-base-palettes";
pub const BG_BASE_PALETTES_FILE: &str = "bg-base-palettes.json";
pub const BG_BASE_PALETTES_FORMAT: &str = "bg-base-palettes-v2-json";
pub const BG_ALTERNATE_PALETTES_ID: &str = "graphics.bg-alternate-palettes";
pub const BG_ALTERNATE_PALETTES_FILE: &str = "bg-alternate-palettes.json";
pub const BG_ALTERNATE_PALETTES_FORMAT: &str = "bg-alternate-palettes-v1-json";
pub const BG_PACKET_PROGRAMS_ID: &str = "graphics.bg-packet-programs";
pub const BG_PACKET_PROGRAMS_FILE: &str = "bg-packet-programs.json";
pub const BG_PACKET_PROGRAMS_FORMAT: &str = "bg-packet-programs-v1-json";
pub const BG_MAIN_GRAPHICS_ID: &str = "graphics.bg-main-graphics";
pub const BG_MAIN_GRAPHICS_FILE: &str = "bg-main-graphics.json";
pub const BG_MAIN_GRAPHICS_FORMAT: &str = "bg-main-graphics-v1-json";
pub const MAX_BG_MAIN_GRAPHICS_FILE_BYTES: u64 = 32 * 1024 * 1024;
pub const FONT_BANK_ID: &str = "font.complete-bank";
pub const FONT_BANK_FILE: &str = "font-bank.json";
pub const FONT_BANK_FORMAT: &str = "font-bank-v1-json";
pub const MAX_FONT_BANK_FILE_BYTES: u64 = 16 * 1024 * 1024;
pub const NAME_KEYBOARD_ID: &str = "text.name-keyboard";
pub const NAME_KEYBOARD_FILE: &str = "name-keyboard.json";
pub const NAME_KEYBOARD_FORMAT: &str = "name-keyboard-v1-json";
pub const MAX_NAME_KEYBOARD_FILE_BYTES: u64 = 1 * 1024 * 1024;
pub const TEXT_CATALOGS_ID: &str = "text.catalogs";
pub const TEXT_CATALOGS_FILE: &str = "text-catalogs.json";
pub const TEXT_CATALOGS_FORMAT: &str = "text-catalogs-v1-json";
pub const MAX_TEXT_CATALOGS_FILE_BYTES: u64 = 512 * 1024 * 1024;
pub const SHORT_TEXTS_ID: &str = "text.short-strings";
pub const SHORT_TEXTS_FILE: &str = "short-texts.json";
pub const SHORT_TEXTS_FORMAT: &str = "short-texts-v1-json";
pub const MAX_SHORT_TEXTS_FILE_BYTES: u64 = 8 * 1024 * 1024;
pub const BG_TRANSFER_PROGRAMS_ID: &str = "graphics.bg-transfer-programs";
pub const BG_TRANSFER_PROGRAMS_FILE: &str = "bg-transfer-programs.json";
pub const BG_TRANSFER_PROGRAMS_FORMAT: &str = "bg-transfer-programs-v1-json";
pub const MAX_BG_TRANSFER_PROGRAMS_FILE_BYTES: u64 = 32 * 1024 * 1024;
pub const MAX_BG_PACKET_PROGRAMS_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const MAX_BG_ALTERNATE_PALETTES_FILE_BYTES: u64 = 4 * 1024 * 1024;
pub const MAX_MANIFEST_FILE_BYTES: u64 = 16 * 1024 * 1024;
pub const MAX_BG_BASE_PALETTES_FILE_BYTES: u64 = 1024 * 1024;
pub const MANIFEST_FILE: &str = "manifest.toml";
pub const MAX_FILE_BYTES: u64 = 16 * 1024;
pub const MAX_BG_DESCRIPTORS_FILE_BYTES: u64 = 256 * 1024;
pub const MAX_DATASET_BYTES: usize = 512 * 1024 * 1024;
pub const DIMENSIONS_SHA256: &str =
    "6ff886b23cde2726a4ceb9cba8c5569bbb4dc6a1185769547f78dfbcb7e2d991";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjSize {
    pub width: u16,
    pub height: u16,
}

/// Shape-major, size-minor indexed metadata. Shape 3 has no record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjDimensions {
    pub entries: [ObjSize; 12],
}

impl ObjDimensions {
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() != 48 {
            return Err("OBJ dimensions require exactly 48 source bytes".into());
        }
        let result = Self {
            entries: std::array::from_fn(|i| ObjSize {
                width: u16::from_le_bytes([bytes[i * 4], bytes[i * 4 + 1]]),
                height: u16::from_le_bytes([bytes[i * 4 + 2], bytes[i * 4 + 3]]),
            }),
        };
        result.validate()?;
        Ok(result)
    }

    pub fn validate(&self) -> Result<(), String> {
        // Hardware OBJ extents; the actual base ordering is authenticated by
        // the source digest, independently of these structural constraints.
        if self
            .entries
            .iter()
            .any(|e| !matches!(e.width, 8 | 16 | 32 | 64) || !matches!(e.height, 8 | 16 | 32 | 64))
        {
            return Err("Invalid OBJ dimension (expected 8, 16, 32 or 64 pixels)".into());
        }
        Ok(())
    }

    pub fn source_bytes(&self) -> [u8; 48] {
        let mut out = [0; 48];
        for (i, e) in self.entries.iter().enumerate() {
            out[i * 4..i * 4 + 2].copy_from_slice(&e.width.to_le_bytes());
            out[i * 4 + 2..i * 4 + 4].copy_from_slice(&e.height.to_le_bytes());
        }
        out
    }

    pub fn get(&self, shape: u8, size: u8) -> Option<&ObjSize> {
        if shape >= 3 || size >= 4 {
            return None;
        }
        self.entries.get(shape as usize * 4 + size as usize)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRange {
    pub offset: u32,
    pub length: u32,
    pub sha256: String,
    pub role: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Resource {
    pub id: String,
    pub path: String,
    pub format: String,
    pub sha256: String,
    pub dependencies: Vec<String>,
    pub sources: Vec<SourceRange>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: u32,
    pub extractor_revision: u32,
    pub decoder_revision: u32,
    pub source_rom_sha256: String,
    pub source_rom_size: u32,
    pub profile: String,
    pub coverage: Vec<String>,
    pub resources: Vec<Resource>,
}

/// A source-authenticated extraction. This is not a mod loader.
pub struct Extraction {
    pub manifest: Manifest,
    pub dimensions: ObjDimensions,
    pub background: Option<bg_descriptors::Descriptors>,
    pub palettes: Option<bg_palettes::Catalog>,
    pub alternate_palettes: Option<bg_alternate_palettes::Catalog>,
    pub packet_programs: Option<bg_packet_data::Catalog>,
    pub main_graphics: Option<bg_graphics::Catalog>,
    pub transfers: Option<bg_transfers::Catalog>,
    pub short_texts: Option<text_short::Catalog>,
    pub font_bank: Option<fonts::Catalog>,
    pub name_keyboard: Option<name_keyboard::Catalog>,
    pub obj_lists: Option<obj_lists::Catalog>,
    pub obj_animations: Option<obj_animations::Catalog>,
    pub obj_graphics: Option<obj_graphics::Catalog>,
    pub ui_palettes: Option<ui_palettes::Catalog>,
    pub ui_graphics: Option<ui_graphics::Catalog>,
    pub items: Option<items::Catalog>,
    pub item_links: Option<item_links::Catalog>,
    pub metadata16: Option<metadata16::Catalog>,
    pub metadata52: Option<metadata52::Catalog>,
    pub metadata4: Option<metadata4::Catalog>,
    pub metadata2: Option<metadata2::Catalog>,
    pub default48: Option<default48::Catalog>,
    pub constructor_alternates: Option<constructor_alternates::Catalog>,
    pub metadata14: Option<metadata14::Catalog>,
    pub metadata12: Option<metadata12::Catalog>,
    pub global_pair_flags: Option<global_pair_flags::Catalog>,
    pub evaluation_parameters: Option<evaluation_parameters::Catalog>,
    pub progression_rewards: Option<progression_rewards::Catalog>,
    pub progression_gates: Option<progression_gates::Catalog>,
    pub progression_selection_gates: Option<progression_selection_gates::Catalog>,
    pub aggregate_flag_keys: Option<aggregate_flag_keys::Catalog>,
    pub trade_parameters: Option<trade_parameters::Catalog>,
    pub item_effect_parameters: Option<item_effect_parameters::Catalog>,
    pub audio_directory: Option<audio_directory::Catalog>,
    pub audio_tones: Option<audio_tones::Catalog>,
    pub audio_sequences: Option<audio_sequences::Catalog>,
    pub audio_driver: Option<audio_driver::Catalog>,
    pub palette_effect_dispatch: Option<palette_effect_dispatch::Catalog>,
    pub palette_programs: Option<palette_programs::Catalog>,
    pub blend_programs: Option<blend_programs::Catalog>,
    pub task_descriptors: Option<task_descriptors::Catalog>,
    pub effect_event_directories: Option<effect_event_directories::Catalog>,
    pub script_commands: Option<script_commands::Catalog>,
    pub script_archive: Option<script_archive::Catalog>,
    pub script_text: Option<script_text::Catalog>,
    pub script_coordinates: Option<script_coordinates::Catalog>,
    pub ui_choice_lists: Option<ui_choice_lists::Catalog>,
    pub unit_obj_initial_pose: Option<unit_obj_initial_pose::Catalog>,
    pub color_mode_palettes: Option<color_mode_palettes::Catalog>,
    pub unit_obj_equipment: Option<unit_obj_equipment::Catalog>,
    pub unit_obj_normal_battle: Option<unit_obj_normal_battle::Catalog>,
    pub unit_obj_alternate_battle: Option<unit_obj_alternate_battle::Catalog>,
    pub map_raster_palettes: Option<map_raster_palettes::Catalog>,
    pub auxiliary_scripts: Option<auxiliary_scripts::Catalog>,
    pub text_catalogs: Option<text_catalogs::Catalog>,
}

/// Typed values borrowed from the exact captured file bytes. The frontend
/// parses them once; source validation and runtime binding cover the whole set.
#[derive(Clone, Copy)]
pub struct BaseFiles<'a> {
    pub manifest: &'a Manifest,
    pub dimensions: (&'a ObjDimensions, &'a [u8]),
    pub background: Option<(&'a bg_descriptors::Descriptors, &'a [u8])>,
    pub palettes: Option<(&'a bg_palettes::Palettes, &'a [u8])>,
    pub alternate_palettes: Option<(&'a bg_alternate_palettes::Palettes, &'a [u8])>,
    pub packet_programs: Option<(&'a bg_packet_data::Programs, &'a [u8])>,
    pub main_graphics: Option<(&'a bg_graphics::Graphics, &'a [u8])>,
    pub transfers: Option<(&'a bg_transfers::Transfers, &'a [u8])>,
    pub short_texts: Option<(&'a text_short::ShortTexts, &'a [u8])>,
    pub font_bank: Option<(&'a fonts::FontBank, &'a [u8])>,
    pub name_keyboard: Option<(&'a name_keyboard::Keyboard, &'a [u8])>,
    pub obj_lists: Option<(&'a obj_lists::Lists, &'a [u8])>,
    pub obj_animations: Option<(&'a obj_animations::Banks, &'a [u8])>,
    pub obj_graphics: Option<(&'a obj_graphics::Graphics, &'a [u8])>,
    pub ui_palettes: Option<(&'a ui_palettes::Palettes, &'a [u8])>,
    pub ui_graphics: Option<(&'a ui_graphics::Graphics, &'a [u8])>,
    pub items: Option<(&'a items::Items, &'a [u8])>,
    pub item_links: Option<(&'a item_links::Links, &'a [u8])>,
    pub metadata16: Option<(&'a metadata16::Metadata16, &'a [u8])>,
    pub metadata52: Option<(&'a metadata52::Metadata52, &'a [u8])>,
    pub metadata4: Option<(&'a metadata4::Metadata4, &'a [u8])>,
    pub metadata2: Option<(&'a metadata2::Metadata2, &'a [u8])>,
    pub default48: Option<(&'a default48::Template, &'a [u8])>,
    pub constructor_alternates: Option<(&'a constructor_alternates::Alternates, &'a [u8])>,
    pub metadata14: Option<(&'a metadata14::Metadata14, &'a [u8])>,
    pub metadata12: Option<(&'a metadata12::Metadata12, &'a [u8])>,
    pub global_pair_flags: Option<(&'a global_pair_flags::GlobalPairFlags, &'a [u8])>,
    pub evaluation_parameters: Option<(&'a evaluation_parameters::EvaluationParameters, &'a [u8])>,
    pub progression_rewards: Option<(&'a progression_rewards::ProgressionRewards, &'a [u8])>,
    pub progression_gates: Option<(&'a progression_gates::ProgressionGates, &'a [u8])>,
    pub progression_selection_gates: Option<(&'a progression_selection_gates::ProgressionSelectionGates, &'a [u8])>,
    pub aggregate_flag_keys: Option<(&'a aggregate_flag_keys::AggregateFlagKeys, &'a [u8])>,
    pub trade_parameters: Option<(&'a trade_parameters::TradeParameters, &'a [u8])>,
    pub item_effect_parameters: Option<(&'a item_effect_parameters::ItemEffectParameters, &'a [u8])>,
    pub audio_directory: Option<(&'a audio_directory::AudioDirectory, &'a [u8])>,
    pub audio_tones: Option<(&'a audio_tones::AudioTones, &'a [u8])>,
    pub audio_sequences: Option<(&'a audio_sequences::AudioSequences, &'a [u8])>,
    pub audio_driver: Option<(&'a audio_driver::AudioDriver, &'a [u8])>,
    pub palette_effect_dispatch: Option<(&'a palette_effect_dispatch::PaletteEffectDispatch, &'a [u8])>,
    pub palette_programs: Option<(&'a palette_programs::PalettePrograms, &'a [u8])>,
    pub blend_programs: Option<(&'a blend_programs::BlendPrograms, &'a [u8])>,
    pub task_descriptors: Option<(&'a task_descriptors::TaskDescriptors, &'a [u8])>,
    pub effect_event_directories: Option<(&'a effect_event_directories::EffectEventDirectories, &'a [u8])>,
    pub script_commands: Option<(&'a script_commands::ScriptCommands, &'a [u8])>,
    pub script_archive: Option<(&'a script_archive::ScriptArchive, &'a [u8])>,
    pub script_text: Option<(&'a script_text::ScriptText, &'a [u8])>,
    pub script_coordinates: Option<(&'a script_coordinates::ScriptCoordinates, &'a [u8])>,
    pub ui_choice_lists: Option<(&'a ui_choice_lists::UiChoiceLists, &'a [u8])>,
    pub unit_obj_initial_pose: Option<(&'a unit_obj_initial_pose::UnitObjInitialPose, &'a [u8])>,
    pub color_mode_palettes: Option<(&'a color_mode_palettes::ColorModePalettes, &'a [u8])>,
    pub unit_obj_equipment: Option<(&'a unit_obj_equipment::UnitObjEquipment, &'a [u8])>,
    pub unit_obj_normal_battle: Option<(&'a unit_obj_normal_battle::UnitObjNormalBattle, &'a [u8])>,
    pub unit_obj_alternate_battle: Option<(&'a unit_obj_alternate_battle::UnitObjAlternateBattle, &'a [u8])>,
    pub map_raster_palettes: Option<(&'a map_raster_palettes::MapRasterPalettes, &'a [u8])>,
    pub auxiliary_commands: Option<(&'a auxiliary_scripts::AuxiliaryCommands, &'a [u8])>,
    pub auxiliary_archive: Option<(&'a auxiliary_scripts::AuxiliaryArchive, &'a [u8])>,
    pub text_catalogs: Option<(&'a text_catalogs::TextCatalogs, &'a [u8])>,
}

impl Extraction {
    pub fn from_rom(rom: &[u8]) -> Result<Self, String> {
        let hash = ffta_rom::digest(rom);
        let (profile, ranges): (&str, &[(u32, &str)]) = match hash.as_str() {
            ffta_rom::SHA256 => ("usa", &[(0x149510, "direct"), (0xa398dc, "iwram-copy")]),
            ffta_rom::EUROPE_SHA256 => (
                "europe",
                &[(0x14ced8, "rom-table"), (0xd976e8, "iwram-copy")],
            ),
            _ => return Err("Decoded resources require a verified FFTA EUR or USA ROM".into()),
        };
        let catalog = bg_descriptors::Catalog::from_rom(rom)?;
        let palettes = bg_palettes::Catalog::from_descriptors(rom, &catalog)?;
        let alternates = bg_alternate_palettes::Catalog::from_rom(rom)?;
        let packet_programs = bg_packet_data::Catalog::from_rom(rom)?;
        let mut result = Self::from_ranges(rom, hash, profile, ranges, DIMENSIONS_SHA256)?;
        result.background = Some(catalog.decoded);
        result.manifest.coverage.push(BG_DESCRIPTORS_ID.into());
        result.manifest.resources.push(Resource {
            id: BG_DESCRIPTORS_ID.into(),
            path: BG_DESCRIPTORS_FILE.into(),
            format: BG_DESCRIPTORS_FORMAT.into(),
            sha256: String::new(),
            dependencies: Vec::new(),
            sources: vec![catalog.source],
        });
        result.manifest.coverage.push(BG_BASE_PALETTES_ID.into());
        result.manifest.resources.push(Resource {
            id: BG_BASE_PALETTES_ID.into(),
            path: BG_BASE_PALETTES_FILE.into(),
            format: BG_BASE_PALETTES_FORMAT.into(),
            sha256: String::new(),
            dependencies: vec![BG_DESCRIPTORS_ID.into()],
            sources: palettes
                .sources
                .iter()
                .map(|source| source.range.clone())
                .collect(),
        });
        result.palettes = Some(palettes);
        result.manifest.coverage.push(BG_ALTERNATE_PALETTES_ID.into());
        result.manifest.resources.push(Resource {
            id: BG_ALTERNATE_PALETTES_ID.into(),
            path: BG_ALTERNATE_PALETTES_FILE.into(),
            format: BG_ALTERNATE_PALETTES_FORMAT.into(),
            sha256: String::new(),
            dependencies: vec![BG_DESCRIPTORS_ID.into()],
            sources: alternates.tables.iter().chain(&alternates.sources).cloned().collect(),
        });
        result.alternate_palettes = Some(alternates);
        result.manifest.coverage.push(BG_PACKET_PROGRAMS_ID.into());
        result.manifest.resources.push(Resource {
            id: BG_PACKET_PROGRAMS_ID.into(),
            path: BG_PACKET_PROGRAMS_FILE.into(),
            format: BG_PACKET_PROGRAMS_FORMAT.into(),
            sha256: String::new(),
            dependencies: vec![BG_DESCRIPTORS_ID.into()],
            sources: packet_programs.sources.clone(),
        });
        result.packet_programs = Some(packet_programs);
        // User priority is EUR. Preserve USA's existing dataset contract until
        // its graphics catalog and new managed path are independently verified.
        if profile == "europe" {
            let graphics = bg_graphics::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(BG_MAIN_GRAPHICS_ID.into());
            result.manifest.resources.push(Resource {
                id: BG_MAIN_GRAPHICS_ID.into(), path: BG_MAIN_GRAPHICS_FILE.into(),
                format: BG_MAIN_GRAPHICS_FORMAT.into(), sha256: String::new(),
                dependencies: vec![BG_DESCRIPTORS_ID.into()], sources: graphics.sources.clone(),
            });
            result.main_graphics = Some(graphics);
            let transfers = bg_transfers::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(BG_TRANSFER_PROGRAMS_ID.into());
            result.manifest.resources.push(Resource {
                id: BG_TRANSFER_PROGRAMS_ID.into(), path: BG_TRANSFER_PROGRAMS_FILE.into(),
                format: BG_TRANSFER_PROGRAMS_FORMAT.into(), sha256: String::new(),
                dependencies: vec![BG_DESCRIPTORS_ID.into()], sources: transfers.sources.clone(),
            });
            result.transfers = Some(transfers);
            let short_texts = text_short::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(SHORT_TEXTS_ID.into());
            result.manifest.resources.push(Resource {
                id: SHORT_TEXTS_ID.into(), path: SHORT_TEXTS_FILE.into(),
                format: SHORT_TEXTS_FORMAT.into(), sha256: String::new(),
                dependencies: Vec::new(), sources: short_texts.sources.clone(),
            });
            result.short_texts = Some(short_texts);
            let texts = text_catalogs::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(TEXT_CATALOGS_ID.into());
            result.manifest.resources.push(Resource {
                id: TEXT_CATALOGS_ID.into(), path: TEXT_CATALOGS_FILE.into(), format: TEXT_CATALOGS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![SHORT_TEXTS_ID.into()], sources: texts.sources.clone(),
            });
            result.text_catalogs = Some(texts);
            let font_bank = fonts::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(FONT_BANK_ID.into());
            result.manifest.resources.push(Resource {
                id: FONT_BANK_ID.into(), path: FONT_BANK_FILE.into(), format: FONT_BANK_FORMAT.into(),
                sha256: String::new(), dependencies: vec![TEXT_CATALOGS_ID.into()],
                sources: font_bank.sources.clone(),
            });
            result.font_bank = Some(font_bank);
            let name_keyboard = name_keyboard::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(NAME_KEYBOARD_ID.into());
            result.manifest.resources.push(Resource {
                id: NAME_KEYBOARD_ID.into(), path: NAME_KEYBOARD_FILE.into(), format: NAME_KEYBOARD_FORMAT.into(),
                sha256: String::new(), dependencies: vec![FONT_BANK_ID.into()],
                sources: name_keyboard.sources.clone(),
            });
            result.name_keyboard = Some(name_keyboard);
            let obj_lists = obj_lists::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(OBJ_STATIC_LISTS_ID.into());
            result.manifest.resources.push(Resource {
                id: OBJ_STATIC_LISTS_ID.into(), path: OBJ_STATIC_LISTS_FILE.into(), format: OBJ_STATIC_LISTS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![OBJ_DIMENSIONS_ID.into()], sources: obj_lists.sources.clone(),
            });
            result.obj_lists = Some(obj_lists);
            let animations = obj_animations::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(OBJ_ANIMATION_BANKS_ID.into());
            result.manifest.resources.push(Resource {
                id: OBJ_ANIMATION_BANKS_ID.into(), path: OBJ_ANIMATION_BANKS_FILE.into(), format: OBJ_ANIMATION_BANKS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![OBJ_DIMENSIONS_ID.into()], sources: animations.sources.clone(),
            });
            result.obj_animations = Some(animations);
            let graphics = obj_graphics::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(OBJ_CUSTOM_GRAPHICS_ID.into());
            result.manifest.resources.push(Resource {
                id: OBJ_CUSTOM_GRAPHICS_ID.into(), path: OBJ_CUSTOM_GRAPHICS_FILE.into(), format: OBJ_CUSTOM_GRAPHICS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![OBJ_DIMENSIONS_ID.into()], sources: graphics.sources.clone(),
            });
            result.obj_graphics = Some(graphics);
            let palettes = ui_palettes::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(UI_RAW_PALETTES_ID.into());
            result.manifest.resources.push(Resource {
                id: UI_RAW_PALETTES_ID.into(), path: UI_RAW_PALETTES_FILE.into(), format: UI_RAW_PALETTES_FORMAT.into(),
                sha256: String::new(), dependencies: vec![OBJ_DIMENSIONS_ID.into()], sources: palettes.sources.clone(),
            });
            result.ui_palettes = Some(palettes);
            let graphics = ui_graphics::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(UI_CUSTOM_GRAPHICS_ID.into());
            result.manifest.resources.push(Resource {
                id: UI_CUSTOM_GRAPHICS_ID.into(), path: UI_CUSTOM_GRAPHICS_FILE.into(), format: UI_CUSTOM_GRAPHICS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![OBJ_DIMENSIONS_ID.into(), OBJ_CUSTOM_GRAPHICS_ID.into()], sources: graphics.sources.clone(),
            });
            result.ui_graphics = Some(graphics);
            let items = items::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(ITEM_RECORDS_ID.into());
            result.manifest.resources.push(Resource {
                id: ITEM_RECORDS_ID.into(), path: ITEM_RECORDS_FILE.into(), format: ITEM_RECORDS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![OBJ_DIMENSIONS_ID.into()], sources: items.sources.clone(),
            });
            result.items = Some(items);
            let links = item_links::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(ITEM_LINKS_ID.into());
            result.manifest.resources.push(Resource {
                id: ITEM_LINKS_ID.into(), path: ITEM_LINKS_FILE.into(), format: ITEM_LINKS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![ITEM_RECORDS_ID.into()], sources: links.sources.clone(),
            });
            result.item_links = Some(links);
            let metadata = metadata16::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(METADATA16_ID.into());
            result.manifest.resources.push(Resource {
                id: METADATA16_ID.into(), path: METADATA16_FILE.into(), format: METADATA16_FORMAT.into(),
                sha256: String::new(), dependencies: vec![OBJ_DIMENSIONS_ID.into()], sources: metadata.sources.clone(),
            });
            result.metadata16 = Some(metadata);
            let metadata = metadata52::Catalog::from_rom(rom)?;
            metadata.decoded.validate_links(&result.item_links.as_ref().unwrap().decoded)?;
            result.manifest.coverage.push(METADATA52_ID.into());
            result.manifest.resources.push(Resource {
                id: METADATA52_ID.into(), path: METADATA52_FILE.into(), format: METADATA52_FORMAT.into(),
                sha256: String::new(), dependencies: vec![ITEM_LINKS_ID.into()], sources: metadata.sources.clone(),
            });
            result.metadata52 = Some(metadata);
            let metadata = metadata4::Catalog::from_rom(rom)?;
            metadata.decoded.validate_metadata52(&result.metadata52.as_ref().unwrap().decoded)?;
            result.manifest.coverage.push(METADATA4_ID.into());
            result.manifest.resources.push(Resource {
                id: METADATA4_ID.into(), path: METADATA4_FILE.into(), format: METADATA4_FORMAT.into(),
                sha256: String::new(), dependencies: vec![METADATA52_ID.into()], sources: metadata.sources.clone(),
            });
            result.metadata4 = Some(metadata);
            let metadata = metadata2::Catalog::from_rom(rom)?;
            metadata.decoded.validate_metadata52(&result.metadata52.as_ref().unwrap().decoded)?;
            result.manifest.coverage.push(METADATA2_ID.into());
            result.manifest.resources.push(Resource {
                id: METADATA2_ID.into(), path: METADATA2_FILE.into(), format: METADATA2_FORMAT.into(),
                sha256: String::new(), dependencies: vec![METADATA52_ID.into()], sources: metadata.sources.clone(),
            });
            result.metadata2 = Some(metadata);
            let metadata = default48::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(DEFAULT48_ID.into());
            result.manifest.resources.push(Resource {
                id: DEFAULT48_ID.into(), path: DEFAULT48_FILE.into(), format: DEFAULT48_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.default48 = Some(metadata);
            let metadata = constructor_alternates::Catalog::from_rom(rom)?;
            metadata.decoded.validate_items(&result.items.as_ref().unwrap().decoded)?;
            result.manifest.coverage.push(CONSTRUCTOR_ALTERNATES_ID.into());
            result.manifest.resources.push(Resource {
                id: CONSTRUCTOR_ALTERNATES_ID.into(), path: CONSTRUCTOR_ALTERNATES_FILE.into(), format: CONSTRUCTOR_ALTERNATES_FORMAT.into(),
                sha256: String::new(), dependencies: vec![ITEM_RECORDS_ID.into()], sources: metadata.sources.clone(),
            });
            result.constructor_alternates = Some(metadata);
            let metadata = metadata14::Catalog::from_rom(rom)?;
            metadata.decoded.validate_metadata52(&result.metadata52.as_ref().unwrap().decoded)?;
            result.manifest.coverage.push(METADATA14_ID.into());
            result.manifest.resources.push(Resource {
                id: METADATA14_ID.into(), path: METADATA14_FILE.into(), format: METADATA14_FORMAT.into(),
                sha256: String::new(), dependencies: vec![METADATA52_ID.into()], sources: metadata.sources.clone(),
            });
            result.metadata14 = Some(metadata);
            let metadata = metadata12::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(METADATA12_ID.into());
            result.manifest.resources.push(Resource {
                id: METADATA12_ID.into(), path: METADATA12_FILE.into(), format: METADATA12_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.metadata12 = Some(metadata);
            let metadata = global_pair_flags::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(GLOBAL_PAIR_FLAGS_ID.into());
            result.manifest.resources.push(Resource {
                id: GLOBAL_PAIR_FLAGS_ID.into(), path: GLOBAL_PAIR_FLAGS_FILE.into(), format: GLOBAL_PAIR_FLAGS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.global_pair_flags = Some(metadata);
            let metadata = evaluation_parameters::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(EVALUATION_PARAMETERS_ID.into());
            result.manifest.resources.push(Resource {
                id: EVALUATION_PARAMETERS_ID.into(), path: EVALUATION_PARAMETERS_FILE.into(), format: EVALUATION_PARAMETERS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.evaluation_parameters = Some(metadata);
            let metadata = progression_rewards::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(PROGRESSION_REWARDS_ID.into());
            result.manifest.resources.push(Resource {
                id: PROGRESSION_REWARDS_ID.into(), path: PROGRESSION_REWARDS_FILE.into(), format: PROGRESSION_REWARDS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.progression_rewards = Some(metadata);
            let metadata = progression_gates::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(PROGRESSION_GATES_ID.into());
            result.manifest.resources.push(Resource {
                id: PROGRESSION_GATES_ID.into(), path: PROGRESSION_GATES_FILE.into(), format: PROGRESSION_GATES_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.progression_gates = Some(metadata);
            let metadata = progression_selection_gates::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(PROGRESSION_SELECTION_GATES_ID.into());
            result.manifest.resources.push(Resource {
                id: PROGRESSION_SELECTION_GATES_ID.into(), path: PROGRESSION_SELECTION_GATES_FILE.into(), format: PROGRESSION_SELECTION_GATES_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.progression_selection_gates = Some(metadata);
            let metadata = aggregate_flag_keys::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(AGGREGATE_FLAG_KEYS_ID.into());
            result.manifest.resources.push(Resource {
                id: AGGREGATE_FLAG_KEYS_ID.into(), path: AGGREGATE_FLAG_KEYS_FILE.into(), format: AGGREGATE_FLAG_KEYS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.aggregate_flag_keys = Some(metadata);
            let metadata = trade_parameters::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(TRADE_PARAMETERS_ID.into());
            result.manifest.resources.push(Resource {
                id: TRADE_PARAMETERS_ID.into(), path: TRADE_PARAMETERS_FILE.into(), format: TRADE_PARAMETERS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.trade_parameters = Some(metadata);
            let metadata = item_effect_parameters::Catalog::from_rom(rom)?;
            metadata.decoded.validate_items(&result.items.as_ref().unwrap().decoded)?;
            result.manifest.coverage.push(ITEM_EFFECT_PARAMETERS_ID.into());
            result.manifest.resources.push(Resource {
                id: ITEM_EFFECT_PARAMETERS_ID.into(), path: ITEM_EFFECT_PARAMETERS_FILE.into(), format: ITEM_EFFECT_PARAMETERS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![ITEM_RECORDS_ID.into()], sources: metadata.sources.clone(),
            });
            result.item_effect_parameters = Some(metadata);
            let metadata = audio_directory::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(AUDIO_DIRECTORY_ID.into());
            result.manifest.resources.push(Resource {
                id: AUDIO_DIRECTORY_ID.into(), path: AUDIO_DIRECTORY_FILE.into(), format: AUDIO_DIRECTORY_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.audio_directory = Some(metadata);
            let metadata = audio_tones::Catalog::from_rom(rom)?;
            metadata.decoded.validate_directory(&result.audio_directory.as_ref().unwrap().decoded)?;
            result.manifest.coverage.push(AUDIO_TONES_ID.into());
            result.manifest.resources.push(Resource {
                id: AUDIO_TONES_ID.into(), path: AUDIO_TONES_FILE.into(), format: AUDIO_TONES_FORMAT.into(),
                sha256: String::new(), dependencies: vec![AUDIO_DIRECTORY_ID.into()], sources: metadata.sources.clone(),
            });
            result.audio_tones = Some(metadata);
            let metadata = audio_sequences::Catalog::from_rom(rom)?;
            metadata.decoded.validate_owners(&result.audio_directory.as_ref().unwrap().decoded,&result.audio_tones.as_ref().unwrap().decoded)?;
            result.manifest.coverage.push(AUDIO_SEQUENCES_ID.into());
            result.manifest.resources.push(Resource {
                id: AUDIO_SEQUENCES_ID.into(), path: AUDIO_SEQUENCES_FILE.into(), format: AUDIO_SEQUENCES_FORMAT.into(),
                sha256: String::new(), dependencies: vec![AUDIO_DIRECTORY_ID.into(),AUDIO_TONES_ID.into()], sources: metadata.sources.clone(),
            });
            result.audio_sequences = Some(metadata);
            let metadata = audio_driver::Catalog::from_rom(rom)?;
            metadata.decoded.validate_sequence_owner(&result.audio_sequences.as_ref().unwrap().decoded)?;
            result.manifest.coverage.push(AUDIO_DRIVER_ID.into());
            result.manifest.resources.push(Resource {
                id: AUDIO_DRIVER_ID.into(), path: AUDIO_DRIVER_FILE.into(), format: AUDIO_DRIVER_FORMAT.into(),
                sha256: String::new(), dependencies: vec![AUDIO_SEQUENCES_ID.into()], sources: metadata.sources.clone(),
            });
            result.audio_driver = Some(metadata);
            let metadata = palette_effect_dispatch::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(PALETTE_EFFECT_DISPATCH_ID.into());
            result.manifest.resources.push(Resource {
                id: PALETTE_EFFECT_DISPATCH_ID.into(), path: PALETTE_EFFECT_DISPATCH_FILE.into(), format: PALETTE_EFFECT_DISPATCH_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.palette_effect_dispatch = Some(metadata);
            let metadata = palette_programs::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(PALETTE_PROGRAMS_ID.into());
            result.manifest.resources.push(Resource {
                id: PALETTE_PROGRAMS_ID.into(), path: PALETTE_PROGRAMS_FILE.into(), format: PALETTE_PROGRAMS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.palette_programs = Some(metadata);
            let metadata = blend_programs::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(BLEND_PROGRAMS_ID.into());
            result.manifest.resources.push(Resource {
                id: BLEND_PROGRAMS_ID.into(), path: BLEND_PROGRAMS_FILE.into(), format: BLEND_PROGRAMS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.blend_programs = Some(metadata);
            let metadata = task_descriptors::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(TASK_DESCRIPTORS_ID.into());
            result.manifest.resources.push(Resource {
                id: TASK_DESCRIPTORS_ID.into(), path: TASK_DESCRIPTORS_FILE.into(), format: TASK_DESCRIPTORS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.task_descriptors = Some(metadata);
            let metadata = effect_event_directories::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(EFFECT_EVENT_DIRECTORIES_ID.into());
            result.manifest.resources.push(Resource {
                id: EFFECT_EVENT_DIRECTORIES_ID.into(), path: EFFECT_EVENT_DIRECTORIES_FILE.into(), format: EFFECT_EVENT_DIRECTORIES_FORMAT.into(),
                sha256: String::new(), dependencies: vec![TASK_DESCRIPTORS_ID.into()], sources: metadata.sources.clone(),
            });
            result.effect_event_directories = Some(metadata);
            let metadata = script_commands::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(SCRIPT_COMMANDS_ID.into());
            result.manifest.resources.push(Resource {
                id: SCRIPT_COMMANDS_ID.into(), path: SCRIPT_COMMANDS_FILE.into(), format: SCRIPT_COMMANDS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.script_commands = Some(metadata);
            let metadata = script_archive::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(SCRIPT_ARCHIVE_ID.into());
            result.manifest.resources.push(Resource {
                id: SCRIPT_ARCHIVE_ID.into(), path: SCRIPT_ARCHIVE_FILE.into(), format: SCRIPT_ARCHIVE_FORMAT.into(),
                sha256: String::new(), dependencies: vec![SCRIPT_COMMANDS_ID.into(), EFFECT_EVENT_DIRECTORIES_ID.into()], sources: metadata.sources.clone(),
            });
            result.script_archive = Some(metadata);
            let metadata = script_text::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(SCRIPT_TEXT_ID.into());
            result.manifest.resources.push(Resource {
                id: SCRIPT_TEXT_ID.into(), path: SCRIPT_TEXT_FILE.into(), format: SCRIPT_TEXT_FORMAT.into(),
                sha256: String::new(), dependencies: vec![SCRIPT_ARCHIVE_ID.into(), FONT_BANK_ID.into()], sources: metadata.sources.clone(),
            });
            result.script_text = Some(metadata);
            let metadata = script_coordinates::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(SCRIPT_COORDINATES_ID.into());
            result.manifest.resources.push(Resource {
                id: SCRIPT_COORDINATES_ID.into(), path: SCRIPT_COORDINATES_FILE.into(), format: SCRIPT_COORDINATES_FORMAT.into(),
                sha256: String::new(), dependencies: vec![SCRIPT_ARCHIVE_ID.into(), SCRIPT_COMMANDS_ID.into()], sources: metadata.sources.clone(),
            });
            result.script_coordinates = Some(metadata);
            let metadata = auxiliary_scripts::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(AUXILIARY_COMMANDS_ID.into());
            result.manifest.resources.push(Resource {
                id: AUXILIARY_COMMANDS_ID.into(), path: AUXILIARY_COMMANDS_FILE.into(), format: AUXILIARY_COMMANDS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: vec![metadata.sources[1].clone()],
            });
            result.manifest.coverage.push(AUXILIARY_ARCHIVE_ID.into());
            result.manifest.resources.push(Resource {
                id: AUXILIARY_ARCHIVE_ID.into(), path: AUXILIARY_ARCHIVE_FILE.into(), format: AUXILIARY_ARCHIVE_FORMAT.into(),
                sha256: String::new(), dependencies: vec![AUXILIARY_COMMANDS_ID.into()], sources: vec![metadata.sources[0].clone()],
            });
            result.auxiliary_scripts = Some(metadata);
            let metadata = map_raster_palettes::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(MAP_RASTER_PALETTES_ID.into());
            result.manifest.resources.push(Resource {
                id: MAP_RASTER_PALETTES_ID.into(), path: MAP_RASTER_PALETTES_FILE.into(), format: MAP_RASTER_PALETTES_FORMAT.into(),
                sha256: String::new(), dependencies: vec![BG_DESCRIPTORS_ID.into()], sources: metadata.sources.clone(),
            });
            result.map_raster_palettes = Some(metadata);
            let metadata = ui_choice_lists::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(UI_CHOICE_LISTS_ID.into());
            result.manifest.resources.push(Resource {
                id: UI_CHOICE_LISTS_ID.into(), path: UI_CHOICE_LISTS_FILE.into(), format: UI_CHOICE_LISTS_FORMAT.into(),
                sha256: String::new(), dependencies: vec![], sources: metadata.sources.clone(),
            });
            result.ui_choice_lists = Some(metadata);
            let metadata = unit_obj_initial_pose::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(UNIT_OBJ_INITIAL_POSE_ID.into());
            result.manifest.resources.push(Resource {
                id: UNIT_OBJ_INITIAL_POSE_ID.into(), path: UNIT_OBJ_INITIAL_POSE_FILE.into(), format: UNIT_OBJ_INITIAL_POSE_FORMAT.into(),
                sha256: String::new(), dependencies: vec![METADATA52_ID.into(), METADATA14_ID.into()], sources: metadata.sources.clone(),
            });
            result.unit_obj_initial_pose = Some(metadata);
            let metadata = color_mode_palettes::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(COLOR_MODE_PALETTES_ID.into());
            result.manifest.resources.push(Resource {
                id: COLOR_MODE_PALETTES_ID.into(), path: COLOR_MODE_PALETTES_FILE.into(), format: COLOR_MODE_PALETTES_FORMAT.into(),
                sha256: String::new(), dependencies: vec![METADATA14_ID.into()], sources: metadata.sources.clone(),
            });
            result.color_mode_palettes = Some(metadata);
            let metadata = unit_obj_equipment::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(UNIT_OBJ_EQUIPMENT_ID.into());
            result.manifest.resources.push(Resource {
                id: UNIT_OBJ_EQUIPMENT_ID.into(), path: UNIT_OBJ_EQUIPMENT_FILE.into(), format: UNIT_OBJ_EQUIPMENT_FORMAT.into(),
                sha256: String::new(), dependencies: vec![ITEM_RECORDS_ID.into()], sources: metadata.sources.clone(),
            });
            result.unit_obj_equipment = Some(metadata);
            let metadata = unit_obj_normal_battle::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(UNIT_OBJ_NORMAL_BATTLE_ID.into());
            result.manifest.resources.push(Resource {
                id: UNIT_OBJ_NORMAL_BATTLE_ID.into(), path: UNIT_OBJ_NORMAL_BATTLE_FILE.into(), format: UNIT_OBJ_NORMAL_BATTLE_FORMAT.into(),
                sha256: String::new(), dependencies: vec![METADATA52_ID.into(), METADATA14_ID.into(), UNIT_OBJ_INITIAL_POSE_ID.into(), UNIT_OBJ_EQUIPMENT_ID.into()], sources: metadata.sources.clone(),
            });
            result.unit_obj_normal_battle = Some(metadata);
            let metadata = unit_obj_alternate_battle::Catalog::from_rom(rom)?;
            result.manifest.coverage.push(UNIT_OBJ_ALTERNATE_BATTLE_ID.into());
            result.manifest.resources.push(Resource {
                id: UNIT_OBJ_ALTERNATE_BATTLE_ID.into(), path: UNIT_OBJ_ALTERNATE_BATTLE_FILE.into(), format: UNIT_OBJ_ALTERNATE_BATTLE_FORMAT.into(),
                sha256: String::new(), dependencies: vec![METADATA52_ID.into(), METADATA14_ID.into(), UNIT_OBJ_INITIAL_POSE_ID.into(), UNIT_OBJ_EQUIPMENT_ID.into(), UNIT_OBJ_NORMAL_BATTLE_ID.into()], sources: metadata.sources.clone(),
            });
            result.unit_obj_alternate_battle = Some(metadata);
        } else {
            result.manifest.extractor_revision = 8;
            result.manifest.decoder_revision = 7;
        }
        Ok(result)
    }

    fn from_ranges(
        rom: &[u8],
        hash: String,
        profile: &str,
        ranges: &[(u32, &str)],
        expected: &str,
    ) -> Result<Self, String> {
        let mut sources = Vec::new();
        let mut dimensions = None;
        for &(offset, role) in ranges {
            let start = offset as usize;
            let bytes = rom
                .get(start..start.checked_add(48).ok_or("Source range overflow")?)
                .ok_or("Truncated OBJ dimensions source range")?;
            let sha256 = ffta_rom::digest(bytes);
            if sha256 != expected {
                return Err(format!("OBJ dimensions source mismatch at {offset:#x}"));
            }
            let decoded = ObjDimensions::decode(bytes)?;
            if dimensions.as_ref().is_some_and(|d| d != &decoded) {
                return Err("OBJ dimensions source aliases disagree".into());
            }
            dimensions = Some(decoded);
            sources.push(SourceRange {
                offset,
                length: 48,
                sha256,
                role: role.into(),
            });
        }
        Ok(Self {
            dimensions: dimensions.ok_or("Missing OBJ dimensions source")?,
            background: None,
            palettes: None,
            alternate_palettes: None,
            packet_programs: None,
            main_graphics: None,
            transfers: None,
            short_texts: None,
            text_catalogs: None,
            name_keyboard: None,
            obj_lists: None,
            obj_animations: None,
            obj_graphics: None,
            ui_palettes: None,
            ui_graphics: None,
            items: None,
            item_links: None,
            metadata16: None,
            metadata52: None,
            metadata4: None,
            metadata2: None,
            default48: None,
            constructor_alternates: None,
            metadata14: None,
            metadata12: None,
            global_pair_flags: None,
            evaluation_parameters: None,
            progression_rewards: None,
            progression_gates: None,
            progression_selection_gates: None,
            aggregate_flag_keys: None,
            trade_parameters: None,
            item_effect_parameters: None,
            audio_directory: None,
            audio_tones: None,
            audio_sequences: None,
            audio_driver: None,
            palette_effect_dispatch: None,
            palette_programs: None,
            blend_programs: None,
            task_descriptors: None,
            effect_event_directories: None,
            script_commands: None,
            script_archive: None,
            script_text: None,
            script_coordinates: None,
            ui_choice_lists: None,
            unit_obj_initial_pose: None,
            color_mode_palettes: None,
            unit_obj_equipment: None,
            unit_obj_normal_battle: None,
            unit_obj_alternate_battle: None,
            map_raster_palettes: None,
            auxiliary_scripts: None,
            font_bank: None,
            manifest: Manifest {
                schema: SCHEMA,
                extractor_revision: EXTRACTOR_REV,
                decoder_revision: DECODER_REV,
                source_rom_sha256: hash,
                source_rom_size: u32::try_from(rom.len()).map_err(|_| "ROM is too large")?,
                profile: profile.into(),
                coverage: vec![OBJ_DIMENSIONS_ID.into()],
                resources: vec![Resource {
                    id: OBJ_DIMENSIONS_ID.into(),
                    path: OBJ_DIMENSIONS_FILE.into(),
                    format: "obj-dimensions-v1".into(),
                    // File bytes are defined by the frontend serializer.
                    sha256: String::new(),
                    dependencies: Vec::new(),
                    sources,
                }],
            },
        })
    }

    pub fn set_file_hashes(&mut self, files: &[&[u8]]) -> Result<(), String> {
        if self.manifest.resources.len()
            != self.resource_count()
            || files.len() != self.manifest.resources.len()
        {
            return Err("Resource set does not match extraction".into());
        }
        for (resource, bytes) in self.manifest.resources.iter_mut().zip(files) {
            resource.sha256 = ffta_rom::digest(bytes);
        }
        Ok(())
    }

    /// The expected manifest comes from the selected source, never the files.
    /// A self-consistent edited resource is still an invalid *base* dataset.
    pub fn validate_files(&self, files: BaseFiles<'_>) -> Result<(), String> {
        if self.palettes.is_some() && self.background.is_none()
            || self.alternate_palettes.is_some() && self.palettes.is_none()
            || self.packet_programs.is_some() && self.alternate_palettes.is_none()
            || self.main_graphics.is_some() && self.packet_programs.is_none()
            || self.transfers.is_some() && self.main_graphics.is_none()
            || self.short_texts.is_some() && self.transfers.is_none()
            || self.text_catalogs.is_some() && self.short_texts.is_none()
            || self.font_bank.is_some() && self.text_catalogs.is_none()
            || self.name_keyboard.is_some() && self.font_bank.is_none()
            || self.obj_lists.is_some() && self.name_keyboard.is_none()
            || self.obj_animations.is_some() && self.obj_lists.is_none()
            || self.obj_graphics.is_some() && self.obj_animations.is_none()
            || self.ui_palettes.is_some() && self.obj_graphics.is_none()
            || self.ui_graphics.is_some() && self.ui_palettes.is_none()
            || self.items.is_some() && self.ui_graphics.is_none()
            || self.item_links.is_some() && self.items.is_none()
            || self.metadata16.is_some() && self.item_links.is_none()
            || self.metadata52.is_some() && self.metadata16.is_none()
            || self.metadata4.is_some() && self.metadata52.is_none()
            || self.metadata2.is_some() && self.metadata52.is_none()
            || self.default48.is_some() && self.metadata2.is_none()
            || self.constructor_alternates.is_some() && self.default48.is_none()
            || self.metadata14.is_some() && self.constructor_alternates.is_none()
            || self.metadata12.is_some() && self.metadata14.is_none()
            || self.global_pair_flags.is_some() && self.metadata12.is_none()
            || self.evaluation_parameters.is_some() && self.global_pair_flags.is_none()
            || self.progression_rewards.is_some() && self.evaluation_parameters.is_none()
            || self.progression_gates.is_some() && self.progression_rewards.is_none()
            || self.progression_selection_gates.is_some() && self.progression_gates.is_none()
            || self.aggregate_flag_keys.is_some() && self.progression_selection_gates.is_none()
            || self.trade_parameters.is_some() && self.aggregate_flag_keys.is_none()
            || self.item_effect_parameters.is_some() && self.trade_parameters.is_none()
            || self.audio_directory.is_some() && self.item_effect_parameters.is_none()
            || self.audio_tones.is_some() && self.audio_directory.is_none()
            || self.audio_sequences.is_some() && self.audio_tones.is_none()
            || self.audio_driver.is_some() && self.audio_sequences.is_none()
            || self.palette_effect_dispatch.is_some() && self.audio_driver.is_none()
            || self.palette_programs.is_some() && self.palette_effect_dispatch.is_none()
            || self.blend_programs.is_some() && self.palette_programs.is_none()
            || self.task_descriptors.is_some() && self.blend_programs.is_none()
            || self.effect_event_directories.is_some() && self.task_descriptors.is_none()
            || self.script_commands.is_some() && self.effect_event_directories.is_none()
            || self.script_archive.is_some() && self.script_commands.is_none()
            || self.script_text.is_some() && self.script_archive.is_none()
            || self.script_coordinates.is_some() && self.script_archive.is_none()
            || self.ui_choice_lists.is_some() && self.map_raster_palettes.is_none()
            || self.unit_obj_initial_pose.is_some() && (self.ui_choice_lists.is_none() || self.metadata52.is_none() || self.metadata14.is_none())
            || self.color_mode_palettes.is_some() && (self.unit_obj_initial_pose.is_none() || self.metadata14.is_none())
            || self.unit_obj_equipment.is_some() && (self.color_mode_palettes.is_none() || self.items.is_none())
            || self.unit_obj_normal_battle.is_some() && (self.unit_obj_equipment.is_none() || self.unit_obj_initial_pose.is_none() || self.metadata52.is_none() || self.metadata14.is_none())
            || self.unit_obj_alternate_battle.is_some() && (self.unit_obj_normal_battle.is_none() || self.unit_obj_equipment.is_none() || self.unit_obj_initial_pose.is_none() || self.metadata52.is_none() || self.metadata14.is_none())
            || self.map_raster_palettes.is_some() && self.auxiliary_scripts.is_none()
            || self.auxiliary_scripts.is_some() && self.script_coordinates.is_none()
            || self.manifest.resources.len() != self.resource_count()
        {
            return Err("Extraction resource set is inconsistent".into());
        }
        let (dimensions, file_bytes) = files.dimensions;
        if file_bytes.len() as u64 > MAX_FILE_BYTES {
            return Err("OBJ dimensions file exceeds size limit".into());
        }
        let mut total = file_bytes.len();
        let mut expected = self.manifest.clone();
        expected.resources[0].sha256 = ffta_rom::digest(file_bytes);
        match (&self.background, files.background) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_BG_DESCRIPTORS_FILE_BYTES {
                    return Err("Background descriptor file exceeds size limit".into());
                }
                total = total.saturating_add(bytes.len());
                expected.resources[1].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected background descriptor resource".into()),
        }
        match (&self.palettes, files.palettes) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_BG_BASE_PALETTES_FILE_BYTES {
                    return Err("Base palette file exceeds size limit".into());
                }
                total = total.saturating_add(bytes.len());
                expected.resources[2].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected base palette resource".into()),
        }
        match (&self.alternate_palettes, files.alternate_palettes) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_BG_ALTERNATE_PALETTES_FILE_BYTES {
                    return Err("Alternate palette file exceeds size limit".into());
                }
                total = total.saturating_add(bytes.len());
                expected.resources[3].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected alternate palette resource".into()),
        }
        match (&self.packet_programs, files.packet_programs) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_BG_PACKET_PROGRAMS_FILE_BYTES {
                    return Err("Packet program file exceeds size limit".into());
                }
                total = total.saturating_add(bytes.len());
                expected.resources[4].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected packet program resource".into()),
        }
        match (&self.main_graphics, files.main_graphics) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_BG_MAIN_GRAPHICS_FILE_BYTES {
                    return Err("Main graphics file exceeds size limit".into());
                }
                total = total.saturating_add(bytes.len());
                expected.resources[5].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected main graphics resource".into()),
        }
        match (&self.transfers, files.transfers) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_BG_TRANSFER_PROGRAMS_FILE_BYTES {
                    return Err("Transfer program file exceeds size limit".into());
                }
                total = total.saturating_add(bytes.len());
                expected.resources[6].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected transfer program resource".into()),
        }
        match (&self.short_texts, files.short_texts) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_SHORT_TEXTS_FILE_BYTES {
                    return Err("Short-text file exceeds size limit".into());
                }
                total = total.saturating_add(bytes.len());
                expected.resources[7].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected short-text resource".into()),
        }
        match (&self.text_catalogs, files.text_catalogs) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_TEXT_CATALOGS_FILE_BYTES { return Err("Text catalog file exceeds size limit".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[8].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected text catalog resource".into()),
        }
        match (&self.font_bank, files.font_bank) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_FONT_BANK_FILE_BYTES { return Err("font-bank file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[9].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected font-bank resource".into()),
        }
        match (&self.name_keyboard, files.name_keyboard) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_NAME_KEYBOARD_FILE_BYTES { return Err("name-keyboard file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[10].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected name-keyboard resource".into()),
        }
        match (&self.obj_lists, files.obj_lists) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_OBJ_STATIC_LISTS_FILE_BYTES { return Err("OBJ static-list file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[11].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected OBJ static-list resource".into()),
        }
        match (&self.obj_animations, files.obj_animations) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_OBJ_ANIMATION_BANKS_FILE_BYTES { return Err("OBJ animation file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[12].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected OBJ animation resource".into()),
        }
        match (&self.obj_graphics, files.obj_graphics) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_OBJ_CUSTOM_GRAPHICS_FILE_BYTES { return Err("OBJ custom graphics file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[13].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected OBJ custom graphics resource".into()),
        }
        match (&self.ui_palettes, files.ui_palettes) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_UI_RAW_PALETTES_FILE_BYTES { return Err("UI raw palette file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[14].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected UI raw palette resource".into()),
        }
        match (&self.ui_graphics, files.ui_graphics) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_UI_CUSTOM_GRAPHICS_FILE_BYTES { return Err("UI custom graphics file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[15].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected UI custom graphics resource".into()),
        }
        match (&self.items, files.items) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_ITEM_RECORDS_FILE_BYTES { return Err("Item file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[16].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected item resource".into()),
        }
        match (&self.item_links, files.item_links) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_ITEM_LINKS_FILE_BYTES { return Err("Item-link file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[17].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected item-link resource".into()),
        }
        match (&self.metadata16, files.metadata16) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_METADATA16_FILE_BYTES { return Err("Metadata16 file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[18].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected metadata16 resource".into()),
        }
        match (&self.metadata52, files.metadata52) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_METADATA52_FILE_BYTES { return Err("Metadata52 file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[19].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected metadata52 resource".into()),
        }
        match (&self.metadata4, files.metadata4) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_METADATA4_FILE_BYTES { return Err("Metadata4 file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[20].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected metadata4 resource".into()),
        }
        match (&self.metadata2, files.metadata2) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_METADATA2_FILE_BYTES { return Err("Metadata2 file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[21].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected metadata2 resource".into()),
        }
        match (&self.default48, files.default48) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_DEFAULT48_FILE_BYTES { return Err("Default48 file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[22].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected default48 resource".into()),
        }
        match (&self.constructor_alternates, files.constructor_alternates) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_CONSTRUCTOR_ALTERNATES_FILE_BYTES { return Err("ConstructorAlternates file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[23].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected constructor_alternates resource".into()),
        }
        match (&self.metadata14, files.metadata14) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_METADATA14_FILE_BYTES { return Err("Metadata14 file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[24].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected metadata14 resource".into()),
        }
        match (&self.metadata12, files.metadata12) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_METADATA12_FILE_BYTES { return Err("Metadata12 file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[25].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected metadata12 resource".into()),
        }
        match (&self.global_pair_flags, files.global_pair_flags) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_GLOBAL_PAIR_FLAGS_FILE_BYTES { return Err("GlobalPairFlags file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[26].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected global_pair_flags resource".into()),
        }
        match (&self.evaluation_parameters, files.evaluation_parameters) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_EVALUATION_PARAMETERS_FILE_BYTES { return Err("EvaluationParameters file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[27].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected evaluation_parameters resource".into()),
        }
        match (&self.progression_rewards, files.progression_rewards) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_PROGRESSION_REWARDS_FILE_BYTES { return Err("ProgressionRewards file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[28].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected progression_rewards resource".into()),
        }
        match (&self.progression_gates, files.progression_gates) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_PROGRESSION_GATES_FILE_BYTES { return Err("ProgressionGates file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[29].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected progression_gates resource".into()),
        }
        match (&self.progression_selection_gates, files.progression_selection_gates) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_PROGRESSION_SELECTION_GATES_FILE_BYTES { return Err("ProgressionSelectionGates file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[30].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected progression_selection_gates resource".into()),
        }
        match (&self.aggregate_flag_keys, files.aggregate_flag_keys) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_AGGREGATE_FLAG_KEYS_FILE_BYTES { return Err("AggregateFlagKeys file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[31].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected aggregate_flag_keys resource".into()),
        }
        match (&self.trade_parameters, files.trade_parameters) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_TRADE_PARAMETERS_FILE_BYTES { return Err("TradeParameters file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[32].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected trade_parameters resource".into()),
        }
        match (&self.item_effect_parameters, files.item_effect_parameters) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_ITEM_EFFECT_PARAMETERS_FILE_BYTES { return Err("ItemEffectParameters file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[33].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected item_effect_parameters resource".into()),
        }
        match (&self.audio_directory, files.audio_directory) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_AUDIO_DIRECTORY_FILE_BYTES { return Err("AudioDirectory file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[34].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected audio_directory resource".into()),
        }
        match (&self.audio_tones, files.audio_tones) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_AUDIO_TONES_FILE_BYTES { return Err("AudioTones file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[35].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected audio_tones resource".into()),
        }
        match (&self.audio_sequences, files.audio_sequences) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_AUDIO_SEQUENCES_FILE_BYTES { return Err("AudioSequences file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[36].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected audio_sequences resource".into()),
        }
        match (&self.audio_driver, files.audio_driver) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_AUDIO_DRIVER_FILE_BYTES { return Err("AudioDriver file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[37].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected audio_driver resource".into()),
        }
        match (&self.palette_effect_dispatch, files.palette_effect_dispatch) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_PALETTE_EFFECT_DISPATCH_FILE_BYTES { return Err("PaletteEffectDispatch file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[38].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected palette_effect_dispatch resource".into()),
        }
        match (&self.palette_programs, files.palette_programs) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_PALETTE_PROGRAMS_FILE_BYTES { return Err("PalettePrograms file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[39].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected palette_programs resource".into()),
        }
        match (&self.blend_programs, files.blend_programs) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_BLEND_PROGRAMS_FILE_BYTES { return Err("BlendPrograms file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[40].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected blend_programs resource".into()),
        }
        match (&self.task_descriptors, files.task_descriptors) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_TASK_DESCRIPTORS_FILE_BYTES { return Err("TaskDescriptors file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[41].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected task_descriptors resource".into()),
        }
        match (&self.effect_event_directories, files.effect_event_directories) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_EFFECT_EVENT_DIRECTORIES_FILE_BYTES { return Err("EffectEventDirectories file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[42].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected effect_event_directories resource".into()),
        }
        match (&self.script_commands, files.script_commands) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_SCRIPT_COMMANDS_FILE_BYTES { return Err("ScriptCommands file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[43].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected script_commands resource".into()),
        }
        match (&self.script_archive, files.script_archive) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_SCRIPT_ARCHIVE_FILE_BYTES { return Err("ScriptArchive file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[44].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected script_archive resource".into()),
        }
        match (&self.script_text, files.script_text) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_SCRIPT_TEXT_FILE_BYTES { return Err("ScriptText file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[45].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected script_text resource".into()),
        }
        match (&self.script_coordinates, files.script_coordinates) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_SCRIPT_COORDINATES_FILE_BYTES { return Err("ScriptCoordinates file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[46].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected script_coordinates resource".into()),
        }
        match (&self.ui_choice_lists, files.ui_choice_lists) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_UI_CHOICE_LISTS_FILE_BYTES { return Err("UiChoiceLists file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[50].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected ui_choice_lists resource".into()),
        }
        match (&self.unit_obj_initial_pose, files.unit_obj_initial_pose) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_UNIT_OBJ_INITIAL_POSE_FILE_BYTES { return Err("UnitObjInitialPose file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[51].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected unit_obj_initial_pose resource".into()),
        }
        match (&self.color_mode_palettes, files.color_mode_palettes) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_COLOR_MODE_PALETTES_FILE_BYTES { return Err("ColorModePalettes file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[52].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected color_mode_palettes resource".into()),
        }
        match (&self.unit_obj_equipment, files.unit_obj_equipment) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_UNIT_OBJ_EQUIPMENT_FILE_BYTES { return Err("UnitObjEquipment file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[53].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected unit_obj_equipment resource".into()),
        }
        match (&self.unit_obj_normal_battle, files.unit_obj_normal_battle) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_UNIT_OBJ_NORMAL_BATTLE_FILE_BYTES { return Err("UnitObjNormalBattle file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[54].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected unit_obj_normal_battle resource".into()),
        }
        match (&self.unit_obj_alternate_battle, files.unit_obj_alternate_battle) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_UNIT_OBJ_ALTERNATE_BATTLE_FILE_BYTES { return Err("UnitObjAlternateBattle file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[55].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected unit_obj_alternate_battle resource".into()),
        }
        match (&self.map_raster_palettes, files.map_raster_palettes) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_MAP_RASTER_PALETTES_FILE_BYTES { return Err("MapRasterPalettes file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[49].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected map_raster_palettes resource".into()),
        }
        match (&self.auxiliary_scripts, files.auxiliary_commands) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_AUXILIARY_COMMANDS_FILE_BYTES { return Err("AuxiliaryCommands file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[47].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected auxiliary_commands resource".into()),
        }
        match (&self.auxiliary_scripts, files.auxiliary_archive) {
            (Some(_), Some((_, bytes))) => {
                if bytes.len() as u64 > MAX_AUXILIARY_ARCHIVE_FILE_BYTES { return Err("AuxiliaryArchive file exceeds input guard".into()); }
                total = total.saturating_add(bytes.len());
                expected.resources[48].sha256 = ffta_rom::digest(bytes);
            }
            (None, None) => {}
            _ => return Err("Missing or unexpected auxiliary_archive resource".into()),
        }

        if total > MAX_DATASET_BYTES {
            return Err("Resource files exceed total size limit".into());
        }
        if files.manifest != &expected {
            return Err(
                "Resource manifest does not match the source/profile/schema or file hash".into(),
            );
        }
        dimensions.validate()?;
        if dimensions != &self.dimensions
            || ffta_rom::digest(&dimensions.source_bytes())
                != self.manifest.resources[0].sources[0].sha256
        {
            return Err("Decoded OBJ dimensions do not reproduce the selected ROM source".into());
        }
        if let (Some(expected), Some((decoded, _))) = (&self.background, files.background) {
            let source = &self.manifest.resources[1].sources[0];
            decoded.validate(source.offset, self.manifest.source_rom_size as usize)?;
            if decoded != expected || ffta_rom::digest(&decoded.source_bytes()?) != source.sha256 {
                return Err(
                    "Decoded background descriptors do not reproduce the selected ROM source"
                        .into(),
                );
            }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.palettes, files.palettes) {
            decoded.validate()?;
            if decoded != &expected.decoded {
                return Err(
                    "Decoded base palettes do not reproduce the selected ROM source".into(),
                );
            }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.alternate_palettes, files.alternate_palettes) {
            decoded.validate()?;
            if decoded != &expected.decoded {
                return Err("Decoded alternate palettes do not reproduce the selected ROM source".into());
            }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.packet_programs, files.packet_programs) {
            decoded.validate()?;
            if decoded != &expected.decoded {
                return Err("Decoded packet programs do not reproduce the selected ROM source".into());
            }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.main_graphics, files.main_graphics) {
            decoded.validate()?;
            if decoded != &expected.decoded {
                return Err("Decoded main graphics do not reproduce the selected ROM source".into());
            }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.transfers, files.transfers) {
            decoded.validate()?;
            if decoded != &expected.decoded {
                return Err("Decoded transfer programs do not reproduce the selected ROM source".into());
            }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.short_texts, files.short_texts) {
            decoded.validate()?;
            if decoded != &expected.decoded {
                return Err("Decoded short texts do not reproduce the selected ROM source".into());
            }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.text_catalogs, files.text_catalogs) {
            decoded.validate()?;
            if decoded != &expected.decoded { return Err("Decoded text catalogs do not reproduce selected ROM source".into()); }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.font_bank, files.font_bank) {
            decoded.validate()?;
            if decoded != &expected.decoded { return Err("Decoded font-bank differs from authenticated source".into()); }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.name_keyboard, files.name_keyboard) {
            decoded.validate()?;
            if decoded != &expected.decoded { return Err("Decoded name-keyboard differs from authenticated source".into()); }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.obj_lists, files.obj_lists) {
            decoded.validate()?;
            if decoded != &expected.decoded { return Err("Decoded OBJ static lists differ from authenticated source".into()); }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.obj_animations, files.obj_animations) {
            decoded.validate()?;
            if decoded != &expected.decoded { return Err("Decoded OBJ animation banks differ from authenticated source".into()); }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.obj_graphics, files.obj_graphics) {
            decoded.validate()?;
            if decoded != &expected.decoded { return Err("Decoded OBJ custom graphics differ from authenticated source".into()); }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.ui_palettes, files.ui_palettes) {
            decoded.validate()?;
            if decoded != &expected.decoded { return Err("Decoded UI raw palettes differ from authenticated source".into()); }
        }
        if let (Some(expected), Some((decoded, _))) = (&self.ui_graphics, files.ui_graphics) {
            decoded.validate()?;
            decoded.validate_shared_obj(files.obj_graphics.ok_or("Missing shared OBJ source owner")?.0)?;
            if decoded != &expected.decoded { return Err("Decoded UI custom banks differ from authenticated source".into()); }
        }
        if let (Some(expected),Some((decoded,_))) = (&self.items, files.items) {
            decoded.validate()?;
            if decoded != &expected.decoded {return Err("Decoded items differ from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.item_links, files.item_links) {
            decoded.validate_items(files.items.ok_or("Missing item-link item owner")?.0)?;
            if decoded != &expected.decoded {return Err("Decoded item links differ from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.metadata16, files.metadata16) {
            decoded.validate()?;
            if decoded != &expected.decoded {return Err("Decoded metadata16 differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.metadata52, files.metadata52) {
            decoded.validate_links(files.item_links.ok_or("Missing metadata52 link owner")?.0)?;
            if decoded != &expected.decoded {return Err("Decoded metadata52 differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.metadata4, files.metadata4) {
            decoded.validate_metadata52(files.metadata52.ok_or("Missing metadata4 owner")?.0)?;
            if decoded != &expected.decoded {return Err("Decoded metadata4 differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.metadata2, files.metadata2) {
            decoded.validate_metadata52(files.metadata52.ok_or("Missing metadata2 owner")?.0)?;
            if decoded != &expected.decoded {return Err("Decoded metadata2 differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.default48, files.default48) {
            decoded.validate()?;
            if decoded != &expected.decoded {return Err("Decoded default48 differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.constructor_alternates, files.constructor_alternates) {
            decoded.validate_items(files.items.ok_or("Missing constructor-alternate item owner")?.0)?;
            if decoded != &expected.decoded {return Err("Decoded constructor_alternates differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.metadata14, files.metadata14) {
            decoded.validate_metadata52(files.metadata52.ok_or("Missing metadata14 job owner")?.0)?;
            if decoded != &expected.decoded {return Err("Decoded metadata14 differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.metadata12, files.metadata12) {
            if decoded != &expected.decoded {return Err("Decoded metadata12 differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.global_pair_flags, files.global_pair_flags) {
            if decoded != &expected.decoded {return Err("Decoded global_pair_flags differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.evaluation_parameters, files.evaluation_parameters) {
            if decoded != &expected.decoded {return Err("Decoded evaluation_parameters differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.progression_rewards, files.progression_rewards) {
            if decoded != &expected.decoded {return Err("Decoded progression_rewards differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.progression_gates, files.progression_gates) {
            if decoded != &expected.decoded {return Err("Decoded progression_gates differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.progression_selection_gates, files.progression_selection_gates) {
            if decoded != &expected.decoded {return Err("Decoded progression_selection_gates differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.aggregate_flag_keys, files.aggregate_flag_keys) {
            if decoded != &expected.decoded {return Err("Decoded aggregate_flag_keys differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.trade_parameters, files.trade_parameters) {
            if decoded != &expected.decoded {return Err("Decoded trade_parameters differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.item_effect_parameters, files.item_effect_parameters) {
            decoded.validate_items(files.items.ok_or("Missing item-effect item owner")?.0)?;
            if decoded != &expected.decoded {return Err("Decoded item_effect_parameters differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.audio_directory, files.audio_directory) {
            if decoded != &expected.decoded {return Err("Decoded audio_directory differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.audio_tones, files.audio_tones) {
            decoded.validate_directory(files.audio_directory.ok_or("Missing actual audio-directory dependency")?.0)?;
            if decoded != &expected.decoded {return Err("Decoded audio_tones differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.audio_sequences, files.audio_sequences) {
            decoded.validate_owners(files.audio_directory.ok_or("Missing actual audio-directory dependency")?.0,files.audio_tones.ok_or("Missing actual audio-tone dependency")?.0)?;
            if decoded != &expected.decoded {return Err("Decoded audio_sequences differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.audio_driver, files.audio_driver) {
            decoded.validate_sequence_owner(files.audio_sequences.ok_or("Missing actual audio-sequence dependency")?.0)?;
            if decoded != &expected.decoded {return Err("Decoded audio_driver differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.palette_effect_dispatch, files.palette_effect_dispatch) {
            decoded.validate()?;
            if decoded != &expected.decoded {return Err("Decoded palette_effect_dispatch differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.palette_programs, files.palette_programs) {
            decoded.validate()?;
            if decoded != &expected.decoded {return Err("Decoded palette_programs differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.blend_programs, files.blend_programs) {
            decoded.validate()?;
            if decoded != &expected.decoded {return Err("Decoded blend_programs differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.task_descriptors, files.task_descriptors) {
            decoded.validate()?;
            if decoded != &expected.decoded {return Err("Decoded task_descriptors differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.effect_event_directories, files.effect_event_directories) {
            decoded.validate()?;
            if decoded != &expected.decoded {return Err("Decoded effect_event_directories differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.script_commands, files.script_commands) {
            decoded.validate()?;
            if decoded != &expected.decoded {return Err("Decoded script_commands differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.script_archive, files.script_archive) {
            let commands=files.script_commands.ok_or("Missing script archive command dependency")?.0;
            let effects=files.effect_event_directories.ok_or("Missing script archive effect dependency")?.0;
            decoded.validate_dependencies(commands,effects)?;
            if decoded != &expected.decoded {return Err("Decoded script_archive differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.script_text, files.script_text) {
            let keys=files.script_archive.ok_or("Missing script text key dependency")?.0;
            let font=files.font_bank.ok_or("Missing script text font dependency")?.0;
            decoded.validate_dependencies(keys,font)?;
            if decoded != &expected.decoded {return Err("Decoded script_text differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.script_coordinates, files.script_coordinates) {
            let archive=files.script_archive.ok_or("Missing coordinate archive dependency")?.0;
            let commands=files.script_commands.ok_or("Missing coordinate command dependency")?.0;
            decoded.validate_dependencies(archive,commands)?;
            if decoded != &expected.decoded {return Err("Decoded script_coordinates differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.ui_choice_lists, files.ui_choice_lists) {
            decoded.validate()?;
            if decoded != &expected.decoded {return Err("Decoded ui_choice_lists differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.unit_obj_initial_pose, files.unit_obj_initial_pose) {
            let jobs=files.metadata52.ok_or("Missing unit OBJ job dependency")?.0;
            let special=files.metadata14.ok_or("Missing unit OBJ special dependency")?.0;
            decoded.validate_producers(jobs,special)?;
            if decoded != &expected.decoded {return Err("Decoded unit_obj_initial_pose differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.color_mode_palettes, files.color_mode_palettes) {
            let special=files.metadata14.ok_or("Missing Color-Mode palette metadata14 dependency")?.0;
            runtime::color_mode_palettes::validate_dependencies(decoded,special)?;
            if decoded != &expected.decoded {return Err("Decoded color_mode_palettes differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.unit_obj_equipment, files.unit_obj_equipment) {
            let items=files.items.ok_or("Missing equipment OBJ item dependency")?.0;
            decoded.validate_producers(items)?;
            if decoded != &expected.decoded {return Err("Decoded unit_obj_equipment differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.unit_obj_normal_battle, files.unit_obj_normal_battle) {
            let jobs=files.metadata52.ok_or("Missing normal battle job dependency")?.0;
            let special=files.metadata14.ok_or("Missing normal battle special dependency")?.0;
            let initial=files.unit_obj_initial_pose.ok_or("Missing normal battle initial dependency")?.0;
            let equipment=files.unit_obj_equipment.ok_or("Missing normal battle equipment dependency")?.0;
            decoded.validate_producers(jobs,special)?;
            decoded.owned_spans(initial,equipment)?;
            if decoded != &expected.decoded {return Err("Decoded unit_obj_normal_battle differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.unit_obj_alternate_battle, files.unit_obj_alternate_battle) {
            let jobs=files.metadata52.ok_or("Missing alternate battle job dependency")?.0;
            let special=files.metadata14.ok_or("Missing alternate battle special dependency")?.0;
            let initial=files.unit_obj_initial_pose.ok_or("Missing alternate battle initial dependency")?.0;
            let equipment=files.unit_obj_equipment.ok_or("Missing alternate battle equipment dependency")?.0;
            let normal=files.unit_obj_normal_battle.ok_or("Missing alternate battle normal dependency")?.0;
            decoded.validate_producers(jobs,special)?;
            decoded.owned_spans(initial,equipment,normal)?;
            if decoded != &expected.decoded {return Err("Decoded unit_obj_alternate_battle differs from authenticated source".into());}
        }
        if let (Some(expected),Some((decoded,_))) = (&self.map_raster_palettes, files.map_raster_palettes) {
            let descriptors=files.background.ok_or("Missing raster palette descriptor dependency")?.0;
            decoded.validate_dependencies(descriptors)?;
            if decoded != &expected.decoded {return Err("Decoded map_raster_palettes differs from authenticated source".into());}
        }
        if let Some(expected) = &self.auxiliary_scripts {
            let commands=files.auxiliary_commands.ok_or("Missing auxiliary command dependency")?.0;
            let archive=files.auxiliary_archive.ok_or("Missing auxiliary archive resource")?.0;
            commands.validate()?;
            archive.validate_dependencies(commands)?;
            if commands != &expected.commands || archive != &expected.decoded {return Err("Decoded auxiliary scripts differ from authenticated source".into());}
        }
        Ok(())
    }

    fn resource_count(&self) -> usize {
        1 + usize::from(self.background.is_some()) + usize::from(self.palettes.is_some())
            + usize::from(self.alternate_palettes.is_some())
            + usize::from(self.packet_programs.is_some())
            + usize::from(self.main_graphics.is_some())
            + usize::from(self.transfers.is_some())
            + usize::from(self.short_texts.is_some())
            + usize::from(self.text_catalogs.is_some())
            + usize::from(self.name_keyboard.is_some())
            + usize::from(self.font_bank.is_some())
            + usize::from(self.obj_lists.is_some())
            + usize::from(self.obj_animations.is_some())
            + usize::from(self.obj_graphics.is_some())
            + usize::from(self.ui_palettes.is_some())
            + usize::from(self.ui_graphics.is_some())
            + usize::from(self.items.is_some())
            + usize::from(self.item_links.is_some())
            + usize::from(self.metadata16.is_some())
            + usize::from(self.metadata52.is_some())
            + usize::from(self.metadata4.is_some())
            + usize::from(self.metadata2.is_some())
            + usize::from(self.default48.is_some())
            + usize::from(self.constructor_alternates.is_some())
            + usize::from(self.metadata14.is_some())
            + usize::from(self.metadata12.is_some())
            + usize::from(self.global_pair_flags.is_some())
            + usize::from(self.evaluation_parameters.is_some())
            + usize::from(self.progression_rewards.is_some())
            + usize::from(self.progression_gates.is_some())
            + usize::from(self.progression_selection_gates.is_some())
            + usize::from(self.aggregate_flag_keys.is_some())
            + usize::from(self.trade_parameters.is_some())
            + usize::from(self.item_effect_parameters.is_some())
            + usize::from(self.audio_directory.is_some())
            + usize::from(self.audio_tones.is_some())
            + usize::from(self.audio_sequences.is_some())
            + usize::from(self.audio_driver.is_some())
            + usize::from(self.palette_effect_dispatch.is_some())
            + usize::from(self.palette_programs.is_some())
            + usize::from(self.blend_programs.is_some())
            + usize::from(self.task_descriptors.is_some())
            + usize::from(self.effect_event_directories.is_some())
            + usize::from(self.script_commands.is_some())
            + usize::from(self.script_archive.is_some())
            + usize::from(self.script_text.is_some())
            + usize::from(self.script_coordinates.is_some())
            + usize::from(self.ui_choice_lists.is_some())
            + usize::from(self.unit_obj_initial_pose.is_some())
            + usize::from(self.color_mode_palettes.is_some())
            + usize::from(self.unit_obj_equipment.is_some())
            + usize::from(self.unit_obj_normal_battle.is_some())
            + usize::from(self.unit_obj_alternate_battle.is_some())
            + usize::from(self.map_raster_palettes.is_some())
            + 2 * usize::from(self.auxiliary_scripts.is_some())
    }

    pub fn version(&self) -> Version {
        if self.manifest.profile == "usa" {
            Version::Usa
        } else {
            Version::Europe
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> [u8; 48] {
        let mut bytes = [0; 48];
        for i in 0..12 {
            bytes[i * 4..i * 4 + 2].copy_from_slice(&(8u16 << (i % 4)).to_le_bytes());
            bytes[i * 4 + 2..i * 4 + 4].copy_from_slice(&(8u16 << ((i + 1) % 4)).to_le_bytes());
        }
        bytes
    }

    #[test]
    fn dimensions_roundtrip_and_invalid_inputs() {
        let bytes = fixture();
        let decoded = ObjDimensions::decode(&bytes).unwrap();
        assert_eq!(decoded.source_bytes(), bytes);
        assert_eq!(decoded.get(2, 3), Some(&decoded.entries[11]));
        assert!(decoded.get(3, 0).is_none());
        assert!(decoded.get(0, 4).is_none());
        assert!(decoded.get(255, 255).is_none());
        for len in 0..48 {
            assert!(ObjDimensions::decode(&bytes[..len]).is_err());
        }
        assert!(ObjDimensions::decode(&[0; 49]).is_err());
        let mut invalid = bytes;
        invalid[0..2].copy_from_slice(&256u16.to_le_bytes());
        assert!(ObjDimensions::decode(&invalid).is_err());
        assert!(ObjDimensions::decode(&[0; 48]).is_err());
    }

    #[test]
    fn independent_aliases_bounds_and_base_authentication() {
        let bytes = fixture();
        let hash = ffta_rom::digest(&bytes);
        let mut rom = bytes.repeat(2);
        let ranges = [(0, "direct"), (48, "iwram-copy")];
        let extraction =
            Extraction::from_ranges(&rom, "synthetic".into(), "test", &ranges, &hash).unwrap();
        let mut manifest = extraction.manifest.clone();
        manifest.resources[0].sha256 = ffta_rom::digest(b"file");
        extraction
            .validate_files(BaseFiles {
                manifest: &manifest,
                dimensions: (&extraction.dimensions, b"file"),
                background: None,
                palettes: None,
                alternate_palettes: None,
                packet_programs: None,
                main_graphics: None,
                transfers: None,
            short_texts: None,
            text_catalogs: None,
            name_keyboard: None,
            obj_lists: None,
            obj_animations: None,
            obj_graphics: None,
            ui_palettes: None,
            ui_graphics: None,
            items: None,
            item_links: None,
            metadata16: None,
            metadata52: None,
            metadata4: None,
            metadata2: None,
            default48: None,
            constructor_alternates: None,
            metadata14: None,
            metadata12: None,
            global_pair_flags: None,
            evaluation_parameters: None,
            progression_rewards: None,
            progression_gates: None,
            progression_selection_gates: None,
            aggregate_flag_keys: None,
            trade_parameters: None,
            item_effect_parameters: None,
            audio_directory: None,
            audio_tones: None,
            audio_sequences: None,
            audio_driver: None,
            palette_effect_dispatch: None,
            palette_programs: None,
            blend_programs: None,
            task_descriptors: None,
            effect_event_directories: None,
            script_commands: None,
            script_archive: None,
            script_text: None,
            script_coordinates: None,
            ui_choice_lists: None,
            unit_obj_initial_pose: None,
            color_mode_palettes: None,
            unit_obj_equipment: None,
            unit_obj_normal_battle: None,
            unit_obj_alternate_battle: None,
            map_raster_palettes: None,
            auxiliary_commands: None,
            auxiliary_archive: None,
            font_bank: None,
            })
            .unwrap();
        let mut edited = extraction.dimensions.clone();
        edited.entries[0].width = 64;
        assert!(extraction
            .validate_files(BaseFiles {
                manifest: &manifest,
                dimensions: (&edited, b"file"),
                background: None,
                palettes: None,
                alternate_palettes: None,
                packet_programs: None,
                main_graphics: None,
                transfers: None,
            short_texts: None,
            text_catalogs: None,
            name_keyboard: None,
            obj_lists: None,
            obj_animations: None,
            obj_graphics: None,
            ui_palettes: None,
            ui_graphics: None,
            items: None,
            item_links: None,
            metadata16: None,
            metadata52: None,
            metadata4: None,
            metadata2: None,
            default48: None,
            constructor_alternates: None,
            metadata14: None,
            metadata12: None,
            global_pair_flags: None,
            evaluation_parameters: None,
            progression_rewards: None,
            progression_gates: None,
            progression_selection_gates: None,
            aggregate_flag_keys: None,
            trade_parameters: None,
            item_effect_parameters: None,
            audio_directory: None,
            audio_tones: None,
            audio_sequences: None,
            audio_driver: None,
            palette_effect_dispatch: None,
            palette_programs: None,
            blend_programs: None,
            task_descriptors: None,
            effect_event_directories: None,
            script_commands: None,
            script_archive: None,
            script_text: None,
            script_coordinates: None,
            ui_choice_lists: None,
            unit_obj_initial_pose: None,
            color_mode_palettes: None,
            unit_obj_equipment: None,
            unit_obj_normal_battle: None,
            unit_obj_alternate_battle: None,
            map_raster_palettes: None,
            auxiliary_commands: None,
            auxiliary_archive: None,
            font_bank: None,
            })
            .is_err());
        for mutate in [0, 1, 2, 3, 4, 5, 6, 7] {
            let mut bad = manifest.clone();
            match mutate {
                0 => bad.schema += 1,
                1 => bad.decoder_revision += 1,
                2 => bad.extractor_revision += 1,
                3 => bad.source_rom_sha256 = "edited".into(),
                4 => bad.resources[0].path = "../escape".into(),
                5 => bad.resources.push(bad.resources[0].clone()),
                6 => bad.resources[0].sources[0].offset += 1,
                _ => bad.coverage.push("unverified".into()),
            }
            assert!(extraction
                .validate_files(BaseFiles {
                    manifest: &bad,
                    dimensions: (&extraction.dimensions, b"file"),
                    background: None,
                    palettes: None,
                    alternate_palettes: None,
                    packet_programs: None,
                    main_graphics: None,
                    transfers: None,
            short_texts: None,
            text_catalogs: None,
            name_keyboard: None,
            obj_lists: None,
            obj_animations: None,
            obj_graphics: None,
            ui_palettes: None,
            ui_graphics: None,
            items: None,
            item_links: None,
            metadata16: None,
            metadata52: None,
            metadata4: None,
            metadata2: None,
            default48: None,
            constructor_alternates: None,
            metadata14: None,
            metadata12: None,
            global_pair_flags: None,
            evaluation_parameters: None,
            progression_rewards: None,
            progression_gates: None,
            progression_selection_gates: None,
            aggregate_flag_keys: None,
            trade_parameters: None,
            item_effect_parameters: None,
            audio_directory: None,
            audio_tones: None,
            audio_sequences: None,
            audio_driver: None,
            palette_effect_dispatch: None,
            palette_programs: None,
            blend_programs: None,
            task_descriptors: None,
            effect_event_directories: None,
            script_commands: None,
            script_archive: None,
            script_text: None,
            script_coordinates: None,
            ui_choice_lists: None,
            unit_obj_initial_pose: None,
            color_mode_palettes: None,
            unit_obj_equipment: None,
            unit_obj_normal_battle: None,
            unit_obj_alternate_battle: None,
            map_raster_palettes: None,
            auxiliary_commands: None,
            auxiliary_archive: None,
            font_bank: None,
                })
                .is_err());
        }
        assert!(extraction
            .validate_files(BaseFiles {
                manifest: &manifest,
                dimensions: (&extraction.dimensions, b"other"),
                background: None,
                palettes: None,
                alternate_palettes: None,
                packet_programs: None,
                main_graphics: None,
                transfers: None,
            short_texts: None,
            text_catalogs: None,
            name_keyboard: None,
            obj_lists: None,
            obj_animations: None,
            obj_graphics: None,
            ui_palettes: None,
            ui_graphics: None,
            items: None,
            item_links: None,
            metadata16: None,
            metadata52: None,
            metadata4: None,
            metadata2: None,
            default48: None,
            constructor_alternates: None,
            metadata14: None,
            metadata12: None,
            global_pair_flags: None,
            evaluation_parameters: None,
            progression_rewards: None,
            progression_gates: None,
            progression_selection_gates: None,
            aggregate_flag_keys: None,
            trade_parameters: None,
            item_effect_parameters: None,
            audio_directory: None,
            audio_tones: None,
            audio_sequences: None,
            audio_driver: None,
            palette_effect_dispatch: None,
            palette_programs: None,
            blend_programs: None,
            task_descriptors: None,
            effect_event_directories: None,
            script_commands: None,
            script_archive: None,
            script_text: None,
            script_coordinates: None,
            ui_choice_lists: None,
            unit_obj_initial_pose: None,
            color_mode_palettes: None,
            unit_obj_equipment: None,
            unit_obj_normal_battle: None,
            unit_obj_alternate_battle: None,
            map_raster_palettes: None,
            auxiliary_commands: None,
            auxiliary_archive: None,
            font_bank: None,
            })
            .is_err());
        rom[48] ^= 1;
        assert!(Extraction::from_ranges(&rom, "test".into(), "test", &ranges, &hash).is_err());
        assert!(
            Extraction::from_ranges(&bytes[..47], "test".into(), "test", &ranges, &hash).is_err()
        );
        assert!(
            Extraction::from_ranges(&rom, "test".into(), "test", &[(u32::MAX, "bad")], &hash)
                .is_err()
        );
        assert!(Extraction::from_rom(&rom).is_err());
    }

    #[test]
    #[ignore = "requires the user's exact USA and EUR ROMs; no game data is bundled"]
    fn regional_extractions_are_independent_and_roundtrip() {
        for (env, version, ranges) in [
            ("FFTA_TEST_ROM_USA", Version::Usa, 2),
            ("FFTA_TEST_ROM_EUR", Version::Europe, 2),
        ] {
            let rom = std::fs::read(std::env::var(env).unwrap()).unwrap();
            let extraction = Extraction::from_rom(&rom).unwrap();
            assert_eq!(extraction.version(), version);
            assert_eq!(extraction.manifest.resources[0].sources.len(), ranges);
            for source in &extraction.manifest.resources[0].sources {
                assert_eq!(
                    &rom[source.offset as usize..source.offset as usize + 48],
                    &extraction.dimensions.source_bytes()
                );
            }
            let mut changed = rom;
            changed[0] ^= 1;
            assert!(Extraction::from_rom(&changed).is_err());
        }
    }
}

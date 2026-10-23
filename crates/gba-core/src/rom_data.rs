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
mod custom_lz;
pub mod lz77;

pub const SCHEMA: u32 = 1;
pub const EXTRACTOR_REV: u32 = 34;
pub const DECODER_REV: u32 = 33;
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

//! Source-authenticated extraction, preparation and verified dataset loading.
use gba_core::rom_data::{
    metadata16::Metadata16, METADATA16_ID, METADATA16_FILE, MAX_METADATA16_FILE_BYTES,
    metadata52::Metadata52, METADATA52_ID, METADATA52_FILE, MAX_METADATA52_FILE_BYTES,
    metadata4::Metadata4, METADATA4_ID, METADATA4_FILE, MAX_METADATA4_FILE_BYTES,
    metadata2::Metadata2, METADATA2_ID, METADATA2_FILE, MAX_METADATA2_FILE_BYTES,
    default48::Template as Default48, DEFAULT48_ID, DEFAULT48_FILE, MAX_DEFAULT48_FILE_BYTES,
    constructor_alternates::Alternates as ConstructorAlternates, CONSTRUCTOR_ALTERNATES_ID, CONSTRUCTOR_ALTERNATES_FILE, MAX_CONSTRUCTOR_ALTERNATES_FILE_BYTES,
    item_links::Links as ItemLinks, ITEM_LINKS_ID, ITEM_LINKS_FILE, MAX_ITEM_LINKS_FILE_BYTES,
    items::Items, ITEM_RECORDS_ID, ITEM_RECORDS_FILE, MAX_ITEM_RECORDS_FILE_BYTES,
    ui_graphics::Graphics as UiGraphics, UI_CUSTOM_GRAPHICS_ID, UI_CUSTOM_GRAPHICS_FILE, MAX_UI_CUSTOM_GRAPHICS_FILE_BYTES,
    ui_palettes::Palettes as UiPalettes, UI_RAW_PALETTES_ID, UI_RAW_PALETTES_FILE, MAX_UI_RAW_PALETTES_FILE_BYTES,
    obj_graphics::Graphics as ObjGraphics, OBJ_CUSTOM_GRAPHICS_ID, OBJ_CUSTOM_GRAPHICS_FILE, MAX_OBJ_CUSTOM_GRAPHICS_FILE_BYTES,
    obj_animations::Banks as ObjAnimationBanks, OBJ_ANIMATION_BANKS_ID, OBJ_ANIMATION_BANKS_FILE, MAX_OBJ_ANIMATION_BANKS_FILE_BYTES,
    obj_lists::Lists as ObjLists, OBJ_STATIC_LISTS_ID, OBJ_STATIC_LISTS_FILE, MAX_OBJ_STATIC_LISTS_FILE_BYTES,
    fonts::FontBank, FONT_BANK_ID, FONT_BANK_FILE, MAX_FONT_BANK_FILE_BYTES,
    name_keyboard::Keyboard, NAME_KEYBOARD_ID, NAME_KEYBOARD_FILE, MAX_NAME_KEYBOARD_FILE_BYTES,
    bg_descriptors::Descriptors, bg_palettes::Palettes, BaseFiles, Extraction, Manifest,
    ObjDimensions, BG_BASE_PALETTES_FILE, BG_BASE_PALETTES_ID, BG_DESCRIPTORS_FILE,
    BG_DESCRIPTORS_ID, MANIFEST_FILE, MAX_BG_BASE_PALETTES_FILE_BYTES,
    MAX_BG_DESCRIPTORS_FILE_BYTES, MAX_DATASET_BYTES, MAX_FILE_BYTES, MAX_MANIFEST_FILE_BYTES,
    OBJ_DIMENSIONS_FILE,
    bg_alternate_palettes::Palettes as AlternatePalettes,
    BG_ALTERNATE_PALETTES_FILE, BG_ALTERNATE_PALETTES_ID,
    MAX_BG_ALTERNATE_PALETTES_FILE_BYTES,
    bg_packet_data::Programs as PacketPrograms,
    BG_PACKET_PROGRAMS_ID, BG_PACKET_PROGRAMS_FILE, MAX_BG_PACKET_PROGRAMS_FILE_BYTES,
    bg_graphics::Graphics, BG_MAIN_GRAPHICS_ID, BG_MAIN_GRAPHICS_FILE,
    MAX_BG_MAIN_GRAPHICS_FILE_BYTES,
    bg_transfers::Transfers, BG_TRANSFER_PROGRAMS_ID, BG_TRANSFER_PROGRAMS_FILE,
    MAX_BG_TRANSFER_PROGRAMS_FILE_BYTES,
    text_short::ShortTexts, SHORT_TEXTS_ID, SHORT_TEXTS_FILE, MAX_SHORT_TEXTS_FILE_BYTES,
    text_catalogs::TextCatalogs, TEXT_CATALOGS_ID, TEXT_CATALOGS_FILE, MAX_TEXT_CATALOGS_FILE_BYTES,
};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[cfg(test)]
mod alternate_palette_tests;
#[cfg(test)]
mod alternate_palette_data_tests;
#[cfg(test)]
mod alternate_palette_runtime_tests;
#[cfg(test)]
mod bg_descriptor_tests;
#[cfg(test)]
mod bg_graphics_tests;
#[cfg(test)]
mod bg_graphics_managed_tests;
#[cfg(test)]
mod bg_graphics_runtime_tests;
#[cfg(test)]
mod bg_transfer_tests;
#[cfg(test)]
mod bg_transfer_managed_tests;
#[cfg(test)]
mod bg_transfer_runtime_tests;
#[cfg(test)]
mod text_short_tests;
#[cfg(test)]
mod text_short_managed_tests;
#[cfg(test)]
mod text_short_runtime_tests;
#[cfg(test)]
mod text_catalog_tests;
#[cfg(test)]
mod text_catalog_managed_tests;
#[cfg(test)]
mod text_catalog_runtime_tests;
#[cfg(test)]
mod font_keyboard_tests;
#[cfg(test)]
mod obj_list_tests;
#[cfg(test)]
mod obj_list_runtime_tests;
#[cfg(test)]
mod obj_extra_tests;
#[cfg(test)]
mod obj_animation_tests;
#[cfg(test)]
mod obj_zero_count_tests;
#[cfg(test)]
mod obj_zero_count_runtime_tests;
#[cfg(test)]
mod obj_localized_tests;
#[cfg(test)]
mod obj_localized_runtime_tests;
#[cfg(test)]
mod obj_graphics_tests;
#[cfg(test)]
mod ui_palette_tests;
#[cfg(test)]
mod ui_graphics_tests;
#[cfg(test)]
mod item_tests;
#[cfg(test)]
mod item_link_tests;
#[cfg(test)]
mod metadata16_tests;
#[cfg(test)]
mod metadata52_tests;
#[cfg(test)]
mod metadata4_tests;
#[cfg(test)]
mod metadata2_tests;
#[cfg(test)]
mod default48_tests;
#[cfg(test)]
mod constructor_alternate_tests;
#[cfg(test)]
mod metadata4_runtime_tests;
#[cfg(test)]
mod metadata2_runtime_tests;
#[cfg(test)]
mod default48_runtime_tests;
#[cfg(test)]
mod constructor_alternate_runtime_tests;
#[cfg(test)]
mod metadata52_runtime_tests;
#[cfg(test)]
mod metadata16_runtime_tests;
#[cfg(test)]
mod item_link_runtime_tests;
#[cfg(test)]
mod item_runtime_tests;
#[cfg(test)]
mod ui_localized_graphics_tests;
#[cfg(test)]
mod ui_localized_graphics_runtime_tests;
#[cfg(test)]
mod ui_graphics_runtime_tests;
#[cfg(test)]
mod ui_palette_runtime_tests;
#[cfg(test)]
mod obj_graphics_runtime_tests;
#[cfg(test)]
mod obj_animation_runtime_tests;
#[cfg(test)]
mod font_keyboard_runtime_tests;
#[cfg(test)]
mod bg_packet_tests;
#[cfg(test)]
mod bg_packet_catalog_tests;
#[cfg(test)]
mod bg_packet_data_tests;
#[cfg(test)]
mod bg_packet_managed_tests;
#[cfg(test)]
mod bg_packet_runtime_tests;
#[cfg(test)]
mod bg_palette_tests;
#[cfg(test)]
mod bg_runtime_tests;
#[cfg(test)]
mod compression_tests;
#[cfg(test)]
mod managed_tests;
#[cfg(test)]
mod native_tests;
#[cfg(test)]
mod palette_data_tests;
#[cfg(test)]
mod palette_runtime_tests;
mod preparation;
#[cfg(test)]
mod validation_tests;
pub use preparation::prepare_startup;

fn read_bounded(path: &Path, limit: u64) -> Result<Vec<u8>, String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|e| format!("Cannot inspect {}: {e}", path.display()))?;
    if !metadata.file_type().is_file() || metadata.len() > limit {
        return Err(format!(
            "Resource must be a regular file of at most {limit} bytes: {}",
            path.display()
        ));
    }
    let mut bytes = Vec::new();
    fs::File::open(path)
        .and_then(|f| f.take(limit + 1).read_to_end(&mut bytes))
        .map_err(|e| format!("Cannot read {}: {e}", path.display()))?;
    if bytes.len() as u64 > limit {
        return Err("Resource grew beyond the file size limit".into());
    }
    Ok(bytes)
}

fn parse<T: serde::de::DeserializeOwned>(bytes: &[u8], label: &str) -> Result<T, String> {
    let value = std::str::from_utf8(bytes).map_err(|_| format!("{label} is not UTF-8"))?;
    toml::from_str(value).map_err(|e| format!("Invalid {label}: {e}"))
}

fn parse_background(bytes: &[u8]) -> Result<Descriptors, String> {
    // Deserialize directly into bounded typed records. A generic TOML/JSON
    // document tree needlessly multiplies temporary memory for this family.
    let result: Descriptors = serde_json::from_slice(bytes)
        .map_err(|e| format!("Invalid background descriptors: {e}"))?;
    if result.entries.is_empty()
        || result.entries.len() > gba_core::rom_data::bg_descriptors::MAX_RECORDS
    {
        return Err("Invalid background descriptor count".into());
    }
    Ok(result)
}

fn encode_background(background: &Descriptors) -> Result<Vec<u8>, String> {
    serde_json::to_vec_pretty(background)
        .map_err(|e| format!("Cannot encode background descriptors: {e}"))
}

fn parse_palettes(bytes: &[u8]) -> Result<Palettes, String> {
    let palettes: Palettes =
        serde_json::from_slice(bytes).map_err(|e| format!("Invalid base palettes: {e}"))?;
    palettes.validate()?;
    Ok(palettes)
}

fn encode_palettes(palettes: &Palettes) -> Result<Vec<u8>, String> {
    palettes.validate()?;
    serde_json::to_vec_pretty(palettes).map_err(|e| format!("Cannot encode base palettes: {e}"))
}

fn parse_alternate_palettes(bytes: &[u8]) -> Result<AlternatePalettes, String> {
    let palettes: AlternatePalettes = serde_json::from_slice(bytes)
        .map_err(|e| format!("Invalid alternate palettes: {e}"))?;
    palettes.validate()?;
    Ok(palettes)
}

fn encode_alternate_palettes(palettes: &AlternatePalettes) -> Result<Vec<u8>, String> {
    palettes.validate()?;
    serde_json::to_vec_pretty(palettes).map_err(|e| format!("Cannot encode alternate palettes: {e}"))
}

fn parse_packet_programs(bytes: &[u8]) -> Result<PacketPrograms, String> {
    let programs: PacketPrograms = serde_json::from_slice(bytes)
        .map_err(|e| format!("Invalid packet programs: {e}"))?;
    programs.validate()?;
    Ok(programs)
}

fn encode_packet_programs(programs: &PacketPrograms) -> Result<Vec<u8>, String> {
    programs.validate()?;
    serde_json::to_vec_pretty(programs).map_err(|e| format!("Cannot encode packet programs: {e}"))
}

fn parse_main_graphics(bytes: &[u8]) -> Result<Graphics, String> {
    let graphics: Graphics = serde_json::from_slice(bytes)
        .map_err(|e| format!("Invalid main graphics: {e}"))?;
    graphics.validate()?;
    Ok(graphics)
}

fn encode_main_graphics(graphics: &Graphics) -> Result<Vec<u8>, String> {
    graphics.validate()?;
    serde_json::to_vec_pretty(graphics).map_err(|e| format!("Cannot encode main graphics: {e}"))
}

fn parse_transfers(bytes: &[u8]) -> Result<Transfers, String> {
    let transfers: Transfers = serde_json::from_slice(bytes)
        .map_err(|e| format!("Invalid transfer programs: {e}"))?;
    transfers.validate()?;
    Ok(transfers)
}

fn encode_transfers(transfers: &Transfers) -> Result<Vec<u8>, String> {
    transfers.validate()?;
    serde_json::to_vec_pretty(transfers).map_err(|e| format!("Cannot encode transfer programs: {e}"))
}

fn parse_short_texts(bytes: &[u8]) -> Result<ShortTexts, String> {
    let value: ShortTexts = serde_json::from_slice(bytes)
        .map_err(|e| format!("Invalid short texts: {e}"))?;
    value.validate()?;
    Ok(value)
}

fn encode_short_texts(value: &ShortTexts) -> Result<Vec<u8>, String> {
    value.validate()?;
    serde_json::to_vec_pretty(value).map_err(|e| format!("Cannot encode short texts: {e}"))
}

fn parse_metadata16(bytes:&[u8])->Result<Metadata16,String> {
    let value:Metadata16=serde_json::from_slice(bytes).map_err(|e|format!("Invalid metadata16: {e}"))?;
    value.validate()?;Ok(value)
}
fn encode_metadata16(value:&Metadata16)->Result<Vec<u8>,String> {serde_json::to_vec_pretty(value).map_err(|e|format!("Cannot encode metadata16: {e}"))}

fn parse_metadata52(bytes:&[u8])->Result<Metadata52,String> {
    let value:Metadata52=serde_json::from_slice(bytes).map_err(|e|format!("Invalid metadata52: {e}"))?;
    value.validate()?;Ok(value)
}
fn encode_metadata52(value:&Metadata52)->Result<Vec<u8>,String> {serde_json::to_vec_pretty(value).map_err(|e|format!("Cannot encode metadata52: {e}"))}

fn parse_metadata4(bytes:&[u8])->Result<Metadata4,String> {
    let value:Metadata4=serde_json::from_slice(bytes).map_err(|e|format!("Invalid metadata4: {e}"))?;
    value.validate()?;Ok(value)
}
fn encode_metadata4(value:&Metadata4)->Result<Vec<u8>,String> {serde_json::to_vec_pretty(value).map_err(|e|format!("Cannot encode metadata4: {e}"))}
fn parse_metadata2(bytes:&[u8])->Result<Metadata2,String> {
    let value:Metadata2=serde_json::from_slice(bytes).map_err(|e|format!("Invalid metadata2: {e}"))?;
    value.validate()?;Ok(value)
}
fn encode_metadata2(value:&Metadata2)->Result<Vec<u8>,String> {serde_json::to_vec_pretty(value).map_err(|e|format!("Cannot encode metadata2: {e}"))}
fn parse_default48(bytes:&[u8])->Result<Default48,String> {
    let value:Default48=serde_json::from_slice(bytes).map_err(|e|format!("Invalid default48: {e}"))?;
    value.validate()?;Ok(value)
}
fn encode_default48(value:&Default48)->Result<Vec<u8>,String> {serde_json::to_vec_pretty(value).map_err(|e|format!("Cannot encode default48: {e}"))}
fn parse_constructor_alternates(bytes:&[u8])->Result<ConstructorAlternates,String> {
    let value:ConstructorAlternates=serde_json::from_slice(bytes).map_err(|e|format!("Invalid constructor_alternates: {e}"))?;
    value.validate()?;Ok(value)
}
fn encode_constructor_alternates(value:&ConstructorAlternates)->Result<Vec<u8>,String> {serde_json::to_vec_pretty(value).map_err(|e|format!("Cannot encode constructor_alternates: {e}"))}

fn parse_item_links(bytes:&[u8])->Result<ItemLinks,String> {
    let value:ItemLinks=serde_json::from_slice(bytes).map_err(|e|format!("Invalid item links: {e}"))?;
    value.validate()?;Ok(value)
}
fn encode_item_links(value:&ItemLinks)->Result<Vec<u8>,String> {serde_json::to_vec_pretty(value).map_err(|e|format!("Cannot encode item links: {e}"))}

fn parse_items(bytes: &[u8]) -> Result<Items, String> {
    let value: Items = serde_json::from_slice(bytes).map_err(|e|format!("Invalid item records: {e}"))?;
    value.validate()?;
    Ok(value)
}
fn encode_items(value: &Items) -> Result<Vec<u8>,String> {
    serde_json::to_vec_pretty(value).map_err(|e|format!("Cannot encode item records: {e}"))
}

fn parse_ui_graphics(bytes: &[u8]) -> Result<UiGraphics, String> {
    let value: UiGraphics = serde_json::from_slice(bytes).map_err(|e| format!("Invalid UI custom graphics: {e}"))?;
    value.validate()?;
    Ok(value)
}
fn encode_ui_graphics(value: &UiGraphics) -> Result<Vec<u8>, String> {
    serde_json::to_vec_pretty(value).map_err(|e| format!("Cannot encode UI custom graphics: {e}"))
}

fn parse_ui_palettes(bytes: &[u8]) -> Result<UiPalettes, String> {
    let value: UiPalettes = serde_json::from_slice(bytes).map_err(|e| format!("Invalid UI raw palettes: {e}"))?;
    value.validate()?;
    Ok(value)
}
fn encode_ui_palettes(value: &UiPalettes) -> Result<Vec<u8>, String> {
    serde_json::to_vec_pretty(value).map_err(|e| format!("Cannot encode UI raw palettes: {e}"))
}

fn parse_obj_graphics(bytes: &[u8]) -> Result<ObjGraphics, String> {
    let value: ObjGraphics = serde_json::from_slice(bytes).map_err(|e| format!("Invalid OBJ custom graphics: {e}"))?;
    value.validate()?;
    Ok(value)
}
fn encode_obj_graphics(value: &ObjGraphics) -> Result<Vec<u8>, String> {
    serde_json::to_vec_pretty(value).map_err(|e| format!("Cannot encode OBJ custom graphics: {e}"))
}

fn parse_obj_animations(bytes: &[u8]) -> Result<ObjAnimationBanks, String> {
    let value: ObjAnimationBanks = serde_json::from_slice(bytes).map_err(|e| format!("Invalid OBJ animation banks: {e}"))?;
    value.validate()?; Ok(value)
}
fn encode_obj_animations(value: &ObjAnimationBanks) -> Result<Vec<u8>, String> {
    value.validate()?; serde_json::to_vec_pretty(value).map_err(|e| format!("Cannot encode OBJ animation banks: {e}"))
}

fn parse_obj_lists(bytes: &[u8]) -> Result<ObjLists, String> {
    let value: ObjLists = serde_json::from_slice(bytes).map_err(|e| format!("Invalid OBJ static lists: {e}"))?;
    value.validate()?;
    Ok(value)
}
fn encode_obj_lists(value: &ObjLists) -> Result<Vec<u8>, String> {
    value.validate()?;
    serde_json::to_vec_pretty(value).map_err(|e| format!("Cannot encode OBJ static lists: {e}"))
}

fn parse_font_bank(bytes: &[u8]) -> Result<FontBank, String> {
    let value: FontBank = serde_json::from_slice(bytes).map_err(|e| format!("Invalid font-bank: {e}"))?;
    value.validate()?;
    Ok(value)
}
fn encode_font_bank(value: &FontBank) -> Result<Vec<u8>, String> {
    value.validate()?;
    serde_json::to_vec_pretty(value).map_err(|e| format!("Cannot encode font-bank: {e}"))
}

fn parse_name_keyboard(bytes: &[u8]) -> Result<Keyboard, String> {
    let value: Keyboard = serde_json::from_slice(bytes).map_err(|e| format!("Invalid name-keyboard: {e}"))?;
    value.validate()?;
    Ok(value)
}
fn encode_name_keyboard(value: &Keyboard) -> Result<Vec<u8>, String> {
    value.validate()?;
    serde_json::to_vec_pretty(value).map_err(|e| format!("Cannot encode name-keyboard: {e}"))
}

fn parse_text_catalogs(bytes: &[u8]) -> Result<TextCatalogs, String> {
    let value: TextCatalogs = serde_json::from_slice(bytes).map_err(|e| format!("Invalid text catalogs: {e}"))?;
    value.validate()?; Ok(value)
}
fn encode_text_catalogs(value: &TextCatalogs) -> Result<Vec<u8>, String> {
    value.validate()?; serde_json::to_vec_pretty(value).map_err(|e| format!("Cannot encode text catalogs: {e}"))
}

pub fn verify(path: &Path, source: &Extraction) -> Result<ObjDimensions, String> {
    let dataset = read_dataset(path)?;
    source.validate_files(dataset.files())?;
    Ok(dataset.dimensions)
}

struct Dataset {
    manifest: Manifest,
    dimensions: ObjDimensions,
    obj_bytes: Vec<u8>,
    background: Descriptors,
    bg_bytes: Vec<u8>,
    palettes: Palettes,
    palette_bytes: Vec<u8>,
    alternate_palettes: AlternatePalettes,
    alternate_palette_bytes: Vec<u8>,
    packet_programs: PacketPrograms,
    packet_program_bytes: Vec<u8>,
    main_graphics: Option<Graphics>,
    main_graphic_bytes: Option<Vec<u8>>,
    transfers: Option<Transfers>,
    transfer_bytes: Option<Vec<u8>>,
    short_texts: Option<ShortTexts>,
    short_text_bytes: Option<Vec<u8>>,
    font_bank: Option<FontBank>,
    font_bank_bytes: Option<Vec<u8>>,
    name_keyboard: Option<Keyboard>,
    name_keyboard_bytes: Option<Vec<u8>>,
    obj_lists: Option<ObjLists>,
    obj_list_bytes: Option<Vec<u8>>,
    obj_animations: Option<ObjAnimationBanks>,
    obj_animation_bytes: Option<Vec<u8>>,
    obj_graphics: Option<ObjGraphics>,
    obj_graphic_bytes: Option<Vec<u8>>,
    ui_palettes: Option<UiPalettes>,
    ui_palette_bytes: Option<Vec<u8>>,
    ui_graphics: Option<UiGraphics>,
    ui_graphic_bytes: Option<Vec<u8>>,
    items: Option<Items>,
    item_bytes: Option<Vec<u8>>,
    item_links: Option<ItemLinks>,
    metadata16: Option<Metadata16>,
    metadata52: Option<Metadata52>,
    metadata4: Option<Metadata4>,
    metadata2: Option<Metadata2>,
    default48: Option<Default48>,
    constructor_alternates: Option<ConstructorAlternates>,
    metadata16_bytes: Option<Vec<u8>>,
    metadata52_bytes: Option<Vec<u8>>,
    metadata4_bytes: Option<Vec<u8>>,
    metadata2_bytes: Option<Vec<u8>>,
    default48_bytes: Option<Vec<u8>>,
    constructor_alternates_bytes: Option<Vec<u8>>,
    item_link_bytes: Option<Vec<u8>>,
    text_catalogs: Option<TextCatalogs>,
    text_catalog_bytes: Option<Vec<u8>>,
}

impl Dataset {
    fn files(&self) -> BaseFiles<'_> {
        BaseFiles {
            manifest: &self.manifest,
            dimensions: (&self.dimensions, &self.obj_bytes),
            background: Some((&self.background, &self.bg_bytes)),
            palettes: Some((&self.palettes, &self.palette_bytes)),
            alternate_palettes: Some((&self.alternate_palettes, &self.alternate_palette_bytes)),
            packet_programs: Some((&self.packet_programs, &self.packet_program_bytes)),
            main_graphics: self.main_graphics.as_ref().zip(self.main_graphic_bytes.as_deref()),
            transfers: self.transfers.as_ref().zip(self.transfer_bytes.as_deref()),
            short_texts: self.short_texts.as_ref().zip(self.short_text_bytes.as_deref()),
            font_bank: self.font_bank.as_ref().zip(self.font_bank_bytes.as_deref()),
            name_keyboard: self.name_keyboard.as_ref().zip(self.name_keyboard_bytes.as_deref()),
            obj_lists: self.obj_lists.as_ref().zip(self.obj_list_bytes.as_deref()),
            obj_animations: self.obj_animations.as_ref().zip(self.obj_animation_bytes.as_deref()),
            obj_graphics: self.obj_graphics.as_ref().zip(self.obj_graphic_bytes.as_deref()),
            ui_palettes: self.ui_palettes.as_ref().zip(self.ui_palette_bytes.as_deref()),
            ui_graphics: self.ui_graphics.as_ref().zip(self.ui_graphic_bytes.as_deref()),
            items: self.items.as_ref().zip(self.item_bytes.as_deref()),
            item_links: self.item_links.as_ref().zip(self.item_link_bytes.as_deref()),
            metadata16: self.metadata16.as_ref().zip(self.metadata16_bytes.as_deref()),
            metadata52: self.metadata52.as_ref().zip(self.metadata52_bytes.as_deref()),
            metadata4: self.metadata4.as_ref().zip(self.metadata4_bytes.as_deref()),
            metadata2: self.metadata2.as_ref().zip(self.metadata2_bytes.as_deref()),
            default48: self.default48.as_ref().zip(self.default48_bytes.as_deref()),
            constructor_alternates: self.constructor_alternates.as_ref().zip(self.constructor_alternates_bytes.as_deref()),
            text_catalogs: self.text_catalogs.as_ref().zip(self.text_catalog_bytes.as_deref()),
        }
    }
}

fn read_dataset(path: &Path) -> Result<Dataset, String> {
    if !fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_dir()) {
        return Err(format!(
            "Dataset must be a directory, not a symlink: {}",
            path.display()
        ));
    }
    let manifest_bytes = read_bounded(&path.join(MANIFEST_FILE), MAX_MANIFEST_FILE_BYTES)?;
    let manifest: Manifest = parse(&manifest_bytes, "resource manifest")?;
    // Never follow paths from a manifest. The supported resource set and names
    // are fixed, and each file has its own bound before parsing.
    let obj_bytes = read_bounded(&path.join(OBJ_DIMENSIONS_FILE), MAX_FILE_BYTES)?;
    let bg_bytes = read_bounded(
        &path.join(BG_DESCRIPTORS_FILE),
        MAX_BG_DESCRIPTORS_FILE_BYTES,
    )?;
    let palette_bytes = read_bounded(
        &path.join(BG_BASE_PALETTES_FILE),
        MAX_BG_BASE_PALETTES_FILE_BYTES,
    )?;
    let alternate_palette_bytes = read_bounded(
        &path.join(BG_ALTERNATE_PALETTES_FILE), MAX_BG_ALTERNATE_PALETTES_FILE_BYTES,
    )?;
    let packet_program_bytes = read_bounded(
        &path.join(BG_PACKET_PROGRAMS_FILE), MAX_BG_PACKET_PROGRAMS_FILE_BYTES,
    )?;
    // Presence selects only a fixed filename, never a manifest-provided path.
    // Authentication below rejects missing/unexpected resources against ROM.
    let main_graphic_bytes = manifest.coverage.iter().any(|id| id == BG_MAIN_GRAPHICS_ID)
        .then(|| read_bounded(&path.join(BG_MAIN_GRAPHICS_FILE), MAX_BG_MAIN_GRAPHICS_FILE_BYTES))
        .transpose()?;
    let transfer_bytes = manifest.coverage.iter().any(|id| id == BG_TRANSFER_PROGRAMS_ID)
        .then(|| read_bounded(&path.join(BG_TRANSFER_PROGRAMS_FILE), MAX_BG_TRANSFER_PROGRAMS_FILE_BYTES))
        .transpose()?;
    let short_text_bytes = manifest.coverage.iter().any(|id| id == SHORT_TEXTS_ID)
        .then(|| read_bounded(&path.join(SHORT_TEXTS_FILE), MAX_SHORT_TEXTS_FILE_BYTES))
        .transpose()?;
    let text_catalog_bytes = manifest.coverage.iter().any(|id| id == TEXT_CATALOGS_ID)
        .then(|| read_bounded(&path.join(TEXT_CATALOGS_FILE), MAX_TEXT_CATALOGS_FILE_BYTES)).transpose()?;
    let font_bank_bytes = manifest.coverage.iter().any(|id| id == FONT_BANK_ID)
        .then(|| read_bounded(&path.join(FONT_BANK_FILE), MAX_FONT_BANK_FILE_BYTES)).transpose()?;
    let name_keyboard_bytes = manifest.coverage.iter().any(|id| id == NAME_KEYBOARD_ID)
        .then(|| read_bounded(&path.join(NAME_KEYBOARD_FILE), MAX_NAME_KEYBOARD_FILE_BYTES)).transpose()?;
    let obj_list_bytes = manifest.coverage.iter().any(|id| id == OBJ_STATIC_LISTS_ID)
        .then(|| read_bounded(&path.join(OBJ_STATIC_LISTS_FILE), MAX_OBJ_STATIC_LISTS_FILE_BYTES)).transpose()?;
    let obj_animation_bytes = manifest.coverage.iter().any(|id| id == OBJ_ANIMATION_BANKS_ID)
        .then(|| read_bounded(&path.join(OBJ_ANIMATION_BANKS_FILE), MAX_OBJ_ANIMATION_BANKS_FILE_BYTES)).transpose()?;
    let obj_graphic_bytes = manifest.coverage.iter().any(|id| id == OBJ_CUSTOM_GRAPHICS_ID)
        .then(|| read_bounded(&path.join(OBJ_CUSTOM_GRAPHICS_FILE), MAX_OBJ_CUSTOM_GRAPHICS_FILE_BYTES)).transpose()?;
    let ui_palette_bytes = manifest.coverage.iter().any(|id| id == UI_RAW_PALETTES_ID)
        .then(|| read_bounded(&path.join(UI_RAW_PALETTES_FILE), MAX_UI_RAW_PALETTES_FILE_BYTES)).transpose()?;
    let ui_graphic_bytes = manifest.coverage.iter().any(|id| id == UI_CUSTOM_GRAPHICS_ID)
        .then(|| read_bounded(&path.join(UI_CUSTOM_GRAPHICS_FILE), MAX_UI_CUSTOM_GRAPHICS_FILE_BYTES)).transpose()?;
    let item_bytes = manifest.coverage.iter().any(|id|id == ITEM_RECORDS_ID)
        .then(||read_bounded(&path.join(ITEM_RECORDS_FILE),MAX_ITEM_RECORDS_FILE_BYTES)).transpose()?;
    let item_link_bytes=manifest.coverage.iter().any(|id|id==ITEM_LINKS_ID)
        .then(||read_bounded(&path.join(ITEM_LINKS_FILE),MAX_ITEM_LINKS_FILE_BYTES)).transpose()?;
    let metadata16_bytes=manifest.coverage.iter().any(|id|id==METADATA16_ID)
        .then(||read_bounded(&path.join(METADATA16_FILE),MAX_METADATA16_FILE_BYTES)).transpose()?;
    let metadata52_bytes=manifest.coverage.iter().any(|id|id==METADATA52_ID)
        .then(||read_bounded(&path.join(METADATA52_FILE),MAX_METADATA52_FILE_BYTES)).transpose()?;
    let metadata4_bytes=manifest.coverage.iter().any(|id|id==METADATA4_ID)
        .then(||read_bounded(&path.join(METADATA4_FILE),MAX_METADATA4_FILE_BYTES)).transpose()?;
    let metadata2_bytes=manifest.coverage.iter().any(|id|id==METADATA2_ID)
        .then(||read_bounded(&path.join(METADATA2_FILE),MAX_METADATA2_FILE_BYTES)).transpose()?;
    let default48_bytes=manifest.coverage.iter().any(|id|id==DEFAULT48_ID)
        .then(||read_bounded(&path.join(DEFAULT48_FILE),MAX_DEFAULT48_FILE_BYTES)).transpose()?;
    let constructor_alternates_bytes=manifest.coverage.iter().any(|id|id==CONSTRUCTOR_ALTERNATES_ID)
        .then(||read_bounded(&path.join(CONSTRUCTOR_ALTERNATES_FILE),MAX_CONSTRUCTOR_ALTERNATES_FILE_BYTES)).transpose()?;
    check_dataset_budget(
        manifest_bytes.len(),
        obj_bytes.len(),
        bg_bytes.len(),
        palette_bytes.len(),
        alternate_palette_bytes.len(),
        packet_program_bytes.len(),
        main_graphic_bytes.as_ref().map_or(0, Vec::len),
        transfer_bytes.as_ref().map_or(0, Vec::len),
        short_text_bytes.as_ref().map_or(0, Vec::len),
        text_catalog_bytes.as_ref().map_or(0, Vec::len),
        font_bank_bytes.as_ref().map_or(0, Vec::len),
        name_keyboard_bytes.as_ref().map_or(0, Vec::len),
        obj_list_bytes.as_ref().map_or(0, Vec::len),
        obj_animation_bytes.as_ref().map_or(0, Vec::len),
        obj_graphic_bytes.as_ref().map_or(0, Vec::len),
        ui_palette_bytes.as_ref().map_or(0, Vec::len),
        ui_graphic_bytes.as_ref().map_or(0, Vec::len),
        item_bytes.as_ref().map_or(0, Vec::len),
        item_link_bytes.as_ref().map_or(0, Vec::len),
        metadata16_bytes.as_ref().map_or(0, Vec::len),
        metadata52_bytes.as_ref().map_or(0, Vec::len),
        metadata4_bytes.as_ref().map_or(0, Vec::len),
        metadata2_bytes.as_ref().map_or(0, Vec::len),
        default48_bytes.as_ref().map_or(0, Vec::len),
        constructor_alternates_bytes.as_ref().map_or(0, Vec::len),
    )?;
    let dimensions = parse(&obj_bytes, "OBJ dimensions")?;
    let background = parse_background(&bg_bytes)?;
    let palettes = parse_palettes(&palette_bytes)?;
    let alternate_palettes = parse_alternate_palettes(&alternate_palette_bytes)?;
    let packet_programs = parse_packet_programs(&packet_program_bytes)?;
    let main_graphics = main_graphic_bytes.as_deref().map(parse_main_graphics).transpose()?;
    let transfers = transfer_bytes.as_deref().map(parse_transfers).transpose()?;
    let short_texts = short_text_bytes.as_deref().map(parse_short_texts).transpose()?;
    let text_catalogs = text_catalog_bytes.as_deref().map(parse_text_catalogs).transpose()?;
    let font_bank = font_bank_bytes.as_deref().map(parse_font_bank).transpose()?;
    let name_keyboard = name_keyboard_bytes.as_deref().map(parse_name_keyboard).transpose()?;
    let obj_lists = obj_list_bytes.as_deref().map(parse_obj_lists).transpose()?;
    let obj_animations = obj_animation_bytes.as_deref().map(parse_obj_animations).transpose()?;
    let obj_graphics = obj_graphic_bytes.as_deref().map(parse_obj_graphics).transpose()?;
    let ui_palettes = ui_palette_bytes.as_deref().map(parse_ui_palettes).transpose()?;
    let ui_graphics = ui_graphic_bytes.as_deref().map(parse_ui_graphics).transpose()?;
    let items = item_bytes.as_deref().map(parse_items).transpose()?;
    let item_links=item_link_bytes.as_deref().map(parse_item_links).transpose()?;
    let metadata16=metadata16_bytes.as_deref().map(parse_metadata16).transpose()?;
    let metadata52=metadata52_bytes.as_deref().map(parse_metadata52).transpose()?;
    let metadata4=metadata4_bytes.as_deref().map(parse_metadata4).transpose()?;
    let metadata2=metadata2_bytes.as_deref().map(parse_metadata2).transpose()?;
    let default48=default48_bytes.as_deref().map(parse_default48).transpose()?;
    let constructor_alternates=constructor_alternates_bytes.as_deref().map(parse_constructor_alternates).transpose()?;
    Ok(Dataset {
        manifest,
        dimensions,
        obj_bytes,
        background,
        bg_bytes,
        palettes,
        palette_bytes,
        alternate_palettes,
        alternate_palette_bytes,
        packet_programs,
        packet_program_bytes,
        main_graphics,
        main_graphic_bytes,
        transfers,
        transfer_bytes,
        short_texts,
        short_text_bytes,
        font_bank,
        font_bank_bytes,
        name_keyboard,
        name_keyboard_bytes,
        obj_lists,
        obj_list_bytes,
        obj_animations,
        obj_animation_bytes,
        obj_graphics,
        obj_graphic_bytes,
        ui_palettes,
        ui_palette_bytes,
        ui_graphics,
        ui_graphic_bytes,
        items,
        item_bytes,
        item_links,
        item_link_bytes,
        metadata16,
        metadata52,
        metadata4,
        metadata2,
        default48,
        constructor_alternates,
        metadata16_bytes,
        metadata52_bytes,
        metadata4_bytes,
        metadata2_bytes,
        default48_bytes,
        constructor_alternates_bytes,
        text_catalogs,
        text_catalog_bytes,
    })
}

fn check_dataset_budget(
    manifest: usize,
    obj: usize,
    bg: usize,
    palettes: usize,
    alternate_palettes: usize,
    packet_programs: usize,
    main_graphics: usize,
    transfers: usize,
    short_texts: usize,
    text_catalogs: usize,
    font_bank: usize,
    name_keyboard: usize,
    obj_lists: usize,
    obj_animations: usize,
    obj_graphics: usize,
    ui_palettes: usize,
    ui_graphics: usize,
    items: usize,
    item_links: usize,
    metadata16: usize,
    metadata52: usize,
    metadata4: usize,
    metadata2: usize,
    default48: usize,
    constructor_alternates: usize,
) -> Result<(), String> {
    if manifest as u64 > MAX_MANIFEST_FILE_BYTES
        || obj as u64 > MAX_FILE_BYTES
        || bg as u64 > MAX_BG_DESCRIPTORS_FILE_BYTES
        || palettes as u64 > MAX_BG_BASE_PALETTES_FILE_BYTES
        || alternate_palettes as u64 > MAX_BG_ALTERNATE_PALETTES_FILE_BYTES
        || packet_programs as u64 > MAX_BG_PACKET_PROGRAMS_FILE_BYTES
        || main_graphics as u64 > MAX_BG_MAIN_GRAPHICS_FILE_BYTES
        || transfers as u64 > MAX_BG_TRANSFER_PROGRAMS_FILE_BYTES
        || short_texts as u64 > MAX_SHORT_TEXTS_FILE_BYTES
        || font_bank as u64 > MAX_FONT_BANK_FILE_BYTES
        || name_keyboard as u64 > MAX_NAME_KEYBOARD_FILE_BYTES
        || obj_lists as u64 > MAX_OBJ_STATIC_LISTS_FILE_BYTES
        || obj_animations as u64 > MAX_OBJ_ANIMATION_BANKS_FILE_BYTES
        || obj_graphics as u64 > MAX_OBJ_CUSTOM_GRAPHICS_FILE_BYTES
        || ui_palettes as u64 > MAX_UI_RAW_PALETTES_FILE_BYTES
        || ui_graphics as u64 > MAX_UI_CUSTOM_GRAPHICS_FILE_BYTES
        || items as u64 > MAX_ITEM_RECORDS_FILE_BYTES
        || item_links as u64 > MAX_ITEM_LINKS_FILE_BYTES
        || metadata16 as u64 > MAX_METADATA16_FILE_BYTES
        || metadata52 as u64 > MAX_METADATA52_FILE_BYTES
        || metadata4 as u64 > MAX_METADATA4_FILE_BYTES
        || metadata2 as u64 > MAX_METADATA2_FILE_BYTES
        || default48 as u64 > MAX_DEFAULT48_FILE_BYTES
        || constructor_alternates as u64 > MAX_CONSTRUCTOR_ALTERNATES_FILE_BYTES
        || text_catalogs as u64 > MAX_TEXT_CATALOGS_FILE_BYTES
    {
        return Err("Resource file exceeds its size budget".into());
    }
    if manifest
        .saturating_add(obj)
        .saturating_add(bg)
        .saturating_add(palettes)
        .saturating_add(alternate_palettes)
        .saturating_add(packet_programs)
        .saturating_add(main_graphics)
        .saturating_add(transfers)
        .saturating_add(short_texts)
        .saturating_add(font_bank)
        .saturating_add(name_keyboard)
        .saturating_add(obj_lists)
        .saturating_add(obj_animations)
        .saturating_add(obj_graphics)
        .saturating_add(ui_palettes)
        .saturating_add(ui_graphics)
        .saturating_add(items)
        .saturating_add(item_links)
        .saturating_add(metadata16)
        .saturating_add(metadata52)
        .saturating_add(metadata4)
        .saturating_add(metadata2)
        .saturating_add(default48)
        .saturating_add(constructor_alternates)
        .saturating_add(text_catalogs)
        > MAX_DATASET_BYTES
    {
        return Err("Resource dataset exceeds its total size budget".into());
    }
    Ok(())
}

pub fn attach_from_directory(machine: &mut gba_core::Machine, path: &Path) -> Result<(), String> {
    let dataset = read_dataset(path)?;
    let resources =
        gba_core::rom_data::RuntimeResources::from_base_files(&machine.bus.rom, dataset.files())?;
    let obj_resident = resources.obj_resident_bytes() + 2 * std::mem::size_of::<usize>();
    let bg_resident = resources.background_resident_bytes();
    let palette_resident = resources.palette_resident_bytes();
    let resident = resources.resident_bytes() + 2 * std::mem::size_of::<usize>();
    if obj_resident > 1024 {
        return Err("Decoded OBJ metadata exceed the session memory budget".into());
    }
    if bg_resident > 64 * 1024 {
        return Err("Decoded BG metadata exceed the session memory budget".into());
    }
    machine.bus.set_rom_resources(Some(resources.clone()))?;
    eprintln!("ROM_DATA runtime=decoded resource={} source={} dataset={} file={} resident_bytes={obj_resident} total_resident_bytes={resident}",
        resources.resource_id(), resources.source_sha256(), resources.dataset_sha256(), resources.file_sha256());
    eprintln!(
        "ROM_DATA runtime=decoded resource={BG_DESCRIPTORS_ID} source={} dataset={} file={} records={} resident_bytes={bg_resident}",
        resources.source_sha256(),resources.dataset_sha256(),
        resources.background_file_sha256().unwrap(),dataset.background.entries.len()
    );
    eprintln!("ROM_DATA runtime=decoded resource={BG_BASE_PALETTES_ID} source={} dataset={} file={} records={} resident_bytes={palette_resident}", resources.source_sha256(), resources.dataset_sha256(), resources.palette_file_sha256().unwrap(), dataset.palettes.entries.len());
    eprintln!("ROM_DATA runtime=decoded resource={BG_ALTERNATE_PALETTES_ID} source={} dataset={} file={} catalogs=2 keys=157", resources.source_sha256(), resources.dataset_sha256(), resources.alternate_palette_file_sha256().unwrap());
    let packet_mode = if resources.packet_file_sha256().is_some() { "decoded" } else { "verified-only" };
    eprintln!("ROM_DATA runtime={packet_mode} resource={BG_PACKET_PROGRAMS_ID} source={} dataset={} file={} programs={}",
        resources.source_sha256(), resources.dataset_sha256(), dataset.manifest.resources[4].sha256,
        dataset.packet_programs.paired.programs.len() + dataset.packet_programs.halfwords.programs.len() + dataset.packet_programs.auxiliary.programs.len());
    if let Some(graphics) = &dataset.main_graphics {
        eprintln!("ROM_DATA runtime=decoded resource={BG_MAIN_GRAPHICS_ID} source={} dataset={} file={} graphics={}",
            resources.source_sha256(), resources.dataset_sha256(), dataset.manifest.resources[5].sha256,
            graphics.graphics.len());
    }
    if let Some(data)= &dataset.metadata16 {
        eprintln!("ROM_DATA runtime=decoded resource={METADATA16_ID} file={} records={}",dataset.manifest.resources[18].sha256,data.records.len());
    }
    if let Some(data)= &dataset.metadata52 {
        eprintln!("ROM_DATA runtime=decoded resource={METADATA52_ID} file={} records={}",dataset.manifest.resources[19].sha256,data.records.len());
    }
    if let Some(data)= &dataset.metadata4 {
        eprintln!("ROM_DATA runtime=decoded resource={METADATA4_ID} file={} records={}",dataset.manifest.resources[20].sha256,data.records.len());
    }
    if let Some(data)= &dataset.metadata2 {
        eprintln!("ROM_DATA runtime=decoded resource={METADATA2_ID} file={} records={}",dataset.manifest.resources[21].sha256,data.records.len());
    }
    if let Some(_data)= &dataset.default48 {
        eprintln!("ROM_DATA runtime=decoded resource={DEFAULT48_ID} file={} templates={}",dataset.manifest.resources[22].sha256,1);
    }
    if let Some(data)= &dataset.constructor_alternates {
        eprintln!("ROM_DATA runtime=decoded resource={CONSTRUCTOR_ALTERNATES_ID} file={} records={}",dataset.manifest.resources[23].sha256,data.records.len());
    }
    if let Some(links)= &dataset.item_links {
        eprintln!("ROM_DATA runtime=decoded resource={ITEM_LINKS_ID} file={} records={}",dataset.manifest.resources[17].sha256,links.records.len());
    }
    if let Some(items) = &dataset.items {
        eprintln!("ROM_DATA runtime=decoded resource={ITEM_RECORDS_ID} file={} records={}",dataset.manifest.resources[16].sha256,items.records.len());
    }
    if let Some(graphics) = &dataset.ui_graphics {
        eprintln!("ROM_DATA runtime=decoded resource={UI_CUSTOM_GRAPHICS_ID} file={} banks={}", dataset.manifest.resources[15].sha256, graphics.banks.len());
    }
    if let Some(palettes) = &dataset.ui_palettes {
        eprintln!("ROM_DATA runtime=decoded resource={UI_RAW_PALETTES_ID} file={} banks={}", dataset.manifest.resources[14].sha256, palettes.banks.len());
    }
    if let Some(graphics) = &dataset.obj_graphics {
        eprintln!("ROM_DATA runtime=decoded resource={OBJ_CUSTOM_GRAPHICS_ID} file={} banks={} selector_tables={}", dataset.manifest.resources[13].sha256, graphics.banks.len(), graphics.pointer_tables.len());
    }
    if let Some(banks) = &dataset.obj_animations {
        eprintln!("ROM_DATA runtime=decoded resource={OBJ_ANIMATION_BANKS_ID} file={} banks={}", dataset.manifest.resources[12].sha256, banks.banks.len());
    }
    if let Some(lists) = &dataset.obj_lists {
        eprintln!("ROM_DATA runtime=decoded resource={OBJ_STATIC_LISTS_ID} file={} lists={}", dataset.manifest.resources[11].sha256, lists.lists.len());
    }
    if let (Some(font), Some(keyboard)) = (&dataset.font_bank, &dataset.name_keyboard) {
        eprintln!("ROM_DATA runtime=decoded resource={FONT_BANK_ID} file={} glyphs={}", dataset.manifest.resources[9].sha256, font.glyphs.len());
        eprintln!("ROM_DATA runtime=decoded resource={NAME_KEYBOARD_ID} file={} pages={}", dataset.manifest.resources[10].sha256, keyboard.programs.len());
    }
    if let Some(texts) = &dataset.text_catalogs {
        eprintln!("ROM_DATA runtime=decoded resource={TEXT_CATALOGS_ID} source={} dataset={} file={} pointer_catalogs={} archive_catalogs={} glyphs={}",
            resources.source_sha256(), resources.dataset_sha256(), dataset.manifest.resources[8].sha256,
            texts.pointer_catalogs.len(), texts.archive_catalogs.len(), texts.glyphs.len());
    }
    if let Some(texts) = &dataset.short_texts {
        let mode = if resources.text_file_sha256().is_some() { "decoded" } else { "verified-only" };
        eprintln!("ROM_DATA runtime={mode} resource={SHORT_TEXTS_ID} source={} dataset={} file={} languages={} strings={} glyphs={}",
            resources.source_sha256(), resources.dataset_sha256(), dataset.manifest.resources[7].sha256,
            texts.languages.len(), texts.strings.len(), texts.glyphs.len());
    }
    if let Some(transfers) = &dataset.transfers {
        eprintln!("ROM_DATA runtime=decoded resource={BG_TRANSFER_PROGRAMS_ID} source={} dataset={} file={} controls={} tiles={}",
            resources.source_sha256(), resources.dataset_sha256(), dataset.manifest.resources[6].sha256,
            transfers.controls.len(), transfers.tiles.len());
    }
    Ok(())
}

pub fn prepare_and_attach(machine: &mut gba_core::Machine, status: bool) -> Result<(), String> {
    let prepared = prepare_startup(&machine.bus.rom, status)?;
    attach_from_directory(machine, &prepared.directory)
}

pub fn report_usage(machine: &gba_core::Machine) {
    if let Some(resources) = machine.bus.rom_resources() {
        let stats = resources.stats();
        eprintln!("ROM_DATA_USE resource={} dataset={} bus_reads={} host_rom_reads={} host_ram_reads={} dynamic_ram_reads={} validation_reads={} bus_source_mask={}",
            resources.resource_id(), resources.dataset_sha256(), stats.bus_reads, stats.host_rom_reads,
            stats.host_ram_reads, stats.dynamic_ram_reads, stats.validation_reads, stats.bus_source_mask);
        if let Some(bg) = resources.background_stats() {
            eprintln!("ROM_DATA_USE resource={BG_DESCRIPTORS_ID} dataset={} bus_reads={} projected_bytes={}",
                resources.dataset_sha256(),bg.bus_reads,bg.projected_bytes);
        }
        if let Some(palette) = resources.palette_stats() {
            eprintln!("ROM_DATA_USE resource={BG_BASE_PALETTES_ID} dataset={} bus_reads={} source_bytes={} lz77_calls={} decoded_bytes={}", resources.dataset_sha256(), palette.bus_reads, palette.source_bytes, palette.lz77_calls, palette.decoded_bytes);
        }
        if let Some(alternate) = resources.alternate_palette_stats() {
            eprintln!("ROM_DATA_USE resource={BG_ALTERNATE_PALETTES_ID} dataset={} bus_reads={} source_bytes={} table_bytes={} decoded_literal_bytes={}", resources.dataset_sha256(), alternate.bus_reads, alternate.source_bytes, alternate.table_bytes, alternate.decoded_literal_bytes);
        }
        if let Some(packet) = resources.packet_stats() {
            eprintln!("ROM_DATA_USE resource={BG_PACKET_PROGRAMS_ID} dataset={} bus_reads={} source_bytes={} typed_value_bytes={} lz77_calls={} decoded_body_bytes={} typed_hle_bytes={}", resources.dataset_sha256(), packet.bus_reads, packet.source_bytes, packet.typed_value_bytes, packet.lz77_calls, packet.decoded_body_bytes, packet.typed_hle_bytes);
        }
        if let Some(graphics) = resources.graphics_stats() {
            eprintln!("ROM_DATA_USE resource={BG_MAIN_GRAPHICS_ID} dataset={} bus_reads={} source_bytes={} decoded_literal_bytes={}",
                resources.dataset_sha256(), graphics.bus_reads, graphics.source_bytes, graphics.decoded_literal_bytes);
        }
        if let Some(i)=resources.metadata16_stats() {
            eprintln!("ROM_DATA_USE resource={METADATA16_ID} dataset={} bus_reads={} source_bytes={} alias_bytes={} display_bytes={} flag_bytes={} value_bytes={} opaque_bytes={}",resources.dataset_sha256(),i.bus_reads,i.source_bytes,i.alias_bytes,i.display_bytes,i.flag_bytes,i.value_bytes,i.opaque_bytes);
        }
        if let Some(i)=resources.metadata52_stats() {
            eprintln!("ROM_DATA_USE resource={METADATA52_ID} dataset={} bus_reads={} source_bytes={} header_bytes={} routing_bytes={} packed_bytes={} tail_bytes={} opaque_bytes={}",resources.dataset_sha256(),i.bus_reads,i.source_bytes,i.header_bytes,i.routing_bytes,i.packed_bytes,i.tail_bytes,i.opaque_bytes);
        }
        if let Some(i)=resources.metadata4_stats() {
            eprintln!("ROM_DATA_USE resource={METADATA4_ID} dataset={} bus_reads={} source_bytes={} selector_bytes={} value_bytes={}",resources.dataset_sha256(),i.bus_reads,i.source_bytes,i.selector_bytes,i.value_bytes);
        }
        if let Some(i)=resources.metadata2_stats() {
            eprintln!("ROM_DATA_USE resource={METADATA2_ID} dataset={} bus_reads={} source_bytes={} key_bytes={} value_bytes={}",resources.dataset_sha256(),i.bus_reads,i.source_bytes,i.key_bytes,i.value_bytes);
        }
        if let Some(i)=resources.default48_stats() {
            eprintln!("ROM_DATA_USE resource={DEFAULT48_ID} dataset={} bus_reads={} source_bytes={} header_bytes={} argument_bytes={} halfword_bytes={} flag_bytes={} tail_bytes={}",resources.dataset_sha256(),i.bus_reads,i.source_bytes,i.header_bytes,i.argument_bytes,i.halfword_bytes,i.flag_bytes,i.tail_bytes);
        }
        if let Some(i)=resources.constructor_alternates_stats() {
            eprintln!("ROM_DATA_USE resource={CONSTRUCTOR_ALTERNATES_ID} dataset={} bus_reads={} source_bytes={} pointer_bytes={} record_mask={} header_bytes={} argument_bytes={} halfword_bytes={} flag_bytes={} tail_bytes={}",resources.dataset_sha256(),i.bus_reads,i.source_bytes,i.pointer_bytes,i.record_mask,i.header_bytes,i.argument_bytes,i.halfword_bytes,i.flag_bytes,i.tail_bytes);
        }
        if let Some(i)=resources.item_link_stats() {
            eprintln!("ROM_DATA_USE resource={ITEM_LINKS_ID} dataset={} bus_reads={} source_bytes={} count_bytes={} selector_bytes={} value_bytes={} opaque_bytes={}",resources.dataset_sha256(),i.bus_reads,i.source_bytes,i.count_bytes,i.selector_bytes,i.value_bytes,i.opaque_bytes);
        }
        if let Some(i)=resources.item_stats() {
            eprintln!("ROM_DATA_USE resource={ITEM_RECORDS_ID} dataset={} bus_reads={} source_bytes={} halfword_bytes={} signed_bytes={} other_bytes={}",resources.dataset_sha256(),i.bus_reads,i.source_bytes,i.halfword_bytes,i.signed_bytes,i.other_bytes);
        }
        if let Some(g) = resources.ui_graphics_stats() {
            eprintln!("ROM_DATA_USE resource={UI_CUSTOM_GRAPHICS_ID} dataset={} bus_reads={} source_bytes={} decoded_literal_bytes={} selector_bytes={} localized_source_bytes={} localized_literal_bytes={}", resources.dataset_sha256(), g.bus_reads, g.source_bytes, g.decoded_literal_bytes,g.selector_bytes,g.localized_source_bytes,g.localized_literal_bytes);
        }
        for (offset, bytes) in resources.ui_graphics_used_sources() {
            eprintln!("ROM_DATA_UI_GRAPHICS_SOURCE source={offset:08x} source_bytes={bytes}");
        }
        if let Some(p) = resources.ui_palette_stats() {
            eprintln!("ROM_DATA_USE resource={UI_RAW_PALETTES_ID} dataset={} bus_reads={} source_bytes={}", resources.dataset_sha256(), p.bus_reads, p.source_bytes);
        }
        if let Some(g) = resources.obj_graphics_stats() {
            eprintln!("ROM_DATA_USE resource={OBJ_CUSTOM_GRAPHICS_ID} dataset={} bus_reads={} source_bytes={} decoded_literal_bytes={} selector_bytes={}", resources.dataset_sha256(), g.bus_reads, g.source_bytes, g.decoded_literal_bytes, g.selector_bytes);
        }
        if let Some(a) = resources.obj_animation_stats() {
            eprintln!("ROM_DATA_USE resource={OBJ_ANIMATION_BANKS_ID} dataset={} bus_reads={} source_bytes={} index_bytes={} count_bytes={} attribute_bytes={} sequence_header_bytes={} frame_list_bytes={} duration_bytes={} host_reads={} zero_count_selector_bytes={} zero_count_host_reads={} localized_selector_bytes={} localized_source_bytes={} localized_host_reads={}",
                resources.dataset_sha256(), a.bus_reads, a.source_bytes, a.index_bytes, a.count_bytes, a.attribute_bytes, a.sequence_header_bytes, a.frame_list_bytes, a.duration_bytes, a.host_reads, a.zero_count_selector_bytes, a.zero_count_host_reads, a.localized_selector_bytes, a.localized_source_bytes, a.localized_host_reads);
        }
        if let Some(l) = resources.obj_list_stats() {
            eprintln!("ROM_DATA_USE resource={OBJ_STATIC_LISTS_ID} dataset={} bus_reads={} source_bytes={} count_bytes={} attribute_bytes={} host_reads={}",
                resources.dataset_sha256(), l.bus_reads, l.source_bytes, l.count_bytes, l.attribute_bytes, l.host_reads);
        }
        for (offset, guest, host) in resources.obj_list_used_sources() {
            eprintln!("ROM_DATA_OBJ_SOURCE source={offset:08x} guest={guest} host={host}");
        }
        if let Some(f) = resources.font_keyboard_stats() {
            eprintln!("ROM_DATA_USE resource={FONT_BANK_ID} dataset={} bus_reads={} glyph_bytes={} width_bytes={}", resources.dataset_sha256(), f.bus_reads, f.glyph_bytes, f.width_bytes);
            eprintln!("ROM_DATA_USE resource={NAME_KEYBOARD_ID} dataset={} index_bytes={} program_bytes={} row_width_bytes={} host_row_width_reads={}", resources.dataset_sha256(), f.keyboard_index_bytes, f.keyboard_program_bytes, f.keyboard_row_width_bytes, f.host_row_width_reads);
        }
        if let Some(t) = resources.text_catalog_stats() {
            eprintln!("ROM_DATA_USE resource={TEXT_CATALOGS_ID} dataset={} bus_reads={} source_bytes={} pointer_table_bytes={} string_bytes={} archive_index_bytes={} header_bytes={} encoded_control_bytes={} decoded_program_bytes={} glyph_bytes={} width_bytes={}",
                resources.dataset_sha256(),t.bus_reads,t.source_bytes,t.pointer_table_bytes,t.string_bytes,t.archive_index_bytes,t.header_bytes,t.encoded_control_bytes,t.decoded_program_bytes,t.glyph_bytes,t.width_bytes);
        }
        if let Some(texts) = resources.text_stats() {
            eprintln!("ROM_DATA_USE resource={SHORT_TEXTS_ID} dataset={} bus_reads={} source_bytes={} table_bytes={} string_bytes={} glyph_bytes={} width_bytes={}",
                resources.dataset_sha256(), texts.bus_reads, texts.source_bytes, texts.table_bytes, texts.string_bytes, texts.glyph_bytes, texts.width_bytes);
        }
        if let Some(transfers) = resources.transfer_stats() {
            eprintln!("ROM_DATA_USE resource={BG_TRANSFER_PROGRAMS_ID} dataset={} bus_reads={} source_bytes={} control_bytes={} tile_bytes={}",
                resources.dataset_sha256(), transfers.bus_reads, transfers.source_bytes, transfers.control_bytes, transfers.tile_bytes);
        }
    }
}

fn write_synced(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("Cannot create {}: {e}", path.display()))?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(|e| format!("Cannot finish {}: {e}", path.display()))
}

/// Publish only into an absent directory. An existing valid dataset is reused;
/// invalid data produce an explicit error, never a silent ROM fallback.
pub fn extract(path: &Path, source: &mut Extraction) -> Result<&'static str, String> {
    match fs::symlink_metadata(path) {
        Ok(_) => {
            verify(path, source)?;
            return Ok("reused");
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(format!("Cannot inspect output directory: {e}")),
    }
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent).map_err(|e| format!("Cannot create resources parent: {e}"))?;
    let temporary = tempfile::Builder::new()
        .prefix(".ffta-resources-")
        .tempdir_in(parent)
        .map_err(|e| format!("Cannot prepare resource directory: {e}"))?;
    write_dataset(temporary.path(), source, &mut write_synced)?;
    match fs::rename(temporary.path(), path) {
        Ok(()) => Ok("created"),
        // A concurrent writer may have published the same complete dataset.
        Err(_) if verify(path, source).is_ok() => Ok("reused"),
        Err(e) => Err(format!("Cannot publish resource directory: {e}")),
    }
}

fn write_dataset(
    path: &Path,
    source: &mut Extraction,
    write: &mut impl FnMut(&Path, &[u8]) -> Result<(), String>,
) -> Result<(), String> {
    let data = toml::to_string_pretty(&source.dimensions)
        .map_err(|e| format!("Cannot encode dimensions: {e}"))?;
    let background = source
        .background
        .as_ref()
        .ok_or("Missing background descriptor extraction")?;
    let bg_data = encode_background(background)?;
    let palette_data = encode_palettes(
        &source
            .palettes
            .as_ref()
            .ok_or("Missing base palette extraction")?
            .decoded,
    )?;
    let alternate_palette_data = encode_alternate_palettes(
        &source.alternate_palettes.as_ref().ok_or("Missing alternate palette extraction")?.decoded,
    )?;
    let packet_program_data = encode_packet_programs(
        &source.packet_programs.as_ref().ok_or("Missing packet program extraction")?.decoded,
    )?;
    let main_graphic_data = source.main_graphics.as_ref()
        .map(|catalog| encode_main_graphics(&catalog.decoded)).transpose()?;
    let transfer_data = source.transfers.as_ref()
        .map(|catalog| encode_transfers(&catalog.decoded)).transpose()?;
    let short_text_data = source.short_texts.as_ref()
        .map(|catalog| encode_short_texts(&catalog.decoded)).transpose()?;
    let text_catalog_data = source.text_catalogs.as_ref().map(|catalog| encode_text_catalogs(&catalog.decoded)).transpose()?;
    let font_bank_data = source.font_bank.as_ref().map(|catalog| encode_font_bank(&catalog.decoded)).transpose()?;
    let name_keyboard_data = source.name_keyboard.as_ref().map(|catalog| encode_name_keyboard(&catalog.decoded)).transpose()?;
    let obj_list_data = source.obj_lists.as_ref().map(|catalog| encode_obj_lists(&catalog.decoded)).transpose()?;
    let obj_animation_data = source.obj_animations.as_ref().map(|catalog| encode_obj_animations(&catalog.decoded)).transpose()?;
    let obj_graphic_data = source.obj_graphics.as_ref().map(|catalog| encode_obj_graphics(&catalog.decoded)).transpose()?;
    let ui_palette_data = source.ui_palettes.as_ref().map(|catalog| encode_ui_palettes(&catalog.decoded)).transpose()?;
    let ui_graphic_data = source.ui_graphics.as_ref().map(|catalog| encode_ui_graphics(&catalog.decoded)).transpose()?;
    let item_data = source.items.as_ref().map(|catalog|encode_items(&catalog.decoded)).transpose()?;
    let item_link_data=source.item_links.as_ref().map(|catalog|encode_item_links(&catalog.decoded)).transpose()?;
    let metadata16_data=source.metadata16.as_ref().map(|catalog|encode_metadata16(&catalog.decoded)).transpose()?;
    let metadata52_data=source.metadata52.as_ref().map(|catalog|encode_metadata52(&catalog.decoded)).transpose()?;
    let metadata4_data=source.metadata4.as_ref().map(|catalog|encode_metadata4(&catalog.decoded)).transpose()?;
    let metadata2_data=source.metadata2.as_ref().map(|catalog|encode_metadata2(&catalog.decoded)).transpose()?;
    let default48_data=source.default48.as_ref().map(|catalog|encode_default48(&catalog.decoded)).transpose()?;
    let constructor_alternates_data=source.constructor_alternates.as_ref().map(|catalog|encode_constructor_alternates(&catalog.decoded)).transpose()?;
    let mut files: Vec<&[u8]> = vec![data.as_bytes(), &bg_data, &palette_data, &alternate_palette_data, &packet_program_data];
    if let Some(bytes) = &main_graphic_data { files.push(bytes); }
    if let Some(bytes) = &transfer_data { files.push(bytes); }
    if let Some(bytes) = &short_text_data { files.push(bytes); }
    if let Some(bytes) = &text_catalog_data { files.push(bytes); }
    if let Some(bytes) = &font_bank_data { files.push(bytes); }
    if let Some(bytes) = &name_keyboard_data { files.push(bytes); }
    if let Some(bytes) = &obj_list_data { files.push(bytes); }
    if let Some(bytes) = &obj_animation_data { files.push(bytes); }
    if let Some(bytes) = &obj_graphic_data { files.push(bytes); }
    if let Some(bytes) = &ui_palette_data { files.push(bytes); }
    if let Some(bytes) = &ui_graphic_data { files.push(bytes); }
    if let Some(bytes) = &item_data {files.push(bytes);}
    if let Some(bytes) = &item_link_data {files.push(bytes);}
    if let Some(bytes) = &metadata16_data {files.push(bytes);}
    if let Some(bytes) = &metadata52_data {files.push(bytes);}
    if let Some(bytes) = &metadata4_data {files.push(bytes);}
    if let Some(bytes) = &metadata2_data {files.push(bytes);}
    if let Some(bytes) = &default48_data {files.push(bytes);}
    if let Some(bytes) = &constructor_alternates_data {files.push(bytes);}
    source.set_file_hashes(&files)?;
    let manifest = toml::to_string_pretty(&source.manifest)
        .map_err(|e| format!("Cannot encode manifest: {e}"))?;
    check_dataset_budget(
        manifest.len(),
        data.len(),
        bg_data.len(),
        palette_data.len(),
        alternate_palette_data.len(),
        packet_program_data.len(),
        main_graphic_data.as_ref().map_or(0, Vec::len),
        transfer_data.as_ref().map_or(0, Vec::len),
        short_text_data.as_ref().map_or(0, Vec::len),
        text_catalog_data.as_ref().map_or(0, Vec::len),
        font_bank_data.as_ref().map_or(0, Vec::len),
        name_keyboard_data.as_ref().map_or(0, Vec::len),
        obj_list_data.as_ref().map_or(0, Vec::len),
        obj_animation_data.as_ref().map_or(0, Vec::len),
        obj_graphic_data.as_ref().map_or(0, Vec::len),
        ui_palette_data.as_ref().map_or(0, Vec::len),
        ui_graphic_data.as_ref().map_or(0, Vec::len),
        item_data.as_ref().map_or(0, Vec::len),
        item_link_data.as_ref().map_or(0, Vec::len),
        metadata16_data.as_ref().map_or(0, Vec::len),
        metadata52_data.as_ref().map_or(0, Vec::len),
        metadata4_data.as_ref().map_or(0, Vec::len),
        metadata2_data.as_ref().map_or(0, Vec::len),
        default48_data.as_ref().map_or(0, Vec::len),
        constructor_alternates_data.as_ref().map_or(0, Vec::len),
    )?;
    write(&path.join(OBJ_DIMENSIONS_FILE), data.as_bytes())?;
    write(&path.join(BG_DESCRIPTORS_FILE), &bg_data)?;
    write(&path.join(BG_BASE_PALETTES_FILE), &palette_data)?;
    write(&path.join(BG_ALTERNATE_PALETTES_FILE), &alternate_palette_data)?;
    write(&path.join(BG_PACKET_PROGRAMS_FILE), &packet_program_data)?;
    if let Some(bytes) = &main_graphic_data { write(&path.join(BG_MAIN_GRAPHICS_FILE), bytes)?; }
    if let Some(bytes) = &transfer_data { write(&path.join(BG_TRANSFER_PROGRAMS_FILE), bytes)?; }
    if let Some(bytes) = &short_text_data { write(&path.join(SHORT_TEXTS_FILE), bytes)?; }
    if let Some(bytes) = &text_catalog_data { write(&path.join(TEXT_CATALOGS_FILE), bytes)?; }
    if let Some(bytes) = &font_bank_data { write(&path.join(FONT_BANK_FILE), bytes)?; }
    if let Some(bytes) = &name_keyboard_data { write(&path.join(NAME_KEYBOARD_FILE), bytes)?; }
    if let Some(bytes) = &obj_list_data { write(&path.join(OBJ_STATIC_LISTS_FILE), bytes)?; }
    if let Some(bytes) = &obj_animation_data { write(&path.join(OBJ_ANIMATION_BANKS_FILE), bytes)?; }
    if let Some(bytes) = &obj_graphic_data { write(&path.join(OBJ_CUSTOM_GRAPHICS_FILE), bytes)?; }
    if let Some(bytes) = &ui_palette_data { write(&path.join(UI_RAW_PALETTES_FILE), bytes)?; }
    if let Some(bytes) = &ui_graphic_data { write(&path.join(UI_CUSTOM_GRAPHICS_FILE), bytes)?; }
    if let Some(bytes) = &item_data {write(&path.join(ITEM_RECORDS_FILE),bytes)?;}
    if let Some(bytes) = &item_link_data {write(&path.join(ITEM_LINKS_FILE),bytes)?;}
    if let Some(bytes) = &metadata16_data {write(&path.join(METADATA16_FILE),bytes)?;}
    if let Some(bytes) = &metadata52_data {write(&path.join(METADATA52_FILE),bytes)?;}
    if let Some(bytes) = &metadata4_data {write(&path.join(METADATA4_FILE),bytes)?;}
    if let Some(bytes) = &metadata2_data {write(&path.join(METADATA2_FILE),bytes)?;}
    if let Some(bytes) = &default48_data {write(&path.join(DEFAULT48_FILE),bytes)?;}
    if let Some(bytes) = &constructor_alternates_data {write(&path.join(CONSTRUCTOR_ALTERNATES_FILE),bytes)?;}
    write(&path.join(MANIFEST_FILE), manifest.as_bytes())?;
    verify(path, source)?;
    Ok(())
}

pub fn cmd(args: &[String]) -> Result<(), String> {
    let operation = args.first().map(String::as_str);
    if !matches!(
        (operation, args.len()),
        (Some("extract" | "verify"), 3) | (Some("prepare"), 2 | 3)
    ) {
        return Err("usage: recomp rom-data <extract|verify> <rom> <dataset-directory> OR rom-data prepare <rom> [resources-root]".into());
    }
    let rom = fs::read(&args[1]).map_err(|e| format!("Cannot read ROM: {e}"))?;
    let mut source = Extraction::from_rom(&rom)?;
    if operation == Some("prepare") {
        let root = args
            .get(2)
            .map(PathBuf::from)
            .map(Ok)
            .unwrap_or_else(preparation::storage_root)?;
        let prepared = preparation::prepare(
            &root,
            &mut source,
            &mut preparation::console_progress(false),
        )?;
        println!("ROM_DATA action={} profile={} source={} schema={} decoded={} runtime=offline directory={}",
            prepared.action, source.manifest.profile, source.manifest.source_rom_sha256,
            source.manifest.schema, source.manifest.coverage.join(","), prepared.directory.display());
        return Ok(());
    }
    let path = Path::new(&args[2]);
    let action = if args[0] == "extract" {
        extract(path, &mut source)?
    } else {
        verify(path, &source)?;
        "verified"
    };
    println!("ROM_DATA action={action} profile={} source={} schema={} decoded={} files={} runtime=offline",
        source.manifest.profile, source.manifest.source_rom_sha256, source.manifest.schema,
        source.manifest.coverage.join(","), source.manifest.resources.len() + 1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Synthetic structure, deliberately not a copy of a game's dimension table.
    pub(super) fn fixture() -> Extraction {
        use gba_core::rom_data::*;
        let dimensions = ObjDimensions {
            entries: std::array::from_fn(|i| ObjSize {
                width: 8 << (i % 4),
                height: 8 << ((i + 1) % 4),
            }),
        };
        let absent = bg_descriptors::TransferLink {
            control_source: 0,
            data_source: 0,
            destination: 0,
        };
        let mut background = Descriptors {
            entries: vec![
                bg_descriptors::Descriptor {
                    graphics_source: 0x4000,
                    paired_planes_source: 0x4100,
                    halfword_planes_source: 0x4200,
                    palette_source: 0x4300,
                    auxiliary_patches_source: 0x4400,
                    transfer: absent.clone(),
                    extra_transfers: std::array::from_fn(|_| absent.clone()),
                    variant_source: 0,
                    selectors: [1, 2],
                    alternate_palette_key: 3,
                    trailing_byte: 0,
                };
                bg_descriptors::BASE_RECORDS
            ],
        };
        for (id, entry) in background.entries.iter_mut().enumerate() {
            entry.alternate_palette_key = if id < bg_alternate_palettes::BASE_KEYS {id as u8} else {255};
        }
        let bg_hash = gba_core::ffta_rom::digest(&background.source_bytes().unwrap());
        let decoded_palettes = Palettes {
            entries: vec![
                bg_palettes::Palette {
                    banks: vec![[0x8123; 16]; 4],
                    encoded_source: [
                        vec![0, 128, 0, 0],
                        (0..64).flat_map(|_| 0x8123u16.to_le_bytes()).collect()
                    ]
                    .concat(),
                };
                bg_descriptors::BASE_RECORDS
            ],
        };
        let palette_sources: Vec<_> = (0..bg_descriptors::BASE_RECORDS)
            .map(|id| bg_palettes::PaletteSource {
                range: SourceRange {
                    offset: 0x4400,
                    length: 132,
                    sha256: "synthetic-palette-source".into(),
                    role: format!("base-palette/{id}"),
                },
                encoding: bg_palettes::Encoding::Raw,
            })
            .collect();
        let palette_ranges = palette_sources
            .iter()
            .map(|source| source.range.clone())
            .collect();
        let alternate_tables = std::array::from_fn(|group| SourceRange {
            offset: 0x6000 + group as u32 * 0x2000, length: 314,
            sha256: format!("synthetic-alternate-links-{group}"),
            role: format!("alternate-palette-links/{group}"),
        });
        let alternate_sources: Vec<_> = (0..314).map(|index| SourceRange {
            offset: alternate_tables[index / 157].offset + 316 + (index % 157) as u32 * 8,
            length: 6, sha256: "synthetic-alternate-source".into(),
            role: format!("alternate-palette/{}/{}", index / 157, index % 157),
        }).collect();
        let alternate_ranges = alternate_tables.iter().chain(&alternate_sources).cloned().collect();
        let alternate_palettes = bg_alternate_palettes::Catalog {
            tables: alternate_tables,
            sources: alternate_sources,
            decoded: AlternatePalettes {
                descriptor_keys: background.entries.iter().map(|e| e.alternate_palette_key).collect(),
                relative_sources: std::array::from_fn(|_| (0..157).map(|i| (316+i*8) as u16).collect()),
                catalogs: std::array::from_fn(|group| vec![bg_alternate_palettes::Palette::decode(&[0,0,1,0,if group==0 {1}else{2},253]).unwrap().0;157]),
            },
        };
        let packet_family = |role, relative, body: &[u8]| {
            let source = [vec![1, 255, 255, 255], body.to_vec()].concat();
            bg_packet_data::Family {
                descriptor_programs: vec![0; bg_descriptors::BASE_RECORDS],
                programs: vec![bg_packet_data::Program::decode(&source, role, relative).unwrap().0],
            }
        };
        let mut paired_body = vec![16, 0, 0, 128, 0, 0, 1, 0, 0, 0];
        paired_body.resize(16, 0);
        paired_body.extend_from_slice(&[7, 0]);
        let halfword_body = [0, 0, 2, 0, 0x23, 0x81, 0, 0];
        let packet_decoded = bg_packet_data::Programs {
            paired: packet_family(bg_packet_data::Role::Paired, 0x4100, &paired_body),
            halfwords: packet_family(bg_packet_data::Role::Halfwords, 0x4200, &halfword_body),
            auxiliary: packet_family(bg_packet_data::Role::Auxiliary, 0x4400, &halfword_body),
        };
        let packet_sources: Vec<_> = bg_packet_data::Role::ALL.into_iter().map(|role| {
            let program = &packet_decoded.family(role).programs[0];
            SourceRange {
                offset: 0x100 + program.relative_source,
                length: program.encoded_source.len() as u32,
                sha256: gba_core::ffta_rom::digest(&program.encoded_source),
                role: format!("bg-packet-program/{}/0", role.name()),
            }
        }).collect();
        let packet_ranges = packet_sources.clone();
        let packet_programs = bg_packet_data::Catalog { decoded: packet_decoded, sources: packet_sources };
        Extraction {
            manifest: Manifest {
                schema: SCHEMA,
                extractor_revision: EXTRACTOR_REV,
                decoder_revision: DECODER_REV,
                source_rom_sha256: "synthetic-source".into(),
                source_rom_size: 0x100000,
                profile: "synthetic".into(),
                coverage: vec![
                    OBJ_DIMENSIONS_ID.into(),
                    BG_DESCRIPTORS_ID.into(),
                    BG_BASE_PALETTES_ID.into(),
                    BG_ALTERNATE_PALETTES_ID.into(),
                    BG_PACKET_PROGRAMS_ID.into(),
                ],
                resources: vec![
                    Resource {
                        id: OBJ_DIMENSIONS_ID.into(),
                        path: OBJ_DIMENSIONS_FILE.into(),
                        format: "obj-dimensions-v1".into(),
                        sha256: String::new(),
                        dependencies: vec![],
                        sources: vec![SourceRange {
                            offset: 0,
                            length: 48,
                            sha256: gba_core::ffta_rom::digest(&dimensions.source_bytes()),
                            role: "synthetic".into(),
                        }],
                    },
                    Resource {
                        id: BG_DESCRIPTORS_ID.into(),
                        path: BG_DESCRIPTORS_FILE.into(),
                        format: gba_core::rom_data::BG_DESCRIPTORS_FORMAT.into(),
                        sha256: String::new(),
                        dependencies: vec![],
                        sources: vec![SourceRange {
                            offset: 0x100,
                            length: (bg_descriptors::BASE_RECORDS * bg_descriptors::RECORD_BYTES)
                                as u32,
                            sha256: bg_hash,
                            role: "synthetic-bg".into(),
                        }],
                    },
                    Resource {
                        id: BG_BASE_PALETTES_ID.into(),
                        path: BG_BASE_PALETTES_FILE.into(),
                        format: BG_BASE_PALETTES_FORMAT.into(),
                        sha256: String::new(),
                        dependencies: vec![BG_DESCRIPTORS_ID.into()],
                        sources: palette_ranges,
                    },
                    Resource {
                        id: BG_ALTERNATE_PALETTES_ID.into(), path: BG_ALTERNATE_PALETTES_FILE.into(),
                        format: BG_ALTERNATE_PALETTES_FORMAT.into(), sha256: String::new(),
                        dependencies: vec![BG_DESCRIPTORS_ID.into()], sources: alternate_ranges,
                    },
                    Resource {
                        id: BG_PACKET_PROGRAMS_ID.into(), path: BG_PACKET_PROGRAMS_FILE.into(),
                        format: BG_PACKET_PROGRAMS_FORMAT.into(), sha256: String::new(),
                        dependencies: vec![BG_DESCRIPTORS_ID.into()], sources: packet_ranges,
                    },
                ],
            },
            dimensions,
            background: Some(background),
            palettes: Some(bg_palettes::Catalog {
                decoded: decoded_palettes,
                sources: palette_sources,
            }),
            alternate_palettes: Some(alternate_palettes),
            packet_programs: Some(packet_programs),
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
            font_bank: None,
        }
    }

    #[test]
    fn deterministic_publication_reuse_and_relocation() {
        let temp = tempfile::tempdir().unwrap();
        let a = temp.path().join("first folder/data");
        let b = temp.path().join("second");
        let mut source = fixture();
        assert_eq!(extract(&a, &mut source).unwrap(), "created");
        let before = fs::read(a.join(MANIFEST_FILE)).unwrap();
        assert_eq!(extract(&a, &mut source).unwrap(), "reused");
        assert_eq!(before, fs::read(a.join(MANIFEST_FILE)).unwrap());
        assert_eq!(extract(&b, &mut source).unwrap(), "created");
        for name in [
            MANIFEST_FILE,
            OBJ_DIMENSIONS_FILE,
            BG_DESCRIPTORS_FILE,
            BG_BASE_PALETTES_FILE,
            BG_ALTERNATE_PALETTES_FILE,
            BG_PACKET_PROGRAMS_FILE,
        ] {
            assert_eq!(
                fs::read(a.join(name)).unwrap(),
                fs::read(b.join(name)).unwrap()
            );
        }
        let moved = temp.path().join("moved folder");
        fs::rename(&a, &moved).unwrap();
        assert_eq!(verify(&moved, &source).unwrap(), source.dimensions);
        assert!(!String::from_utf8(before)
            .unwrap()
            .contains(&temp.path().display().to_string()));
    }

    #[test]
    fn corruption_missing_files_and_edits_fail_without_overwriting() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("data");
        let mut source = fixture();
        extract(&path, &mut source).unwrap();
        let original = fs::read(path.join(OBJ_DIMENSIONS_FILE)).unwrap();
        fs::write(path.join(OBJ_DIMENSIONS_FILE), b"broken").unwrap();
        assert!(verify(&path, &source).is_err());
        assert!(extract(&path, &mut source).is_err());
        assert_eq!(fs::read(path.join(OBJ_DIMENSIONS_FILE)).unwrap(), b"broken");
        fs::write(path.join(OBJ_DIMENSIONS_FILE), &original).unwrap();
        let mut changed = source.dimensions.clone();
        changed.entries[0].width = 64;
        let bytes = toml::to_string_pretty(&changed).unwrap();
        let mut manifest = source.manifest.clone();
        manifest.resources[0].sha256 = gba_core::ffta_rom::digest(bytes.as_bytes());
        fs::write(path.join(OBJ_DIMENSIONS_FILE), &bytes).unwrap();
        fs::write(
            path.join(MANIFEST_FILE),
            toml::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
        assert!(verify(&path, &source).is_err());
        fs::remove_file(path.join(OBJ_DIMENSIONS_FILE)).unwrap();
        assert!(verify(&path, &source).is_err());
        fs::write(
            path.join(OBJ_DIMENSIONS_FILE),
            [0; MAX_FILE_BYTES as usize + 1],
        )
        .unwrap();
        assert!(verify(&path, &source).is_err());
    }

    #[test]
    fn strict_parser_and_fixed_paths() {
        let source = fixture();
        let bytes = toml::to_string_pretty(&source.dimensions).unwrap();
        let wrong = format!("unknown = 1\n{bytes}");
        assert!(parse::<ObjDimensions>(wrong.as_bytes(), "test").is_err());
        let wrong = bytes.replacen("width = 8", "width = 65536", 1);
        assert!(parse::<ObjDimensions>(wrong.as_bytes(), "test").is_err());
        assert!(parse::<ObjDimensions>(&[255], "test").is_err());
        let fewer = ObjDimensions {
            entries: source.dimensions.entries,
        };
        let text = toml::to_string_pretty(&fewer).unwrap();
        let cut = text.rfind("[[entries]]").unwrap();
        assert!(parse::<ObjDimensions>(text[..cut].as_bytes(), "test").is_err());
        assert!(cmd(&[]).is_err());
        assert!(cmd(&["unknown".into(), "rom".into(), "out".into()]).is_err());
    }

    #[test]
    fn background_json_is_direct_strict_and_deterministic() {
        let source = fixture();
        let background = source.background.as_ref().unwrap();
        let bytes = encode_background(background).unwrap();
        assert_eq!(bytes, encode_background(background).unwrap());
        assert_eq!(&parse_background(&bytes).unwrap(), background);
        assert!(bytes.starts_with(b"{\n  \"entries\": ["));
        assert!(bytes.len() as u64 <= MAX_BG_DESCRIPTORS_FILE_BYTES);
        assert!(parse_background(&[255]).is_err());
        assert!(parse_background(&bytes[..bytes.len() - 1]).is_err());
        let mut trailing = bytes.clone();
        trailing.extend_from_slice(b" true");
        assert!(parse_background(&trailing).is_err());
        assert!(parse_background(br#"{"entries":[],"entries":[]}"#).is_err());
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        for invalid in 0..6 {
            let mut changed = value.clone();
            match invalid {
                0 => changed["entries"][0]["unknown"] = serde_json::json!(0),
                1 => changed["entries"][0]["graphics_source"] = serde_json::json!(4294967296u64),
                2 => changed["entries"][0]["selectors"][0] = serde_json::json!(256),
                3 => {
                    changed["entries"][0]
                        .as_object_mut()
                        .unwrap()
                        .remove("selectors");
                }
                4 => {
                    changed["entries"][0]["extra_transfers"]
                        .as_array_mut()
                        .unwrap()
                        .pop();
                }
                5 => changed["entries"] = serde_json::json!([]),
                _ => unreachable!(),
            }
            assert!(parse_background(&serde_json::to_vec(&changed).unwrap()).is_err());
        }
    }

    #[test]
    fn background_is_required_and_rehashed_edits_are_not_base_data() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("data");
        let mut source = fixture();
        extract(&path, &mut source).unwrap();
        let original = fs::read(path.join(BG_DESCRIPTORS_FILE)).unwrap();
        assert!(original.len() > MAX_FILE_BYTES as usize);
        let mut edited = source.background.clone().unwrap();
        edited.entries[0].selectors[0] ^= 1;
        let bytes = encode_background(&edited).unwrap();
        let mut manifest = source.manifest.clone();
        manifest.resources[1].sha256 = gba_core::ffta_rom::digest(&bytes);
        fs::write(path.join(BG_DESCRIPTORS_FILE), &bytes).unwrap();
        fs::write(
            path.join(MANIFEST_FILE),
            toml::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
        assert!(verify(&path, &source).unwrap_err().contains("reproduce"));
        assert!(extract(&path, &mut source).is_err());
        assert_eq!(
            fs::read(path.join(BG_DESCRIPTORS_FILE)).unwrap(),
            bytes.as_slice()
        );
        fs::write(path.join(BG_DESCRIPTORS_FILE), original).unwrap();
        for change in 0..4 {
            let mut bad = source.manifest.clone();
            match change {
                0 => {
                    bad.resources.remove(1);
                }
                1 => bad.resources.push(bad.resources[1].clone()),
                2 => bad.resources[1].path = "../outside".into(),
                _ => bad.resources[1].sources[0].offset += 1,
            }
            fs::write(
                path.join(MANIFEST_FILE),
                toml::to_string_pretty(&bad).unwrap(),
            )
            .unwrap();
            assert!(verify(&path, &source).is_err());
        }
        fs::write(
            path.join(MANIFEST_FILE),
            toml::to_string_pretty(&source.manifest).unwrap(),
        )
        .unwrap();
        fs::remove_file(path.join(BG_DESCRIPTORS_FILE)).unwrap();
        assert!(verify(&path, &source).is_err());
    }

    #[test]
    fn palette_json_preserves_words_and_rejects_malformed_or_forged_data() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("data");
        let mut source = fixture();
        extract(&path, &mut source).unwrap();
        let bytes = fs::read(path.join(BG_BASE_PALETTES_FILE)).unwrap();
        let palettes = parse_palettes(&bytes).unwrap();
        assert_eq!(encode_palettes(&palettes).unwrap(), bytes);
        assert_eq!(palettes.entries[0].banks[0][0], 0x8123);
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        for invalid in 0..6 {
            let mut changed = value.clone();
            match invalid {
                0 => changed["entries"][0]["unknown"] = serde_json::json!(0),
                1 => changed["entries"][0]["banks"][0][0] = serde_json::json!(65536),
                2 => {
                    changed["entries"][0]["banks"][0]
                        .as_array_mut()
                        .unwrap()
                        .pop();
                }
                3 => changed["entries"][0]["banks"] = serde_json::json!([]),
                4 => changed["entries"] = serde_json::json!([]),
                5 => changed["entries"][0]["banks"][0][0] = serde_json::json!(1),
                _ => unreachable!(),
            }
            assert!(parse_palettes(&serde_json::to_vec(&changed).unwrap()).is_err());
        }
        assert!(parse_palettes(&bytes[..bytes.len() - 1]).is_err());
        // Equal decoded words with different source bytes would change guest
        // branches/reads, so even a coherent re-encoding is not valid base data.
        let mut recoded = palettes.clone();
        let words = recoded.entries[0].source_bytes().unwrap();
        let mut encoded = (((words.len() as u32) << 8) | 0x10).to_le_bytes().to_vec();
        for group in words.chunks(8) {
            encoded.push(0);
            encoded.extend_from_slice(group);
        }
        recoded.entries[0].encoded_source = encoded;
        let recoded = encode_palettes(&recoded).unwrap();
        let mut manifest = source.manifest.clone();
        manifest.resources[2].sha256 = gba_core::ffta_rom::digest(&recoded);
        fs::write(path.join(BG_BASE_PALETTES_FILE), recoded).unwrap();
        fs::write(
            path.join(MANIFEST_FILE),
            toml::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
        assert!(verify(&path, &source).unwrap_err().contains("reproduce"));
        let mut edited = palettes;
        edited.entries[0].banks[0][0] ^= 0x8000;
        let word = edited.entries[0].banks[0][0].to_le_bytes();
        edited.entries[0].encoded_source[4..6].copy_from_slice(&word);
        let edited = encode_palettes(&edited).unwrap();
        let mut manifest = source.manifest.clone();
        manifest.resources[2].sha256 = gba_core::ffta_rom::digest(&edited);
        fs::write(path.join(BG_BASE_PALETTES_FILE), edited).unwrap();
        fs::write(
            path.join(MANIFEST_FILE),
            toml::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
        assert!(verify(&path, &source).unwrap_err().contains("reproduce"));
        fs::write(path.join(BG_BASE_PALETTES_FILE), bytes).unwrap();
        for invalid in 0..4 {
            let mut manifest = source.manifest.clone();
            match invalid {
                0 => {
                    manifest.resources.remove(2);
                }
                1 => manifest.resources[2].path = "../outside".into(),
                2 => manifest.resources[2].dependencies.clear(),
                3 => manifest.resources[2].sources[0].offset += 4,
                _ => unreachable!(),
            }
            fs::write(
                path.join(MANIFEST_FILE),
                toml::to_string_pretty(&manifest).unwrap(),
            )
            .unwrap();
            assert!(verify(&path, &source).is_err());
        }
        fs::remove_file(path.join(BG_BASE_PALETTES_FILE)).unwrap();
        assert!(verify(&path, &source).is_err());
    }

    #[test]
    fn per_family_input_bounds_and_complete_set_limit() {
        fn padded(mut bytes: Vec<u8>, length: usize) -> Vec<u8> {
            bytes.resize(length, b' ');
            bytes
        }
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("data");
        let mut source = fixture();
        extract(&path, &mut source).unwrap();
        let names = [
            MANIFEST_FILE,
            OBJ_DIMENSIONS_FILE,
            BG_DESCRIPTORS_FILE,
            BG_BASE_PALETTES_FILE,
            BG_ALTERNATE_PALETTES_FILE,
            BG_PACKET_PROGRAMS_FILE,
        ];
        let original: Vec<_> = names
            .iter()
            .map(|name| fs::read(path.join(name)).unwrap())
            .collect();
        let bg = padded(original[2].clone(), MAX_BG_DESCRIPTORS_FILE_BYTES as usize);
        let palettes = padded(
            original[3].clone(),
            MAX_BG_BASE_PALETTES_FILE_BYTES as usize,
        );
        let mut manifest = source.manifest.clone();
        manifest.resources[1].sha256 = gba_core::ffta_rom::digest(&bg);
        fs::write(path.join(BG_DESCRIPTORS_FILE), &bg).unwrap();
        fs::write(
            path.join(MANIFEST_FILE),
            toml::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
        verify(&path, &source).unwrap();
        manifest.resources[2].sha256 = gba_core::ffta_rom::digest(&palettes);
        fs::write(path.join(BG_BASE_PALETTES_FILE), &palettes).unwrap();
        fs::write(
            path.join(MANIFEST_FILE),
            toml::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
        verify(&path, &source).unwrap();
        let alternates = padded(original[4].clone(), MAX_BG_ALTERNATE_PALETTES_FILE_BYTES as usize);
        manifest.resources[3].sha256 = gba_core::ffta_rom::digest(&alternates);
        fs::write(path.join(BG_ALTERNATE_PALETTES_FILE), &alternates).unwrap();
        fs::write(path.join(MANIFEST_FILE), toml::to_string_pretty(&manifest).unwrap()).unwrap();
        let packets = padded(original[5].clone(), MAX_BG_PACKET_PROGRAMS_FILE_BYTES as usize);
        manifest.resources[4].sha256 = gba_core::ffta_rom::digest(&packets);
        fs::write(path.join(BG_PACKET_PROGRAMS_FILE), &packets).unwrap();
        fs::write(path.join(MANIFEST_FILE), toml::to_string_pretty(&manifest).unwrap()).unwrap();
        // The older six-file fixture now fits inside the complete-set limit.
        verify(&path, &source).unwrap();
        // Exercise the actual aggregate boundary without allocating that much
        // JSON. Each individual file still fits its own input guard.
        check_dataset_budget(0, 0, 0, 0, 0, 0, 0, 0, 0, MAX_DATASET_BYTES, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).unwrap();
        assert!(check_dataset_budget(0, 1, 0, 0, 0, 0, 0, 0, 0, MAX_DATASET_BYTES, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            .unwrap_err()
            .contains("total size"));
        for (name, limit) in [
            (MANIFEST_FILE, MAX_MANIFEST_FILE_BYTES),
            (OBJ_DIMENSIONS_FILE, MAX_FILE_BYTES),
            (BG_DESCRIPTORS_FILE, MAX_BG_DESCRIPTORS_FILE_BYTES),
            (BG_BASE_PALETTES_FILE, MAX_BG_BASE_PALETTES_FILE_BYTES),
            (BG_ALTERNATE_PALETTES_FILE, MAX_BG_ALTERNATE_PALETTES_FILE_BYTES),
            (BG_PACKET_PROGRAMS_FILE, MAX_BG_PACKET_PROGRAMS_FILE_BYTES),
        ] {
            for (name, bytes) in names.iter().zip(&original) {
                fs::write(path.join(name), bytes).unwrap();
            }
            fs::File::options()
                .write(true)
                .open(path.join(name))
                .unwrap()
                .set_len(limit + 1)
                .unwrap();
            assert!(verify(&path, &source)
                .unwrap_err()
                .contains(&format!("at most {limit}")));
        }
    }

    #[test]
    fn alternate_json_is_strict_and_rehashed_edits_are_not_base_data() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("data");
        let mut source = fixture();
        extract(&path, &mut source).unwrap();
        let original = fs::read(path.join(BG_ALTERNATE_PALETTES_FILE)).unwrap();
        let parsed = parse_alternate_palettes(&original).unwrap();
        assert_eq!(encode_alternate_palettes(&parsed).unwrap(), original);
        let value: serde_json::Value = serde_json::from_slice(&original).unwrap();
        for invalid in 0..8 {
            let mut changed = value.clone();
            match invalid {
                0 => changed["unknown"] = serde_json::json!(0),
                1 => changed["catalogs"][0][0]["banks"][0][0] = serde_json::json!(65536),
                2 => { changed["catalogs"][0][0]["banks"][0].as_array_mut().unwrap().pop(); },
                3 => changed["catalogs"][0][0]["encoded_source"] = serde_json::json!([0,0,1,0,0x80]),
                4 => changed["descriptor_keys"][0] = serde_json::json!(157),
                5 => changed["relative_sources"][0][0] = serde_json::json!(0),
                6 => changed["catalogs"] = serde_json::json!([]),
                _ => changed["descriptor_keys"][0] = serde_json::json!(255),
            }
            assert!(parse_alternate_palettes(&serde_json::to_vec(&changed).unwrap()).is_err());
        }
        assert!(parse_alternate_palettes(&original[..original.len()-1]).is_err());
        // A new canonical source representation with equal banks is still an
        // edit to original guest reads and cannot enter as authenticated base.
        let mut recoded = parsed;
        recoded.catalogs[0][0].encoded_source.insert(4, 3);
        let recoded = encode_alternate_palettes(&recoded).unwrap();
        let mut manifest = source.manifest.clone();
        manifest.resources[3].sha256 = gba_core::ffta_rom::digest(&recoded);
        fs::write(path.join(BG_ALTERNATE_PALETTES_FILE), recoded).unwrap();
        fs::write(path.join(MANIFEST_FILE), toml::to_string_pretty(&manifest).unwrap()).unwrap();
        assert!(verify(&path, &source).unwrap_err().contains("reproduce"));
        fs::write(path.join(BG_ALTERNATE_PALETTES_FILE), original).unwrap();
        for invalid in 0..4 {
            let mut manifest = source.manifest.clone();
            match invalid {
                0 => manifest.resources[3].path = "../outside".into(),
                1 => manifest.resources[3].dependencies.clear(),
                2 => manifest.resources[3].sources[0].offset += 2,
                _ => { manifest.resources.pop(); },
            }
            fs::write(path.join(MANIFEST_FILE), toml::to_string_pretty(&manifest).unwrap()).unwrap();
            assert!(verify(&path, &source).is_err());
        }
    }

    #[test]
    fn competing_publishers_leave_one_verified_dataset() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("data");
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(4));
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let path = path.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    extract(&path, &mut fixture()).unwrap()
                })
            })
            .collect();
        let outcomes: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(outcomes.iter().filter(|&&v| v == "created").count(), 1);
        verify(&path, &fixture()).unwrap();
        assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 1);
    }

    #[test]
    #[ignore = "requires the user's exact USA and EUR ROMs; no game data is bundled"]
    fn regional_cli_extraction_verification_and_identity() {
        let temp = tempfile::tempdir().unwrap();
        let mut decoded = Vec::new();
        let mut manifests = Vec::new();
        for (env, profile) in [
            ("FFTA_TEST_ROM_USA", "usa"),
            ("FFTA_TEST_ROM_EUR", "europe"),
        ] {
            let rom_path = std::env::var(env).unwrap();
            let output = temp.path().join(profile);
            let output_arg = output.to_str().unwrap().to_owned();
            cmd(&["extract".into(), rom_path.clone(), output_arg.clone()]).unwrap();
            let manifest = fs::read(output.join(MANIFEST_FILE)).unwrap();
            cmd(&["extract".into(), rom_path.clone(), output_arg.clone()]).unwrap();
            assert_eq!(manifest, fs::read(output.join(MANIFEST_FILE)).unwrap());
            cmd(&["verify".into(), rom_path.clone(), output_arg]).unwrap();
            let source = Extraction::from_rom(&fs::read(&rom_path).unwrap()).unwrap();
            assert_eq!(verify(&output, &source).unwrap(), source.dimensions);
            decoded.push(fs::read(output.join(OBJ_DIMENSIONS_FILE)).unwrap());
            manifests.push(manifest);
        }
        assert_eq!(decoded[0], decoded[1]);
        assert_ne!(manifests[0], manifests[1]);
        let usa = fs::read(std::env::var("FFTA_TEST_ROM_USA").unwrap()).unwrap();
        assert!(verify(
            &temp.path().join("europe"),
            &Extraction::from_rom(&usa).unwrap()
        )
        .is_err());
    }

    #[test]
    #[ignore = "requires the user's exact USA and EUR ROMs; no game data is bundled"]
    fn runtime_files_bind_transactionally_and_survive_slots_undo_and_backup_reboot() {
        use gba_core::Bus;
        use std::sync::Arc;
        for env in ["FFTA_TEST_ROM_USA", "FFTA_TEST_ROM_EUR"] {
            let temp = tempfile::tempdir().unwrap();
            let rom = fs::read(std::env::var(env).unwrap()).unwrap();
            let mut source = Extraction::from_rom(&rom).unwrap();
            let path = temp.path().join("decoded");
            extract(&path, &mut source).unwrap();
            let mut machine = super::super::make_machine(rom, None);
            let boot = machine.save_state().unwrap();
            attach_from_directory(&mut machine, &path).unwrap();
            let resources = machine.bus.rom_resources().unwrap().clone();
            assert_eq!(
                resources.file_sha256(),
                gba_core::ffta_rom::digest(&fs::read(path.join(OBJ_DIMENSIONS_FILE)).unwrap())
            );
            assert_eq!(boot, machine.save_state().unwrap());
            let offset = source.manifest.resources[0].sources[0].offset;
            let expected = machine.bus.read32(0x08000000 + offset);
            let save = temp.path().join("saves/test.sav");
            fs::create_dir_all(save.parent().unwrap()).unwrap();
            let mut slots = super::super::save_state::Slots::new(&save, &machine.bus.rom);
            let preview = vec![[0; 4]; 240 * 160];
            slots.save(&machine, 1, &preview, false).unwrap();
            machine.cpu.regs[4] = 37;
            let before = machine.save_state().unwrap();
            slots.load(&mut machine, 1).unwrap();
            assert_eq!(machine.cpu.regs[4], 0);
            assert!(Arc::ptr_eq(
                machine.bus.rom_resources().unwrap(),
                &resources
            ));
            slots.undo(&mut machine).unwrap();
            assert_eq!(machine.save_state().unwrap(), before);
            assert!(Arc::ptr_eq(
                machine.bus.rom_resources().unwrap(),
                &resources
            ));
            // A new activation must reject absent files; a running session
            // retains its already verified immutable in-memory resources.
            fs::remove_file(path.join(OBJ_DIMENSIONS_FILE)).unwrap();
            assert!(attach_from_directory(&mut machine, &path).is_err());
            assert!(Arc::ptr_eq(
                machine.bus.rom_resources().unwrap(),
                &resources
            ));
            let mut fresh = super::super::make_machine(machine.bus.rom.clone(), None);
            assert!(attach_from_directory(&mut fresh, &path).is_err());
            assert!(fresh.bus.rom_resources().is_none());
            assert_eq!(machine.bus.read32(0x08000000 + offset), expected);
            let count = resources.stats();
            machine.load_state(&boot).unwrap();
            assert!(Arc::ptr_eq(
                machine.bus.rom_resources().unwrap(),
                &resources
            ));
            assert_eq!(resources.stats(), count);
            let length = machine.bus.save_data().unwrap().len();
            let backups = super::super::save_backups::Backups::new(&save);
            fs::write(&save, vec![7; length]).unwrap();
            backups.capture_startup().unwrap();
            let generation = backups.choices(length)[0].generation.unwrap();
            machine.cpu.regs[4] = 99;
            backups.restore(generation, &mut machine, &boot).unwrap();
            assert_eq!(machine.cpu.regs[4], 0);
            assert!(Arc::ptr_eq(
                machine.bus.rom_resources().unwrap(),
                &resources
            ));
            assert_eq!(machine.bus.save_data().unwrap(), vec![7; length]);
            assert_eq!(machine.bus.read32(0x08000000 + offset), expected);
            assert!(resources.stats().bus_reads > 0);
        }
    }

    #[cfg(unix)]
    #[test]
    fn symbolic_resources_are_rejected() {
        let temp = tempfile::tempdir().unwrap();
        let real = temp.path().join("real");
        let alias = temp.path().join("alias");
        let mut source = fixture();
        extract(&real, &mut source).unwrap();
        std::os::unix::fs::symlink(&real, &alias).unwrap();
        assert!(verify(&alias, &source).is_err());
        fs::rename(real.join(OBJ_DIMENSIONS_FILE), temp.path().join("outside")).unwrap();
        std::os::unix::fs::symlink(temp.path().join("outside"), real.join(OBJ_DIMENSIONS_FILE))
            .unwrap();
        assert!(verify(&real, &source).is_err());
        fs::remove_file(real.join(OBJ_DIMENSIONS_FILE)).unwrap();
        fs::rename(temp.path().join("outside"), real.join(OBJ_DIMENSIONS_FILE)).unwrap();
        fs::rename(
            real.join(BG_DESCRIPTORS_FILE),
            temp.path().join("outside-bg"),
        )
        .unwrap();
        std::os::unix::fs::symlink(
            temp.path().join("outside-bg"),
            real.join(BG_DESCRIPTORS_FILE),
        )
        .unwrap();
        assert!(verify(&real, &source).is_err());
    }
}

//! Host-only bridge for the reusable picker; all IO runs after explicit actions.
use crate::debug_content::{Index, Kind, Root, MAX_ROOTS};
use crate::debug_inputs::{self, Reference};
use menu::ContentSlot;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
    thread::JoinHandle,
};

enum ResultData {
    Scan(Index),
    Select(Reference),
    Preview(String, Vec<[u8; 4]>, String),
}
struct Job {
    view: u64,
    handle: JoinHandle<Result<ResultData, String>>,
}
#[derive(Default)]
pub(crate) struct Picker {
    roots: Vec<Root>,
    index: Option<Arc<Index>>,
    selected: Option<Reference>,
    slot: Option<ContentSlot>,
    slots: BTreeMap<ContentSlot, Reference>,
    job: Option<Job>,
    generation: u64,
    view: u64,
    rom: Option<String>,
}
fn rows(index: &Index) -> Vec<menu::ContentEntry> {
    index
        .entries
        .iter()
        .map(|e| menu::ContentEntry {
            id: e.id.clone(),
            name: e.name.clone(),
            kind: e.kind.name(),
            description: e.description.clone(),
            location: format!("{} / {}", e.root_id, e.relative_path),
            identity: format!(
                "SHA256 {} | ROM {} | {}",
                e.sha256.as_deref().unwrap_or("not hashed"),
                e.rom_sha256.as_deref().unwrap_or("unknown"),
                e.region.as_deref().unwrap_or("region unknown")
            ),
            status: e.availability.clone(),
            compatibility: e.compatibility.clone(),
            reason: format!("{} | {}", e.verification, e.reason),
            available: e.availability == "available",
        })
        .collect()
}
impl Picker {
    pub fn open(&mut self, menu: &mut menu::Menu, rom: String) -> Result<(), String> {
        self.view = self.view.wrapping_add(1);
        self.slot = None;
        self.rom = Some(rom);
        if self.roots.is_empty() {
            self.roots = crate::debug_content::defaults()?;
        }
        let entries = self.index.as_ref().map_or_else(Vec::new, |i| rows(i));
        menu.open_content(
            "Debug inputs".into(),
            entries,
            "Refresh scans declared roots. Highlighting never runs a tool.".into(),
            self.generation,
        );
        self.scope(menu);
        menu.content_busy(self.job.is_some());
        Ok(())
    }
    pub fn open_for(
        &mut self,
        menu: &mut menu::Menu,
        rom: String,
        slot: ContentSlot,
        message: String,
    ) -> Result<(), String> {
        self.open(menu, rom)?;
        self.slot = Some(slot);
        menu.open_content(
            slot.label().into(),
            self.index.as_ref().map_or_else(Vec::new, |i| rows(i)),
            String::new(),
            self.generation,
        );
        self.scope(menu);
        let current = self
            .slots
            .get(&slot)
            .map(|s| {
                format!(
                    "Current: {} | {} | {}",
                    s.entry.name, s.entry.id, s.entry.relative_path
                )
            })
            .unwrap_or_else(|| "No current selection".into());
        menu.content_message(format!(
            "{} | {} | Select only; return and invoke the tool separately.",
            message, current
        ));
        Ok(())
    }
    fn scope(&self, menu: &mut menu::Menu) {
        menu.content_scope(
            self.slot.map_or_else(Vec::new, |s| {
                debug_inputs::kinds(s)
                    .into_iter()
                    .map(|k| k.name())
                    .collect()
            }),
            self.slot
                .and_then(|s| self.slots.get(&s))
                .or(self.selected.as_ref().filter(|_| self.slot.is_none()))
                .map(|s| s.entry.id.clone()),
        );
        menu.set_tool_inputs(
            ContentSlot::ALL
                .into_iter()
                .filter_map(|slot| {
                    self.slots.get(&slot).map(|s| {
                        (
                            slot,
                            format!(
                                "{} | {} | needs action preflight",
                                s.entry.name,
                                &s.entry.id[8..16]
                            ),
                        )
                    })
                })
                .collect(),
        );
    }
    pub fn references(&self)->BTreeMap<ContentSlot,Reference>{self.slots.clone()}
    pub fn has_slot(&self, slot: ContentSlot) -> bool {
        self.slots.contains_key(&slot)
    }
    pub fn slot_path(&self, slot: ContentSlot, rom: &str) -> Result<PathBuf, String> {
        self.slots
            .get(&slot)
            .ok_or_else(|| format!("Select {} in Tool inputs first", slot.label()))?
            .recheck(slot, rom)
    }
    pub fn adopt(
        &mut self,
        slot: ContentSlot,
        path: &Path,
        rom: &str,
        menu: &mut menu::Menu,
    ) -> Result<(), String> {
        let reference = Reference::explicit(path, slot, rom)?;
        self.slots.insert(slot, reference);
        self.scope(menu);
        Ok(())
    }
    pub fn cancel(&mut self) {
        self.view = self.view.wrapping_add(1);
    }
    fn start(
        &mut self,
        menu: &mut menu::Menu,
        work: impl FnOnce() -> Result<ResultData, String> + Send + 'static,
    ) -> Result<(), String> {
        if self.job.is_some() {
            return Err("Catalog operation is finishing; wait or cancel the selection.".into());
        }
        self.job = Some(Job {
            view: self.view,
            handle: std::thread::spawn(work),
        });
        menu.content_busy(true);
        menu.content_message("Working on selected catalog inputs; guest remains paused.".into());
        Ok(())
    }
    pub fn refresh(&mut self, menu: &mut menu::Menu) -> Result<(), String> {
        let roots = self.roots.clone();
        let rom = self.rom.clone();
        self.start(menu, move || Index::scan(roots, rom).map(ResultData::Scan))
    }
    pub fn external_result(
        &mut self,
        path: Option<PathBuf>,
        file: bool,
        menu: &mut menu::Menu,
    ) -> Result<(), String> {
        let Some(path) = path else {
            return Ok(());
        };
        if self.job.is_some() {
            return Err("Wait for the current catalog operation before adding a root".into());
        }
        let root = if file {
            Root::single_file(&path)?
        } else {
            Root::new(&path, false)?
        };
        if self.roots.iter().any(|r| r.id == root.id) {
            menu.content_message(
                "This content root is already listed. Refresh explicitly if needed.".into(),
            );
            return Ok(());
        }
        if self.roots.len() >= MAX_ROOTS {
            return Err(
                "Eight content roots are already retained; the new root was not added.".into(),
            );
        }
        self.roots.push(root);
        self.refresh(menu)
    }
    pub fn browse_file(&mut self, menu: &mut menu::Menu) -> Result<(), String> {
        if self.job.is_some() {
            return Err("Wait for the current catalog operation".into());
        }
        let path = rfd::FileDialog::new()
            .set_title("Browse external file - add only, never run")
            .pick_file();
        self.external_result(path, true, menu)
    }
    pub fn browse_folder(&mut self, menu: &mut menu::Menu) -> Result<(), String> {
        if self.job.is_some() {
            return Err("Wait for the current catalog operation".into());
        }
        let path = rfd::FileDialog::new()
            .set_title("Add explicit content folder - never run")
            .pick_folder();
        self.external_result(path, false, menu)
    }
    pub fn choose(
        &mut self,
        menu: &mut menu::Menu,
        index: usize,
        generation: u64,
        preview: bool,
    ) -> Result<(), String> {
        if generation != self.generation {
            return Err("The catalog changed; select the item again".into());
        }
        let slot = self.slot;
        let catalog = self
            .index
            .clone()
            .ok_or("Refresh the content catalog first")?;
        let entry = catalog
            .entries
            .get(index)
            .ok_or("Unknown selected content row")?
            .clone();
        self.start(menu, move || {
            let reference =
                Reference::resolve(catalog, &entry.id, if preview { None } else { slot })?;
            let checked = &reference.entry;
            let path = &reference.path;
            if !preview {
                return Ok(ResultData::Select(reference));
            }
            if checked.kind != Kind::Snapshot {
                return Ok(ResultData::Preview(
                    checked.id.clone(),
                    Vec::new(),
                    "No embedded snapshot preview for this type. Full details remain scrollable."
                        .into(),
                ));
            }
            let bytes = crate::diagnostics::read(
                &path,
                gba_core::save_state::MAX_STATE_BYTES as u64 + 240 * 160 * 4 + 48,
            )?;
            if checked
                .sha256
                .as_ref()
                .is_none_or(|sha| *sha != gba_core::ffta_rom::digest(&bytes))
            {
                return Err("Snapshot changed or was not hashed; refresh before preview".into());
            }
            let preview = crate::save_state::preview(&bytes)?;
            let message = if preview.is_empty() {
                "Raw snapshot has no embedded image; no machine was loaded."
            } else {
                "Authenticated embedded preview; no machine was loaded."
            };
            Ok(ResultData::Preview(
                checked.id.clone(),
                preview,
                message.into(),
            ))
        })
    }
    pub fn poll(&mut self, menu: &mut menu::Menu) -> bool {
        if !self.job.as_ref().is_some_and(|j| j.handle.is_finished()) {
            return false;
        }
        let job = self.job.take().unwrap();
        let result = job
            .handle
            .join()
            .unwrap_or_else(|_| Err("Content worker failed".into()));
        menu.content_busy(false);
        if job.view != self.view || !menu.content_active() {
            return true;
        }
        match result {
            Ok(ResultData::Scan(index)) => {
                self.generation = self.generation.wrapping_add(1);
                let message = format!(
                    "{} entries | {} issues | {} | selection needs action preflight",
                    index.entries.len(),
                    index.issues.len(),
                    if index.stats.truncated {
                        "PARTIAL: scan cap"
                    } else {
                        "listing complete"
                    }
                );
                let message = if index.issues.is_empty() {
                    message
                } else {
                    format!(
                        "{} | First {} issues: {}",
                        message,
                        index.issues.len().min(8),
                        index
                            .issues
                            .iter()
                            .take(8)
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(" | ")
                    )
                };
                menu.open_content(
                    self.slot.map_or("Debug inputs", |s| s.label()).into(),
                    rows(&index),
                    message,
                    self.generation,
                );
                self.index = Some(Arc::new(index));
                self.scope(menu);
            }
            Ok(ResultData::Select(selection)) => {
                menu.content_selected(&selection.entry.id);
                if let Some(slot) = self.slot {
                    self.slots.insert(slot, selection.clone());
                }
                self.selected = Some(selection);
                self.scope(menu);
            }
            Ok(ResultData::Preview(id, pixels, message)) => {
                if menu.content_highlight() == Some(id.as_str()) {
                    menu.content_message(message.clone());
                    menu.content_preview(pixels, message);
                } else {
                    menu.content_message("Preview completed for a different row. Open Details for the current selection.".into());
                }
            }
            Err(error) => menu.content_message(error),
        }
        true
    }
    pub fn status(&self) -> Value {
        json!({"slot":self.slot.map(|s|s.label()),"inputs":ContentSlot::ALL.into_iter().filter_map(|slot|self.slots.get(&slot).map(|s|json!({"slot":slot.label(),"id":s.entry.id,"kind":s.entry.kind,"relative_path":s.entry.relative_path,"sha256":s.entry.sha256,"scope":"Pinned host reference; rechecked before use; runtime ABI and decoded guest fields still need action preflight"}))).collect::<Vec<_>>(),"generation":self.generation,"busy":self.job.is_some(),"roots":self.roots.len(),"entries":self.index.as_ref().map_or(0,|i|i.entries.len()),"selected":self.selected.as_ref().map(|s|json!({"id":s.entry.id,"kind":s.entry.kind,"root_id":s.entry.root_id,"relative_path":s.entry.relative_path,"sha256":s.entry.sha256,"path":s.path})),"scope":"Host content selection only; no execution or full action verification"})
    }
}
#[cfg(test)]
pub(crate) mod tests;

//! Shared catalog metadata, filtered by the controller without filesystem access.
use super::{canvas::Canvas,Layout,Menu,MenuAction,MenuInput,Row,rows_of};
#[derive(Clone,Default)]
pub struct CatalogEntry {
    pub key:String,pub name:String,pub kind:String,pub case:String,pub region:String,
    pub status:String,pub slot:Option<u8>,pub source:String,pub evidence:String,
    pub available:bool,pub error:String,
}
#[derive(Default)]
pub(super) struct CatalogState {
    entries:Vec<CatalogEntry>,region:usize,status:usize,kind:usize,case:usize,index:usize,
    pub preview:Vec<[u8;4]>,pub message:String,
}
const REGIONS:[&str;3]=["All","EUR","USA"];
const STATES:[&str;6]=["All","validata","parziale","non_coperta","variante","origine"];
const KINDS:[&str;6]=["All","case","variant","IN","GEN","NEW"];
impl CatalogState {
    pub fn new(entries:Vec<CatalogEntry>,region:&str)->Self {Self {entries,region:REGIONS.iter().position(|r|*r==region).unwrap_or(0),..Default::default()}}
    fn cases(&self)->Vec<String> {let mut v:Vec<_>=self.entries.iter().filter(|e|!e.case.is_empty()).map(|e|e.case.clone()).collect();v.sort();v.dedup();v.insert(0,"All".into());v}
    fn matches(&self)->Vec<usize> {
        let cases=self.cases();
        self.entries.iter().enumerate().filter(|(_,e)|
            (self.region==0||e.region.is_empty()||e.region==REGIONS[self.region])
            &&(self.status==0||e.status==STATES[self.status])
            &&(self.kind==0||e.kind==KINDS[self.kind])
            &&(self.case==0||e.case==cases[self.case])).map(|(i,_)|i).collect()
    }
    pub fn selected(&self)->Option<usize> {self.matches().get(self.index).copied()}
    pub fn handle(&mut self,row:Row,input:MenuInput,isolated:bool)->Option<Option<MenuAction>> {
        let advance=|value:&mut usize,count:usize|*value=(*value+if input==MenuInput::Left{count-1}else{1})%count;
        match row {
            Row::CatalogRegion=>advance(&mut self.region,REGIONS.len()),
            Row::CatalogStatus=>advance(&mut self.status,STATES.len()),
            Row::CatalogKind=>advance(&mut self.kind,KINDS.len()),
            Row::CatalogCase=>{let n=self.cases().len();advance(&mut self.case,n);},
            Row::CatalogEntry=>{let n=self.matches().len();if n>0 {advance(&mut self.index,n);}},
            Row::CatalogImport|Row::CatalogOpen if input==MenuInput::Confirm=>{
                if row==Row::CatalogImport&&!isolated {self.message="Import requires a debug copy. Use Open isolated case.".into();return Some(None);}
                let Some(index)=self.selected() else{return Some(None);};
                if !self.entries[index].available {self.message="No verified file for this entry; its recorded status is unchanged.".into();return Some(None);}
                return Some(Some(if row==Row::CatalogImport{MenuAction::CatalogImport(index)}else{MenuAction::CatalogOpen(index)}));
            },
            Row::CatalogChoose if input==MenuInput::Confirm=>return Some(Some(MenuAction::ChooseDebugCatalog)),
            _=>return None,
        }
        if row!=Row::CatalogEntry {self.index=0;}
        self.preview.clear();self.message.clear();Some(self.selected().map(MenuAction::CatalogSelection))
    }
    fn value(&self,row:Row)->String {
        match row {Row::CatalogRegion=>REGIONS[self.region].into(),Row::CatalogStatus=>STATES[self.status].into(),
            Row::CatalogKind=>KINDS[self.kind].into(),Row::CatalogCase=>self.cases()[self.case].clone(),
            Row::CatalogEntry=>self.selected().map_or_else(||"No matches".into(),|i|format!("{} / {}  {}",self.index+1,self.matches().len(),self.entries[i].key)),_=>String::new()}
    }
}
pub(super) fn layout(w:usize,h:usize)->Layout {
    let scale=(w as f32/800.).min(h as f32/480.).max(0.1);
    let pad=(12.*scale)as usize;let line_h=(28.*scale).max(3.)as usize;
    let panel_w=(720.*scale)as usize;let panel_w=panel_w.min(w.saturating_sub(pad*2));
    let header=(171.*scale)as usize;let panel_h=(pad*2+header+line_h*9+20).min(h);
    let panel_x=w.saturating_sub(panel_w)/2;let panel_y=h.saturating_sub(panel_h)/2;
    Layout {panel_x,panel_y,panel_w,panel_h,pad,line_h,rows_y:panel_y+pad+header,visible_rows:9}
}
pub(super) fn draw(menu:&Menu,px:&mut[[u8;4]],w:usize,h:usize) {
    for p in px.iter_mut().take(w*h) {for c in &mut p[..3]{*c=(*c as u16/4)as u8;}}
    let l=layout(w,h);let scale=l.line_h as f32/28.;let x=l.panel_x as f32;let y=l.panel_y as f32;let pad=l.pad as f32;let width=l.panel_w as f32;
    let mut c=Canvas::new(px,w,h);let ink=[35,24,16];
    c.rect(x,y,width,l.panel_h as f32,7.,[251,244,226],[235,220,185],[184,146,65]);
    c.text(x+pad,y+pad,"DEBUG SAVE CATALOG",ink,17.*scale,true,width-2.*pad);
    if let Some(i)=menu.catalog.selected(){let e=&menu.catalog.entries[i];
        let text_width=(width-180.*scale-2.*pad).max(40.);
        let lines=[format!("{} - {}",e.kind,e.name),format!("{} | {} | source slot {}",e.region,e.status,e.slot.map_or_else(||"-".into(),|n|n.to_string())),
            format!("Source: {}",e.source),format!("Evidence: {}",e.evidence),e.error.clone()];
        for(n,line)in lines.iter().enumerate(){c.text(x+pad,y+pad+(30.+n as f32*22.)*scale,line,ink,12.*scale,n==0,text_width);}
        let iw=(144.*scale).max(1.)as usize;let ih=iw*2/3;let ix=(x+width-pad-iw as f32)as usize;let iy=(y+pad+28.*scale)as usize;
        c.rect(ix as f32,iy as f32,iw as f32,ih as f32,0.,[40,32,22],[40,32,22],[80,55,22]);
        if menu.catalog.preview.len()==240*160 {for yy in 0..ih {for xx in 0..iw {let tx=ix+xx;let ty=iy+yy;if tx<w&&ty<h {let at=(ty*w+tx)*4;c.pix.data_mut()[at..at+4].copy_from_slice(&menu.catalog.preview[(yy*160/ih)*240+xx*240/iw]);}}}}
        else{c.text(ix as f32+4.,iy as f32+ih as f32/2.,"NO PREVIEW",[240,220,180],11.*scale,false,iw as f32-8.);}
    }
    let message=if menu.catalog.message.is_empty(){"Left/Right: filter or select. Import preserves the original file."}else{&menu.catalog.message};
    c.text(x+pad,l.rows_y as f32-23.*scale,message,ink,11.*scale,false,width-2.*pad);
    for(n,row)in rows_of(menu.page).iter().enumerate(){let yy=(l.rows_y+n*l.line_h)as f32;let active=menu.selected==n;
        c.rect(x+pad,yy,width-2.*pad,l.line_h as f32-2.,3.,if active{[115,75,12]}else{[254,252,245]},if active{[55,32,4]}else{[235,220,185]},[184,146,65]);
        c.text(x+pad+7.,yy+5.*scale,row.label(),if active{[255,225,142]}else{ink},12.*scale,true,width*0.42);
        c.text(x+width*0.43,yy+5.*scale,&menu.catalog.value(*row),if active{[255,225,142]}else{ink},11.*scale,false,width*0.55-pad);
    }
    c.finish(px);
}

#[cfg(test)]mod tests {
 use super::*;
 fn entries()->Vec<CatalogEntry>{vec![CatalogEntry{key:"DS-20/EUR/20".into(),name:"Failed dispatch".into(),kind:"case".into(),case:"DS-20".into(),region:"EUR".into(),status:"validata".into(),slot:Some(20),source:"USER-DS-20-EUR".into(),available:true,..Default::default()},
 CatalogEntry{key:"DS-03//none".into(),kind:"case".into(),case:"DS-03".into(),status:"non_coperta".into(),..Default::default()},
 CatalogEntry{key:"IN-20/EUR".into(),kind:"IN".into(),region:"EUR".into(),status:"origine".into(),available:true,..Default::default()}]}
 #[test]fn filters_keep_uncovered_cases_and_in_identifiers_distinct(){
  let mut s=CatalogState::new(entries(),"EUR");assert_eq!(s.matches(),vec![0,1,2]);
  s.handle(Row::CatalogKind,MenuInput::Right,true);assert_eq!(s.matches(),vec![0,1]);
  s.handle(Row::CatalogCase,MenuInput::Left,true);assert_eq!(s.matches(),vec![0]);
  assert_eq!(s.handle(Row::CatalogImport,MenuInput::Confirm,false),Some(None));assert!(s.message.contains("debug copy"));
  assert_eq!(s.handle(Row::CatalogImport,MenuInput::Confirm,true),Some(Some(MenuAction::CatalogImport(0))));
  s.handle(Row::CatalogCase,MenuInput::Left,true);assert_eq!(s.matches(),vec![1]);assert_eq!(s.handle(Row::CatalogOpen,MenuInput::Confirm,true),Some(None));
 }
 #[test]fn catalog_details_preview_and_actions_render_with_shared_hit_geometry(){
  let mut menu=Menu::new(true,true);menu.set_catalog(entries(),"EUR");menu.catalog_preview(vec![[70,90,120,255];240*160]);
  let av=input_config::AvConfig::default();
  for(w,h)in[(852,480),(1280,720),(620,480)]{let mut px=vec![[0,0,0,255];w*h];menu.draw(&av,&std::array::from_fn(|_|String::new()),"","","","",&mut px,w,h);let l=layout(w,h);assert!(l.rows_y+9*l.line_h<=h);assert!(px.iter().any(|p|p[0]>30));}
 }
}

//! Offline diagnostic of private RAM/VRAM snapshots; no ROM data is embedded.
use gba_core::{mem::{self,MemMap},ppu};
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let args:Vec<_>=std::env::args().collect();
    let prefix=args.get(1).ok_or("snapshot prefix required")?;
    let wide=args.iter().any(|a|a=="--wide");
    mem::set_extra(if wide {448}else{0});
    mem::set_extra_y(if wide {80}else{0});
    let mut m=MemMap::new(vec![0;192]);
    for (name,dst) in [("ewram",&mut m.ewram),("iwram",&mut m.iwram),("vram",&mut m.vram),("io",&mut m.io),("oam",&mut m.oam),("palette",&mut m.palette)] {
        dst.copy_from_slice(&std::fs::read(format!("{prefix}-{name}.bin"))?);
    }
    // Enable only completion of extra rows; no inferred host planes or sprites.
    m.ffta_movement.enabled=wide;
    if args.iter().any(|a|a=="--stats") {
        // Explicit diagnostic tagging for the measured frame-13000 snapshot.
        // Production tagging is gated by the exact ROM and producer list.
        m.ffta_sprites.active=Some((8..12).map(|slot| {
            let p=&m.oam[slot*8..slot*8+6];
            let attrs=std::array::from_fn(|i|u16::from_le_bytes([p[i*2],p[i*2+1]]));
            gba_core::ffta_sprites::HostSprite {x:(attrs[1]&511) as i32,y:(attrs[0]&255) as i32,
                attrs,slot,replaces:true,stats_background:true,..Default::default()}
        }).collect());
    }
    for line in 0..160 {ppu::render_scanline(&mut m,line);}
    if wide {ppu::render_ffta_zoom_rows(&mut m);}
    let (w,h,pixels)=if wide {(mem::render_width(),mem::render_height(),&m.wide_framebuffer)}else{(240,160,&m.framebuffer)};
    let mut out=format!("P6\n{w} {h}\n255\n").into_bytes();
    for &p in &pixels[..w*h] {for c in [p&31,(p>>5)&31,(p>>10)&31] {out.push((c<<3|c>>2) as u8);}}
    let suffix=if args.iter().any(|a|a=="--stats") {"-stats"}else{""};
    let path=format!("{prefix}-raw-{}{suffix}.ppm",if wide {"wide"}else{"native"});
    std::fs::write(&path,out)?;
    println!("SNAPSHOT_RENDER {path}");
    Ok(())
}

use std::time::Instant;
use std::hint::black_box;
use std::io::Cursor;

// dependencies:
// peppi: 10 direct (104 total) 
// slp_parser: 2 direct (25 total)
// slpz: 1 direct (4 total)

// lines of code:
// peppi: 6800
// slp_parser: 7400
// slpz: 700

// max supported slp version (current 3.18.0):
// peppi: 3.16.0 (3.18.0 implemented but not pushed to crates.io)
// slp_parser: n/a
// slpz: n/a

// min supported slp version:
// peppi: n/a (?) unsure
// slp_parser: 3.0
// slpz: n/a

fn split(task: &str, t: &mut Instant) {
    println!("{}us\t{}", t.elapsed().as_micros(), task);
    *t = std::time::Instant::now();
}
fn split_ms(task: &str, t: &mut Instant) {
    println!("{}ms\t{}", t.elapsed().as_millis(), task);
    *t = std::time::Instant::now();
}
fn ratio(new: usize, old: usize) {
    println!("{}kb\t({}%)", new / 1000, new * 100 / old);
}

fn main() {
    let slp_bytes = std::fs::read("test.slp").unwrap();
    let slpz_bytes = std::fs::read("test.slpz").unwrap();
    let slpp_bytes = std::fs::read("test.slpp").unwrap();
    
    // parse entire game -------
    
    let t = &mut Instant::now();
    
    black_box(peppi::io::slippi::read(Cursor::new(slp_bytes.as_slice()), None).unwrap());
    split("peppi slp parse full", t);
    
    black_box(peppi::io::peppi::read(Cursor::new(slpp_bytes.as_slice()), None).unwrap());
    split("peppi slpp parse full", t);
    
    black_box(slp_parser::parse_file(&slp_bytes).unwrap());
    split("slp_parser slp parse full", t);
    
    black_box(slp_parser::parse_file_slpz(&slpz_bytes).unwrap());
    split("slp_parser slpz parse full", t);
    
    // parse info -------
    
    println!();
    let t = &mut Instant::now();
    
    black_box(peppi::io::slippi::read(
        Cursor::new(slp_bytes.as_slice()),
        Some(&peppi::io::slippi::de::Opts { skip_frames: true, compute_hash: false, debug: None })
    ).unwrap());
    split("peppi slp parse info", t);
    
    black_box(peppi::io::peppi::read(Cursor::new(
        slpp_bytes.as_slice()),
        Some(&peppi::io::peppi::de::Opts { skip_frames: true })
    ).unwrap());
    split("peppi slpp parse info", t);
    
    black_box(slp_parser::parse_file_info(&mut Cursor::new(slp_bytes.as_slice())).unwrap());
    split("slp_parser slp parse info", t);
    
    black_box(slp_parser::parse_file_info_slpz(&mut Cursor::new(slpz_bytes.as_slice())).unwrap());
    split("slp_parser slpz parse info", t);
    
    // compress -------
    
    println!();
    let mut buf = Vec::with_capacity(slp_bytes.len());
    
    let t = &mut Instant::now();
    let game = peppi::io::slippi::read(
        Cursor::new(slp_bytes.as_slice()),
        Some(&peppi::io::slippi::de::Opts { skip_frames: false, compute_hash: false, debug: None })
    ).unwrap();
    black_box(peppi::io::peppi::write(&mut buf, game, Some(
        &peppi::io::peppi::ser::Opts { compression: Some(arrow2::io::ipc::write::Compression::ZSTD) }
    )).unwrap());
    split_ms("peppi compress slp zstd", t);
    ratio(buf.len(), slp_bytes.len());
    
    let t = &mut Instant::now();
    let game = peppi::io::slippi::read(
        Cursor::new(slp_bytes.as_slice()),
        Some(&peppi::io::slippi::de::Opts { skip_frames: false, compute_hash: false, debug: None })
    ).unwrap();
    black_box(peppi::io::peppi::write(&mut buf, game, Some(
        &peppi::io::peppi::ser::Opts { compression: Some(arrow2::io::ipc::write::Compression::LZ4) }
    )).unwrap());
    split_ms("peppi compress slp lzma", t);
    ratio(buf.len(), slp_bytes.len());
    
    let t = &mut Instant::now();
    let mut comp = slpz::Compressor::new(3).unwrap();
    let buf = black_box(slpz::compress(&mut comp, slp_bytes.as_slice()).unwrap());
    split_ms("slpz compress slp fast", t);
    ratio(buf.len(), slp_bytes.len());
    
    let t = &mut Instant::now();
    let mut comp = slpz::Compressor::new(12).unwrap();
    let buf = black_box(slpz::compress(&mut comp, slp_bytes.as_slice()).unwrap());
    split_ms("slpz compress slp slow", t);
    ratio(buf.len(), slp_bytes.len());
}

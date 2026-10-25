use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tiny_skia::{
    Color, FillRule, GradientStop, Paint, PathBuilder, Pixmap, Point, RadialGradient, SpreadMode, Stroke, Transform,
};
use windows_sys::Win32::System::LibraryLoader::{BeginUpdateResourceW, EndUpdateResourceW, UpdateResourceW};

fn render_formation_icon(size: u32) -> Pixmap {
    let mut pixmap = Pixmap::new(size, size).unwrap();
    let cx = size as f32 / 2.0;
    let cy = size as f32 / 2.0;
    let r = size as f32 * 0.46;

    // 1. Dark celestial cosmic base circle
    let mut bg = PathBuilder::new();
    bg.push_circle(cx, cy, r);
    if let Some(p) = bg.finish() {
        let mut paint = Paint::default();
        paint.anti_alias = true;
        paint.set_color_rgba8(10, 14, 24, 240);
        pixmap.fill_path(&p, &paint, FillRule::Winding, Transform::identity(), None);

        // Golden rim
        let mut rim_paint = Paint::default();
        rim_paint.anti_alias = true;
        rim_paint.set_color_rgba8(255, 205, 50, 255);
        let stroke = Stroke { width: (size as f32 * 0.025).max(1.5), ..Default::default() };
        pixmap.stroke_path(&p, &rim_paint, &stroke, Transform::identity(), None);
    }

    // 2. Outer segmented aura rings
    let mut ring = PathBuilder::new();
    ring.push_circle(cx, cy, r * 0.82);
    if let Some(p) = ring.finish() {
        let mut rp = Paint::default();
        rp.anti_alias = true;
        rp.set_color_rgba8(255, 170, 30, 160);
        pixmap.stroke_path(&p, &rp, &Stroke { width: (size as f32 * 0.015).max(1.0), ..Default::default() }, Transform::identity(), None);
    }

    // 3. 8 Trigrams / Nodes
    for i in 0..8 {
        let a = (i as f32) * (2.0 * PI / 8.0);
        let nx = cx + a.cos() * (r * 0.82);
        let ny = cy + a.sin() * (r * 0.82);

        let mut dot = PathBuilder::new();
        dot.push_circle(nx, ny, (size as f32 * 0.035).max(1.5));
        if let Some(dp) = dot.finish() {
            let mut p = Paint::default();
            p.anti_alias = true;
            p.set_color_rgba8(255, 235, 150, 240);
            pixmap.fill_path(&dp, &p, FillRule::Winding, Transform::identity(), None);
        }
    }

    // 4. Interlocking Sacred Geometry (Octagram / 8-pointed star)
    let star_r = r * 0.58;
    for sq in 0..2 {
        let offset = (sq as f32) * (PI * 0.25);
        let mut star_b = PathBuilder::new();
        for i in 0..4 {
            let a = offset + (i as f32) * (PI * 0.5);
            let sx = cx + a.cos() * star_r;
            let sy = cy + a.sin() * star_r;
            if i == 0 { star_b.move_to(sx, sy); } else { star_b.line_to(sx, sy); }
        }
        star_b.close();
        if let Some(sp) = star_b.finish() {
            let mut p = Paint::default();
            p.anti_alias = true;
            p.set_color_rgba8(255, 190, 40, 200);
            pixmap.stroke_path(&sp, &p, &Stroke { width: (size as f32 * 0.018).max(1.0), ..Default::default() }, Transform::identity(), None);
        }
    }

    // 5. Taiji Core (Thái Cực Đồ)
    let taiji_r = r * 0.32;

    // Glowing halo
    let mut halo_b = PathBuilder::new();
    halo_b.push_circle(cx, cy, taiji_r * 1.5);
    if let Some(hp) = halo_b.finish() {
        let mut hpaint = Paint::default();
        hpaint.anti_alias = true;
        if let Some(shader) = RadialGradient::new(
            Point::from_xy(cx, cy),
            0.0,
            Point::from_xy(cx, cy),
            taiji_r * 1.5,
            vec![
                GradientStop::new(0.0, Color::from_rgba8(255, 255, 230, 210)),
                GradientStop::new(0.5, Color::from_rgba8(255, 180, 20, 110)),
                GradientStop::new(1.0, Color::from_rgba8(255, 180, 20, 0)),
            ],
            SpreadMode::Pad,
            Transform::identity(),
        ) {
            hpaint.shader = shader;
            pixmap.fill_path(&hp, &hpaint, FillRule::Winding, Transform::identity(), None);
        }
    }

    // Yang Base (Golden)
    let mut base_b = PathBuilder::new();
    base_b.push_circle(cx, cy, taiji_r);
    let base_path = base_b.finish();
    if let Some(ref bp) = base_path {
        let mut p = Paint::default();
        p.anti_alias = true;
        p.set_color_rgba8(255, 215, 50, 255);
        pixmap.fill_path(bp, &p, FillRule::Winding, Transform::identity(), None);
    }


    // Yin Half S-curve (Deep Midnight Blue)
    let mut yin_b = PathBuilder::new();
    yin_b.move_to(cx, cy - taiji_r);
    let half_r = taiji_r * 0.5;
    let steps = 16;
    for s in 0..=steps {
        let a = -PI * 0.5 + PI * (s as f32 / steps as f32);
        yin_b.line_to(cx + a.cos() * half_r, cy - half_r + a.sin() * half_r);
    }
    for s in 0..=steps {
        let a = -PI * 0.5 - PI * (s as f32 / steps as f32);
        yin_b.line_to(cx + a.cos() * half_r, cy + half_r + a.sin() * half_r);
    }
    for s in 0..=steps {
        let a = PI * 0.5 + PI * (s as f32 / steps as f32);
        yin_b.line_to(cx + a.cos() * taiji_r, cy + a.sin() * taiji_r);
    }
    yin_b.close();

    if let Some(yp) = yin_b.finish() {
        let mut p = Paint::default();
        p.anti_alias = true;
        p.set_color_rgba8(14, 20, 36, 255);
        pixmap.fill_path(&yp, &p, FillRule::Winding, Transform::identity(), None);
    }

    // Fish eyes
    let eye_r = (taiji_r * 0.18).max(1.2);
    // Top eye (Dark)
    let mut eye1 = PathBuilder::new();
    eye1.push_circle(cx, cy - half_r, eye_r);
    if let Some(ep) = eye1.finish() {
        let mut p = Paint::default();
        p.anti_alias = true;
        p.set_color_rgba8(14, 20, 36, 255);
        pixmap.fill_path(&ep, &p, FillRule::Winding, Transform::identity(), None);
    }
    // Bottom eye (Bright Gold)
    let mut eye2 = PathBuilder::new();
    eye2.push_circle(cx, cy + half_r, eye_r);
    if let Some(ep) = eye2.finish() {
        let mut p = Paint::default();
        p.anti_alias = true;
        p.set_color_rgba8(255, 255, 230, 255);
        pixmap.fill_path(&ep, &p, FillRule::Winding, Transform::identity(), None);
    }

    // Outer rim of Taiji
    if let Some(ref bp) = base_path {
        let mut p = Paint::default();
        p.anti_alias = true;
        p.set_color_rgba8(255, 255, 200, 220);
        pixmap.stroke_path(bp, &p, &Stroke { width: (size as f32 * 0.015).max(1.0), ..Default::default() }, Transform::identity(), None);
    }


    pixmap
}

/// Convert a Pixmap to Windows 32-bit DIB bytes for ICO
fn pixmap_to_ico_dib(pixmap: &Pixmap) -> Vec<u8> {
    let width = pixmap.width();
    let height = pixmap.height();

    let mut dib = Vec::new();
    // BITMAPINFOHEADER (40 bytes)
    dib.extend_from_slice(&40u32.to_le_bytes()); // biSize
    dib.extend_from_slice(&(width as i32).to_le_bytes()); // biWidth
    dib.extend_from_slice(&(height as i32 * 2).to_le_bytes()); // biHeight (* 2 for ICO format!)
    dib.extend_from_slice(&1u16.to_le_bytes()); // biPlanes
    dib.extend_from_slice(&32u16.to_le_bytes()); // biBitCount
    dib.extend_from_slice(&0u32.to_le_bytes()); // biCompression (BI_RGB)
    dib.extend_from_slice(&((width * height * 4) as u32).to_le_bytes()); // biSizeImage
    dib.extend_from_slice(&0i32.to_le_bytes()); // biXPelsPerMeter
    dib.extend_from_slice(&0i32.to_le_bytes()); // biYPelsPerMeter
    dib.extend_from_slice(&0u32.to_le_bytes()); // biClrUsed
    dib.extend_from_slice(&0u32.to_le_bytes()); // biClrImportant

    // Pixel data: bottom-up 32-bit BGRA
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = pixmap.pixel(x, y).unwrap();
            dib.push(pixel.blue());
            dib.push(pixel.green());
            dib.push(pixel.red());
            dib.push(pixel.alpha());
        }
    }

    // AND mask: 1 bit per pixel, row padded to 32 bits
    let and_row_bytes = ((width + 31) / 32) * 4;
    let and_mask_size = (and_row_bytes * height) as usize;
    dib.resize(dib.len() + and_mask_size, 0);

    dib
}

fn create_ico_file(sizes: &[u32]) -> (Vec<u8>, Vec<(u32, Vec<u8>)>) {
    let mut dib_images = Vec::new();
    for &s in sizes {
        let pm = render_formation_icon(s);
        let dib = pixmap_to_ico_dib(&pm);
        dib_images.push((s, dib));
    }

    let mut ico_data = Vec::new();
    let num_images = dib_images.len() as u16;

    // 1. ICONDIR (6 bytes)
    ico_data.extend_from_slice(&0u16.to_le_bytes()); // idReserved
    ico_data.extend_from_slice(&1u16.to_le_bytes()); // idType (1 for Icon)
    ico_data.extend_from_slice(&num_images.to_le_bytes()); // idCount

    // 2. ICONDIRENTRY (16 bytes each)
    let header_size = 6 + (num_images as usize) * 16;
    let mut current_offset = header_size as u32;

    for (size, dib) in &dib_images {
        let b_size = if *size >= 256 { 0u8 } else { *size as u8 };
        ico_data.push(b_size); // bWidth
        ico_data.push(b_size); // bHeight
        ico_data.push(0);      // bColorCount
        ico_data.push(0);      // bReserved
        ico_data.extend_from_slice(&1u16.to_le_bytes()); // wPlanes
        ico_data.extend_from_slice(&32u16.to_le_bytes()); // wBitCount
        ico_data.extend_from_slice(&(dib.len() as u32).to_le_bytes()); // dwBytesInRes
        ico_data.extend_from_slice(&current_offset.to_le_bytes()); // dwImageOffset

        current_offset += dib.len() as u32;
    }

    // 3. Image DIB data
    for (_, dib) in &dib_images {
        ico_data.extend_from_slice(dib);
    }

    (ico_data, dib_images)
}

/// Inject the generated icon directly into the Windows executable using Win32 UpdateResource API
fn inject_icon_into_exe(exe_path: &Path, dib_images: &[(u32, Vec<u8>)]) -> Result<(), String> {
    if !exe_path.exists() {
        return Err(format!("File does not exist: {:?}", exe_path));
    }

    unsafe {
        let path_u16: Vec<u16> = exe_path
            .to_string_lossy()
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        // FALSE = do not discard existing resources
        let h_update = BeginUpdateResourceW(path_u16.as_ptr(), 0);
        if h_update == 0 {
            return Err("BeginUpdateResourceW failed".to_string());
        }

        let rt_icon = 3 as usize as *const u16;
        let rt_group_icon = 14 as usize as *const u16;

        // Build GRPICONDIR for the group resource
        let mut grp_data = Vec::new();
        let num_images = dib_images.len() as u16;
        grp_data.extend_from_slice(&0u16.to_le_bytes());
        grp_data.extend_from_slice(&1u16.to_le_bytes());
        grp_data.extend_from_slice(&num_images.to_le_bytes());

        for (idx, (size, dib)) in dib_images.iter().enumerate() {
            let res_id = (idx + 1) as u16;
            let b_size = if *size >= 256 { 0u8 } else { *size as u8 };

            // GRPICONDIRENTRY (14 bytes)
            grp_data.push(b_size);
            grp_data.push(b_size);
            grp_data.push(0);
            grp_data.push(0);
            grp_data.extend_from_slice(&1u16.to_le_bytes());
            grp_data.extend_from_slice(&32u16.to_le_bytes());
            grp_data.extend_from_slice(&(dib.len() as u32).to_le_bytes());
            grp_data.extend_from_slice(&res_id.to_le_bytes()); // nID

            // Update individual RT_ICON resource
            let res_id_ptr = res_id as usize as *const u16;
            let ok = UpdateResourceW(
                h_update,
                rt_icon,
                res_id_ptr,
                0, // MAKELANGID(LANG_NEUTRAL, SUBLANG_NEUTRAL)
                dib.as_ptr() as *const _,
                dib.len() as u32,
            );
            if ok == 0 {
                EndUpdateResourceW(h_update, 1);
                return Err(format!("UpdateResourceW failed for icon {}", res_id));
            }
        }

        // Update RT_GROUP_ICON resource with ID = 1 (main app icon)
        let group_id_ptr = 1 as usize as *const u16;
        let ok = UpdateResourceW(
            h_update,
            rt_group_icon,
            group_id_ptr,
            0,
            grp_data.as_ptr() as *const _,
            grp_data.len() as u32,
        );
        if ok == 0 {
            EndUpdateResourceW(h_update, 1);
            return Err("UpdateResourceW failed for RT_GROUP_ICON".to_string());
        }

        // FALSE = commit changes
        if EndUpdateResourceW(h_update, 0) == 0 {
            return Err("EndUpdateResourceW failed to write changes".to_string());
        }

        Ok(())
    }
}

fn main() {
    println!("=== DANG TAO ICON PHAP TRAN TU TIEN ===");

    let sizes = [256, 128, 64, 48, 32, 16];
    let (ico_bytes, dib_images) = create_ico_file(&sizes);

    // 1. Save icon.ico to assets and root
    std::fs::create_dir_all("assets").unwrap();
    File::create("assets/icon.ico").unwrap().write_all(&ico_bytes).unwrap();
    println!("* Da luu: assets/icon.ico ({} bytes)", ico_bytes.len());

    File::create("../icon.ico").unwrap().write_all(&ico_bytes).unwrap();
    println!("* Da luu: ../icon.ico");

    // 2. Inject icon into executable files
    let targets = [
        Path::new("target/release/tu_tien_tran.exe"),
        Path::new("../bin/tu_tien_tran.exe"),
    ];

    for target in &targets {
        if target.exists() {
            match inject_icon_into_exe(target, &dib_images) {
                Ok(_) => println!("* Da gan icon truc tiep vao: {:?}", target),
                Err(e) => eprintln!("! Khong the gan icon vao {:?}: {}", target, e),
            }
        } else {
            println!("(Bo qua target chua ton tai: {:?})", target);
        }
    }

    println!("=== HOAN TAT TAO VA GAN ICON ===");
}

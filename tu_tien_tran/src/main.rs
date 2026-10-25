mod clock;
mod formation;
mod hud;
mod particles;
mod theme;
mod window;

use std::time::{Duration, Instant};
use tiny_skia::{Color, Pixmap};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

use clock::CelestialClock;
use formation::MagicFormation;
use hud::HudState;
use theme::RealmTheme;
use window::{
    DesktopOverlayWindow, HOTKEY_CYCLE_CLOCK, HOTKEY_CYCLE_SPEED, HOTKEY_EXIT,
    HOTKEY_NEXT_THEME, HOTKEY_TOGGLE_CLICKTHROUGH,
};

fn main() {
    println!("======================================================");
    println!("   KHOI DONG PHAP TRAN TU TIEN (DESKTOP OVERLAY)      ");
    println!("======================================================");
    println!("* Chuot Trai          : Keo di chuyen phap tran");
    println!("* Chuot Phai / Space  : Doi Canh Gioi (Theme)");
    println!("* Phim G / C / Alt + G: Doi Che Do Dong Ho (Full / Minimal / Tat)");
    println!("* Chuot Giua (Nhan)   : Doi Toc Do Quay (Tuan hoan 5 muc)");
    println!("* Phim S hoac Alt + S : Doi Toc Do Quay (Hoat dong toan cuc)");
    println!("* Phim 1 -> 5         : Chon truc tiep 5 cap toc do");
    println!("* Phim [ / ]          : Giam / Tang toc do tung bac");
    println!("* Shift + Cuon Chuot  : Tang / Giam toc do quay");
    println!("* Cuon Chuot / (+/-)  : Phong to / Thu nho (360px - 920px)");
    println!("* Phim T hoac Alt + T : Bat/Tat che do Xuyen Thau Chuot");
    println!("* Phim P              : Bat/Tat linh khi dom dom");
    println!("* Phim H              : Bat/Tat bang huong dan");
    println!("* Phim Esc / Q / Alt+Q: Thu hoi phap tran (Thoat)");
    println!("======================================================");

    let mut current_size = 620u32;
    let mut window = match DesktopOverlayWindow::new("Pháp Trận Tu Tiên", current_size) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Khong the tao cua so phap tran: {}", e);
            return;
        }
    };

    let mut formation = MagicFormation::new();
    let mut clock = CelestialClock::new();
    let mut hud = HudState::new();
    let mut theme = RealmTheme::KimDan;

    // 5 Distinct Speed Profiles: (Multiplier, Name)
    let speeds: [(f32, &'static str); 5] = [
        (0.3, "1/5 TINH TAM (0.3X)"),
        (0.6, "2/5 TIEU DAO (0.6X)"),
        (1.0, "3/5 TIEU CHUAN (1.0X)"),
        (1.8, "4/5 PHI THANG (1.8X)"),
        (3.2, "5/5 BAO PHAT (3.2X)"),
    ];
    let mut speed_idx = 2; // Default: Tiêu Chuẩn 1.0x

    let mut show_particles = true;
    let mut last_frame_time = Instant::now();
    let mut running = true;

    // Pixmap buffer
    let mut pixmap = Pixmap::new(current_size, current_size).unwrap();

    while running {
        let frame_start = Instant::now();
        let dt = (frame_start - last_frame_time).as_secs_f32().clamp(0.001, 0.05);
        last_frame_time = frame_start;

        // Process Win32 Message Queue
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            while PeekMessageW(&mut msg, 0, 0, 0, PM_REMOVE) != 0 {
                if msg.message == WM_QUIT {
                    running = false;
                    break;
                }

                match msg.message {
                    WM_HOTKEY => {
                        match msg.wParam as i32 {
                            HOTKEY_TOGGLE_CLICKTHROUGH => {
                                let ct = window.toggle_click_through();
                                hud.notify(if ct {
                                    "XUYEN THAU CHUOT: BAT (BAM ALT+T DE TAT)"
                                } else {
                                    "XUYEN THAU CHUOT: TAT (CO THE KEO PHAP TRAN)"
                                });
                            }
                            HOTKEY_NEXT_THEME => {
                                theme = theme.next();
                                hud.notify(format!("CANH GIOI: {}", theme.short_name()));
                            }
                            HOTKEY_CYCLE_SPEED => {
                                speed_idx = (speed_idx + 1) % speeds.len();
                                hud.notify(format!("TOC DO: {}", speeds[speed_idx].1));
                            }
                            HOTKEY_CYCLE_CLOCK => {
                                clock.mode = clock.mode.next();
                                hud.notify(clock.mode.name());
                            }
                            HOTKEY_EXIT => {
                                running = false;
                            }
                            _ => {}
                        }
                    }

                    WM_KEYDOWN => {
                        let key = msg.wParam as u16;
                        match key {
                            VK_SPACE => {
                                theme = theme.next();
                                hud.notify(format!("CANH GIOI: {}", theme.short_name()));
                            }
                            0x47 | 0x43 => { // 'G' or 'C' key: Cycle clock mode
                                clock.mode = clock.mode.next();
                                hud.notify(clock.mode.name());
                            }
                            0x54 => { // 'T' key: Toggle Click-Through
                                let ct = window.toggle_click_through();
                                hud.notify(if ct {
                                    "XUYEN THAU CHUOT: BAT (BAM ALT+T DE TAT)"
                                } else {
                                    "XUYEN THAU CHUOT: TAT (CO THE KEO PHAP TRAN)"
                                });
                            }
                            0x53 => { // 'S' key: Cycle speed
                                speed_idx = (speed_idx + 1) % speeds.len();
                                hud.notify(format!("TOC DO: {}", speeds[speed_idx].1));
                            }
                            // Number keys '1' through '5'
                            0x31 | VK_NUMPAD1 => {
                                speed_idx = 0;
                                hud.notify(format!("TOC DO: {}", speeds[0].1));
                            }
                            0x32 | VK_NUMPAD2 => {
                                speed_idx = 1;
                                hud.notify(format!("TOC DO: {}", speeds[1].1));
                            }
                            0x33 | VK_NUMPAD3 => {
                                speed_idx = 2;
                                hud.notify(format!("TOC DO: {}", speeds[2].1));
                            }
                            0x34 | VK_NUMPAD4 => {
                                speed_idx = 3;
                                hud.notify(format!("TOC DO: {}", speeds[3].1));
                            }
                            0x35 | VK_NUMPAD5 => {
                                speed_idx = 4;
                                hud.notify(format!("TOC DO: {}", speeds[4].1));
                            }
                            // Step speed down: '[' or Down arrow
                            0xDB | VK_DOWN => {
                                speed_idx = (speed_idx + speeds.len() - 1) % speeds.len();
                                hud.notify(format!("TOC DO: {}", speeds[speed_idx].1));
                            }
                            // Step speed up: ']' or Up arrow
                            0xDD | VK_UP => {
                                speed_idx = (speed_idx + 1) % speeds.len();
                                hud.notify(format!("TOC DO: {}", speeds[speed_idx].1));
                            }
                            0x50 => { // 'P' key
                                show_particles = !show_particles;
                                hud.notify(if show_particles { "LINH KHI: BAT" } else { "LINH KHI: TAT" });
                            }
                            0x48 => { // 'H' key
                                hud.toggle_help();
                            }
                            VK_OEM_PLUS | VK_ADD => { // '+' or '='
                                if current_size < 920 {
                                    current_size += 40;
                                    window.resize(current_size);
                                    pixmap = Pixmap::new(current_size, current_size).unwrap();
                                    hud.notify(format!("KICH THUOC: {}PX", current_size));
                                }
                            }
                            VK_OEM_MINUS | VK_SUBTRACT => { // '-'
                                if current_size > 360 {
                                    current_size -= 40;
                                    window.resize(current_size);
                                    pixmap = Pixmap::new(current_size, current_size).unwrap();
                                    hud.notify(format!("KICH THUOC: {}PX", current_size));
                                }
                            }
                            VK_ESCAPE | 0x51 => { // Esc or 'Q'
                                running = false;
                            }
                            _ => {}
                        }
                    }

                    WM_LBUTTONDOWN => {
                        window.focus();
                        window.start_drag();
                    }

                    WM_RBUTTONDOWN => {
                        window.focus();
                        theme = theme.next();
                        hud.notify(format!("CANH GIOI: {}", theme.short_name()));
                    }

                    WM_MBUTTONDOWN => { // Middle click: Cycle speed!
                        window.focus();
                        speed_idx = (speed_idx + 1) % speeds.len();
                        hud.notify(format!("TOC DO: {}", speeds[speed_idx].1));
                    }

                    WM_MOUSEWHEEL => {
                        let delta = ((msg.wParam >> 16) as i16) as i32;

                        // Check if Shift or Ctrl is pressed
                        let is_shift = GetKeyState(VK_SHIFT as i32) < 0;
                        let is_ctrl = GetKeyState(VK_CONTROL as i32) < 0;

                        if is_shift || is_ctrl {
                            // Wheel adjusts speed
                            if delta > 0 {
                                speed_idx = (speed_idx + 1) % speeds.len();
                            } else if delta < 0 {
                                speed_idx = (speed_idx + speeds.len() - 1) % speeds.len();
                            }
                            hud.notify(format!("TOC DO: {}", speeds[speed_idx].1));
                        } else {
                            // Wheel adjusts size
                            if delta > 0 && current_size < 920 {
                                current_size += 30;
                                window.resize(current_size);
                                pixmap = Pixmap::new(current_size, current_size).unwrap();
                                hud.notify(format!("KICH THUOC: {}PX", current_size));
                            } else if delta < 0 && current_size > 360 {
                                current_size -= 30;
                                window.resize(current_size);
                                pixmap = Pixmap::new(current_size, current_size).unwrap();
                                hud.notify(format!("KICH THUOC: {}PX", current_size));
                            }
                        }
                    }

                    _ => {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }
            }
        }

        if !running {
            break;
        }

        // Clear surface to 100% transparent
        pixmap.fill(Color::TRANSPARENT);

        let cx = (current_size as f32) / 2.0;
        let cy = (current_size as f32) / 2.0;
        let max_radius = cx * 0.90;

        let (speed_mult, speed_name) = speeds[speed_idx];

        // 1. Update and render cultivation formation
        formation.update(dt, speed_mult, max_radius);
        formation.render(
            &mut pixmap,
            cx,
            cy,
            max_radius,
            &theme,
            show_particles,
        );

        // 2. Update and render Celestial Clock (Đồng Hồ Tiên Đạo)
        clock.update();
        clock.draw(
            &mut pixmap,
            cx,
            cy,
            max_radius,
            &theme,
        );

        // 3. Update and render HUD notifications
        hud.update(dt);
        hud.draw(
            &mut pixmap,
            current_size as f32,
            current_size as f32,
            &theme,
            window.click_through,
            speed_name,
        );

        // 4. Blit transparent surface to desktop
        window.render_frame(&pixmap);

        // 5. Smooth 60 FPS frame rate limiting (~16.6ms)
        let elapsed = frame_start.elapsed();
        let target_frame_time = Duration::from_micros(16_666);
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
    }

    println!("Pháp trận tu tiên đã thu hồi thành công!");
}

use std::ffi::c_void;
use tiny_skia::Pixmap;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

pub const HOTKEY_TOGGLE_CLICKTHROUGH: i32 = 101;
pub const HOTKEY_NEXT_THEME: i32 = 102;
pub const HOTKEY_EXIT: i32 = 103;
pub const HOTKEY_CYCLE_SPEED: i32 = 104;


pub struct DesktopOverlayWindow {
    pub hwnd: HWND,
    pub width: u32,
    pub height: u32,
    hdc_screen: HDC,
    hdc_mem: HDC,
    hbitmap: HBITMAP,
    old_bitmap: HGDIOBJ,
    bitmap_bits: *mut u8,
    pub click_through: bool,
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

impl DesktopOverlayWindow {
    pub fn new(title: &str, initial_size: u32) -> Result<Self, String> {
        unsafe {
            let hinstance = GetModuleHandleW(std::ptr::null());

            // Convert class name and title to UTF-16
            let class_name: Vec<u16> = "TuTienTranClass\0".encode_utf16().collect();
            let window_title: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();

            let cursor = LoadCursorW(0, IDC_ARROW);

            let wnd_class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: hinstance,
                hIcon: 0,
                hCursor: cursor,
                hbrBackground: 0,
                lpszMenuName: std::ptr::null(),
                lpszClassName: class_name.as_ptr(),
            };

            RegisterClassW(&wnd_class);

            let screen_w = GetSystemMetrics(SM_CXSCREEN);
            let screen_h = GetSystemMetrics(SM_CYSCREEN);
            let x = (screen_w - initial_size as i32) / 2;
            let y = (screen_h - initial_size as i32) / 2;

            let ex_style = WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TOOLWINDOW;
            let style = WS_POPUP | WS_VISIBLE;

            let hwnd = CreateWindowExW(
                ex_style,
                class_name.as_ptr(),
                window_title.as_ptr(),
                style,
                x,
                y,
                initial_size as i32,
                initial_size as i32,
                0,
                0,
                hinstance,
                std::ptr::null_mut(),
            );

            if hwnd == 0 {
                return Err("Failed to create Win32 window".to_string());
            }

            // Register Global Hotkeys so user can control the formation even in Click-Through mode!
            // Alt + T: Toggle Click-Through
            RegisterHotKey(hwnd, HOTKEY_TOGGLE_CLICKTHROUGH, MOD_ALT as u32, 'T' as u32);
            // Alt + C: Cycle Realm Theme
            RegisterHotKey(hwnd, HOTKEY_NEXT_THEME, MOD_ALT as u32, 'C' as u32);
            // Alt + S: Cycle Speed
            RegisterHotKey(hwnd, HOTKEY_CYCLE_SPEED, MOD_ALT as u32, 'S' as u32);
            // Alt + Q: Exit
            RegisterHotKey(hwnd, HOTKEY_EXIT, MOD_ALT as u32, 'Q' as u32);


            let hdc_screen = GetDC(0);
            let hdc_mem = CreateCompatibleDC(hdc_screen);

            let mut win = Self {
                hwnd,
                width: initial_size,
                height: initial_size,
                hdc_screen,
                hdc_mem,
                hbitmap: 0,
                old_bitmap: 0,
                bitmap_bits: std::ptr::null_mut(),
                click_through: false,
            };

            win.create_dib(initial_size, initial_size)?;

            ShowWindow(hwnd, SW_SHOW);
            UpdateWindow(hwnd);

            Ok(win)
        }
    }

    unsafe fn create_dib(&mut self, w: u32, h: u32) -> Result<(), String> {
        unsafe {
            if self.hbitmap != 0 {
                SelectObject(self.hdc_mem, self.old_bitmap);
                DeleteObject(self.hbitmap);
                self.hbitmap = 0;
                self.bitmap_bits = std::ptr::null_mut();
            }

            self.width = w;
            self.height = h;

            let mut bmi: BITMAPINFO = std::mem::zeroed();
            bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
            bmi.bmiHeader.biWidth = w as i32;
            bmi.bmiHeader.biHeight = -(h as i32); // Top-down DIB
            bmi.bmiHeader.biPlanes = 1;
            bmi.bmiHeader.biBitCount = 32;
            bmi.bmiHeader.biCompression = BI_RGB as u32;

            let mut bits: *mut c_void = std::ptr::null_mut();
            let hbitmap = CreateDIBSection(
                self.hdc_screen,
                &bmi,
                DIB_RGB_COLORS,
                &mut bits,
                0,
                0,
            );

            if hbitmap == 0 || bits.is_null() {
                return Err("Failed to create DIB section for layered window".to_string());
            }

            self.old_bitmap = SelectObject(self.hdc_mem, hbitmap);
            self.hbitmap = hbitmap;
            self.bitmap_bits = bits as *mut u8;

            Ok(())
        }
    }

    pub fn resize(&mut self, new_size: u32) {
        if new_size == self.width {
            return;
        }
        unsafe {
            let mut rect: RECT = std::mem::zeroed();
            GetWindowRect(self.hwnd, &mut rect);
            let cur_cx = rect.left + (rect.right - rect.left) / 2;
            let cur_cy = rect.top + (rect.bottom - rect.top) / 2;

            let new_x = cur_cx - (new_size as i32) / 2;
            let new_y = cur_cy - (new_size as i32) / 2;

            let _ = self.create_dib(new_size, new_size);

            SetWindowPos(
                self.hwnd,
                HWND_TOPMOST,
                new_x,
                new_y,
                new_size as i32,
                new_size as i32,
                SWP_NOACTIVATE,
            );
        }
    }

    pub fn toggle_click_through(&mut self) -> bool {
        self.click_through = !self.click_through;
        unsafe {
            let mut ex_style = GetWindowLongPtrW(self.hwnd, GWL_EXSTYLE) as u32;
            if self.click_through {
                ex_style |= WS_EX_TRANSPARENT;
            } else {
                ex_style &= !WS_EX_TRANSPARENT;
            }
            SetWindowLongPtrW(self.hwnd, GWL_EXSTYLE, ex_style as isize);
        }
        self.click_through
    }

    pub fn focus(&self) {
        unsafe {
            SetForegroundWindow(self.hwnd);
            SetFocus(self.hwnd);
        }
    }

    pub fn start_drag(&self) {
        if self.click_through {
            return;
        }
        self.focus();
        unsafe {
            ReleaseCapture();
            SendMessageW(self.hwnd, WM_NCLBUTTONDOWN, HTCAPTION as usize, 0);
        }
    }


    pub fn render_frame(&mut self, pixmap: &Pixmap) {
        if self.bitmap_bits.is_null() {
            return;
        }

        let total_pixels = (self.width * self.height) as usize;
        let src_data = pixmap.data();

        // Convert Premultiplied RGBA from tiny-skia to Windows Premultiplied BGRA
        unsafe {
            let dest_slice = std::slice::from_raw_parts_mut(self.bitmap_bits, total_pixels * 4);
            let src_slice = &src_data[..total_pixels * 4];

            // 4-byte chunk conversion
            for (src_chunk, dest_chunk) in src_slice.chunks_exact(4).zip(dest_slice.chunks_exact_mut(4)) {
                let r = src_chunk[0];
                let g = src_chunk[1];
                let b = src_chunk[2];
                let a = src_chunk[3];

                dest_chunk[0] = b; // Blue
                dest_chunk[1] = g; // Green
                dest_chunk[2] = r; // Red
                dest_chunk[3] = a; // Alpha
            }

            let blend = BLENDFUNCTION {
                BlendOp: AC_SRC_OVER as u8,
                BlendFlags: 0,
                SourceConstantAlpha: 255,
                AlphaFormat: AC_SRC_ALPHA as u8,
            };

            let pt_src = POINT { x: 0, y: 0 };
            let size = SIZE {
                cx: self.width as i32,
                cy: self.height as i32,
            };

            UpdateLayeredWindow(
                self.hwnd,
                self.hdc_screen,
                std::ptr::null(), // keep current window position
                &size,
                self.hdc_mem,
                &pt_src,
                0,
                &blend,
                ULW_ALPHA,
            );
        }
    }
}

impl Drop for DesktopOverlayWindow {
    fn drop(&mut self) {
        unsafe {
            UnregisterHotKey(self.hwnd, HOTKEY_TOGGLE_CLICKTHROUGH);
            UnregisterHotKey(self.hwnd, HOTKEY_NEXT_THEME);
            UnregisterHotKey(self.hwnd, HOTKEY_CYCLE_SPEED);
            UnregisterHotKey(self.hwnd, HOTKEY_EXIT);


            if self.hbitmap != 0 {
                SelectObject(self.hdc_mem, self.old_bitmap);
                DeleteObject(self.hbitmap);
            }
            if self.hdc_mem != 0 {
                DeleteDC(self.hdc_mem);
            }
            if self.hdc_screen != 0 {
                ReleaseDC(0, self.hdc_screen);
            }
            if self.hwnd != 0 {
                DestroyWindow(self.hwnd);
            }
        }
    }
}

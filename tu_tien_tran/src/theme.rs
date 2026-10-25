use tiny_skia::Color;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RealmTheme {
    KimDan,       // Kim Đan Thần Quang (Hoàng Kim Thánh Quang)
    ThaiThanh,    // Thái Thanh Lam Tiên (Băng Hồn Tiên Khí)
    CuuU,         // Cửu U Ma Trận (Ma Đạo Huyết Sát)
    TuLoi,        // Cửu Thiên Tử Lôi (Lôi Kiếp Cảnh)
    TruongSinh,   // Thần Mộc Trường Sinh (Mộc Linh Tiên Khí)
}

#[allow(dead_code)]
impl RealmTheme {
    pub const ALL: [RealmTheme; 5] = [
        RealmTheme::KimDan,
        RealmTheme::ThaiThanh,
        RealmTheme::CuuU,
        RealmTheme::TuLoi,
        RealmTheme::TruongSinh,
    ];

    pub fn next(&self) -> RealmTheme {
        match self {
            RealmTheme::KimDan => RealmTheme::ThaiThanh,
            RealmTheme::ThaiThanh => RealmTheme::CuuU,
            RealmTheme::CuuU => RealmTheme::TuLoi,
            RealmTheme::TuLoi => RealmTheme::TruongSinh,
            RealmTheme::TruongSinh => RealmTheme::KimDan,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            RealmTheme::KimDan => "Kim Đan Cảnh (Hoàng Kim Thánh Quang)",
            RealmTheme::ThaiThanh => "Thái Thanh Cảnh (Băng Hồn Lam Diễm)",
            RealmTheme::CuuU => "Cửu U Ma Trận (Huyết Ma U Minh)",
            RealmTheme::TuLoi => "Cửu Thiên Cảnh (Tử Lôi Phá Thiên)",
            RealmTheme::TruongSinh => "Trường Sinh Cảnh (Phỉ Thúy Thần Mộc)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            RealmTheme::KimDan => "KIM DAN",
            RealmTheme::ThaiThanh => "THAI THANH",
            RealmTheme::CuuU => "CUU U MA DAO",
            RealmTheme::TuLoi => "CUU THIEN TU LOI",
            RealmTheme::TruongSinh => "TRUONG SINH MOC",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            RealmTheme::KimDan => "Chân hỏa đại thành - Vạn kiếp bất diệt",
            RealmTheme::ThaiThanh => "Cực hàn huyền băng - Đóng băng hư không",
            RealmTheme::CuuU => "Huyết sát ngút trời - Vạn quỷ quy tông",
            RealmTheme::TuLoi => "Thiên kiếp lôi phạt - Tịnh hóa tà ma",
            RealmTheme::TruongSinh => "Sinh mệnh vô tận - Vạn vật hồi xuân",
        }
    }

    /// Primary radiant color (strokes, primary glow)
    pub fn primary(&self) -> (u8, u8, u8) {
        match self {
            RealmTheme::KimDan => (255, 205, 50),     // Radiant Gold
            RealmTheme::ThaiThanh => (0, 225, 255),   // Celestial Cyan
            RealmTheme::CuuU => (255, 30, 70),        // Blood Ruby
            RealmTheme::TuLoi => (210, 60, 255),      // Electric Violet
            RealmTheme::TruongSinh => (30, 245, 130), // Jade Emerald
        }
    }

    /// Secondary contrast color (accents, inner rings, sparks)
    pub fn secondary(&self) -> (u8, u8, u8) {
        match self {
            RealmTheme::KimDan => (255, 120, 20),     // Solar Amber
            RealmTheme::ThaiThanh => (60, 110, 255),  // Deep Mystic Blue
            RealmTheme::CuuU => (170, 10, 30),        // Dark Crimson
            RealmTheme::TuLoi => (130, 10, 230),      // Royal Nebula Purple
            RealmTheme::TruongSinh => (10, 170, 90),  // Deep Forest Green
        }
    }

    /// Brightest core color (pure spirit energy, center flashes)
    pub fn core(&self) -> (u8, u8, u8) {
        match self {
            RealmTheme::KimDan => (255, 255, 220),
            RealmTheme::ThaiThanh => (220, 255, 255),
            RealmTheme::CuuU => (255, 200, 210),
            RealmTheme::TuLoi => (245, 220, 255),
            RealmTheme::TruongSinh => (220, 255, 235),
        }
    }

    /// Soft glow color with alpha
    pub fn glow_color(&self, alpha: u8) -> Color {
        let (r, g, b) = self.primary();
        Color::from_rgba8(r, g, b, alpha)
    }

    pub fn primary_color(&self, alpha: u8) -> Color {
        let (r, g, b) = self.primary();
        Color::from_rgba8(r, g, b, alpha)
    }

    pub fn secondary_color(&self, alpha: u8) -> Color {
        let (r, g, b) = self.secondary();
        Color::from_rgba8(r, g, b, alpha)
    }

    pub fn core_color(&self, alpha: u8) -> Color {
        let (r, g, b) = self.core();
        Color::from_rgba8(r, g, b, alpha)
    }
}

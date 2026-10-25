# 🌀 PHÁP TRẬN TU TIÊN NGOÀI MÀN HÌNH (DESKTOP CULTIVATION OVERLAY)

Ứng dụng hiển thị **Pháp Trận Tu Tiên** xoay động lơ lửng trực tiếp trên màn hình máy tính (Windows Desktop Overlay) được viết hoàn toàn bằng ngôn ngữ **Rust**.

---

## 🌟 ĐẶC ĐIỂM NỔI BẬT

1. **Hiệu ứng Trong suốt Tuyệt đối (True Alpha Transparency):**
   - Sử dụng Windows Layered Window (`WS_EX_LAYERED`) với chuẩn 32-bit ARGB Premultiplied.
   - Hoàn toàn **không có viền đen** hay khung cửa sổ vuông. Chỉ có pháp trận linh quang tỏa sáng hòa quyện vào hình nền desktop và các cửa sổ ứng dụng khác.

2. **Cấu trúc Pháp Trận Đạo Giáo Đỉnh Cao:**
   - **Tâm Trận (Thái Cực Đồ):** Vòng xoay Âm Dương song ngư với 2 nhãn quang, bao bọc bởi luồng hào quang hô hấp (Breathing Qi Halo).
   - **Tầng 1 (Lục Hợp / Thần Tinh):** Mạng lưới sao năng lượng với các hạt linh châu phát sáng.
   - **Tầng 2 (Bát Quái Trận Đồ):** Đầy đủ 8 quẻ Tiên Thiên Bát Quái: **Càn (☰), Đoài (☱), Ly (☲), Chấn (☳), Tốn (☴), Khảm (☵), Cấn (☶), Khôn (☷)** với các vạch Liền (Dương) và Đứt (Âm) được bao bọc trong khung cổ tự.
   - **Tầng 3 (Bát Phương Thần Trận):** Hai khối vuông lồng ghép (Bát Giác Tinh) xoay nghịch chiều với 8 viên kim cương phù ấn ở đỉnh.
   - **Tầng 4 (Chu Thiên Phù Lục & Tinh Đẩu):** 72 vạch đo đạc Địa Sát tinh vĩ, 12 Tiên Thiên bảo châu, và 16 Cổ Phù Triện Đạo gia biến hóa tuần hoàn.
   - **Tầng 5 (Cửu Trọng Khiên & Tinh Cầu):** Hệ thống khiên linh lực phân đoạn và 4 vệ tinh linh châu quay quanh biên giới trận pháp.
   - **Linh Lực Ba Động (Energy Shockwaves):** Các vòng sóng xung kích năng lượng tỏa từ tâm trận lan tỏa ra rìa ngoài.
   - **Lôi Điện Tiên Khí (Crackling Lightning):** Các tia sét tiên đạo phóng điện ngẫu nhiên giữa các tầng trận pháp.
   - **Linh Khí Du Động (Spirit Particles):** Hơn 50 đom đóm linh khí xoắn ốc tụ khí về tâm trận với vệt đuôi sao băng lấp lánh.

3. **5 Đại Cảnh Giới Tu Vi (5 Bảng Màu Tuyệt Mỹ):**
   - 🌟 **Kim Đan Cảnh (Hoàng Kim Thánh Quang):** Hào quang hoàng kim kim đan, ánh lửa thuần dương.
   - ❄️ **Thái Thanh Cảnh (Băng Hồn Lam Diễm):** Băng lam tiên khí huyền ảo, thanh khiết lạnh lùng.
   - 🩸 **Cửu U Ma Trận (Huyết Ma U Minh):** Huyết hồng sát khí ma đạo cuồn cuộn.
   - ⚡ **Cửu Thiên Cảnh (Tử Lôi Phá Thiên):** Sét tím thiên kiếp trừng phạt tà ma.
   - 🍃 **Trường Sinh Cảnh (Phỉ Thúy Thần Mộc):** Mộc linh lục bích tràn đầy sinh mệnh vô tận.

4. **Tương tác Linh hoạt trên Desktop:**
   - **Kéo di chuyển:** Nhấp chuột trái vào pháp trận và kéo đến bất kỳ vị trí nào trên màn hình.
   - **Chế độ Xuyên Thấu Chuột (Click-Through):** Bấm `T` hoặc `Alt + T` để biến pháp trận thành vô hình với chuột, cho phép bạn làm việc, lướt web, chơi game bình thường trong khi pháp trận vẫn xoay huyền ảo bên dưới con trỏ chuột!
   - **Phóng to / Thu nhỏ:** Lăn chuột giữa hoặc bấm `+` / `-` để điều chỉnh kích cỡ linh hoạt từ 360px đến 920px.
   - **Tối ưu cực đại:** Tốc độ khung hình 60 FPS mượt mà nhưng chiếm chưa đến 1-2% CPU nhờ thuật toán đồ họa tối ưu.

---

## 🎮 HƯỚNG DẪN ĐIỀU KHIỂN

| Thao tác | Phím tắt | Chức năng |
| :--- | :---: | :--- |
| **Kéo cửa sổ** | **Chuột trái** | Giữ và kéo pháp trận di chuyển khắp màn hình |
| **Đổi Cảnh Giới** | **Chuột phải** / `Space` / `Alt + C` | Chuyển đổi 5 cảnh giới (Kim Đan, Thái Thanh, Cửu U, Tử Lôi, Trường Sinh) |
| **Đổi tốc độ nhanh** | **Nhấn chuột giữa** (Cuộn chuột) | Chuyển tuần hoàn 5 mức tốc độ |
| **Đổi tốc độ phím** | `S` hoặc `Alt + S` | Đổi tốc độ quay (hoạt động toàn cục ngay cả khi xuyên thấu) |
| **Chọn tốc độ trực tiếp** | Phím `1`, `2`, `3`, `4`, `5` | `1`: Tĩnh Tâm (0.3x) - `2`: Tiêu Dao (0.6x) - `3`: Tiêu Chuẩn (1.0x) - `4`: Phi Thăng (1.8x) - `5`: Bạo Phát (3.2x) |
| **Tăng/Giảm tốc độ** | `[` / `]` hoặc Mũi tên `Up` / `Down` | Tăng hoặc giảm tốc độ từng nấc |
| **Cuộn đổi tốc độ** | `Shift` + **Cuộn chuột** | Cuộn chuột lên/xuống khi giữ Shift để tăng/giảm tốc độ mượt mà |
| **Xuyên thấu chuột** | `T` hoặc `Alt + T` | Bật/Tắt chế độ click-through (chuột bấm xuyên qua pháp trận) |
| **Phóng to / Thu nhỏ**| **Cuộn chuột** hoặc `+` / `-` | Thay đổi đường kính pháp trận (360px - 920px) |
| **Bật/Tắt Linh Khí** | `P` | Ẩn/Hiện các đom đóm linh khí du động |
| **Bảng Hướng Dẫn** | `H` | Bật/Tắt bảng thông tin phím tắt hiển thị trên màn hình |
| **Thu hồi (Thoát)** | `Esc` / `Q` / `Alt + Q` | Đóng chương trình |

> **Mẹo hữu ích:** Khi bật chế độ **Xuyên Thấu Chuột**, chuột sẽ bấm xuyên qua pháp trận. Để điều khiển bất cứ lúc nào từ bất kỳ cửa sổ nào, bạn có thể dùng tổ hợp phím toàn cục:
> - `Alt + S`: Đổi tốc độ quay
> - `Alt + T`: Bật/Tắt lại xuyên thấu chuột
> - `Alt + C`: Đổi cảnh giới tu vi
> - `Alt + Q`: Thoát chương trình


---

## 🚀 CÁCH KHỞI CHẠY

### Cách 1: Chạy file thực thi đã biên dịch sẵn (Nhanh nhất)
Chạy file thực thi đã biên dịch tối ưu (Release mode) tại:
```powershell
d:\dev\tu tiên\tu_tien_tran\target\release\tu_tien_tran.exe
```
Hoặc chạy file `Chay_Phap_Tran.bat` ngoài thư mục gốc.

### Cách 2: Khởi chạy bằng Cargo (Mã nguồn)
Mở PowerShell tại thư mục dự án và chạy:
```powershell
cd "d:\dev\tu tiên\tu_tien_tran"
cargo run --release
```

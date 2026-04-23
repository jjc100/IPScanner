#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::ffi::c_void;
use std::fs;
use std::io::Write;
use std::mem::size_of;
use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use dns_lookup::lookup_addr;
use eframe::egui::{
    self, Align, Color32, FontData, FontDefinitions, FontFamily, Layout, RichText, TextEdit,
};
use eframe::{App, Frame, NativeOptions};
use egui_extras::{Column, TableBuilder};
#[cfg(not(windows))]
use if_addrs::IfAddr;
#[cfg(windows)]
use ipconfig::Adapter;
use ipnet::IpNet;
use rfd::FileDialog;
use rust_xlsxwriter::{Format, FormatAlign, Workbook};
use serde::{Deserialize, Serialize};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

const DEFAULT_OUI_JSON: &str = r#"{
  "00:00:0C": "Cisco Systems",
  "00:03:93": "Apple",
  "00:05:69": "VMware",
  "00:09:0F": "Fortinet",
  "00:0C:29": "VMware",
  "00:0F:66": "Cisco-Linksys",
  "00:11:22": "Cimsys",
  "00:13:10": "Linksys",
  "00:14:22": "Dell",
  "00:15:5D": "Microsoft",
  "00:16:6F": "Intel",
  "00:17:88": "Philips",
  "00:18:E7": "Cisco-Linksys",
  "00:1A:11": "Google",
  "00:1B:63": "Apple",
  "00:1C:B3": "Apple",
  "00:1D:7E": "Cisco Systems",
  "00:1E:C2": "Apple",
  "00:1F:3A": "Samsung Electronics",
  "00:21:5A": "Sony Mobile",
  "00:22:68": "Hewlett Packard",
  "00:23:69": "Huawei Technologies",
  "00:24:D7": "Ubiquiti",
  "00:25:9C": "Cisco Systems",
  "00:26:18": "Asustek Computer",
  "00:26:B6": "Apple",
  "00:30:48": "Supermicro",
  "00:50:56": "VMware",
  "08:00:27": "Oracle VirtualBox",
  "08:11:96": "Huawei Technologies",
  "08:3A:88": "Samsung Electronics",
  "0C:1D:AF": "Hon Hai Precision",
  "10:1D:C0": "Intel",
  "10:7B:44": "Apple",
  "14:49:BC": "Samsung Electronics",
  "18:31:BF": "Samsung Electronics",
  "18:65:90": "Apple",
  "1C:1B:0D": "Apple",
  "1C:5F:2B": "Huawei Technologies",
  "20:16:B9": "Samsung Electronics",
  "20:4E:7F": "Asustek Computer",
  "24:18:1D": "Ubiquiti",
  "24:5A:4C": "Xiaomi Communications",
  "28:16:AD": "Samsung Electronics",
  "28:6C:07": "Apple",
  "2C:54:2D": "Cisco Meraki",
  "2C:CF:67": "Intel",
  "30:07:4D": "TP-Link",
  "30:85:A9": "Asustek Computer",
  "34:17:EB": "Dell",
  "34:97:F6": "LG Electronics",
  "38:F9:D3": "Apple",
  "3C:52:82": "Hewlett Packard Enterprise",
  "3C:84:6A": "Microsoft",
  "40:16:7E": "Xiaomi Communications",
  "44:65:0D": "Intel",
  "48:2C:A0": "Samsung Electronics",
  "48:5F:99": "Apple",
  "4C:32:75": "Intel",
  "50:2B:73": "Cisco Systems",
  "50:C7:BF": "Samsung Electronics",
  "54:27:1E": "Xiaomi Communications",
  "58:6D:8F": "Cisco Meraki",
  "58:CB:52": "Google",
  "5C:51:4F": "LG Electronics",
  "60:03:08": "Apple",
  "64:16:66": "Samsung Electronics",
  "64:66:B3": "Intel",
  "68:54:5A": "Intel",
  "68:72:51": "Ubiquiti",
  "6C:2B:59": "Dell",
  "70:3A:CB": "Google",
  "70:4D:7B": "Asustek Computer",
  "74:83:C2": "Samsung Electronics",
  "78:11:DC": "Huawei Technologies",
  "78:45:58": "TP-Link",
  "7C:2E:BD": "Apple",
  "80:2A:A8": "Ubiquiti",
  "80:45:DD": "Hewlett Packard",
  "84:3A:4B": "Samsung Electronics",
  "84:A9:38": "Ubiquiti",
  "88:32:9B": "Apple",
  "8C:85:90": "Huawei Technologies",
  "90:9F:33": "Intel",
  "94:65:2D": "OnePlus",
  "98:01:A7": "Apple",
  "98:DA:C4": "TP-Link",
  "9C:FC:01": "Ubiquiti",
  "A0:99:9B": "Apple",
  "A4:2B:B0": "TP-Link",
  "A4:77:33": "Intel",
  "A8:5E:45": "Google",
  "AC:1F:6B": "Cisco Systems",
  "AC:22:0B": "Asustek Computer",
  "AC:BC:32": "Apple",
  "B0:4E:26": "Huawei Technologies",
  "B4:2E:99": "Apple",
  "B8:27:EB": "Raspberry Pi Foundation",
  "BC:14:EF": "Samsung Electronics",
  "BC:54:36": "Apple",
  "C0:25:06": "Intel",
  "C0:56:27": "TP-Link",
  "C4:E9:84": "Samsung Electronics",
  "C8:2A:14": "Apple",
  "CC:46:D6": "Cisco Systems",
  "D0:17:C2": "Asustek Computer",
  "D4:6E:0E": "Samsung Electronics",
  "D8:3A:DD": "Google",
  "DC:A6:32": "Raspberry Pi Trading",
  "E0:63:DA": "Ubiquiti",
  "E0:B9:BA": "Apple",
  "E4:5F:01": "Raspberry Pi Trading",
  "E8:48:B8": "Samsung Electronics",
  "EC:08:6B": "Asustek Computer",
  "F0:18:98": "Apple",
  "F4:F5:D8": "Google",
  "F8:32:E4": "Samsung Electronics",
  "FC:34:97": "Apple"
}"#;

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([1180.0, 720.0])
            .with_title("IPScanner")
            .with_icon(Arc::new(app_icon_data())),
        ..Default::default()
    };

    eframe::run_native(
        "IPScanner",
        native_options,
        Box::new(|cc| {
            configure_fonts(&cc.egui_ctx);
            configure_theme(&cc.egui_ctx);
            Ok(Box::new(FaIpScannerApp::default()))
        }),
    )
}

fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    let mut custom_font_names = Vec::new();
    for (font_name, font_data) in load_system_ui_fonts() {
        fonts
            .font_data
            .insert(font_name.clone(), FontData::from_owned(font_data).into());
        custom_font_names.push(font_name);
    }

    if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        for font_name in custom_font_names.iter().rev() {
            family.insert(0, font_name.clone());
        }
    }

    if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
        for font_name in custom_font_names.iter().rev() {
            family.insert(0, font_name.clone());
        }
    }

    ctx.set_fonts(fonts);
}

fn configure_theme(ctx: &egui::Context) {
    ctx.set_visuals(egui::Visuals::light());

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.button_padding = egui::vec2(14.0, 8.0);
    style.spacing.interact_size.y = 32.0;
    ctx.set_style(style);
}

fn app_icon_data() -> egui::IconData {
    const SIZE: u32 = 256;
    let mut rgba = vec![0_u8; (SIZE * SIZE * 4) as usize];

    let top = [52_u8, 120_u8, 246_u8, 255_u8];
    let bottom = [31_u8, 188_u8, 156_u8, 255_u8];

    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as f32 - 128.0;
            let dy = y as f32 - 128.0;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= 112.0 {
                let t = y as f32 / (SIZE - 1) as f32;
                let color = [
                    lerp_channel(top[0], bottom[0], t),
                    lerp_channel(top[1], bottom[1], t),
                    lerp_channel(top[2], bottom[2], t),
                    255,
                ];
                blend_pixel(&mut rgba, SIZE, x, y, color);
            }
        }
    }

    let white = [255_u8, 255_u8, 255_u8, 255_u8];
    let points = [
        (75.0_f32, 95.0_f32),
        (180.0_f32, 72.0_f32),
        (190.0_f32, 178.0_f32),
        (88.0_f32, 194.0_f32),
    ];

    draw_line(&mut rgba, SIZE, points[0], points[1], 9.0, white);
    draw_line(&mut rgba, SIZE, points[1], points[2], 9.0, white);
    draw_line(&mut rgba, SIZE, points[2], points[3], 9.0, white);
    draw_line(&mut rgba, SIZE, points[3], points[0], 9.0, white);
    draw_line(&mut rgba, SIZE, points[0], points[2], 9.0, white);

    for point in points {
        draw_circle(&mut rgba, SIZE, point, 18.0, white);
    }

    egui::IconData {
        rgba,
        width: SIZE,
        height: SIZE,
    }
}

fn lerp_channel(start: u8, end: u8, t: f32) -> u8 {
    (start as f32 + (end as f32 - start as f32) * t).round() as u8
}

fn blend_pixel(buffer: &mut [u8], width: u32, x: u32, y: u32, color: [u8; 4]) {
    let index = ((y * width + x) * 4) as usize;
    buffer[index..index + 4].copy_from_slice(&color);
}

fn draw_circle(buffer: &mut [u8], width: u32, center: (f32, f32), radius: f32, color: [u8; 4]) {
    let min_x = (center.0 - radius).floor().max(0.0) as u32;
    let max_x = (center.0 + radius).ceil().min((width - 1) as f32) as u32;
    let min_y = (center.1 - radius).floor().max(0.0) as u32;
    let max_y = (center.1 + radius).ceil().min((width - 1) as f32) as u32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let dx = x as f32 - center.0;
            let dy = y as f32 - center.1;
            if dx * dx + dy * dy <= radius * radius {
                blend_pixel(buffer, width, x, y, color);
            }
        }
    }
}

fn draw_line(
    buffer: &mut [u8],
    width: u32,
    start: (f32, f32),
    end: (f32, f32),
    thickness: f32,
    color: [u8; 4],
) {
    let min_x = start.0.min(end.0).floor().max(0.0) as u32;
    let max_x = start.0.max(end.0).ceil().min((width - 1) as f32) as u32;
    let min_y = start.1.min(end.1).floor().max(0.0) as u32;
    let max_y = start.1.max(end.1).ceil().min((width - 1) as f32) as u32;
    let padding = thickness.ceil() as u32 + 2;

    let min_x = min_x.saturating_sub(padding);
    let min_y = min_y.saturating_sub(padding);
    let max_x = (max_x + padding).min(width - 1);
    let max_y = (max_y + padding).min(width - 1);

    let line_dx = end.0 - start.0;
    let line_dy = end.1 - start.1;
    let line_length_sq = line_dx * line_dx + line_dy * line_dy;

    if line_length_sq == 0.0 {
        draw_circle(buffer, width, start, thickness * 0.5, color);
        return;
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = x as f32;
            let py = y as f32;
            let t = (((px - start.0) * line_dx + (py - start.1) * line_dy) / line_length_sq)
                .clamp(0.0, 1.0);
            let projection_x = start.0 + t * line_dx;
            let projection_y = start.1 + t * line_dy;
            let dx = px - projection_x;
            let dy = py - projection_y;

            if dx * dx + dy * dy <= (thickness * 0.5) * (thickness * 0.5) {
                blend_pixel(buffer, width, x, y, color);
            }
        }
    }
}

fn load_system_ui_fonts() -> Vec<(String, Vec<u8>)> {
    system_ui_font_candidates()
        .iter()
        .filter_map(|(font_name, path)| {
            fs::read(path)
                .ok()
                .map(|data| ((*font_name).to_string(), data))
        })
        .collect()
}

#[cfg(windows)]
fn system_ui_font_candidates() -> &'static [(&'static str, &'static str)] {
    &[
        ("microsoft-yahei", r"C:\Windows\Fonts\msyh.ttc"),
        ("microsoft-yahei-bold", r"C:\Windows\Fonts\msyhbd.ttc"),
        ("simhei", r"C:\Windows\Fonts\simhei.ttf"),
        ("simsun", r"C:\Windows\Fonts\simsun.ttc"),
        ("malgun", r"C:\Windows\Fonts\malgun.ttf"),
        ("malgun-light", r"C:\Windows\Fonts\malgunsl.ttf"),
        ("nanum-gothic", r"C:\Windows\Fonts\NanumGothic.ttf"),
        ("deng", r"C:\Windows\Fonts\Deng.ttf"),
        ("nirmala", r"C:\Windows\Fonts\Nirmala.ttf"),
        ("segoe-ui", r"C:\Windows\Fonts\segoeui.ttf"),
    ]
}

#[cfg(target_os = "macos")]
fn system_ui_font_candidates() -> &'static [(&'static str, &'static str)] {
    &[
        (
            "apple-sd-gothic",
            "/System/Library/Fonts/Supplemental/Apple SD Gothic Neo.ttc",
        ),
        ("pingfang", "/System/Library/Fonts/PingFang.ttc"),
        (
            "hiragino-sans",
            "/System/Library/Fonts/Supplemental/Hiragino Sans GB.ttc",
        ),
        (
            "kohinoor-devanagari",
            "/System/Library/Fonts/Supplemental/Kohinoor Devanagari.ttc",
        ),
        ("sf-pro", "/System/Library/Fonts/SFNS.ttf"),
    ]
}

#[cfg(all(not(windows), not(target_os = "macos")))]
fn system_ui_font_candidates() -> &'static [(&'static str, &'static str)] {
    &[
        (
            "noto-sans-cjk",
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        ),
        (
            "noto-devanagari",
            "/usr/share/fonts/truetype/noto/NotoSansDevanagari-Regular.ttf",
        ),
        (
            "dejavu-sans",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        ),
    ]
}

#[cfg(windows)]
#[repr(C)]
struct IpOptionInformation {
    ttl: u8,
    tos: u8,
    flags: u8,
    options_size: u8,
    options_data: *mut u8,
}

#[cfg(windows)]
#[repr(C)]
struct IcmpEchoReply {
    address: u32,
    status: u32,
    round_trip_time: u32,
    data_size: u16,
    reserved: u16,
    data: *mut c_void,
    options: IpOptionInformation,
}

#[cfg(windows)]
#[link(name = "iphlpapi")]
unsafe extern "system" {
    fn IcmpCreateFile() -> *mut c_void;
    fn IcmpCloseHandle(handle: *mut c_void) -> i32;
    fn IcmpSendEcho(
        handle: *mut c_void,
        destination_address: u32,
        request_data: *const c_void,
        request_size: u16,
        request_options: *const c_void,
        reply_buffer: *mut c_void,
        reply_size: u32,
        timeout: u32,
    ) -> u32;
    fn SendARP(dest_ip: u32, src_ip: u32, mac_addr: *mut c_void, phy_addr_len: *mut u32) -> u32;
}

#[cfg(windows)]
#[derive(Default)]
struct IcmpThreadHandle {
    handle: *mut c_void,
}

#[cfg(windows)]
impl IcmpThreadHandle {
    fn get_or_create(&mut self) -> Option<*mut c_void> {
        if self.handle.is_null() {
            self.handle = unsafe { IcmpCreateFile() };
        }

        (!self.handle.is_null()).then_some(self.handle)
    }
}

#[cfg(windows)]
impl Drop for IcmpThreadHandle {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                let _ = IcmpCloseHandle(self.handle);
            }
        }
    }
}

#[cfg(windows)]
thread_local! {
    static ICMP_THREAD_HANDLE: RefCell<IcmpThreadHandle> = RefCell::new(IcmpThreadHandle::default());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DeviceStatus {
    Pending,
    InUse,
    Available,
}

impl DeviceStatus {
    fn color(self) -> Color32 {
        match self {
            Self::Pending => Color32::from_rgb(0x3A, 0x74, 0xC9),
            Self::InUse => Color32::from_rgb(0x1D, 0x8F, 0x4E),
            Self::Available => Color32::from_rgb(0xB5, 0x8B, 0x00),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Language {
    #[default]
    Korean,
    English,
    ChineseSimplified,
    French,
    German,
    Hindi,
}

impl Language {
    fn all() -> &'static [Language] {
        &[
            Self::Korean,
            Self::English,
            Self::ChineseSimplified,
            Self::French,
            Self::German,
            Self::Hindi,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            Self::Korean => "대한민국 · 한국어",
            Self::English => "United States · English",
            Self::ChineseSimplified => "中国 · 简体中文",
            Self::French => "France · Français",
            Self::German => "Deutschland · Deutsch",
            Self::Hindi => "भारत · हिन्दी",
        }
    }

    fn app_title(self) -> &'static str {
        "IPScanner"
    }

    fn app_subtitle(self) -> &'static str {
        match self {
            Self::Korean => "같은 네트워크 대역의 장비를 빠르게 찾고 이름, 제조사, 결과 파일까지 한 화면에서 관리합니다.",
            Self::English => "A friendlier network scanner for finding active devices, hostnames, vendors, and exportable results in one place.",
            Self::ChineseSimplified => "更友好的局域网扫描器，可在同一界面查看在线设备、主机名、厂商信息并导出结果。",
            Self::French => "Un scanner reseau plus convivial pour reperer les appareils actifs, leurs noms, leurs fabricants et exporter les resultats au meme endroit.",
            Self::German => "Ein benutzerfreundlicherer Netzwerkscanner, um aktive Geraete, Hostnamen, Hersteller und exportierbare Ergebnisse an einem Ort zu sehen.",
            Self::Hindi => "एक अधिक उपयोगकर्ता-अनुकूल नेटवर्क स्कैनर, जिसमें सक्रिय डिवाइस, होस्टनाम, निर्माता और निर्यात किए जा सकने वाले परिणाम एक ही जगह दिखते हैं।",
        }
    }

    fn language_label(self) -> &'static str {
        match self {
            Self::Korean => "국가 / 언어",
            Self::English => "Country / language",
            Self::ChineseSimplified => "国家 / 语言",
            Self::French => "Pays / langue",
            Self::German => "Land / Sprache",
            Self::Hindi => "देश / भाषा",
        }
    }

    fn range_input_label(self) -> &'static str {
        match self {
            Self::Korean => "스캔 범위",
            Self::English => "Scan range",
            Self::ChineseSimplified => "扫描范围",
            Self::French => "Plage d'analyse",
            Self::German => "Scanbereich",
            Self::Hindi => "स्कैन सीमा",
        }
    }

    fn range_hint(self) -> &'static str {
        match self {
            Self::Korean => "예: 192.168.1 또는 192.168.1.1-254",
            Self::English => "Example: 192.168.1 or 192.168.1.1-254",
            Self::ChineseSimplified => "例如：192.168.1 或 192.168.1.1-254",
            Self::French => "Exemple : 192.168.1 ou 192.168.1.1-254",
            Self::German => "Beispiel: 192.168.1 oder 192.168.1.1-254",
            Self::Hindi => "उदाहरण: 192.168.1 या 192.168.1.1-254",
        }
    }

    fn presets_label(self) -> &'static str {
        match self {
            Self::Korean => "추천 범위",
            Self::English => "Suggested range",
            Self::ChineseSimplified => "推荐范围",
            Self::French => "Plage suggeree",
            Self::German => "Empfohlener Bereich",
            Self::Hindi => "सुझाई गई सीमा",
        }
    }

    fn device_names_label(self) -> &'static str {
        match self {
            Self::Korean => "기기 이름도 찾기 (조금 느림)",
            Self::English => "Look up device names too (slower)",
            Self::ChineseSimplified => "同时查找设备名称（较慢）",
            Self::French => "Rechercher aussi les noms d'appareil (plus lent)",
            Self::German => "Auch Geraetenamen aufloesen (langsamer)",
            Self::Hindi => "डिवाइस नाम भी खोजें (थोड़ा धीमा)",
        }
    }

    fn view_mode_label(self) -> &'static str {
        match self {
            Self::Korean => "보기 방식",
            Self::English => "View mode",
            Self::ChineseSimplified => "视图模式",
            Self::French => "Mode d'affichage",
            Self::German => "Ansicht",
            Self::Hindi => "दृश्य मोड",
        }
    }

    fn map_view_label(self) -> &'static str {
        match self {
            Self::Korean => "한눈에 보기",
            Self::English => "Map view",
            Self::ChineseSimplified => "网格视图",
            Self::French => "Vue grille",
            Self::German => "Rasteransicht",
            Self::Hindi => "मैप दृश्य",
        }
    }

    fn table_view_label(self) -> &'static str {
        match self {
            Self::Korean => "목록 보기",
            Self::English => "Table view",
            Self::ChineseSimplified => "列表视图",
            Self::French => "Vue tableau",
            Self::German => "Tabellenansicht",
            Self::Hindi => "तालिका दृश्य",
        }
    }

    fn start_scan_label(self) -> &'static str {
        match self {
            Self::Korean => "스캔 시작",
            Self::English => "Start scan",
            Self::ChineseSimplified => "开始扫描",
            Self::French => "Demarrer l'analyse",
            Self::German => "Scan starten",
            Self::Hindi => "स्कैन शुरू करें",
        }
    }

    fn stop_scan_label(self) -> &'static str {
        match self {
            Self::Korean => "스캔 중지",
            Self::English => "Stop scan",
            Self::ChineseSimplified => "停止扫描",
            Self::French => "Arreter l'analyse",
            Self::German => "Scan stoppen",
            Self::Hindi => "स्कैन रोकें",
        }
    }

    fn export_results_label(self) -> &'static str {
        match self {
            Self::Korean => "결과 저장",
            Self::English => "Export results",
            Self::ChineseSimplified => "导出结果",
            Self::French => "Exporter les resultats",
            Self::German => "Ergebnisse exportieren",
            Self::Hindi => "परिणाम निर्यात करें",
        }
    }

    fn idle_status(self) -> &'static str {
        match self {
            Self::Korean => "스캔할 범위를 입력하고 시작해 보세요.",
            Self::English => "Choose a range and start scanning when you're ready.",
            Self::ChineseSimplified => "选择一个范围，然后开始扫描。",
            Self::French => "Choisissez une plage puis lancez l'analyse.",
            Self::German => "Waehlen Sie einen Bereich und starten Sie dann den Scan.",
            Self::Hindi => "एक सीमा चुनें और फिर स्कैन शुरू करें।",
        }
    }

    fn stop_requested_status(self) -> &'static str {
        match self {
            Self::Korean => "스캔 중지를 요청했습니다...",
            Self::English => "Stopping the scan...",
            Self::ChineseSimplified => "正在停止扫描...",
            Self::French => "Arret de l'analyse...",
            Self::German => "Scan wird gestoppt...",
            Self::Hindi => "स्कैन रोका जा रहा है...",
        }
    }

    fn no_targets_status(self) -> &'static str {
        match self {
            Self::Korean => "스캔할 대상이 없습니다.",
            Self::English => "There are no targets to scan.",
            Self::ChineseSimplified => "没有可扫描的目标。",
            Self::French => "Aucune cible a analyser.",
            Self::German => "Es gibt keine Ziele zum Scannen.",
            Self::Hindi => "स्कैन करने के लिए कोई लक्ष्य नहीं है।",
        }
    }

    fn scan_started_status(self, total_targets: usize) -> String {
        match self {
            Self::Korean => format!("{total_targets}개 주소를 검사하는 중입니다..."),
            Self::English => format!("Scanning {total_targets} addresses..."),
            Self::ChineseSimplified => format!("正在扫描 {total_targets} 个地址..."),
            Self::French => format!("Analyse de {total_targets} adresses..."),
            Self::German => format!("{total_targets} Adressen werden gescannt..."),
            Self::Hindi => format!("{total_targets} पतों को स्कैन किया जा रहा है..."),
        }
    }

    fn scanner_thread_pool_error(self, err: &str) -> String {
        match self {
            Self::Korean => format!("스캐너 스레드 풀을 만들지 못했습니다: {err}"),
            Self::English => format!("Couldn't create the scan worker pool: {err}"),
            Self::ChineseSimplified => format!("无法创建扫描线程池：{err}"),
            Self::French => format!("Impossible de creer le pool de threads d'analyse : {err}"),
            Self::German => format!("Der Scan-Thread-Pool konnte nicht erstellt werden: {err}"),
            Self::Hindi => format!("स्कैन वर्कर पूल नहीं बनाया जा सका: {err}"),
        }
    }

    fn scan_cancelled_status(self, processed: usize, total_targets: usize) -> String {
        match self {
            Self::Korean => format!("스캔이 중지되었습니다. 완료 {processed} / {total_targets}"),
            Self::English => {
                format!("Scan stopped. Finished {processed} of {total_targets} targets.")
            }
            Self::ChineseSimplified => format!("扫描已停止。已完成 {processed} / {total_targets}"),
            Self::French => {
                format!("Analyse arretee. {processed} cibles terminees sur {total_targets}.")
            }
            Self::German => {
                format!("Scan gestoppt. {processed} von {total_targets} Zielen abgeschlossen.")
            }
            Self::Hindi => format!("स्कैन रोक दिया गया। {total_targets} में से {processed} पूरे हुए।"),
        }
    }

    fn scan_completed_status(self, in_use: usize, total_targets: usize) -> String {
        match self {
            Self::Korean => format!("스캔 완료: 사용 중 {in_use} / 전체 {total_targets}"),
            Self::English => {
                format!("Scan complete: {in_use} active out of {total_targets} targets.")
            }
            Self::ChineseSimplified => {
                format!("扫描完成：活跃设备 {in_use} / 总数 {total_targets}")
            }
            Self::French => {
                format!("Analyse terminee : {in_use} appareils actifs sur {total_targets}.")
            }
            Self::German => {
                format!("Scan abgeschlossen: {in_use} aktive Geraete von {total_targets} Zielen.")
            }
            Self::Hindi => format!("स्कैन पूरा हुआ: {total_targets} में से {in_use} सक्रिय डिवाइस।"),
        }
    }

    fn save_results_title(self) -> &'static str {
        match self {
            Self::Korean => "스캔 결과 저장",
            Self::English => "Save scan results",
            Self::ChineseSimplified => "保存扫描结果",
            Self::French => "Enregistrer les resultats",
            Self::German => "Scannergebnisse speichern",
            Self::Hindi => "स्कैन परिणाम सहेजें",
        }
    }

    fn saved_results_status(self, path: &Path) -> String {
        match self {
            Self::Korean => format!("결과를 저장했습니다: {}", path.display()),
            Self::English => format!("Results saved to {}", path.display()),
            Self::ChineseSimplified => format!("结果已保存到 {}", path.display()),
            Self::French => format!("Resultats enregistres dans {}", path.display()),
            Self::German => format!("Ergebnisse gespeichert unter {}", path.display()),
            Self::Hindi => format!("परिणाम यहां सहेजे गए: {}", path.display()),
        }
    }

    fn save_results_failed_status(self, err: &str) -> String {
        match self {
            Self::Korean => format!("결과 저장 실패: {err}"),
            Self::English => format!("Couldn't save the results: {err}"),
            Self::ChineseSimplified => format!("无法保存结果：{err}"),
            Self::French => format!("Impossible d'enregistrer les resultats : {err}"),
            Self::German => format!("Ergebnisse konnten nicht gespeichert werden: {err}"),
            Self::Hindi => format!("परिणाम सहेजे नहीं जा सके: {err}"),
        }
    }

    fn settings_save_failed_status(self, err: &str) -> String {
        match self {
            Self::Korean => format!("설정 저장 실패: {err}"),
            Self::English => format!("Couldn't save your settings: {err}"),
            Self::ChineseSimplified => format!("无法保存设置：{err}"),
            Self::French => format!("Impossible d'enregistrer les parametres : {err}"),
            Self::German => format!("Einstellungen konnten nicht gespeichert werden: {err}"),
            Self::Hindi => format!("आपकी सेटिंग्स सहेजी नहीं जा सकीं: {err}"),
        }
    }

    fn scan_summary(self, in_use: usize, total_targets: usize, processed: usize) -> String {
        match self {
            Self::Korean => format!("사용 중 {in_use} / 전체 {total_targets} / 진행 {processed}"),
            Self::English => {
                format!("Active {in_use} / Total {total_targets} / Processed {processed}")
            }
            Self::ChineseSimplified => {
                format!("活跃 {in_use} / 总数 {total_targets} / 已处理 {processed}")
            }
            Self::French => {
                format!("Actifs {in_use} / Total {total_targets} / Traites {processed}")
            }
            Self::German => {
                format!("Aktiv {in_use} / Gesamt {total_targets} / Verarbeitet {processed}")
            }
            Self::Hindi => format!("सक्रिय {in_use} / कुल {total_targets} / संसाधित {processed}"),
        }
    }

    fn status_field_label(self) -> &'static str {
        match self {
            Self::Korean => "상태",
            Self::English => "Status",
            Self::ChineseSimplified => "状态",
            Self::French => "Etat",
            Self::German => "Status",
            Self::Hindi => "स्थिति",
        }
    }

    fn ip_field_label(self) -> &'static str {
        match self {
            Self::Korean => "IP 주소",
            Self::English => "IP address",
            Self::ChineseSimplified => "IP 地址",
            Self::French => "Adresse IP",
            Self::German => "IP-Adresse",
            Self::Hindi => "IP पता",
        }
    }

    fn mac_field_label(self) -> &'static str {
        match self {
            Self::Korean => "MAC 주소",
            Self::English => "MAC address",
            Self::ChineseSimplified => "MAC 地址",
            Self::French => "Adresse MAC",
            Self::German => "MAC-Adresse",
            Self::Hindi => "MAC पता",
        }
    }

    fn vendor_field_label(self) -> &'static str {
        match self {
            Self::Korean => "제조사",
            Self::English => "Vendor",
            Self::ChineseSimplified => "厂商",
            Self::French => "Fabricant",
            Self::German => "Hersteller",
            Self::Hindi => "निर्माता",
        }
    }

    fn hostname_field_label(self) -> &'static str {
        match self {
            Self::Korean => "기기 이름",
            Self::English => "Hostname",
            Self::ChineseSimplified => "主机名",
            Self::French => "Nom d'hote",
            Self::German => "Hostname",
            Self::Hindi => "होस्टनाम",
        }
    }

    fn selected_device_label(self) -> &'static str {
        match self {
            Self::Korean => "선택한 장비",
            Self::English => "Selected device",
            Self::ChineseSimplified => "已选择的设备",
            Self::French => "Appareil selectionne",
            Self::German => "Ausgewaehltes Geraet",
            Self::Hindi => "चयनित डिवाइस",
        }
    }

    fn map_legend_pending(self) -> &'static str {
        match self {
            Self::Korean => "확인 중",
            Self::English => "Checking",
            Self::ChineseSimplified => "检查中",
            Self::French => "Verification",
            Self::German => "Pruefung",
            Self::Hindi => "जांच जारी",
        }
    }

    fn map_legend_in_use(self) -> &'static str {
        match self {
            Self::Korean => "사용 중",
            Self::English => "In use",
            Self::ChineseSimplified => "使用中",
            Self::French => "Actif",
            Self::German => "Belegt",
            Self::Hindi => "उपयोग में",
        }
    }

    fn map_legend_available(self) -> &'static str {
        match self {
            Self::Korean => "사용 가능",
            Self::English => "Available",
            Self::ChineseSimplified => "可用",
            Self::French => "Disponible",
            Self::German => "Verfuegbar",
            Self::Hindi => "उपलब्ध",
        }
    }

    fn empty_title(self) -> &'static str {
        match self {
            Self::Korean => "스캔 준비가 되었습니다",
            Self::English => "Ready to scan your network",
            Self::ChineseSimplified => "已准备好开始扫描网络",
            Self::French => "Pret a analyser votre reseau",
            Self::German => "Bereit fuer den Netzwerkscan",
            Self::Hindi => "आपका नेटवर्क स्कैन करने के लिए तैयार",
        }
    }

    fn empty_body(self) -> &'static str {
        match self {
            Self::Korean => "추천 범위를 선택하거나 직접 입력한 뒤 시작 버튼을 누르면 현재 사용 중인 장비를 보기 좋게 정리해 드립니다.",
            Self::English => "Pick a suggested range or type your own, then start scanning to see active devices in a cleaner, easier-to-read view.",
            Self::ChineseSimplified => "选择推荐范围或输入自定义范围，然后开始扫描，以更清晰的方式查看活跃设备。",
            Self::French => "Choisissez une plage suggeree ou saisissez la votre, puis lancez l'analyse pour voir les appareils actifs dans une vue plus claire.",
            Self::German => "Waehlen Sie einen vorgeschlagenen Bereich oder geben Sie einen eigenen ein, um aktive Geraete in einer uebersichtlicheren Ansicht zu sehen.",
            Self::Hindi => "सुझाई गई सीमा चुनें या अपनी सीमा लिखें, फिर सक्रिय डिवाइस को अधिक साफ तरीके से देखने के लिए स्कैन शुरू करें।",
        }
    }

    fn empty_tip(self) -> &'static str {
        match self {
            Self::Korean => "입력 예시: 192.168.1, 192.168.1.1-254, 192.168.1.0/24",
            Self::English => "Examples: 192.168.1, 192.168.1.1-254, 192.168.1.0/24",
            Self::ChineseSimplified => "示例：192.168.1、192.168.1.1-254、192.168.1.0/24",
            Self::French => "Exemples : 192.168.1, 192.168.1.1-254, 192.168.1.0/24",
            Self::German => "Beispiele: 192.168.1, 192.168.1.1-254, 192.168.1.0/24",
            Self::Hindi => "उदाहरण: 192.168.1, 192.168.1.1-254, 192.168.1.0/24",
        }
    }

    fn apply_detected_range_label(self) -> &'static str {
        match self {
            Self::Korean => "추천 범위 적용",
            Self::English => "Use suggested range",
            Self::ChineseSimplified => "使用推荐范围",
            Self::French => "Utiliser la plage suggeree",
            Self::German => "Empfohlenen Bereich verwenden",
            Self::Hindi => "सुझाई गई सीमा का उपयोग करें",
        }
    }

    fn summary_total_label(self) -> &'static str {
        match self {
            Self::Korean => "전체 주소",
            Self::English => "Total targets",
            Self::ChineseSimplified => "目标总数",
            Self::French => "Cibles totales",
            Self::German => "Gesamtziele",
            Self::Hindi => "कुल लक्ष्य",
        }
    }

    fn summary_active_label(self) -> &'static str {
        match self {
            Self::Korean => "사용 중 장비",
            Self::English => "Active devices",
            Self::ChineseSimplified => "活跃设备",
            Self::French => "Appareils actifs",
            Self::German => "Aktive Geraete",
            Self::Hindi => "सक्रिय डिवाइस",
        }
    }

    fn summary_available_label(self) -> &'static str {
        match self {
            Self::Korean => "비어 있는 주소",
            Self::English => "Available addresses",
            Self::ChineseSimplified => "可用地址",
            Self::French => "Adresses disponibles",
            Self::German => "Verfuegbare Adressen",
            Self::Hindi => "उपलब्ध पते",
        }
    }

    fn summary_progress_label(self) -> &'static str {
        match self {
            Self::Korean => "진행 상황",
            Self::English => "Progress",
            Self::ChineseSimplified => "进度",
            Self::French => "Progression",
            Self::German => "Fortschritt",
            Self::Hindi => "प्रगति",
        }
    }

    fn summary_progress_detail(self, processed: usize, total_targets: usize) -> String {
        match self {
            Self::Korean => format!("{processed} / {total_targets} 완료"),
            Self::English => format!("{processed} / {total_targets} processed"),
            Self::ChineseSimplified => format!("已处理 {processed} / {total_targets}"),
            Self::French => format!("{processed} / {total_targets} traites"),
            Self::German => format!("{processed} / {total_targets} verarbeitet"),
            Self::Hindi => format!("{processed} / {total_targets} संसाधित"),
        }
    }

    fn summary_active_detail(self, total_targets: usize) -> String {
        match self {
            Self::Korean => format!("대상 {total_targets}개 중 응답함"),
            Self::English => format!("Responding devices out of {total_targets}"),
            Self::ChineseSimplified => format!("{total_targets} 个目标中有响应的设备"),
            Self::French => format!("Appareils repondant sur {total_targets} cibles"),
            Self::German => format!("Antwortende Geraete von {total_targets} Zielen"),
            Self::Hindi => format!("{total_targets} लक्ष्यों में प्रतिक्रिया देने वाले डिवाइस"),
        }
    }

    fn summary_available_detail(self) -> &'static str {
        match self {
            Self::Korean => "현재 응답이 없거나 비어 있는 주소",
            Self::English => "Addresses with no current response",
            Self::ChineseSimplified => "当前没有响应的地址",
            Self::French => "Adresses sans reponse actuelle",
            Self::German => "Adressen ohne aktuelle Antwort",
            Self::Hindi => "वे पते जिनसे अभी कोई प्रतिक्रिया नहीं मिली",
        }
    }

    fn summary_total_detail(self) -> &'static str {
        match self {
            Self::Korean => "이번 스캔에 포함된 범위",
            Self::English => "Addresses included in this scan",
            Self::ChineseSimplified => "本次扫描包含的地址",
            Self::French => "Adresses incluses dans cette analyse",
            Self::German => "Adressen in diesem Scan",
            Self::Hindi => "इस स्कैन में शामिल पते",
        }
    }

    fn no_value(self) -> &'static str {
        "-"
    }

    fn parse_targets_required_error(self) -> String {
        match self {
            Self::Korean => "IP 범위를 입력하세요.".to_string(),
            Self::English => "Enter an IP range to scan.".to_string(),
            Self::ChineseSimplified => "请输入要扫描的 IP 范围。".to_string(),
            Self::French => "Saisissez une plage IP a analyser.".to_string(),
            Self::German => "Geben Sie einen IP-Bereich zum Scannen ein.".to_string(),
            Self::Hindi => "स्कैन करने के लिए एक IP सीमा दर्ज करें।".to_string(),
        }
    }

    fn parse_targets_ipv4_only_error(self) -> String {
        match self {
            Self::Korean => "IPv4 대역만 지원합니다.".to_string(),
            Self::English => "Only IPv4 ranges are supported.".to_string(),
            Self::ChineseSimplified => "仅支持 IPv4 范围。".to_string(),
            Self::French => "Seules les plages IPv4 sont prises en charge.".to_string(),
            Self::German => "Es werden nur IPv4-Bereiche unterstuetzt.".to_string(),
            Self::Hindi => "केवल IPv4 सीमाएं समर्थित हैं।".to_string(),
        }
    }

    fn parse_targets_cidr_empty_error(self) -> String {
        match self {
            Self::Korean => "CIDR 범위에서 스캔할 IPv4 호스트가 없습니다.".to_string(),
            Self::English => {
                "That CIDR range doesn't contain any scannable IPv4 hosts.".to_string()
            }
            Self::ChineseSimplified => "该 CIDR 范围中没有可扫描的 IPv4 主机。".to_string(),
            Self::French => "Cette plage CIDR ne contient aucun hote IPv4 analysable.".to_string(),
            Self::German => "Dieser CIDR-Bereich enthaelt keine scanbaren IPv4-Hosts.".to_string(),
            Self::Hindi => "इस CIDR सीमा में स्कैन करने योग्य कोई IPv4 होस्ट नहीं है।".to_string(),
        }
    }

    fn parse_targets_invalid_start_ip_error(self) -> String {
        match self {
            Self::Korean => "시작 IP 형식이 올바르지 않습니다.".to_string(),
            Self::English => "The starting IP address isn't valid.".to_string(),
            Self::ChineseSimplified => "起始 IP 地址无效。".to_string(),
            Self::French => "L'adresse IP de debut n'est pas valide.".to_string(),
            Self::German => "Die Start-IP-Adresse ist ungueltig.".to_string(),
            Self::Hindi => "शुरुआती IP पता मान्य नहीं है।".to_string(),
        }
    }

    fn parse_targets_invalid_end_ip_error(self) -> String {
        match self {
            Self::Korean => "종료 IP 형식이 올바르지 않습니다.".to_string(),
            Self::English => "The ending IP address isn't valid.".to_string(),
            Self::ChineseSimplified => "结束 IP 地址无效。".to_string(),
            Self::French => "L'adresse IP de fin n'est pas valide.".to_string(),
            Self::German => "Die End-IP-Adresse ist ungueltig.".to_string(),
            Self::Hindi => "अंतिम IP पता मान्य नहीं है।".to_string(),
        }
    }

    fn parse_targets_invalid_last_octet_error(self) -> String {
        match self {
            Self::Korean => "끝 범위는 마지막 옥텟(0-255) 또는 전체 IP여야 합니다.".to_string(),
            Self::English => "The range end must be a last octet (0-255) or a full IP address.".to_string(),
            Self::ChineseSimplified => "范围结束值必须是最后一个八位字节（0-255）或完整 IP 地址。".to_string(),
            Self::French => "La fin de plage doit etre un dernier octet (0-255) ou une adresse IP complete.".to_string(),
            Self::German => "Das Bereichsende muss ein letztes Oktett (0-255) oder eine vollstaendige IP-Adresse sein.".to_string(),
            Self::Hindi => "सीमा का अंत 0-255 का अंतिम ऑक्टेट या पूरा IP पता होना चाहिए।".to_string(),
        }
    }

    fn parse_targets_supported_formats_error(self) -> String {
        match self {
            Self::Korean => "지원 형식: 192.168.1, 192.168.1.1-254, 192.168.1.1-192.168.1.50, 192.168.1.0/24".to_string(),
            Self::English => "Supported formats: 192.168.1, 192.168.1.1-254, 192.168.1.1-192.168.1.50, 192.168.1.0/24".to_string(),
            Self::ChineseSimplified => "支持的格式：192.168.1、192.168.1.1-254、192.168.1.1-192.168.1.50、192.168.1.0/24".to_string(),
            Self::French => "Formats pris en charge : 192.168.1, 192.168.1.1-254, 192.168.1.1-192.168.1.50, 192.168.1.0/24".to_string(),
            Self::German => "Unterstuetzte Formate: 192.168.1, 192.168.1.1-254, 192.168.1.1-192.168.1.50, 192.168.1.0/24".to_string(),
            Self::Hindi => "समर्थित प्रारूप: 192.168.1, 192.168.1.1-254, 192.168.1.1-192.168.1.50, 192.168.1.0/24".to_string(),
        }
    }

    fn expand_range_invalid_order_error(self) -> String {
        match self {
            Self::Korean => "시작 IP가 종료 IP보다 클 수 없습니다.".to_string(),
            Self::English => {
                "The starting IP address can't be greater than the ending IP address.".to_string()
            }
            Self::ChineseSimplified => "起始 IP 地址不能大于结束 IP 地址。".to_string(),
            Self::French => {
                "L'adresse IP de debut ne peut pas etre superieure a l'adresse IP de fin."
                    .to_string()
            }
            Self::German => {
                "Die Start-IP-Adresse darf nicht groesser als die End-IP-Adresse sein.".to_string()
            }
            Self::Hindi => "शुरुआती IP पता अंतिम IP पते से बड़ा नहीं हो सकता।".to_string(),
        }
    }

    fn expand_range_too_large_error(self) -> String {
        match self {
            Self::Korean => "한 번에 최대 65,536개 IP까지만 스캔할 수 있습니다.".to_string(),
            Self::English => "You can scan up to 65,536 IP addresses at a time.".to_string(),
            Self::ChineseSimplified => "一次最多只能扫描 65,536 个 IP 地址。".to_string(),
            Self::French => {
                "Vous pouvez analyser jusqu'a 65 536 adresses IP a la fois.".to_string()
            }
            Self::German => "Sie koennen bis zu 65.536 IP-Adressen auf einmal scannen.".to_string(),
            Self::Hindi => "आप एक बार में अधिकतम 65,536 IP पते स्कैन कर सकते हैं।".to_string(),
        }
    }

    fn export_headers(self) -> [&'static str; 5] {
        [
            self.ip_field_label(),
            self.status_field_label(),
            self.mac_field_label(),
            self.vendor_field_label(),
            self.hostname_field_label(),
        ]
    }
}

impl DeviceStatus {
    fn label(self, language: Language) -> &'static str {
        match (self, language) {
            (Self::Pending, Language::Korean) => "확인중",
            (Self::InUse, Language::Korean) => "사용중",
            (Self::Available, Language::Korean) => "사용가능",
            (Self::Pending, Language::English) => "Checking",
            (Self::InUse, Language::English) => "In use",
            (Self::Available, Language::English) => "Available",
            (Self::Pending, Language::ChineseSimplified) => "检查中",
            (Self::InUse, Language::ChineseSimplified) => "使用中",
            (Self::Available, Language::ChineseSimplified) => "可用",
            (Self::Pending, Language::French) => "Verification",
            (Self::InUse, Language::French) => "Actif",
            (Self::Available, Language::French) => "Disponible",
            (Self::Pending, Language::German) => "Pruefung",
            (Self::InUse, Language::German) => "Belegt",
            (Self::Available, Language::German) => "Verfuegbar",
            (Self::Pending, Language::Hindi) => "जांच जारी",
            (Self::InUse, Language::Hindi) => "उपयोग में",
            (Self::Available, Language::Hindi) => "उपलब्ध",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SortColumn {
    Ip,
    Status,
    Mac,
    Vendor,
    Hostname,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum ResultView {
    Map,
    Table,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
struct AppSettings {
    last_range: String,
    result_view: ResultView,
    resolve_hostnames: bool,
    language: Language,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            last_range: String::new(),
            result_view: ResultView::Map,
            resolve_hostnames: false,
            language: Language::Korean,
        }
    }
}

#[derive(Clone, Debug)]
struct ScanRecord {
    ip: Ipv4Addr,
    status: DeviceStatus,
    mac: Option<String>,
    vendor: Option<String>,
    hostname: Option<String>,
}

enum WorkerMessage {
    Record(ScanRecord),
    HostnameResolved {
        ip: Ipv4Addr,
        hostname: Option<String>,
    },
    Finished {
        cancelled: bool,
    },
    Error(String),
}

struct FaIpScannerApp {
    language: Language,
    ip_range_input: String,
    range_presets: Vec<String>,
    records: Vec<ScanRecord>,
    status_line: String,
    total_targets: usize,
    receiver: Option<Receiver<WorkerMessage>>,
    cancel_flag: Option<Arc<AtomicBool>>,
    processed_count: Arc<AtomicUsize>,
    vendor_db: Arc<HashMap<String, String>>,
    is_scanning: bool,
    sort_column: SortColumn,
    sort_ascending: bool,
    result_view: ResultView,
    resolve_hostnames: bool,
    selected_ip: Option<Ipv4Addr>,
}

impl Default for FaIpScannerApp {
    fn default() -> Self {
        let range_presets = discover_range_presets();
        let saved = load_app_settings();
        let ip_range_input = if !saved.last_range.trim().is_empty() {
            saved.last_range
        } else {
            range_presets
                .first()
                .cloned()
                .unwrap_or_else(|| "192.168.1.1-254".to_string())
        };

        Self {
            language: saved.language,
            ip_range_input,
            range_presets,
            records: Vec::new(),
            status_line: saved.language.idle_status().to_string(),
            total_targets: 0,
            receiver: None,
            cancel_flag: None,
            processed_count: Arc::new(AtomicUsize::new(0)),
            vendor_db: Arc::new(load_vendor_database()),
            is_scanning: false,
            sort_column: SortColumn::Ip,
            sort_ascending: true,
            result_view: saved.result_view,
            resolve_hostnames: saved.resolve_hostnames,
            selected_ip: None,
        }
    }
}

impl App for FaIpScannerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.drain_worker_messages();

        if self.is_scanning {
            ctx.request_repaint_after(Duration::from_millis(100));
        }

        egui::TopBottomPanel::top("toolbar")
            .resizable(false)
            .show(ctx, |ui| {
                let previous_language = self.language;
                let mut settings_changed = false;
                ui.add_space(10.0);

                show_subdued_panel(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.vertical(|ui| {
                            ui.heading(self.language.app_title());
                            ui.label(self.language.app_subtitle());
                        });

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            egui::ComboBox::from_id_salt("language-selector")
                                .selected_text(self.language.label())
                                .width(140.0)
                                .show_ui(ui, |ui| {
                                    for language in Language::all() {
                                        settings_changed |= ui
                                            .selectable_value(
                                                &mut self.language,
                                                *language,
                                                language.label(),
                                            )
                                            .changed();
                                    }
                                });
                            ui.label(self.language.language_label());
                        });
                    });
                });

                ui.add_space(8.0);

                show_subdued_panel(ui, |ui| {
                    self.active_controls(ui, &mut settings_changed);
                });

                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    let total_targets = self.scan_target_count();
                    self.render_summary_card(
                        ui,
                        self.language.summary_total_label(),
                        total_targets.to_string(),
                        self.language.summary_total_detail(),
                    );
                    self.render_summary_card(
                        ui,
                        self.language.summary_active_label(),
                        self.active_count().to_string(),
                        &self.language.summary_active_detail(total_targets),
                    );
                    self.render_summary_card(
                        ui,
                        self.language.summary_available_label(),
                        self.available_count().to_string(),
                        self.language.summary_available_detail(),
                    );
                    self.render_summary_card(
                        ui,
                        self.language.summary_progress_label(),
                        self.processed_count().to_string(),
                        &self
                            .language
                            .summary_progress_detail(self.processed_count(), total_targets),
                    );
                });
                ui.add_space(6.0);

                if settings_changed {
                    if self.language != previous_language && !self.is_scanning {
                        self.status_line = if self.records.is_empty() {
                            self.language.idle_status().to_string()
                        } else {
                            self.language.scan_completed_status(
                                self.active_count(),
                                self.scan_target_count(),
                            )
                        };
                    }
                    self.persist_settings();
                }
            });

        egui::TopBottomPanel::bottom("status_bar")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(&self.status_line);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(self.language.scan_summary(
                            self.active_count(),
                            self.scan_target_count(),
                            self.processed_count(),
                        ));
                    });
                });
                ui.add_space(6.0);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.records.is_empty() {
                self.render_empty_state(ui);
            } else {
                self.render_legend(ui);
                ui.add_space(8.0);
                match self.result_view {
                    ResultView::Map => self.render_map_view(ui),
                    ResultView::Table => self.render_table_view(ui),
                }
            }
        });
    }
}

impl FaIpScannerApp {
    fn scan_target_count(&self) -> usize {
        self.total_targets.max(self.records.len())
    }

    fn active_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.status == DeviceStatus::InUse)
            .count()
    }

    fn available_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.status == DeviceStatus::Available)
            .count()
    }

    fn processed_count(&self) -> usize {
        self.processed_count.load(Ordering::Relaxed)
    }

    fn render_summary_card(&self, ui: &mut egui::Ui, title: &str, value: String, detail: &str) {
        egui::Frame::new()
            .fill(Color32::from_rgb(247, 249, 252))
            .stroke(egui::Stroke::new(1.0, Color32::from_rgb(225, 231, 240)))
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.set_min_width(190.0);
                ui.label(RichText::new(title).color(Color32::from_rgb(88, 102, 126)));
                ui.label(RichText::new(value).size(22.0).strong());
                ui.label(
                    RichText::new(detail)
                        .small()
                        .color(Color32::from_rgb(112, 122, 140)),
                );
            });
    }

    fn start_scan_button(&self) -> egui::Button<'static> {
        egui::Button::new(
            RichText::new(self.language.start_scan_label())
                .strong()
                .color(Color32::WHITE),
        )
        .fill(Color32::from_rgb(41, 98, 255))
        .stroke(egui::Stroke::NONE)
        .min_size(egui::vec2(140.0, 36.0))
    }

    fn stop_scan_button(&self) -> egui::Button<'static> {
        egui::Button::new(self.language.stop_scan_label())
            .fill(Color32::from_rgb(245, 247, 250))
            .stroke(egui::Stroke::new(1.0, Color32::from_rgb(212, 219, 230)))
            .min_size(egui::vec2(110.0, 36.0))
    }

    fn export_results_button(&self) -> egui::Button<'static> {
        egui::Button::new(self.language.export_results_label())
            .fill(Color32::from_rgb(245, 247, 250))
            .stroke(egui::Stroke::new(1.0, Color32::from_rgb(212, 219, 230)))
            .min_size(egui::vec2(120.0, 36.0))
    }

    fn view_toggle_button(&self, view: ResultView, label: &str) -> egui::Button<'static> {
        let selected = self.result_view == view;
        let fill = if selected {
            Color32::from_rgb(227, 238, 255)
        } else {
            Color32::from_rgb(245, 247, 250)
        };
        let stroke = if selected {
            egui::Stroke::new(1.0, Color32::from_rgb(80, 127, 230))
        } else {
            egui::Stroke::new(1.0, Color32::from_rgb(212, 219, 230))
        };

        egui::Button::new(label).fill(fill).stroke(stroke)
    }
}

impl FaIpScannerApp {
    fn active_controls(&mut self, ui: &mut egui::Ui, settings_changed: &mut bool) {
        ui.horizontal_wrapped(|ui| {
            ui.label(self.language.range_input_label());
            let response = ui.add_sized(
                [340.0, 30.0],
                TextEdit::singleline(&mut self.ip_range_input)
                    .hint_text(self.language.range_hint()),
            );
            *settings_changed |= response.changed();
            if response.lost_focus() {
                self.ip_range_input = normalize_range_input(&self.ip_range_input);
                *settings_changed = true;
            }

            egui::ComboBox::from_id_salt("range-presets")
                .selected_text(self.language.presets_label())
                .width(170.0)
                .show_ui(ui, |ui| {
                    for preset in &self.range_presets {
                        if ui.selectable_label(false, preset).clicked() {
                            self.ip_range_input = preset.clone();
                            *settings_changed = true;
                        }
                    }
                });

            *settings_changed |= ui
                .checkbox(
                    &mut self.resolve_hostnames,
                    self.language.device_names_label(),
                )
                .changed();

            ui.separator();
            ui.label(self.language.view_mode_label());
            let map_clicked = ui
                .add(self.view_toggle_button(ResultView::Map, self.language.map_view_label()))
                .clicked();
            let table_clicked = ui
                .add(self.view_toggle_button(ResultView::Table, self.language.table_view_label()))
                .clicked();
            if map_clicked && self.result_view != ResultView::Map {
                self.result_view = ResultView::Map;
                *settings_changed = true;
            }
            if table_clicked && self.result_view != ResultView::Table {
                self.result_view = ResultView::Table;
                *settings_changed = true;
            }

            let can_start = !self.is_scanning;
            if ui
                .add_enabled(can_start, self.start_scan_button())
                .clicked()
            {
                self.start_scan();
            }

            if ui
                .add_enabled(self.is_scanning, self.stop_scan_button())
                .clicked()
            {
                if let Some(flag) = &self.cancel_flag {
                    flag.store(true, Ordering::Relaxed);
                    self.status_line = self.language.stop_requested_status().to_string();
                }
            }

            if ui
                .add_enabled(!self.records.is_empty(), self.export_results_button())
                .clicked()
            {
                self.save_results();
            }
        });
    }

    fn render_legend(&self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            legend_chip(
                ui,
                map_cell_color(DeviceStatus::Pending),
                self.language.map_legend_pending(),
            );
            legend_chip(
                ui,
                map_cell_color(DeviceStatus::InUse),
                self.language.map_legend_in_use(),
            );
            legend_chip(
                ui,
                map_cell_color(DeviceStatus::Available),
                self.language.map_legend_available(),
            );
        });
    }

    fn render_empty_state(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(80.0);
            ui.heading(self.language.empty_title());
            ui.add_space(8.0);
            ui.label(self.language.empty_body());
            ui.label(self.language.empty_tip());
            ui.add_space(12.0);

            if let Some(preset) = self.range_presets.first().cloned() {
                if ui
                    .button(format!(
                        "{}: {}",
                        self.language.apply_detected_range_label(),
                        preset
                    ))
                    .clicked()
                {
                    self.ip_range_input = preset;
                    self.persist_settings();
                }
            }
        });
    }

    fn render_map_view(&mut self, ui: &mut egui::Ui) {
        let cell_width = 78.0;
        let cell_height = 50.0;
        let cell_spacing = egui::vec2(6.0, 6.0);
        let content_padding = egui::vec2(2.0, 2.0);
        let mut clicked_ip = None;

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(content_padding.y);
                ui.horizontal(|ui| {
                    ui.add_space(content_padding.x);

                    ui.vertical(|ui| {
                        let columns = map_columns_for_width(
                            (ui.available_width() - content_padding.x).max(cell_width),
                            cell_width,
                            cell_spacing.x,
                        );

                        egui::Grid::new("ip-map-grid")
                            .spacing([cell_spacing.x, cell_spacing.y])
                            .show(ui, |ui| {
                                for (index, record) in self.records.iter().enumerate() {
                                    let fill = map_cell_color(record.status);
                                    let text = format!(
                                        "{}\n{}",
                                        record.ip.octets()[3],
                                        grid_secondary_text(record, self.language)
                                    );

                                    let button = egui::Button::new(
                                        RichText::new(text).size(11.0).color(Color32::BLACK),
                                    )
                                    .min_size(egui::vec2(cell_width, cell_height))
                                    .fill(fill)
                                    .stroke(
                                        if self.selected_ip == Some(record.ip) {
                                            egui::Stroke::new(2.0, Color32::BLACK)
                                        } else {
                                            egui::Stroke::new(1.0, Color32::GRAY)
                                        },
                                    );

                                    let response = ui.add(button).on_hover_text(format!(
                                        "{}\n{}: {}\n{}: {}\n{}: {}\n{}: {}",
                                        record.ip,
                                        self.language.status_field_label(),
                                        record.status.label(self.language),
                                        self.language.mac_field_label(),
                                        record.mac.as_deref().unwrap_or(self.language.no_value()),
                                        self.language.vendor_field_label(),
                                        record
                                            .vendor
                                            .as_deref()
                                            .unwrap_or(self.language.no_value()),
                                        self.language.hostname_field_label(),
                                        record
                                            .hostname
                                            .as_deref()
                                            .unwrap_or(self.language.no_value())
                                    ));

                                    if response.clicked() {
                                        clicked_ip = Some(record.ip);
                                    }

                                    if (index + 1) % columns == 0 {
                                        ui.end_row();
                                    }
                                }
                            });

                        ui.add_space(content_padding.y);
                    });

                    ui.add_space(content_padding.x);
                });
            });

        if let Some(ip) = clicked_ip {
            self.selected_ip = Some(ip);
        }

        ui.add_space(8.0);
        if let Some(selected) = self
            .selected_ip
            .and_then(|ip| self.records.iter().find(|record| record.ip == ip))
        {
            ui.group(|ui| {
                ui.strong(self.language.selected_device_label());
                ui.label(format!(
                    "{} {} | {} {} | {} {} | {} {} | {} {}",
                    self.language.ip_field_label(),
                    selected.ip,
                    self.language.status_field_label(),
                    selected.status.label(self.language),
                    self.language.mac_field_label(),
                    selected.mac.as_deref().unwrap_or(self.language.no_value()),
                    self.language.vendor_field_label(),
                    selected
                        .vendor
                        .as_deref()
                        .unwrap_or(self.language.no_value()),
                    self.language.hostname_field_label(),
                    selected
                        .hostname
                        .as_deref()
                        .unwrap_or(self.language.no_value())
                ));
            });
        }
    }

    fn render_table_view(&mut self, ui: &mut egui::Ui) {
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::initial(150.0))
            .column(Column::initial(100.0))
            .column(Column::initial(160.0))
            .column(Column::remainder().at_least(180.0))
            .column(Column::remainder().at_least(180.0))
            .min_scrolled_height(0.0);

        table
            .header(28.0, |mut header| {
                header.col(|ui| {
                    sort_header_button(ui, self.language.ip_field_label(), SortColumn::Ip, self);
                });
                header.col(|ui| {
                    sort_header_button(
                        ui,
                        self.language.status_field_label(),
                        SortColumn::Status,
                        self,
                    );
                });
                header.col(|ui| {
                    sort_header_button(ui, self.language.mac_field_label(), SortColumn::Mac, self);
                });
                header.col(|ui| {
                    sort_header_button(
                        ui,
                        self.language.vendor_field_label(),
                        SortColumn::Vendor,
                        self,
                    );
                });
                header.col(|ui| {
                    sort_header_button(
                        ui,
                        self.language.hostname_field_label(),
                        SortColumn::Hostname,
                        self,
                    );
                });
            })
            .body(|body| {
                body.rows(26.0, self.records.len(), |mut row| {
                    let record = &self.records[row.index()];
                    row.col(|ui| {
                        ui.label(record.ip.to_string());
                    });
                    row.col(|ui| {
                        ui.label(
                            RichText::new(record.status.label(self.language))
                                .color(record.status.color()),
                        );
                    });
                    row.col(|ui| {
                        ui.label(record.mac.as_deref().unwrap_or(self.language.no_value()));
                    });
                    row.col(|ui| {
                        ui.label(record.vendor.as_deref().unwrap_or(self.language.no_value()));
                    });
                    row.col(|ui| {
                        ui.label(
                            record
                                .hostname
                                .as_deref()
                                .unwrap_or(self.language.no_value()),
                        );
                    });
                });
            });
    }

    fn start_scan(&mut self) {
        self.ip_range_input = normalize_range_input(&self.ip_range_input);
        self.selected_ip = None;
        let resolve_hostnames = self.resolve_hostnames;
        self.persist_settings();

        let targets = match parse_targets(&self.ip_range_input, self.language) {
            Ok(targets) if !targets.is_empty() => targets,
            Ok(_) => {
                self.status_line = self.language.no_targets_status().to_string();
                return;
            }
            Err(err) => {
                self.status_line = err;
                return;
            }
        };

        let (tx, rx) = mpsc::channel();
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let processed_count = Arc::new(AtomicUsize::new(0));
        let vendor_db = Arc::clone(&self.vendor_db);
        let total_targets = targets.len();
        let worker_cancel = Arc::clone(&cancel_flag);
        let worker_processed = Arc::clone(&processed_count);

        self.records = targets
            .iter()
            .copied()
            .map(|ip| ScanRecord {
                ip,
                status: DeviceStatus::Pending,
                mac: None,
                vendor: None,
                hostname: None,
            })
            .collect();
        self.total_targets = total_targets;
        self.receiver = Some(rx);
        self.cancel_flag = Some(cancel_flag);
        self.processed_count = processed_count;
        self.is_scanning = true;
        self.sort_records();
        self.status_line = self.language.scan_started_status(total_targets);
        let language = self.language;

        thread::spawn(move || {
            let threads = total_targets.clamp(1, 256);

            let pool = match rayon::ThreadPoolBuilder::new().num_threads(threads).build() {
                Ok(pool) => pool,
                Err(err) => {
                    let _ = tx.send(WorkerMessage::Error(
                        language.scanner_thread_pool_error(&err.to_string()),
                    ));
                    return;
                }
            };

            pool.scope(|scope| {
                for ip in targets {
                    let tx = tx.clone();
                    let worker_cancel = Arc::clone(&worker_cancel);
                    let worker_processed = Arc::clone(&worker_processed);
                    let vendor_db = Arc::clone(&vendor_db);

                    scope.spawn(move |_| {
                        if worker_cancel.load(Ordering::Relaxed) {
                            worker_processed.fetch_add(1, Ordering::Relaxed);
                            return;
                        }

                        let record = probe_host(ip, &vendor_db);
                        let should_lookup_name =
                            resolve_hostnames && record.status == DeviceStatus::InUse;
                        let _ = tx.send(WorkerMessage::Record(record));

                        if should_lookup_name && !worker_cancel.load(Ordering::Relaxed) {
                            let _ = tx.send(WorkerMessage::HostnameResolved {
                                ip,
                                hostname: lookup_hostname(ip),
                            });
                        }

                        worker_processed.fetch_add(1, Ordering::Relaxed);
                    });
                }
            });

            let _ = tx.send(WorkerMessage::Finished {
                cancelled: worker_cancel.load(Ordering::Relaxed),
            });
        });
    }

    fn drain_worker_messages(&mut self) {
        let mut should_drop_receiver = false;

        loop {
            let Some(message) = self
                .receiver
                .as_ref()
                .and_then(|receiver| receiver.try_recv().ok())
            else {
                break;
            };

            match message {
                WorkerMessage::Record(record) => {
                    if let Some(existing) = self
                        .records
                        .iter_mut()
                        .find(|existing| existing.ip == record.ip)
                    {
                        *existing = record;
                    } else {
                        self.records.push(record);
                    }
                    if self.sort_column != SortColumn::Ip || !self.sort_ascending {
                        self.sort_records();
                    }
                }
                WorkerMessage::HostnameResolved { ip, hostname } => {
                    if let Some(record) = self.records.iter_mut().find(|record| record.ip == ip) {
                        record.hostname = hostname;
                        if self.sort_column != SortColumn::Ip || !self.sort_ascending {
                            self.sort_records();
                        }
                    }
                }
                WorkerMessage::Finished { cancelled } => {
                    self.sort_records();
                    self.is_scanning = false;
                    self.cancel_flag = None;
                    should_drop_receiver = true;

                    let processed = self.processed_count.load(Ordering::Relaxed);
                    if cancelled {
                        self.status_line = self
                            .language
                            .scan_cancelled_status(processed, self.total_targets);
                    } else {
                        self.status_line = self
                            .language
                            .scan_completed_status(self.active_count(), self.total_targets);
                    }
                }
                WorkerMessage::Error(err) => {
                    self.is_scanning = false;
                    self.cancel_flag = None;
                    self.status_line = err;
                    should_drop_receiver = true;
                }
            }
        }

        if should_drop_receiver {
            self.receiver = None;
        }
    }

    fn save_results(&mut self) {
        let file = FileDialog::new()
            .set_title(self.language.save_results_title())
            .add_filter("Excel Workbook", &["xlsx"])
            .add_filter("CSV", &["csv"])
            .set_file_name("scan-results.xlsx")
            .save_file();

        let Some(path) = file else {
            return;
        };

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .unwrap_or_default();

        let result = match extension.as_str() {
            "csv" => write_csv(&path, &self.records, self.language),
            _ => write_xlsx(
                &ensure_extension(path, "xlsx"),
                &self.records,
                self.language,
            ),
        };

        match result {
            Ok(saved_path) => {
                self.status_line = self.language.saved_results_status(&saved_path);
            }
            Err(err) => {
                self.status_line = self.language.save_results_failed_status(&err);
            }
        }
    }

    fn toggle_sort(&mut self, column: SortColumn) {
        if self.sort_column == column {
            self.sort_ascending = !self.sort_ascending;
        } else {
            self.sort_column = column;
            self.sort_ascending = true;
        }

        self.sort_records();
    }

    fn sort_records(&mut self) {
        let column = self.sort_column;
        let ascending = self.sort_ascending;

        self.records.sort_by(|left, right| {
            let ordering = compare_records(left, right, column);
            let ordering = if ordering == std::cmp::Ordering::Equal {
                compare_records(left, right, SortColumn::Ip)
            } else {
                ordering
            };

            if ascending {
                ordering
            } else {
                ordering.reverse()
            }
        });
    }

    fn persist_settings(&mut self) {
        if let Err(err) = save_app_settings(&AppSettings {
            last_range: self.ip_range_input.clone(),
            result_view: self.result_view,
            resolve_hostnames: self.resolve_hostnames,
            language: self.language,
        }) {
            self.status_line = self.language.settings_save_failed_status(&err);
        }
    }
}

fn discover_range_presets() -> Vec<String> {
    let mut presets = BTreeSet::new();

    #[cfg(windows)]
    if let Ok(adapters) = ipconfig::get_adapters() {
        for adapter in adapters {
            collect_adapter_presets(&adapter, &mut presets);
        }
    }

    #[cfg(not(windows))]
    if let Ok(interfaces) = if_addrs::get_if_addrs() {
        for interface in interfaces {
            collect_interface_presets(&interface.addr, &mut presets);
        }
    }

    if presets.is_empty() {
        presets.insert("192.168.1.1-254".to_string());
    }

    presets.into_iter().collect()
}

fn collect_adapter_presets(adapter: &Adapter, presets: &mut BTreeSet<String>) {
    for ip in adapter.ip_addresses() {
        let IpAddr::V4(ip) = ip else {
            continue;
        };
        collect_ipv4_preset(*ip, presets);
    }
}

#[cfg(not(windows))]
fn collect_interface_presets(address: &IfAddr, presets: &mut BTreeSet<String>) {
    let IfAddr::V4(ipv4) = address else {
        return;
    };
    collect_ipv4_preset(ipv4.ip, presets);
}

fn collect_ipv4_preset(ip: Ipv4Addr, presets: &mut BTreeSet<String>) {
    if ip.is_loopback() || ip.octets()[0] == 169 {
        return;
    }

    let [a, b, c, _] = ip.octets();
    presets.insert(format!("{a}.{b}.{c}.1-254"));
}

fn parse_targets(input: &str, language: Language) -> Result<Vec<Ipv4Addr>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(language.parse_targets_required_error());
    }

    if let Ok(net) = trimmed.parse::<IpNet>() {
        let IpNet::V4(net) = net else {
            return Err(language.parse_targets_ipv4_only_error());
        };

        let hosts: Vec<Ipv4Addr> = net.hosts().collect();
        if hosts.is_empty() {
            return Err(language.parse_targets_cidr_empty_error());
        }
        return Ok(hosts);
    }

    if let Some((start, end)) = trimmed.split_once('-') {
        let start_ip = start
            .trim()
            .parse::<Ipv4Addr>()
            .map_err(|_| language.parse_targets_invalid_start_ip_error())?;

        let end_ip = if end.contains('.') {
            end.trim()
                .parse::<Ipv4Addr>()
                .map_err(|_| language.parse_targets_invalid_end_ip_error())?
        } else {
            let last_octet = end
                .trim()
                .parse::<u8>()
                .map_err(|_| language.parse_targets_invalid_last_octet_error())?;
            let [a, b, c, _] = start_ip.octets();
            Ipv4Addr::new(a, b, c, last_octet)
        };

        return expand_range(start_ip, end_ip, language);
    }

    let single = trimmed
        .parse::<Ipv4Addr>()
        .map_err(|_| language.parse_targets_supported_formats_error())?;
    Ok(vec![single])
}

fn expand_range(
    start: Ipv4Addr,
    end: Ipv4Addr,
    language: Language,
) -> Result<Vec<Ipv4Addr>, String> {
    let start_num = ipv4_sort_key(start);
    let end_num = ipv4_sort_key(end);

    if start_num > end_num {
        return Err(language.expand_range_invalid_order_error());
    }

    let total = end_num - start_num + 1;
    if total > 65_536 {
        return Err(language.expand_range_too_large_error());
    }

    Ok((start_num..=end_num).map(u32_to_ipv4).collect())
}

fn normalize_range_input(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.contains('-') || trimmed.contains('/') {
        return trimmed.to_string();
    }

    if let Some((a, b, c)) = parse_three_octets(trimmed) {
        return format!("{a}.{b}.{c}.1-254");
    }

    if let Ok(ip) = trimmed.parse::<Ipv4Addr>() {
        let [a, b, c, _] = ip.octets();
        return format!("{a}.{b}.{c}.1-254");
    }

    trimmed.to_string()
}

fn parse_three_octets(input: &str) -> Option<(u8, u8, u8)> {
    let mut parts = input.split('.');
    let a = parts.next()?.parse().ok()?;
    let b = parts.next()?.parse().ok()?;
    let c = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }

    Some((a, b, c))
}

fn probe_host(ip: Ipv4Addr, vendor_db: &HashMap<String, String>) -> ScanRecord {
    let first_mac = try_send_arp(ip);
    let ping_ok = first_mac.is_some() || ping_host(ip);
    let mac = first_mac.or_else(|| if ping_ok { try_send_arp(ip) } else { None });
    let vendor = mac
        .as_deref()
        .and_then(|mac| vendor_from_mac(mac, vendor_db));

    ScanRecord {
        ip,
        status: if mac.is_some() || ping_ok {
            DeviceStatus::InUse
        } else {
            DeviceStatus::Available
        },
        mac,
        vendor,
        hostname: None,
    }
}

fn ping_host(ip: Ipv4Addr) -> bool {
    #[cfg(windows)]
    {
        if let Some(result) = native_ping_host(ip, 45) {
            return result;
        }

        let output = run_hidden_command(
            SystemUtility::Ping,
            &["-n", "1", "-w", "45", &ip.to_string()],
        );
        return match output {
            Ok(output) => output.status.success(),
            Err(_) => false,
        };
    }

    #[cfg(not(windows))]
    {
        let output = run_hidden_command(
            SystemUtility::Ping,
            &["-c", "1", "-W", "1000", &ip.to_string()],
        );
        match output {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
}

fn try_send_arp(ip: Ipv4Addr) -> Option<String> {
    #[cfg(windows)]
    if let Some(mac) = send_arp_request(ip) {
        return Some(mac);
    }

    #[cfg(not(windows))]
    if let Some(mac) = query_arp_cache(ip) {
        return Some(mac);
    }

    let _ = ip;
    None
}

#[cfg(windows)]
fn native_ping_host(ip: Ipv4Addr, timeout_ms: u32) -> Option<bool> {
    ICMP_THREAD_HANDLE.with(|icmp_handle| {
        let mut icmp_handle = icmp_handle.borrow_mut();
        let handle = icmp_handle.get_or_create()?;

        let payload = b"faip";
        let mut reply_buffer = vec![0u8; size_of::<IcmpEchoReply>() + payload.len() + 8];
        let result = unsafe {
            IcmpSendEcho(
                handle,
                windows_ip_addr(ip),
                payload.as_ptr().cast(),
                payload.len() as u16,
                std::ptr::null(),
                reply_buffer.as_mut_ptr().cast(),
                reply_buffer.len() as u32,
                timeout_ms,
            )
        };

        if result == 0 {
            return Some(false);
        }

        let reply = unsafe { &*(reply_buffer.as_ptr().cast::<IcmpEchoReply>()) };
        Some(reply.status == 0)
    })
}

#[cfg(windows)]
fn send_arp_request(ip: Ipv4Addr) -> Option<String> {
    let mut mac_buffer = [0u8; 8];
    let mut mac_len = mac_buffer.len() as u32;
    let result = unsafe {
        SendARP(
            windows_ip_addr(ip),
            0,
            mac_buffer.as_mut_ptr().cast(),
            &mut mac_len,
        )
    };

    if result != 0 || mac_len < 6 {
        return None;
    }

    Some(
        mac_buffer[..mac_len as usize]
            .iter()
            .take(6)
            .map(|byte| format!("{byte:02X}"))
            .collect::<Vec<_>>()
            .join(":"),
    )
}

#[cfg(windows)]
fn windows_ip_addr(ip: Ipv4Addr) -> u32 {
    u32::from_ne_bytes(ip.octets())
}

fn lookup_hostname(ip: Ipv4Addr) -> Option<String> {
    let ip_addr = IpAddr::V4(ip);

    if let Ok(name) = lookup_addr(&ip_addr) {
        let normalized = name.trim_end_matches('.').trim().to_string();
        if !normalized.is_empty() && normalized != ip.to_string() {
            return Some(normalized);
        }
    }

    #[cfg(windows)]
    {
        return lookup_nbtstat_name(ip);
    }

    #[cfg(not(windows))]
    {
        let _ = ip;
        None
    }
}

#[cfg(windows)]
fn lookup_nbtstat_name(ip: Ipv4Addr) -> Option<String> {
    let output = run_hidden_command(SystemUtility::NbtStat, &["-A", &ip.to_string()]).ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || !trimmed.contains("<00>")
            || trimmed.contains("GROUP")
            || trimmed.contains("MSBROWSE")
        {
            continue;
        }

        let candidate = trimmed
            .split("<00>")
            .next()
            .map(str::trim)
            .unwrap_or_default();

        if !candidate.is_empty() && !candidate.contains("NetBIOS") {
            return Some(candidate.to_string());
        }
    }

    None
}

#[cfg(not(windows))]
fn query_arp_cache(ip: Ipv4Addr) -> Option<String> {
    let output = run_hidden_command(SystemUtility::Arp, &["-n", &ip.to_string()]).ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        let mut words = line.split_whitespace();
        while let Some(word) = words.next() {
            if word == "at" {
                let candidate = words.next()?;
                return normalize_mac(candidate);
            }
        }
    }

    None
}

#[cfg(any(test, not(windows)))]
fn normalize_mac(candidate: &str) -> Option<String> {
    let cleaned = candidate
        .trim_matches(|ch: char| ch == '(' || ch == ')' || ch == '[' || ch == ']')
        .replace('-', ":");
    if cleaned.eq_ignore_ascii_case("(incomplete)") || cleaned.eq_ignore_ascii_case("incomplete") {
        return None;
    }

    let parts = cleaned.split(':').collect::<Vec<_>>();
    if parts.len() != 6 {
        return None;
    }

    let mut normalized = Vec::with_capacity(6);
    for part in parts {
        if part.is_empty() || part.len() > 2 || !part.chars().all(|ch| ch.is_ascii_hexdigit()) {
            return None;
        }
        normalized.push(format!("{:02X}", u8::from_str_radix(part, 16).ok()?));
    }

    Some(normalized.join(":"))
}

fn vendor_from_mac(mac: &str, vendor_db: &HashMap<String, String>) -> Option<String> {
    let prefix = mac.chars().take(8).collect::<String>();
    vendor_db.get(&prefix).cloned()
}

fn compare_records(
    left: &ScanRecord,
    right: &ScanRecord,
    column: SortColumn,
) -> std::cmp::Ordering {
    match column {
        SortColumn::Ip => ipv4_sort_key(left.ip).cmp(&ipv4_sort_key(right.ip)),
        SortColumn::Status => {
            device_status_rank(left.status).cmp(&device_status_rank(right.status))
        }
        SortColumn::Mac => compare_optional_text(left.mac.as_deref(), right.mac.as_deref()),
        SortColumn::Vendor => {
            compare_optional_text(left.vendor.as_deref(), right.vendor.as_deref())
        }
        SortColumn::Hostname => {
            compare_optional_text(left.hostname.as_deref(), right.hostname.as_deref())
        }
    }
}

fn device_status_rank(status: DeviceStatus) -> u8 {
    match status {
        DeviceStatus::Pending => 0,
        DeviceStatus::InUse => 1,
        DeviceStatus::Available => 2,
    }
}

fn compare_optional_text(left: Option<&str>, right: Option<&str>) -> std::cmp::Ordering {
    normalize_sort_text(left).cmp(&normalize_sort_text(right))
}

fn normalize_sort_text(value: Option<&str>) -> String {
    value.unwrap_or("").trim().to_ascii_lowercase()
}

fn sort_header_button(
    ui: &mut egui::Ui,
    title: &str,
    column: SortColumn,
    app: &mut FaIpScannerApp,
) {
    let suffix = if app.sort_column == column {
        if app.sort_ascending {
            " ▲"
        } else {
            " ▼"
        }
    } else {
        ""
    };

    if ui.button(format!("{title}{suffix}")).clicked() {
        app.toggle_sort(column);
    }
}

fn map_cell_color(status: DeviceStatus) -> Color32 {
    match status {
        DeviceStatus::Pending => Color32::from_rgb(0xB8, 0xD7, 0xFF),
        DeviceStatus::InUse => Color32::from_rgb(0xFF, 0xA2, 0x2B),
        DeviceStatus::Available => Color32::from_rgb(0xB8, 0xFF, 0xA7),
    }
}

fn grid_secondary_text(record: &ScanRecord, language: Language) -> String {
    let text = record
        .hostname
        .as_deref()
        .or(record.vendor.as_deref())
        .or(record.mac.as_deref())
        .unwrap_or_else(|| match record.status {
            DeviceStatus::Pending => DeviceStatus::Pending.label(language),
            DeviceStatus::InUse => DeviceStatus::InUse.label(language),
            DeviceStatus::Available => DeviceStatus::Available.label(language),
        });

    truncate_for_grid(text, 8)
}

fn legend_chip(ui: &mut egui::Ui, color: Color32, label: &str) {
    ui.horizontal(|ui| {
        ui.colored_label(color, "■");
        ui.label(label);
    });
}

fn show_subdued_panel<R>(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::new()
        .fill(Color32::from_rgb(250, 251, 253))
        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(232, 236, 242)))
        .inner_margin(12.0)
        .show(ui, add_contents)
        .inner
}

fn map_columns_for_width(available_width: f32, cell_width: f32, cell_spacing: f32) -> usize {
    (((available_width + cell_spacing) / (cell_width + cell_spacing)).floor() as usize).max(1)
}

fn truncate_for_grid(text: &str, max_chars: usize) -> String {
    let mut result = String::new();
    for (index, ch) in text.chars().enumerate() {
        if index >= max_chars {
            result.push_str("...");
            break;
        }
        result.push(ch);
    }
    result
}

fn load_app_settings() -> AppSettings {
    let path = app_settings_path();
    let Ok(text) = fs::read_to_string(path) else {
        return AppSettings::default();
    };

    serde_json::from_str(&text).unwrap_or_default()
}

fn save_app_settings(settings: &AppSettings) -> Result<(), String> {
    let path = app_settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }

    let text = serde_json::to_string_pretty(settings).map_err(|err| err.to_string())?;
    fs::write(path, text).map_err(|err| err.to_string())
}

fn app_settings_path() -> PathBuf {
    let base = app_data_base_dir();

    base.join("IPScanner").join("settings.json")
}

fn app_data_base_dir() -> PathBuf {
    std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("LOCALAPPDATA").map(PathBuf::from))
        .unwrap_or_else(std::env::temp_dir)
}

fn load_vendor_database() -> HashMap<String, String> {
    for candidate in vendor_db_paths() {
        if let Ok(text) = fs::read_to_string(&candidate) {
            if let Ok(map) = serde_json::from_str::<HashMap<String, String>>(&text) {
                let normalized = normalize_vendor_map(map);
                if !normalized.is_empty() {
                    return normalized;
                }
            }
        }
    }

    let fallback: HashMap<String, String> =
        serde_json::from_str(DEFAULT_OUI_JSON).expect("embedded vendor database is valid");
    normalize_vendor_map(fallback)
}

fn vendor_db_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            paths.push(parent.join("oui.json"));
        }
    }

    paths.push(app_data_base_dir().join("IPScanner").join("oui.json"));

    paths
}

fn normalize_vendor_map(input: HashMap<String, String>) -> HashMap<String, String> {
    input
        .into_iter()
        .filter_map(|(key, value)| {
            let normalized = key.replace('-', ":").to_ascii_uppercase();
            let prefix = normalized.chars().take(8).collect::<String>();

            if prefix.len() == 8 && !value.trim().is_empty() {
                Some((prefix, value.trim().to_string()))
            } else {
                None
            }
        })
        .collect()
}

fn ensure_extension(path: PathBuf, extension: &str) -> PathBuf {
    if path.extension().is_some() {
        path
    } else {
        path.with_extension(extension)
    }
}

fn write_csv(path: &Path, records: &[ScanRecord], language: Language) -> Result<PathBuf, String> {
    let path = ensure_extension(path.to_path_buf(), "csv");
    let mut file = fs::File::create(&path).map_err(|err| err.to_string())?;
    file.write_all(&[0xEF, 0xBB, 0xBF])
        .map_err(|err| err.to_string())?;

    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);

    writer
        .write_record(language.export_headers())
        .map_err(|err| err.to_string())?;

    for record in records {
        writer
            .write_record([
                record.ip.to_string(),
                record.status.label(language).to_string(),
                record
                    .mac
                    .clone()
                    .unwrap_or_else(|| language.no_value().to_string()),
                record
                    .vendor
                    .clone()
                    .unwrap_or_else(|| language.no_value().to_string()),
                record
                    .hostname
                    .clone()
                    .unwrap_or_else(|| language.no_value().to_string()),
            ])
            .map_err(|err| err.to_string())?;
    }

    writer.flush().map_err(|err| err.to_string())?;
    Ok(path)
}

fn write_xlsx(path: &Path, records: &[ScanRecord], language: Language) -> Result<PathBuf, String> {
    let path = ensure_extension(path.to_path_buf(), "xlsx");
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let header = Format::new().set_bold().set_align(FormatAlign::Center);

    for (col, title) in language.export_headers().iter().enumerate() {
        worksheet
            .write_string_with_format(0, col as u16, *title, &header)
            .map_err(|err| err.to_string())?;
    }

    worksheet
        .set_column_width(0, 18.0)
        .map_err(|err| err.to_string())?;
    worksheet
        .set_column_width(1, 12.0)
        .map_err(|err| err.to_string())?;
    worksheet
        .set_column_width(2, 20.0)
        .map_err(|err| err.to_string())?;
    worksheet
        .set_column_width(3, 28.0)
        .map_err(|err| err.to_string())?;
    worksheet
        .set_column_width(4, 28.0)
        .map_err(|err| err.to_string())?;

    for (row, record) in records.iter().enumerate() {
        let row = (row + 1) as u32;
        worksheet
            .write_string(row, 0, record.ip.to_string())
            .map_err(|err| err.to_string())?;
        worksheet
            .write_string(row, 1, record.status.label(language))
            .map_err(|err| err.to_string())?;
        worksheet
            .write_string(row, 2, record.mac.as_deref().unwrap_or(language.no_value()))
            .map_err(|err| err.to_string())?;
        worksheet
            .write_string(
                row,
                3,
                record.vendor.as_deref().unwrap_or(language.no_value()),
            )
            .map_err(|err| err.to_string())?;
        worksheet
            .write_string(
                row,
                4,
                record.hostname.as_deref().unwrap_or(language.no_value()),
            )
            .map_err(|err| err.to_string())?;
    }

    workbook.save(&path).map_err(|err| err.to_string())?;
    Ok(path)
}

#[derive(Clone, Copy)]
enum SystemUtility {
    Ping,
    #[cfg(not(windows))]
    Arp,
    #[cfg(windows)]
    NbtStat,
}

impl SystemUtility {
    fn executable_name(self) -> &'static str {
        match self {
            Self::Ping => executable_name_for_platform("ping"),
            #[cfg(not(windows))]
            Self::Arp => executable_name_for_platform("arp"),
            #[cfg(windows)]
            Self::NbtStat => executable_name_for_platform("nbtstat"),
        }
    }
}

fn run_hidden_command(program: SystemUtility, args: &[&str]) -> std::io::Result<Output> {
    let executable = resolve_system_utility(program)?;
    let mut command = Command::new(executable);
    command.args(args);

    #[cfg(windows)]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }

    command.output()
}

fn resolve_system_utility(program: SystemUtility) -> std::io::Result<PathBuf> {
    #[cfg(windows)]
    {
        let system_root = std::env::var_os("SystemRoot").ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "%SystemRoot% is not set")
        })?;
        let path = PathBuf::from(system_root)
            .join("System32")
            .join(program.executable_name());

        if path.is_file() {
            Ok(path)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("system utility not found: {}", path.display()),
            ))
        }
    }

    #[cfg(not(windows))]
    {
        for candidate in unix_utility_candidates(program.executable_name()) {
            if candidate.is_file() {
                return Ok(candidate);
            }
        }

        Ok(PathBuf::from(program.executable_name()))
    }
}

#[cfg(windows)]
fn executable_name_for_platform(base: &'static str) -> &'static str {
    match base {
        "ping" => "ping.exe",
        "arp" => "arp.exe",
        "nbtstat" => "nbtstat.exe",
        _ => base,
    }
}

#[cfg(not(windows))]
fn executable_name_for_platform(base: &'static str) -> &'static str {
    base
}

#[cfg(not(windows))]
fn unix_utility_candidates(executable: &'static str) -> Vec<PathBuf> {
    [
        "/usr/sbin",
        "/usr/bin",
        "/bin",
        "/sbin",
        "/opt/homebrew/bin",
        "/opt/homebrew/sbin",
    ]
    .into_iter()
    .map(|dir| PathBuf::from(dir).join(executable))
    .collect()
}

fn ipv4_sort_key(ip: Ipv4Addr) -> u32 {
    u32::from_be_bytes(ip.octets())
}

fn u32_to_ipv4(value: u32) -> Ipv4Addr {
    Ipv4Addr::from(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_data_base_dir_avoids_current_directory_fallback() {
        let path = app_data_base_dir();
        assert!(!path.as_os_str().is_empty());
    }

    #[test]
    fn vendor_db_paths_include_only_trusted_locations() {
        let current_dir_candidate = std::env::current_dir().unwrap().join("oui.json");
        let paths = vendor_db_paths();

        assert!(!paths.contains(&current_dir_candidate));
        assert!(paths
            .iter()
            .any(|path| path.ends_with(Path::new("IPScanner").join("oui.json"))));
    }

    #[cfg(windows)]
    #[test]
    fn resolve_system_utility_uses_system32() {
        let path = resolve_system_utility(SystemUtility::Ping).unwrap();
        let normalized = path.to_string_lossy().to_ascii_lowercase();

        assert!(normalized.ends_with(r"system32\ping.exe"));
    }

    #[test]
    fn english_validation_messages_are_localized() {
        let err = parse_targets("", Language::English).unwrap_err();
        assert_eq!(err, "Enter an IP range to scan.");
    }

    #[test]
    fn additional_languages_are_registered() {
        assert_eq!(Language::all().len(), 6);
        assert!(Language::all().contains(&Language::ChineseSimplified));
        assert!(Language::all().contains(&Language::French));
        assert!(Language::all().contains(&Language::German));
        assert!(Language::all().contains(&Language::Hindi));
        assert_eq!(Language::Korean.label(), "대한민국 · 한국어");
        assert_eq!(Language::English.label(), "United States · English");
    }

    #[test]
    fn hindi_validation_messages_are_localized() {
        let err = parse_targets("", Language::Hindi).unwrap_err();
        assert_eq!(err, "स्कैन करने के लिए एक IP सीमा दर्ज करें।");
    }

    #[test]
    fn language_is_persisted_in_settings_defaults() {
        let settings = AppSettings::default();
        assert_eq!(settings.language, Language::Korean);
    }

    #[test]
    fn map_columns_account_for_spacing() {
        assert_eq!(map_columns_for_width(1714.0, 78.0, 6.0), 20);
        assert_eq!(map_columns_for_width(78.0, 78.0, 6.0), 1);
    }

    #[test]
    fn app_icon_has_expected_dimensions() {
        let icon = app_icon_data();
        assert_eq!(icon.width, 256);
        assert_eq!(icon.height, 256);
        assert_eq!(icon.rgba.len(), (256 * 256 * 4) as usize);
    }

    #[test]
    fn normalize_mac_formats_mixed_separators() {
        assert_eq!(
            normalize_mac("a:b:0c:1D:ee:f").as_deref(),
            Some("0A:0B:0C:1D:EE:0F")
        );
        assert_eq!(
            normalize_mac("aa-bb-cc-dd-ee-ff").as_deref(),
            Some("AA:BB:CC:DD:EE:FF")
        );
    }
}

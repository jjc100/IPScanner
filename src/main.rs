#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
use eframe::egui::{self, Align, Color32, FontData, FontDefinitions, FontFamily, Layout, RichText, TextEdit};
use eframe::{App, Frame, NativeOptions};
use egui_extras::{Column, TableBuilder};
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
            .with_title("FaIPScanner Clone"),
        ..Default::default()
    };

    eframe::run_native(
        "FaIPScanner Clone",
        native_options,
        Box::new(|cc| {
            configure_fonts(&cc.egui_ctx);
            Ok(Box::new(FaIpScannerApp::default()))
        }),
    )
}

fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    if let Some(font_data) = load_windows_korean_font() {
        fonts
            .font_data
            .insert("windows-korean".to_owned(), FontData::from_owned(font_data).into());

        if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
            family.insert(0, "windows-korean".to_owned());
        }

        if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
            family.insert(0, "windows-korean".to_owned());
        }
    }

    ctx.set_fonts(fonts);
}

fn load_windows_korean_font() -> Option<Vec<u8>> {
    let candidates = [
        r"C:\Windows\Fonts\malgun.ttf",
        r"C:\Windows\Fonts\malgunsl.ttf",
        r"C:\Windows\Fonts\NanumGothic.ttf",
    ];

    candidates.iter().find_map(|path| fs::read(path).ok())
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
    fn SendARP(
        dest_ip: u32,
        src_ip: u32,
        mac_addr: *mut c_void,
        phy_addr_len: *mut u32,
    ) -> u32;
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
    fn label(self) -> &'static str {
        match self {
            Self::Pending => "확인중",
            Self::InUse => "사용중",
            Self::Available => "사용가능",
        }
    }

    fn color(self) -> Color32 {
        match self {
            Self::Pending => Color32::from_rgb(0x3A, 0x74, 0xC9),
            Self::InUse => Color32::from_rgb(0x1D, 0x8F, 0x4E),
            Self::Available => Color32::from_rgb(0xB5, 0x8B, 0x00),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
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
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            last_range: String::new(),
            result_view: ResultView::Map,
            resolve_hostnames: false,
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
    Finished { cancelled: bool },
    Error(String),
}

struct FaIpScannerApp {
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
            ip_range_input,
            range_presets,
            records: Vec::new(),
            status_line: "스캔 대기 중".to_string(),
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
                let mut settings_changed = false;
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label("IP 범위:");
                    let response = ui.add_sized(
                        [320.0, 28.0],
                        TextEdit::singleline(&mut self.ip_range_input)
                            .hint_text("예: 192.168.1 또는 192.168.1.1-254"),
                    );
                    settings_changed |= response.changed();
                    if response.lost_focus() {
                        self.ip_range_input = normalize_range_input(&self.ip_range_input);
                        settings_changed = true;
                    }

                    egui::ComboBox::from_id_salt("range-presets")
                        .selected_text("추천 범위")
                        .width(160.0)
                        .show_ui(ui, |ui| {
                            for preset in &self.range_presets {
                                if ui.selectable_label(false, preset).clicked() {
                                    self.ip_range_input = preset.clone();
                                    settings_changed = true;
                                }
                            }
                        });

                    settings_changed |= ui
                        .checkbox(&mut self.resolve_hostnames, "기기 이름도 찾기 (조금 느림)")
                        .changed();

                    settings_changed |= ui
                        .selectable_value(&mut self.result_view, ResultView::Map, "한눈에 보기")
                        .changed();
                    settings_changed |= ui
                        .selectable_value(&mut self.result_view, ResultView::Table, "목록 보기")
                        .changed();

                    let can_start = !self.is_scanning;
                    if ui
                        .add_enabled(can_start, egui::Button::new("스캔 시작"))
                        .clicked()
                    {
                        self.start_scan();
                    }

                    if ui
                        .add_enabled(self.is_scanning, egui::Button::new("스캔 중지"))
                        .clicked()
                    {
                        if let Some(flag) = &self.cancel_flag {
                            flag.store(true, Ordering::Relaxed);
                            self.status_line = "스캔 중지 요청 중...".to_string();
                        }
                    }

                    if ui
                        .add_enabled(!self.records.is_empty(), egui::Button::new("결과 저장"))
                        .clicked()
                    {
                        self.save_results();
                    }
                });
                ui.add_space(6.0);

                if settings_changed {
                    self.persist_settings();
                }
            });

        egui::TopBottomPanel::bottom("status_bar")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    let processed = self.processed_count.load(Ordering::Relaxed);
                    ui.label(&self.status_line);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(format!(
                            "사용중 {} / 전체 {} / 진행 {}",
                            self.records
                                .iter()
                                .filter(|record| record.status == DeviceStatus::InUse)
                                .count(),
                            self.total_targets,
                            processed
                        ));
                    });
                });
                ui.add_space(6.0);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.result_view {
                ResultView::Map => self.render_map_view(ui),
                ResultView::Table => self.render_table_view(ui),
            }
        });
    }
}

impl FaIpScannerApp {
    fn render_map_view(&mut self, ui: &mut egui::Ui) {
        let cell_width = 68.0;
        let cell_height = 42.0;
        let columns = ((ui.available_width() / cell_width).floor() as usize).max(1);
        let mut clicked_ip = None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("ip-map-grid")
                .spacing([2.0, 2.0])
                .show(ui, |ui| {
                    for (index, record) in self.records.iter().enumerate() {
                        let fill = map_cell_color(record.status);
                        let text = format!(
                            "{}\n{}",
                            record.ip.octets()[3],
                            grid_secondary_text(record)
                        );

                        let button = egui::Button::new(
                            RichText::new(text)
                                .size(10.5)
                                .color(Color32::BLACK),
                        )
                        .min_size(egui::vec2(cell_width, cell_height))
                        .fill(fill)
                        .stroke(if self.selected_ip == Some(record.ip) {
                            egui::Stroke::new(2.0, Color32::BLACK)
                        } else {
                            egui::Stroke::new(1.0, Color32::GRAY)
                        });

                        let response = ui.add(button).on_hover_text(format!(
                            "{}\n상태: {}\nMAC: {}\n제조사: {}\n기기 이름: {}",
                            record.ip,
                            record.status.label(),
                            record.mac.as_deref().unwrap_or("-"),
                            record.vendor.as_deref().unwrap_or("-"),
                            record.hostname.as_deref().unwrap_or("-")
                        ));

                        if response.clicked() {
                            clicked_ip = Some(record.ip);
                        }

                        if (index + 1) % columns == 0 {
                            ui.end_row();
                        }
                    }
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
                ui.horizontal_wrapped(|ui| {
                    ui.strong("선택한 칸:");
                    ui.label(format!(
                        "IP {} | 상태 {} | MAC {} | 제조사 {} | 기기 이름 {}",
                        selected.ip,
                        selected.status.label(),
                        selected.mac.as_deref().unwrap_or("-"),
                        selected.vendor.as_deref().unwrap_or("-"),
                        selected.hostname.as_deref().unwrap_or("-")
                    ));
                });
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
                    sort_header_button(ui, "IP 주소", SortColumn::Ip, self);
                });
                header.col(|ui| {
                    sort_header_button(ui, "상태", SortColumn::Status, self);
                });
                header.col(|ui| {
                    sort_header_button(ui, "MAC 주소", SortColumn::Mac, self);
                });
                header.col(|ui| {
                    sort_header_button(ui, "제조사", SortColumn::Vendor, self);
                });
                header.col(|ui| {
                    sort_header_button(ui, "기기 이름", SortColumn::Hostname, self);
                });
            })
            .body(|body| {
                body.rows(26.0, self.records.len(), |mut row| {
                    let record = &self.records[row.index()];
                    row.col(|ui| {
                        ui.label(record.ip.to_string());
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(record.status.label()).color(record.status.color()));
                    });
                    row.col(|ui| {
                        ui.label(record.mac.as_deref().unwrap_or("-"));
                    });
                    row.col(|ui| {
                        ui.label(record.vendor.as_deref().unwrap_or("-"));
                    });
                    row.col(|ui| {
                        ui.label(record.hostname.as_deref().unwrap_or("-"));
                    });
                });
            });
    }

    fn start_scan(&mut self) {
        self.ip_range_input = normalize_range_input(&self.ip_range_input);
        self.selected_ip = None;
        let resolve_hostnames = self.resolve_hostnames;
        self.persist_settings();

        let targets = match parse_targets(&self.ip_range_input) {
            Ok(targets) if !targets.is_empty() => targets,
            Ok(_) => {
                self.status_line = "스캔 대상이 없습니다.".to_string();
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
        self.status_line = format!("{}개 IP 스캔 시작...", total_targets);

        thread::spawn(move || {
            let threads = total_targets.clamp(1, 256);

            let pool = match rayon::ThreadPoolBuilder::new().num_threads(threads).build() {
                Ok(pool) => pool,
                Err(err) => {
                    let _ = tx.send(WorkerMessage::Error(format!(
                        "스캐너 스레드 풀을 만들지 못했습니다: {err}"
                    )));
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
                        let should_lookup_name = resolve_hostnames && record.status == DeviceStatus::InUse;
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
            let Some(message) = self.receiver.as_ref().and_then(|receiver| receiver.try_recv().ok()) else {
                break;
            };

            match message {
                WorkerMessage::Record(record) => {
                    if let Some(existing) = self.records.iter_mut().find(|existing| existing.ip == record.ip) {
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
                        self.status_line =
                            format!("스캔이 중지되었습니다. 완료 {processed} / {}", self.total_targets);
                    } else {
                        self.status_line = format!(
                            "스캔 완료: 사용중 {} / 전체 {}",
                            self.records
                                .iter()
                                .filter(|record| record.status == DeviceStatus::InUse)
                                .count(),
                            self.total_targets
                        );
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
            .set_title("스캔 결과 저장")
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
            "csv" => write_csv(&path, &self.records),
            _ => write_xlsx(&ensure_extension(path, "xlsx"), &self.records),
        };

        match result {
            Ok(saved_path) => {
                self.status_line = format!("결과를 저장했습니다: {}", saved_path.display());
            }
            Err(err) => {
                self.status_line = format!("결과 저장 실패: {err}");
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
        }) {
            self.status_line = format!("설정 저장 실패: {err}");
        }
    }
}

fn discover_range_presets() -> Vec<String> {
    let mut presets = BTreeSet::new();

    if let Ok(adapters) = ipconfig::get_adapters() {
        for adapter in adapters {
            collect_adapter_presets(&adapter, &mut presets);
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

        if ip.is_loopback() || ip.octets()[0] == 169 {
            continue;
        }

        let [a, b, c, _] = ip.octets();
        presets.insert(format!("{a}.{b}.{c}.1-254"));
    }
}

fn parse_targets(input: &str) -> Result<Vec<Ipv4Addr>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("IP 범위를 입력하세요.".to_string());
    }

    if let Ok(net) = trimmed.parse::<IpNet>() {
        let IpNet::V4(net) = net else {
            return Err("IPv4 대역만 지원합니다.".to_string());
        };

        let hosts: Vec<Ipv4Addr> = net.hosts().collect();
        if hosts.is_empty() {
            return Err("CIDR 범위에서 스캔할 IPv4 호스트가 없습니다.".to_string());
        }
        return Ok(hosts);
    }

    if let Some((start, end)) = trimmed.split_once('-') {
        let start_ip = start
            .trim()
            .parse::<Ipv4Addr>()
            .map_err(|_| "시작 IP 형식이 올바르지 않습니다.".to_string())?;

        let end_ip = if end.contains('.') {
            end.trim()
                .parse::<Ipv4Addr>()
                .map_err(|_| "종료 IP 형식이 올바르지 않습니다.".to_string())?
        } else {
            let last_octet = end
                .trim()
                .parse::<u8>()
                .map_err(|_| "끝 범위는 마지막 옥텟(0-255) 또는 전체 IP여야 합니다.".to_string())?;
            let [a, b, c, _] = start_ip.octets();
            Ipv4Addr::new(a, b, c, last_octet)
        };

        return expand_range(start_ip, end_ip);
    }

    let single = trimmed
        .parse::<Ipv4Addr>()
        .map_err(|_| "지원 형식: 192.168.1, 192.168.1.1-254, 192.168.1.1-192.168.1.50, 192.168.1.0/24".to_string())?;
    Ok(vec![single])
}

fn expand_range(start: Ipv4Addr, end: Ipv4Addr) -> Result<Vec<Ipv4Addr>, String> {
    let start_num = ipv4_sort_key(start);
    let end_num = ipv4_sort_key(end);

    if start_num > end_num {
        return Err("시작 IP가 종료 IP보다 클 수 없습니다.".to_string());
    }

    let total = end_num - start_num + 1;
    if total > 65_536 {
        return Err("한 번에 최대 65,536개 IP까지만 스캔할 수 있습니다.".to_string());
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
    let mac = first_mac.or_else(|| {
        if ping_ok {
            try_send_arp(ip)
        } else {
            None
        }
    });
    let vendor = mac.as_deref().and_then(|mac| vendor_from_mac(mac, vendor_db));

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
    }

    let output = run_hidden_command(SystemUtility::Ping, &["-n", "1", "-w", "45", &ip.to_string()]);
    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn try_send_arp(ip: Ipv4Addr) -> Option<String> {
    #[cfg(windows)]
    if let Some(mac) = send_arp_request(ip) {
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

    lookup_nbtstat_name(ip)
}

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

fn vendor_from_mac(mac: &str, vendor_db: &HashMap<String, String>) -> Option<String> {
    let prefix = mac.chars().take(8).collect::<String>();
    vendor_db.get(&prefix).cloned()
}

fn compare_records(left: &ScanRecord, right: &ScanRecord, column: SortColumn) -> std::cmp::Ordering {
    match column {
        SortColumn::Ip => ipv4_sort_key(left.ip).cmp(&ipv4_sort_key(right.ip)),
        SortColumn::Status => device_status_rank(left.status).cmp(&device_status_rank(right.status)),
        SortColumn::Mac => compare_optional_text(left.mac.as_deref(), right.mac.as_deref()),
        SortColumn::Vendor => compare_optional_text(left.vendor.as_deref(), right.vendor.as_deref()),
        SortColumn::Hostname => compare_optional_text(left.hostname.as_deref(), right.hostname.as_deref()),
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

fn grid_secondary_text(record: &ScanRecord) -> String {
    let text = record
        .hostname
        .as_deref()
        .or(record.vendor.as_deref())
        .or(record.mac.as_deref())
        .unwrap_or_else(|| match record.status {
            DeviceStatus::Pending => "확인중",
            DeviceStatus::InUse => "사용중",
            DeviceStatus::Available => "사용가능",
        });

    truncate_for_grid(text, 8)
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

    base.join("FaIPScannerClone").join("settings.json")
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

    paths.push(app_data_base_dir().join("FaIPScannerClone").join("oui.json"));

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

fn write_csv(path: &Path, records: &[ScanRecord]) -> Result<PathBuf, String> {
    let path = ensure_extension(path.to_path_buf(), "csv");
    let mut file = fs::File::create(&path).map_err(|err| err.to_string())?;
    file.write_all(&[0xEF, 0xBB, 0xBF])
        .map_err(|err| err.to_string())?;

    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);

    writer
        .write_record(["IP 주소", "상태", "MAC 주소", "제조사", "기기 이름"])
        .map_err(|err| err.to_string())?;

    for record in records {
        writer
            .write_record([
                record.ip.to_string(),
                record.status.label().to_string(),
                record.mac.clone().unwrap_or_else(|| "-".to_string()),
                record.vendor.clone().unwrap_or_else(|| "-".to_string()),
                record.hostname.clone().unwrap_or_else(|| "-".to_string()),
            ])
            .map_err(|err| err.to_string())?;
    }

    writer.flush().map_err(|err| err.to_string())?;
    Ok(path)
}

fn write_xlsx(path: &Path, records: &[ScanRecord]) -> Result<PathBuf, String> {
    let path = ensure_extension(path.to_path_buf(), "xlsx");
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let header = Format::new().set_bold().set_align(FormatAlign::Center);

    for (col, title) in ["IP 주소", "상태", "MAC 주소", "제조사", "기기 이름"]
        .iter()
        .enumerate()
    {
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
            .write_string(row, 1, record.status.label())
            .map_err(|err| err.to_string())?;
        worksheet
            .write_string(row, 2, record.mac.as_deref().unwrap_or("-"))
            .map_err(|err| err.to_string())?;
        worksheet
            .write_string(row, 3, record.vendor.as_deref().unwrap_or("-"))
            .map_err(|err| err.to_string())?;
        worksheet
            .write_string(row, 4, record.hostname.as_deref().unwrap_or("-"))
            .map_err(|err| err.to_string())?;
    }

    workbook.save(&path).map_err(|err| err.to_string())?;
    Ok(path)
}

#[derive(Clone, Copy)]
enum SystemUtility {
    Ping,
    NbtStat,
}

impl SystemUtility {
    fn executable_name(self) -> &'static str {
        match self {
            Self::Ping => "ping.exe",
            Self::NbtStat => "nbtstat.exe",
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
        assert!(paths.iter().any(|path| path.ends_with(Path::new("FaIPScannerClone").join("oui.json"))));
    }

    #[cfg(windows)]
    #[test]
    fn resolve_system_utility_uses_system32() {
        let path = resolve_system_utility(SystemUtility::Ping).unwrap();
        let normalized = path.to_string_lossy().to_ascii_lowercase();

        assert!(normalized.ends_with(r"system32\ping.exe"));
    }
}

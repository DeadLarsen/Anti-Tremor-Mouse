mod calibration;
mod tremor_filter;
mod gui;

use device_query::{DeviceQuery, DeviceState, MouseState};
use enigo::{Enigo, MouseControllable};
use eframe::{egui, NativeOptions};

struct TremorApp {
    gui: gui::TremorGUI,
    tremor_filter: tremor_filter::TremorFilter,
    mouse_control: Enigo,
    device_state: DeviceState,
}

impl eframe::App for TremorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mouse_state: MouseState = self.device_state.get_mouse();
        let current_pos = (mouse_state.coords.0 as f64, mouse_state.coords.1 as f64);
        let filtered_pos = self.tremor_filter.process_movement(current_pos.0, current_pos.1);
        self.mouse_control.mouse_move_to(filtered_pos.0 as i32, filtered_pos.1 as i32);
        let (frequency, amplitude, tremor_type) = self.tremor_filter.get_tremor_metrics();
        self.gui.update_metrics(current_pos, filtered_pos, tremor_type, frequency, amplitude);
        self.gui.show(ctx);
        ctx.request_repaint();
    }
}

fn main() {
    let mouse_control = Enigo::new();
    let device_state = DeviceState::new();
    println!("Начинаем калибровку...");
    let calibration_data = calibration::perform_calibration(&device_state).expect("Calibration failed");
    println!("Калибровка завершена: {:?}", calibration_data);
    let tremor_filter = tremor_filter::TremorFilter::new(calibration_data);
    let gui = gui::TremorGUI::new();
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Mouse Tremor Stabilization",
        options,
        Box::new(|_cc| Box::new(TremorApp {
            gui,
            tremor_filter,
            mouse_control,
            device_state,
        }))
    );
} 
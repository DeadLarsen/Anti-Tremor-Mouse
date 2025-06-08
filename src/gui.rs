use eframe::egui;
use std::collections::VecDeque;
use crate::tremor_filter::TremorType;
use std::time::{Instant, Duration};

pub struct TremorGUI {
    movement_history: VecDeque<(f64, f64)>,
    filtered_history: VecDeque<(f64, f64)>,
    tremor_type: TremorType,
    frequency: f64,
    amplitude: f64,
    start_time: Instant,
}

impl TremorGUI {
    pub fn new() -> Self {
        let start_time = Instant::now();
        println!("TremorGUI::new: start_time = {:?}", start_time);
        Self {
            movement_history: VecDeque::with_capacity(100),
            filtered_history: VecDeque::with_capacity(100),
            tremor_type: TremorType::Mixed,
            frequency: 0.0,
            amplitude: 0.0,
            start_time,
        }
    }

    pub fn update_metrics(&mut self, raw_pos: (f64, f64), filtered_pos: (f64, f64), 
                         tremor_type: TremorType, frequency: f64, amplitude: f64) {
        self.movement_history.push_back(raw_pos);
        self.filtered_history.push_back(filtered_pos);
        self.tremor_type = tremor_type;
        self.frequency = frequency;
        self.amplitude = amplitude;

        if self.movement_history.len() > 100 {
            self.movement_history.pop_front();
            self.filtered_history.pop_front();
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let elapsed = self.start_time.elapsed();
            println!("TremorGUI::show: elapsed = {:?}", elapsed);
            if elapsed < Duration::from_secs(3) {
                ui.colored_label(egui::Color32::LIGHT_GREEN, "Калибровка завершена! Программа готова к работе.");
                ui.add_space(10.0);
            }
            ui.heading("Mouse Tremor Stabilization");
            
            let tremor_type_str = match self.tremor_type {
                TremorType::Resting => "Тремор в покое",
                TremorType::Action => "Тремор при движении",
                TremorType::Mixed => "Смешанный тип",
                TremorType::HighFrequency => "Высокочастотный тремор",
                TremorType::HighAmplitude => "Тремор с большой амплитудой",
            };
            ui.label(format!("Тип тремора: {}", tremor_type_str));
            ui.label(format!("Частота тремора: {:.2} Гц", self.frequency));
            ui.label(format!("Амплитуда тремора: {:.2} пикселей", self.amplitude));

            // Выделяем область для графика
            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::hover()
            );

            let rect = response.rect;
            let center = rect.center();
            
            // Вычисляем масштаб на основе размера области и истории движений
            let max_movement = self.movement_history.iter()
                .map(|(x, y)| (x * x + y * y).sqrt())
                .fold(0.0, f64::max);
            let scale = if max_movement > 0.0 {
                (rect.height() * 0.4) / max_movement as f32
            } else {
                1.0
            };

            // Рисуем сетку
            let grid_size = 20.0;
            let grid_color = egui::Color32::from_gray(40);
            
            // Горизонтальные линии
            for i in -5..=5 {
                let y = center.y + i as f32 * grid_size;
                painter.line_segment(
                    [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                    egui::Stroke::new(1.0, grid_color)
                );
            }
            
            // Вертикальные линии
            for i in -5..=5 {
                let x = center.x + i as f32 * grid_size;
                painter.line_segment(
                    [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                    egui::Stroke::new(1.0, grid_color)
                );
            }

            // Рисуем исходное движение (красным)
            if self.movement_history.len() > 1 {
                let points: Vec<egui::Pos2> = self.movement_history.iter()
                    .map(|(x, y)| {
                        let dx = *x - self.movement_history[0].0;
                        let dy = *y - self.movement_history[0].1;
                        egui::pos2(
                            center.x + dx as f32 * scale,
                            center.y + dy as f32 * scale
                        )
                    })
                    .collect();
                
                painter.add(egui::Shape::line(
                    points,
                    egui::Stroke::new(2.0, egui::Color32::RED)
                ));
            }

            // Рисуем отфильтрованное движение (зеленым)
            if self.filtered_history.len() > 1 {
                let points: Vec<egui::Pos2> = self.filtered_history.iter()
                    .map(|(x, y)| {
                        let dx = *x - self.filtered_history[0].0;
                        let dy = *y - self.filtered_history[0].1;
                        egui::pos2(
                            center.x + dx as f32 * scale,
                            center.y + dy as f32 * scale
                        )
                    })
                    .collect();
                
                painter.add(egui::Shape::line(
                    points,
                    egui::Stroke::new(2.0, egui::Color32::GREEN)
                ));
            }

            // Добавляем легенду
            let legend_y = rect.bottom() - 30.0;
            let legend_x = rect.left() + 10.0;
            
            // Красная линия для исходного движения
            painter.line_segment(
                [egui::pos2(legend_x, legend_y), egui::pos2(legend_x + 20.0, legend_y)],
                egui::Stroke::new(2.0, egui::Color32::RED)
            );
            painter.text(
                egui::pos2(legend_x + 25.0, legend_y),
                egui::Align2::LEFT_CENTER,
                "Исходное движение",
                egui::FontId::default(),
                egui::Color32::WHITE
            );
            
            // Зеленая линия для отфильтрованного движения
            painter.line_segment(
                [egui::pos2(legend_x + 150.0, legend_y), egui::pos2(legend_x + 170.0, legend_y)],
                egui::Stroke::new(2.0, egui::Color32::GREEN)
            );
            painter.text(
                egui::pos2(legend_x + 175.0, legend_y),
                egui::Align2::LEFT_CENTER,
                "Отфильтрованное движение",
                egui::FontId::default(),
                egui::Color32::WHITE
            );
        });
    }
} 
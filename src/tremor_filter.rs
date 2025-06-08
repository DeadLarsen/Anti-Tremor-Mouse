use crate::calibration::CalibrationData;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub enum TremorType {
    Resting,    // Тремор в покое
    Action,     // Тремор при движении
    Mixed,      // Смешанный тип
    HighFrequency, // Высокочастотный тремор
    HighAmplitude, // Тремор с большой амплитудой
}

struct KalmanFilter {
    x: f64,      // State estimate
    p: f64,      // Error covariance
    q: f64,      // Process noise
    r: f64,      // Measurement noise
}

impl KalmanFilter {
    fn new(x: f64, p: f64, q: f64, r: f64) -> Self {
        println!("=== Инициализация фильтра Калмана ===");
        println!("Начальные параметры:");
        println!("- x: {:.2}", x);
        println!("- p: {:.2}", p);
        println!("- q: {:.2}", q);
        println!("- r: {:.2}", r);
        println!("================================");
        
        KalmanFilter { x, p, q, r }
    }

    fn update(&mut self, measurement: f64) -> f64 {
        // Prediction
        let x_pred = self.x;
        let p_pred = self.p + self.q;

        // Update
        let k = p_pred / (p_pred + self.r); // Kalman gain
        self.x = x_pred + k * (measurement - x_pred);
        self.p = (1.0 - k) * p_pred;

        println!("=== Шаг фильтра Калмана ===");
        println!("Измерение: {:.2}", measurement);
        println!("Коэффициент Калмана: {:.2}", k);
        println!("Обновленное состояние: {:.2}", self.x);
        println!("Обновленная ковариация: {:.2}", self.p);
        println!("=========================");

        self.x
    }

    fn adjust_noise(&mut self, new_r: f64) {
        self.r = new_r;
    }
}

pub struct TremorFilter {
    kalman_x: KalmanFilter,
    kalman_y: KalmanFilter,
    smoothing_factor: f64,
    tremor_type: TremorType,
    movement_history: VecDeque<(f64, f64)>,
    frequency_history: VecDeque<f64>,
    amplitude_history: VecDeque<f64>,
}

impl TremorFilter {
    pub fn new(calibration: CalibrationData) -> Self {
        println!("=== Инициализация TremorFilter ===");
        println!("Параметры калибровки:");
        println!("- Базовая стабильность: {:.2}", calibration.baseline_stability);
        println!("- Амплитуда тремора: {:.2}", calibration.tremor_amplitude);
        println!("- Частота тремора: {:.2}", calibration.tremor_frequency);
        println!("================================");

        let kalman_x = KalmanFilter::new(
            0.0,
            1.0,
            0.1,
            5.0,
        );
        let kalman_y = KalmanFilter::new(
            0.0,
            1.0,
            0.1,
            5.0,
        );

        TremorFilter {
            kalman_x,
            kalman_y,
            smoothing_factor: 0.7, // Устанавливаем фиксированный фактор сглаживания
            tremor_type: TremorType::Mixed,
            movement_history: VecDeque::with_capacity(100),
            frequency_history: VecDeque::with_capacity(50),
            amplitude_history: VecDeque::with_capacity(50),
        }
    }

    pub fn process_movement(&mut self, x: f64, y: f64) -> (f64, f64) {
        // Сохраняем историю движений
        self.movement_history.push_back((x, y));
        if self.movement_history.len() > 100 {
            self.movement_history.pop_front();
        }

        // Применяем фильтр Калмана
        let filtered_x = self.kalman_x.update(x);
        let filtered_y = self.kalman_y.update(y);

        // Применяем дополнительное сглаживание
        let smoothed_x = x + (filtered_x - x) * self.smoothing_factor;
        let smoothed_y = y + (filtered_y - y) * self.smoothing_factor;

        println!("=== Отладка фильтрации ===");
        println!("Входные координаты: ({:.2}, {:.2})", x, y);
        println!("После фильтра Калмана: ({:.2}, {:.2})", filtered_x, filtered_y);
        println!("После сглаживания: ({:.2}, {:.2})", smoothed_x, smoothed_y);
        println!("Разница: dx={:.2}, dy={:.2}", smoothed_x - x, smoothed_y - y);
        println!("Параметры фильтра:");
        println!("- Фактор сглаживания: {:.2}", self.smoothing_factor);
        println!("- Тип тремора: {:?}", self.tremor_type);
        println!("========================");

        (smoothed_x, smoothed_y)
    }

    fn adjust_filter_parameters(&mut self) {
        // Анализируем историю движений для определения типа тремора
        if self.movement_history.len() < 10 {
            return;
        }

        // Вычисляем амплитуду и частоту тремора
        let (amplitude, frequency) = self.calculate_tremor_metrics();
        
        // Обновляем историю метрик
        self.amplitude_history.push_back(amplitude);
        self.frequency_history.push_back(frequency);
        
        if self.amplitude_history.len() > 50 {
            self.amplitude_history.pop_front();
        }
        if self.frequency_history.len() > 50 {
            self.frequency_history.pop_front();
        }

        // Определяем тип тремора на основе метрик
        self.tremor_type = if frequency > 8.0 {
            TremorType::HighFrequency
        } else if amplitude > 5.0 {
            TremorType::HighAmplitude
        } else if amplitude < 0.1 {
            TremorType::Resting
        } else if frequency > 0.3 {
            TremorType::Action
        } else {
            TremorType::Mixed
        };

        // Адаптируем параметры фильтра в зависимости от типа тремора
        match self.tremor_type {
            TremorType::Resting => {
                self.smoothing_factor = 0.9; // Максимальное сглаживание для тремора в покое
            }
            TremorType::Action => {
                self.smoothing_factor = 0.5; // Минимальное сглаживание для тремора при движении
            }
            TremorType::HighFrequency => {
                self.smoothing_factor = 0.8; // Сильное сглаживание для высокочастотного тремора
            }
            TremorType::HighAmplitude => {
                self.smoothing_factor = 0.6; // Умеренное сглаживание для тремора с большой амплитудой
            }
            TremorType::Mixed => {
                self.smoothing_factor = 0.7; // Баланс между сглаживанием и отзывчивостью
            }
        }
    }

    fn calculate_tremor_metrics(&self) -> (f64, f64) {
        if self.movement_history.is_empty() {
            return (0.0, 0.0);
        }

        let mut total_movement = 0.0;
        let mut zero_crossings = 0;
        let mut prev_movement = 0.0;

        for (x, y) in self.movement_history.iter() {
            let movement = (x * x + y * y).sqrt();
            total_movement += movement;

            if prev_movement * movement < 0.0 {
                zero_crossings += 1;
            }
            prev_movement = movement;
        }

        let avg_movement = total_movement / self.movement_history.len() as f64;
        let frequency = zero_crossings as f64 / self.movement_history.len() as f64;

        (avg_movement, frequency)
    }

    pub fn get_tremor_metrics(&self) -> (f64, f64, TremorType) {
        let avg_frequency = if self.frequency_history.len() > 0 {
            self.frequency_history.iter().sum::<f64>() / self.frequency_history.len() as f64
        } else { 0.0 };
        let avg_amplitude = if self.amplitude_history.len() > 0 {
            self.amplitude_history.iter().sum::<f64>() / self.amplitude_history.len() as f64
        } else { 0.0 };
        (avg_frequency, avg_amplitude, self.tremor_type)
    }
} 
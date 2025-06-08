# Mouse Tremor Stabilization

[English](#english) | [Русский](#russian)

## English

Mouse Tremor Stabilization is a desktop application designed to help people with hand tremors use a computer mouse more effectively. The application uses advanced filtering algorithms to reduce unwanted mouse movements while maintaining precise control.

### Features

- Real-time tremor filtering using Kalman filter
- Adaptive filtering based on tremor type and characteristics
- Visual feedback with movement graphs
- Automatic calibration
- Support for different types of tremors:
  - Resting tremor
  - Action tremor
  - High-frequency tremor
  - High-amplitude tremor
  - Mixed type

### Requirements

- Windows 10 or later
- Rust 1.70.0 or later

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/mouse-tremor-stop.git
cd mouse-tremor-stop
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

### Usage

1. When you first start the application, it will perform a calibration:
   - Keep your mouse still for 5 seconds
   - The application will analyze your tremor characteristics
   - A notification will appear when calibration is complete

2. After calibration:
   - The application will automatically filter your mouse movements
   - You can see the original (red) and filtered (green) movement graphs
   - The current tremor type and metrics are displayed in the window

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Русский

Mouse Tremor Stabilization - это десктопное приложение, разработанное для помощи людям с тремором рук в более эффективном использовании компьютерной мыши. Приложение использует продвинутые алгоритмы фильтрации для уменьшения нежелательных движений мыши при сохранении точного контроля.

### Возможности

- Фильтрация тремора в реальном времени с использованием фильтра Калмана
- Адаптивная фильтрация в зависимости от типа и характеристик тремора
- Визуальная обратная связь с графиками движения
- Автоматическая калибровка
- Поддержка различных типов тремора:
  - Тремор в покое
  - Тремор при движении
  - Высокочастотный тремор
  - Тремор с большой амплитудой
  - Смешанный тип

### Требования

- Windows 10 или новее
- Rust 1.70.0 или новее

### Установка

1. Клонируйте репозиторий:
```bash
git clone https://github.com/yourusername/mouse-tremor-stop.git
cd mouse-tremor-stop
```

2. Соберите проект:
```bash
cargo build --release
```

3. Запустите приложение:
```bash
cargo run --release
```

### Использование

1. При первом запуске приложение выполнит калибровку:
   - Держите мышь неподвижно в течение 5 секунд
   - Приложение проанализирует характеристики вашего тремора
   - Появится уведомление о завершении калибровки

2. После калибровки:
   - Приложение будет автоматически фильтровать движения мыши
   - Вы можете видеть графики исходного (красный) и отфильтрованного (зеленый) движения
   - В окне отображается текущий тип тремора и метрики

### Участие в разработке

Мы приветствуем ваш вклад в проект! Не стесняйтесь отправлять Pull Request.

### Лицензия

Этот проект распространяется под лицензией MIT - подробности в файле [LICENSE](LICENSE). 
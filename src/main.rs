use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::gpio::{Output, OutputPin};
use esp_idf_svc::hal::{gpio::PinDriver, peripherals::Peripherals};
use esp_idf_svc::wifi::ClientConfiguration;
use esp_idf_svc::wifi::EspWifi;

struct Engine<'d, P: OutputPin> {
    gpio: PinDriver<'d, P, Output>,
}

impl<'d, P: OutputPin> Engine<'d, P> {
    fn new(gpio: PinDriver<'d, P, Output>) -> Self {
        Self { gpio }
    }

    fn start(&mut self) -> anyhow::Result<()> {
        log::info!("setting low...");
        self.gpio.set_low()?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        log::info!("setting high...");
        self.gpio.set_high()?;
        Ok(())
    }
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let dp = Peripherals::take().unwrap();
    // let mut gpio = PinDriver::output(dp.pins.gpio8).unwrap();
    let mut gpio = PinDriver::output_od(dp.pins.gpio8).unwrap();
    gpio.set_high().unwrap();

    // let mut ssid = heapless::String::<heapless::consts::U32>::new();

    // let sysloop = EspSystemEventLoop::take().unwrap();
    // let wifi = EspWifi::new(dp.modem, sysloop, None).unwrap();
    // wifi.set_configuration(&esp_idf_svc::wifi::Configuration::Client(
    //     ClientConfiguration {
    //         ssid: heapless::string::String:: "shelfdev".into(),
    //         password: "somepass".into(),
    //         ..Default::default()
    //     },
    // ));

    // wifi.start().unwrap();
    // wifi.connect().unwrap();

    log::info!("waiting 10 secs...");
    std::thread::sleep(std::time::Duration::from_secs(10));

    let mut engine = Engine::new(gpio);
    engine.start().unwrap();

    loop {
        log::info!("sleeping 10 secs...");
        // gpio.toggle().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

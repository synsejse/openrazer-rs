//! Each test launches its own dbus-daemon and connects only to its returned
//! address. No inherited session address or desktop/hardware service is used.
use openrazer::{Client, Error, methods::*};
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use zbus::{connection::Builder, fdo, interface};
struct Bus {
    child: Child,
    address: String,
}
impl Bus {
    fn start() -> Self {
        let mut child = Command::new("dbus-daemon")
            .args(["--session", "--nofork", "--print-address=1"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("install dbus-daemon to run offline tests");
        let mut address = String::new();
        BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut address)
            .unwrap();
        Self {
            child,
            address: address.trim().into(),
        }
    }
    async fn client(&self) -> Client {
        Client::from_connection(
            Builder::address(self.address.as_str())
                .build()
                .await
                .unwrap(),
        )
    }
}
impl Drop for Bus {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
struct Root;
#[interface(name = "razer.devices")]
impl Root {
    #[zbus(name = "getDevices")]
    fn devices(&self) -> Vec<String> {
        vec!["FAKE".into()]
    }
}
struct Dpi(Arc<AtomicUsize>);
#[interface(name = "razer.device.dpi")]
impl Dpi {
    #[zbus(name = "getDPI")]
    fn dpi(&self) -> Vec<i32> {
        vec![800, 0]
    }
    #[zbus(name = "setDPI")]
    fn set_dpi(&self, x: u16, y: u16) -> fdo::Result<()> {
        self.0.fetch_add(1, Ordering::SeqCst);
        assert_eq!((x, y), (900, 901));
        Err(fdo::Error::Failed("synthetic write failure".into()))
    }
    #[zbus(name = "getDPIStages")]
    fn stages(&self) -> ((u8, Vec<(u16, u16)>),) {
        ((1, vec![(800, 800), (1600, 1600)]),)
    }
    #[zbus(name = "setDPIStages")]
    fn set_stages(&self, active: u8, stages: Vec<(u16, u16)>) {
        assert_eq!((active, stages), (2, vec![(800, 800), (1600, 1600)]));
    }
    #[zbus(name = "maxDPI")]
    async fn maximum(&self) -> i32 {
        tokio::time::sleep(Duration::from_millis(100)).await;
        30000
    }
}
struct Power;
#[interface(name = "razer.device.power")]
impl Power {
    #[zbus(name = "getBattery")]
    fn battery(&self) -> f64 {
        -1.0
    }
    #[zbus(name = "isCharging")]
    fn charging(&self) -> bool {
        false
    }
    // Deliberately incompatible: must be rejected by capability checks.
    #[zbus(name = "setIdleTime")]
    fn idle(&self, _seconds: i32) {
        panic!("incompatible setter dispatched")
    }
}
struct Lighting;
#[interface(name = "razer.device.lighting.chroma")]
impl Lighting {
    #[zbus(name = "setKeyRow")]
    fn row(&self, bytes: Vec<u8>) {
        assert_eq!(bytes, vec![0, 0, 0, 255, 0, 128]);
    }
    #[zbus(name = "setStarlightSingle")]
    fn starlight(&self, r: u8, g: u8, b: u8, speed: u8) {
        assert_eq!((r, g, b, speed), (1, 2, 3, 2));
    }
    #[zbus(name = "getEffectColors")]
    fn colors(&self) -> Vec<u8> {
        vec![1, 2, 3]
    }
}
#[tokio::test]
async fn typed_roundtrips_errors_timeouts_and_unsupported() {
    let bus = Bus::start();
    let writes = Arc::new(AtomicUsize::new(0));
    let server = Builder::address(bus.address.as_str())
        .name("org.razer")
        .serve_at("/org/razer", Root)
        .serve_at("/org/razer/device/FAKE", Dpi(writes.clone()))
        .serve_at("/org/razer/device/FAKE", Power)
        .serve_at("/org/razer/device/FAKE", Lighting)
        .build()
        .await
        .unwrap();
    let client = bus.client().await;
    assert_eq!(client.serials().await.unwrap(), vec!["FAKE"]);
    let device = client.device("FAKE").await.unwrap();
    assert_eq!(
        device.call(&device_dpi::GET_DPI, &()).await.unwrap(),
        (vec![800, 0],)
    );
    assert_eq!(
        device.call(&device_dpi::GET_DPI_STAGES, &()).await.unwrap(),
        ((1, vec![(800, 800), (1600, 1600)]),)
    );
    device
        .call(
            &device_dpi::SET_DPI_STAGES,
            &(2, vec![(800, 800), (1600, 1600)]),
        )
        .await
        .unwrap();
    assert_eq!(
        device.call(&device_power::GET_BATTERY, &()).await.unwrap(),
        (-1.0,)
    );
    assert!(matches!(
        device.call(&device_power::SET_IDLE_TIME, &(10,)).await,
        Err(Error::Unsupported(_))
    ));
    assert!(matches!(
        device.call(&device_misc::GET_DEVICE_NAME, &()).await,
        Err(Error::Unsupported(_))
    ));
    let err = device
        .call(&device_dpi::SET_DPI, &(900, 901))
        .await
        .unwrap_err();
    assert!(err.to_string().contains("synthetic write failure"));
    assert_eq!(writes.load(Ordering::SeqCst), 1);
    device
        .call(
            &device_lighting_chroma::SET_KEY_ROW,
            &(vec![0, 0, 0, 255, 0, 128],),
        )
        .await
        .unwrap();
    device
        .call(&device_lighting_chroma::SET_STARLIGHT_SINGLE, &(1, 2, 3, 2))
        .await
        .unwrap();
    assert_eq!(
        device
            .call(&device_lighting_chroma::GET_EFFECT_COLORS, &())
            .await
            .unwrap(),
        (vec![1, 2, 3],)
    );
    assert!(matches!(
        client.device("../bad").await,
        Err(Error::InvalidSerial)
    ));
    let fast = client.clone().with_timeout(Duration::from_millis(10));
    let error: Result<i32, _> = fast
        .call(device.path(), "razer.device.dpi", "maxDPI", &())
        .await;
    assert!(matches!(error, Err(Error::Timeout)));
    let malformed: Result<String, _> = client
        .call(device.path(), "razer.device.dpi", "getDPI", &())
        .await;
    assert!(matches!(malformed, Err(Error::Bus(_))));
    drop(server);
}
#[tokio::test]
async fn absent_daemon_is_an_error() {
    let bus = Bus::start();
    assert!(matches!(
        bus.client().await.serials().await,
        Err(Error::Bus(_))
    ));
}

#[tokio::test]
async fn hotplug_and_owner_events_are_observable() {
    use futures_util::StreamExt;
    let bus = Bus::start();
    let client = bus.client().await;
    let mut owners = client.owner_events().await.unwrap();
    let server = Builder::address(bus.address.as_str())
        .name("org.razer")
        .serve_at("/org/razer", Root)
        .build()
        .await
        .unwrap();
    let event = tokio::time::timeout(Duration::from_secs(2), owners.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let (name, old, new): (String, String, String) = event.body().deserialize().unwrap();
    assert_eq!(name, "org.razer");
    assert!(old.is_empty() && !new.is_empty());
    let mut events = client.device_events().await.unwrap();
    for member in ["device_added", "device_removed"] {
        server
            .emit_signal(None::<&str>, "/org/razer", "razer.devices", member, &())
            .await
            .unwrap();
        let event = tokio::time::timeout(Duration::from_secs(2), events.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(event.header().member().unwrap().as_str(), member);
        event.body().deserialize::<()>().unwrap();
    }
    server.release_name("org.razer").await.unwrap();
    let event = tokio::time::timeout(Duration::from_secs(2), owners.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let (_, old, new): (String, String, String) = event.body().deserialize().unwrap();
    assert!(!old.is_empty() && new.is_empty());
}

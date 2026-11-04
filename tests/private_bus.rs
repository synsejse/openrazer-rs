//! Each test launches its own dbus-daemon and connects only to its returned
//! address. No inherited session address or desktop/hardware service is used.

use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::Duration,
};

use futures_util::StreamExt;
use openrazer::{Client, Error, methods::*};
use zbus::{
    Connection, MessageStream,
    connection::Builder,
    fdo, interface,
    message::{Flags, Header, Message},
};

const DEVICE: &str = "/org/razer/device/FAKE";

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

    /// Own `org.razer` with a fake root object only.
    async fn serve_root(&self, root: Root) -> Connection {
        Builder::address(self.address.as_str())
            .name("org.razer")
            .serve_at("/org/razer", root)
            .build()
            .await
            .unwrap()
    }

    /// Own `org.razer` with a fake device; `writes` counts `setDPI` calls.
    async fn serve_device(&self, writes: Arc<AtomicUsize>) -> Connection {
        Builder::address(self.address.as_str())
            .name("org.razer")
            .serve_at("/org/razer", Root::default())
            .serve_at(DEVICE, Dpi(writes))
            .serve_at(DEVICE, Power)
            .serve_at(DEVICE, Lighting)
            .build()
            .await
            .unwrap()
    }
}

impl Drop for Bus {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[derive(Default)]
struct Root {
    forbade_auto_start: Arc<AtomicBool>,
}

#[interface(name = "razer.devices")]
impl Root {
    #[zbus(name = "getDevices")]
    fn devices(&self, #[zbus(header)] header: Header<'_>) -> Vec<String> {
        let flags = header.primary().flags();
        self.forbade_auto_start
            .store(flags.contains(Flags::NO_AUTO_START), Ordering::SeqCst);
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

async fn next_event(stream: &mut MessageStream) -> Message {
    tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap()
}

#[tokio::test]
async fn typed_calls_roundtrip_exact_wire_shapes() {
    let bus = Bus::start();
    let _server = bus.serve_device(Arc::default()).await;
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
}

#[tokio::test]
async fn unadvertised_methods_are_not_dispatched() {
    let bus = Bus::start();
    let _server = bus.serve_device(Arc::default()).await;
    let device = bus.client().await.device("FAKE").await.unwrap();

    assert!(matches!(
        device.call(&device_power::SET_IDLE_TIME, &(10,)).await,
        Err(Error::Unsupported(_))
    ));
    assert!(matches!(
        device.call(&device_misc::GET_DEVICE_NAME, &()).await,
        Err(Error::Unsupported(_))
    ));
}

#[tokio::test]
async fn a_failed_setter_reports_its_error_and_is_sent_once() {
    let bus = Bus::start();
    let writes = Arc::new(AtomicUsize::new(0));
    let _server = bus.serve_device(writes.clone()).await;
    let device = bus.client().await.device("FAKE").await.unwrap();

    let error = device
        .call(&device_dpi::SET_DPI, &(900, 901))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("synthetic write failure"));
    assert_eq!(writes.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn invalid_serials_never_reach_the_bus() {
    let bus = Bus::start();
    let client = bus.client().await;

    for serial in ["", "../bad", "with/slash", "with-dash"] {
        assert!(matches!(
            client.device(serial).await,
            Err(Error::InvalidSerial)
        ));
    }
}

#[tokio::test]
async fn timeouts_and_malformed_replies_are_distinct_errors() {
    let bus = Bus::start();
    let _server = bus.serve_device(Arc::default()).await;
    let client = bus.client().await;

    let fast = client.clone().with_timeout(Duration::from_millis(10));
    let error: Result<i32, _> = fast.call(DEVICE, "razer.device.dpi", "maxDPI", &()).await;
    assert!(matches!(error, Err(Error::Timeout)));

    let malformed: Result<String, _> = client.call(DEVICE, "razer.device.dpi", "getDPI", &()).await;
    assert!(matches!(malformed, Err(Error::Bus(_))));
}

#[tokio::test]
async fn calls_forbid_starting_the_daemon() {
    let bus = Bus::start();
    let forbade_auto_start = Arc::new(AtomicBool::new(false));
    let _server = bus
        .serve_root(Root {
            forbade_auto_start: forbade_auto_start.clone(),
        })
        .await;

    bus.client().await.serials().await.unwrap();
    assert!(forbade_auto_start.load(Ordering::SeqCst));
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
    let bus = Bus::start();
    let client = bus.client().await;
    let mut owners = client.owner_events().await.unwrap();
    let server = bus.serve_root(Root::default()).await;

    let event = next_event(&mut owners).await;
    let (name, old, new): (String, String, String) = event.body().deserialize().unwrap();
    assert_eq!(name, "org.razer");
    assert!(old.is_empty() && !new.is_empty());

    let mut events = client.device_events().await.unwrap();
    for member in ["device_added", "device_removed"] {
        server
            .emit_signal(None::<&str>, "/org/razer", "razer.devices", member, &())
            .await
            .unwrap();
        let event = next_event(&mut events).await;
        assert_eq!(event.header().member().unwrap().as_str(), member);
        event.body().deserialize::<()>().unwrap();
    }

    server.release_name("org.razer").await.unwrap();
    let event = next_event(&mut owners).await;
    let (_, old, new): (String, String, String) = event.body().deserialize().unwrap();
    assert!(!old.is_empty() && new.is_empty());
}

#[tokio::test]
async fn daemon_activation_is_an_explicit_reversible_opt_in() {
    let bus = Bus::start();
    let forbade_auto_start = Arc::new(AtomicBool::new(true));
    let _server = bus
        .serve_root(Root {
            forbade_auto_start: forbade_auto_start.clone(),
        })
        .await;
    let client = bus.client().await.with_auto_start(true);
    client.serials().await.unwrap();
    assert!(!forbade_auto_start.load(Ordering::SeqCst));
    client.with_auto_start(false).serials().await.unwrap();
    assert!(forbade_auto_start.load(Ordering::SeqCst));
}

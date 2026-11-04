# Migration to openrazer 0.2

This fork preserves the original GitLab history and MIT copyright. Version 0.2
intentionally replaces the 0.1 device-specific wrappers with one introspected
`Endpoint`, and replaces dbus-async/Tokio 0.2 with Tokio 1 / zbus 6.1.0.
The zbus dependency is pinned to the consuming application's 6.1.0 version.
Rust 1.99 is the tested minimum; older Rust releases have not been tested.

## Usage

```rust,no_run
use openrazer::{Client, methods::{device_dpi, device_power}};
# async fn example() -> Result<(), openrazer::Error> {
let client = Client::connect().await?;
for serial in client.serials().await? {
    let device = client.device(&serial).await?;
    if device.capabilities().supports(&device_dpi::GET_DPI) {
        let (axes,) = device.call(&device_dpi::GET_DPI, &()).await?;
        println!("{serial}: {axes:?}");
    }
    if device.capabilities().supports(&device_power::GET_BATTERY) {
        let (battery,) = device.call(&device_power::GET_BATTERY, &()).await?;
        // -1.0 means unavailable; do not turn it into a percentage.
        println!("battery wire value: {battery}");
    }
}
# Ok(()) }
```

All method arguments and replies are **argument-list tuples**. A single argument
uses `(value,)`; no arguments use `()`. A struct output therefore has an extra
outer tuple. `GET_DPI_STAGES` returns `((u8, Vec<(u16,u16)>),)`; `SET_DPI_STAGES`
takes `(u8, Vec<(u16,u16)>)`. The active index is one-based. No silent narrowing,
clamping, retry or default-value substitution is performed.

For example, an explicitly requested DPI setter is
`device.call(&device_dpi::SET_DPI, &(x_u16, y_u16)).await?`.
Calling a setter acknowledges the daemon reply only; hardware readback remains
an application responsibility. Constructors, discovery and event subscriptions
never invoke hardware setters. Calls set D-Bus `NO_AUTO_START`.

## razerr-rs API map

Add `openrazer = { git = "https://github.com/synsejse/openrazer-rs", rev = "<second commit>" }`
to workspace dependencies and to the adapter crate. Add that exact repository to
`deny.toml`'s `[sources].allow-git`; `unknown-git = "deny"` currently rejects it.
The adapter's package name `razerr-openrazer` need not change.

| Existing binding or responsibility | Replacement |
| --- | --- |
| `Client::connect`, connection storage | `openrazer::Client::connect` or `from_connection` |
| raw `getDevices` / serial enumeration | `Client::serials`; retain app `Serial` validation |
| raw Introspect and XML parser | `Client::device(serial)`, `Endpoint::capabilities` |
| exact `has_method` checks | `Introspection::has_method`, preferably `supports(&METHOD)` |
| identity, name, image, firmware, matrix/layout | `methods::device_misc::{GET_VID_PID, GET_DEVICE_NAME, GET_DEVICE_TYPE, GET_DEVICE_IMAGE, GET_FIRMWARE, GET_DRIVER_VERSION, GET_MATRIX_DIMENSIONS, GET_KEYBOARD_LAYOUT}` |
| image legacy JSON fallback | `device_misc::GET_RAZER_URLS`; keep JSON/image interpretation in app |
| DPI, maximum, discrete values, stages | `device_dpi::{GET_DPI, SET_DPI, MAX_DPI, AVAILABLE_DPI, GET_DPI_STAGES, SET_DPI_STAGES}` |
| polling rate / advertised rates | `device_misc::{GET_POLL_RATE, SET_POLL_RATE, GET_SUPPORTED_POLL_RATES}` |
| battery, charging, idle, threshold | `device_power::{GET_BATTERY, IS_CHARGING, GET_IDLE_TIME, SET_IDLE_TIME, GET_LOW_BATTERY_THRESHOLD, SET_LOW_BATTERY_THRESHOLD}` |
| main brightness | `device_lighting_brightness::{GET_BRIGHTNESS, SET_BRIGHTNESS}` |
| main effects/readback | `device_lighting_chroma::{SET_*, GET_EFFECT, GET_EFFECT_COLORS, GET_EFFECT_SPEED, GET_WAVE_DIR}` |
| ripple / random ripple | `device_lighting_custom::{SET_RIPPLE, SET_RIPPLE_RANDOM_COLOUR}` (`yyyd`, `d`) |
| logo, scroll, backlight, left, right effects | corresponding `device_lighting_*` module, e.g. `device_lighting_logo::SET_LOGO_STATIC` |
| daemon version/sync/screensaver | root endpoint with `daemon::*`, `devices::*` |
| custom frame upload | `device_lighting_chroma::{SET_KEY_ROW, SET_CUSTOM}`; app builds row bytes |
| hotplug watcher | `Client::device_events()`; filter `device_added` / `device_removed`, both have empty bodies |
| owner watcher | `Client::owner_events()`; deserialize `(String, String, String)` |
| timeout handling | `Error::Timeout`, default 10 seconds; `Client::with_timeout` |
| daemon/transport failure | `Error::Bus(zbus::Error)` preserves the underlying error |
| missing or incompatible capability | `Error::Unsupported(MethodInfo)` before dispatch |
| malformed introspection | `Error::Introspection` |
| app `probe`, `live`, model construction, warnings | remain in app, using these endpoints |
| worker queue, identity check, readback, cache/TTL, retries/reconnect | remain in app |

`Client::call` is a typed low-level escape hatch without a capability check.
Prefer `Endpoint::call` with the generated descriptors to eliminate local
interface strings and wire signatures. `methods::ALL` exposes inventory metadata
for tooling. Check getter and setter independently: a readable method does not
imply a writable control. A failed optional getter must still produce the app's
warning/unavailable reading, not abort enumeration of unrelated devices.

Retain app handling of inactive receivers (`getDPI` zero/invalid replies), legacy
single-axis DPI, battery `-1`, invalid percentages, fallback polling lists,
transport VID/PID checks immediately before writes, delayed readback, stale-cache
expiry, and partial enumeration failures. These are not encoded as device values
by this library. Recreate endpoints after daemon owner changes or transport
handoffs; capability snapshots do not refresh automatically.

**Wave-direction discrepancy:** upstream Python constants and the old Rust API
use right=1, left=2. The app's current `write.rs` maps Left=1, Right=2. Bindings
accept the exact `i32` wire value; preserve existing app behavior during migration
and handle any user-visible relabeling as a separate deliberate decision.

## Migration from the old Rust API

`DeviceManager::new()` becomes `Client::connect()`; `devices()` becomes `serials()`
followed by independent `device()` calls. There is no name-based `Huntsman` enum:
all devices use advertised capabilities, including devices added after compilation.
The old `GenericMethods`, effect wrappers, frame indexing helpers and capability
boolean structs are removed. Use descriptors and tuples, and build frame bytes in
the caller. Macro payloads, supported-device metadata, URLs and device mode remain
wire JSON strings; the library does not impose a schema beyond upstream's `s`.

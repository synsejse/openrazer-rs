# OpenRazer source audit

Canonical source: https://github.com/openrazer/openrazer, confirmed through the
OpenRazer GitHub organization. Fresh full clone used for comparison:
`/tmp/openrazer-audit.FD6zMF/source`.

Audited HEAD: **477f4d6d3ab7e9deb6310d5bd3742560c5cbe80d**.
Original Rust HEAD: **68e04813c055d4651c132d26823157ac3079fe9f** (2020-08-27).
Historical OpenRazer comparison: **371745ac7f3d0d4ab4667060489907b267490cc6**
(last commit before 2020-08-28). The source inventories contain 134 historical
and 225 current distinct signatures: 103 added and 12 removed signature records.
See `changes-since-2020.json` for the exact delta, including replacement signatures.

## Audit method and scope

`tools/audit.py` parses every Python file below `daemon/` with Python's AST.
It extracts `@endpoint` declarations and registration tuples, without importing
or executing daemon code. This includes `daemon.py` root registration and
`hardware/device_base.py` unconditional and conditional effect-readback methods,
not only `dbus_methods/*.py`. All `add_dbus_method` call sites were inspected:
service.py supplies registration, daemon.py registers root methods, and
hardware/device_base.py registers base, effect and device-selected endpoints.

`docs/protocol.json` records all 225 signatures and their source file/line
locations. `tools/generate.py` emits typed descriptors for all of them and an
aggregate synthetic XML fixture. There are no unbound audited method signatures.
Multiple implementations sharing the same wire signature are represented by a
single descriptor; this does not claim identical hardware behavior.

The Python client `client/device_manager.py`, `client/devices/mice.py`,
`client/fx.py`, `client/constants.py`, daemon endpoint implementations, base-device
registration and relevant commit history were also inspected for semantic rules.
The source audit covers these families:

- Daemon version/stop, enumeration, supported-device JSON, sync/screensaver policy.
- Identity, firmware/driver, images and legacy URLs, VID/PID, type, matrix/layout,
  device mode, suspend/resume and dedicated-macro-key discovery.
- DPI and stages, advertised limits, polling, battery/charging, idle/low battery.
- Main and per-zone lighting; breathing, wave, spectrum, reactive, starlight,
  custom rows, ripple, wheel, on/off and effect readback.
- Charging, fast-charging and fully-charged zones; charging color/effect.
- ARGB channel count, size and brightness; profile LEDs; legacy BW2013/Kraken.
- Keyboard game/macro mode, macro JSON operations, key switch optimization.
- Scroll mode, acceleration, smart reel, mug presence and dongle indicators.

Only protocol facts are extracted. No upstream GPL implementation is copied into
this MIT library. The original MIT LICENSE and author metadata are unchanged.

## Behavioral conclusions

DPI getter replies use signed `ai`, setters use `qq`. Polling getter uses `i`,
setter `q`, supported rates `aq`. DPI-stage getter is **one struct output**,
`(ya(qq))`, while its setter is two arguments `ya(qq)`. Battery is a double with
`-1` as unavailable. Idle time is `q`; threshold and RGB channels are `y`.
Reactive/starlight setters put RGB bytes before speed. Brightness is `d`.
Ripple refresh is `d`, on the custom interface. Wave direction is `i`, with
upstream right=1 and left=2. Custom rows are `ay`, beginning each row with row,
start-column and end-column followed by RGB triplets.

Effect getters expose daemon-maintained state and may not be a fresh hardware
observation. Device registration is conditional, so never infer capabilities from
model names or the aggregate inventory. Existing serials can switch USB transport
and advertise different methods. Hotplug signals have no payload. Owner changes
must invalidate retained capability/identity state.

## Validation and limits

Offline tests exercise all 225 descriptors' input/output Rust serialization types,
all inventory signatures and wrong-signature rejection, and the app's wired and
wireless DeathAdder introspection captures. Those captures are existing fixtures,
not hardware observations made during this work; provenance is in fixtures/README.

Private-bus tests spawn their own `dbus-daemon --session --nofork`, connect via its
returned address, own a fake org.razer and terminate only that child process.
They test enumeration, exact struct/array/byte/double roundtrips, battery sentinel,
unsupported and incompatible methods, preserved setter failure without retry,
malformed replies, timeout, invalid serials, absent daemon, hotplug and owner events.
No test uses the inherited desktop bus. No real hardware setter or service restart
was performed. No real hardware semantics, lighting rendering, macro execution,
firmware behavior or cross-device synchronization are verified.

The API is a protocol binding rather than a full Python-client reimplementation.
It does not provide macro JSON editors, animation/frame scheduling, persistence,
application models, value-range policy, cached readbacks or automatic reconnect.
Unknown future methods require `Client::call` or regeneration; absent methods on
older daemons produce `Unsupported`. Private non-D-Bus driver/sysfs APIs are not
covered. Old 0.1 wrapper source compatibility is intentionally unsupported.

Reproduce with Python 3, Rust 1.99+ and dbus-daemon:

```
python tools/audit.py /path/to/openrazer-at-audited-revision
python tools/generate.py
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo test --locked --doc
cargo doc --locked --no-deps
```

Generated source is checked in; consumers do not need Python or an OpenRazer
checkout to compile. CI checks builds and private-bus tests; regeneration against
a fresh upstream checkout is an explicit maintainer task.

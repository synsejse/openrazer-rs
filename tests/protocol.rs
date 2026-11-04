use openrazer::{Introspection, methods::*};

#[test]
fn complete_inventory_and_exact_signatures() {
    let caps = Introspection::parse(include_str!("fixtures/all.xml")).unwrap();
    assert_eq!(ALL.len(), 225);
    for m in ALL {
        assert!(
            caps.has_method(m.interface, m.name, m.input, m.output),
            "{m:?}"
        );
        assert!(!caps.has_method(m.interface, m.name, &format!("{}y", m.input), m.output));
        assert!(!caps.has_method(m.interface, m.name, m.input, &format!("{}y", m.output)));
        assert!(!caps.has_method("invalid.interface", m.name, m.input, m.output));
    }
}

#[test]
fn captures_distinguish_transport_and_missing_features() {
    for (xml, wireless) in [
        (include_str!("fixtures/deathadder_v3_pro.xml"), false),
        (
            include_str!("fixtures/deathadder_v3_pro_wireless.xml"),
            true,
        ),
    ] {
        let caps = Introspection::parse(xml).unwrap();
        assert!(caps.supports(&device_dpi::GET_DPI));
        assert!(caps.supports(&device_dpi::SET_DPI));
        assert!(caps.supports(&device_dpi::GET_DPI_STAGES));
        assert!(caps.supports(&device_dpi::SET_DPI_STAGES));
        assert_eq!(
            caps.supports(&device_misc::GET_SUPPORTED_POLL_RATES),
            wireless
        );
        assert!(!caps.supports(&device_lighting_chroma::SET_STATIC));
        assert!(!caps.has_method("razer.device.dpi", "getDPIStages", "", "ya(qq)"));
    }
}

#[test]
fn omitted_direction_is_input_and_malformed_xml_is_rejected() {
    let caps = Introspection::parse(
        r#"<node>
            <interface name="razer.device.dpi">
                <method name="setDPI"><arg type="q"/><arg type="q"/></method>
            </interface>
        </node>"#,
    )
    .unwrap();
    assert!(caps.supports(&device_dpi::SET_DPI));
    assert!(!caps.supports(&device_dpi::GET_DPI));
    assert!(Introspection::parse("<node><interface").is_err());
}

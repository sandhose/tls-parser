#[macro_use]
extern crate nom;
extern crate tls_parser;

mod tls_extensions {
use tls_parser::*;
use nom::IResult;

static CLIENT_EXTENSIONS1: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x13, 0x00, 0x11, 0x00, 0x00, 0x0e, 0x77, 0x77, 0x77, 0x2e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x00, 0x0b, 0x00, 0x04, 0x03, 0x00, 0x01, 0x02, 0x00,
    0x0a, 0x00, 0x1c, 0x00, 0x1a, 0x00, 0x17, 0x00, 0x19, 0x00, 0x1c, 0x00, 0x1b, 0x00, 0x18, 0x00,
    0x1a, 0x00, 0x16, 0x00, 0x0e, 0x00, 0x0d, 0x00, 0x0b, 0x00, 0x0c, 0x00, 0x09, 0x00, 0x0a, 0x00,
    0x23, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x20, 0x00, 0x1e, 0x06, 0x01, 0x06, 0x02, 0x06, 0x03, 0x05,
    0x01, 0x05, 0x02, 0x05, 0x03, 0x04, 0x01, 0x04, 0x02, 0x04, 0x03, 0x03, 0x01, 0x03, 0x02, 0x03,
    0x03, 0x02, 0x01, 0x02, 0x02, 0x02, 0x03, 0x00, 0x05, 0x00, 0x05, 0x01, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x0f, 0x00, 0x01, 0x01
];

#[test]
fn test_tls_extensions() {
    let empty = &b""[..];
    let bytes = CLIENT_EXTENSIONS1;
    let ec_point_formats = &[0,1,2];
    let ext1 = &[0, 0, 0, 0];
    let expected = IResult::Done(empty, vec![
        TlsExtension::SNI(vec![(0,b"www.google.com")]),
        TlsExtension::EcPointFormats(ec_point_formats),
        TlsExtension::EllipticCurves(vec![23, 25, 28, 27, 24, 26, 22, 14, 13, 11, 12, 9, 10]),
        TlsExtension::SessionTicket(&empty),
        TlsExtension::SignatureAlgorithms(vec![
            (6, 1), (6, 2), (6, 3), (5, 1), (5, 2), (5, 3), (4, 1), (4, 2), (4, 3), (3, 1), (3, 2), (3, 3), (2, 1), (2, 2), (2, 3)
        ]),
        TlsExtension::StatusRequest(Some((0x1,ext1))),
        TlsExtension::Heartbeat(1),
    ]);

    let res = parse_tls_extensions(bytes);

    assert_eq!(res,expected);
}

#[test]
fn test_tls_extension_max_fragment_length() {
    let empty = &b""[..];
    let bytes = &[
        0x00, 0x01, 0x00, 0x01, 0x04
    ];
    let expected = IResult::Done(empty,
        TlsExtension::MaxFragmentLength(4),
    );

    let res = parse_tls_extension(bytes);

    assert_eq!(res,expected);
}

#[test]
fn test_tls_extension_alpn() {
    let empty = &b""[..];
    let bytes = &[
        0x00, 0x10, 0x00, 0x29, 0x00, 0x27, 0x05, 0x68, 0x32, 0x2d, 0x31, 0x36,
        0x05, 0x68, 0x32, 0x2d, 0x31, 0x35, 0x05, 0x68, 0x32, 0x2d, 0x31, 0x34,
        0x02, 0x68, 0x32, 0x08, 0x73, 0x70, 0x64, 0x79, 0x2f, 0x33, 0x2e, 0x31,
        0x08, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x31, 0x2e, 0x31
    ];
    let expected = IResult::Done(empty,
        TlsExtension::ALPN(vec![
                           b"h2-16",
                           b"h2-15",
                           b"h2-14",
                           b"h2",
                           b"spdy/3.1",
                           b"http/1.1",
        ]),
    );

    let res = parse_tls_extension(bytes);

    assert_eq!(res,expected);
}

#[test]
fn test_tls_extension_encrypt_then_mac() {
    let empty = &b""[..];
    let bytes = &[
        0x00, 0x16, 0x00, 0x00
    ];
    let expected = IResult::Done(empty,
        TlsExtension::EncryptThenMac,
    );

    let res = parse_tls_extension(bytes);

    assert_eq!(res,expected);
}

#[test]
fn test_tls_extension_extended_master_secret() {
    let empty = &b""[..];
    let bytes = &[
        0x00, 0x17, 0x00, 0x00
    ];
    let expected = IResult::Done(empty,
        TlsExtension::ExtendedMasterSecret,
    );

    let res = parse_tls_extension(bytes);

    assert_eq!(res,expected);
}

#[test]
fn test_tls_extension_npn() {
    let empty = &b""[..];
    let bytes = &[
        0x33, 0x74, 0x00, 0x00
    ];
    let expected = IResult::Done(empty,
        TlsExtension::NextProtocolNegotiation,
    );

    let res = parse_tls_extension(bytes);

    assert_eq!(res,expected);
}

#[test]
fn test_tls_extension_list() {
    let empty = &b""[..];
    let bytes = &[
        0, 5, 0, 0, 0, 23, 0, 0, 255, 1, 0, 1, 0
    ];
    let expected = IResult::Done(empty, vec![
        TlsExtension::StatusRequest(None),
        TlsExtension::ExtendedMasterSecret,
        TlsExtension::RenegotiationInfo(&[]),
    ]
    );

    let res = parse_tls_extensions(bytes);
    println!("{:?}",res);

    assert_eq!(res,expected);
}

} // mod tls_extensions

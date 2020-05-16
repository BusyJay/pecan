pub mod conformance_pb;
pub mod google;

use std::io::*;
use pecan::{self, Message};
use std::{io, mem};
use self::conformance_pb::*;
use self::google::protobuf::test_messages_proto3_pb::TestAllTypesProto3;

#[derive(Debug)]
pub enum Error {
    Pecan(pecan::Error),
    Io(io::Error),
}

impl From<pecan::Error> for Error {
    fn from(e: pecan::Error) -> Error {
        Error::Pecan(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

fn do_test(req: ConformanceRequest, resp: &mut ConformanceResponse) -> Result<()> {
    if req.requested_output_format() == WireFormat::Json {
        resp.set_skipped("JSON not supported.".to_owned());
        return Ok(());
    }
    let payload = match &req.payload {
        Some(ConformanceRequestNestedPayload::ProtobufPayload(p)) => p.clone(),
        Some(ConformanceRequestNestedPayload::JsonPayload(_)) => {
            resp.set_skipped("JSON not supported.".to_owned());
            return Ok(());
        }
        Some(ConformanceRequestNestedPayload::TextPayload(_)) => {
            resp.set_skipped("TEXT format not suppored.".to_owned());
            return Ok(());
        }
        _ => {
            resp.set_skipped("Payload not set.".to_owned());
            return Ok(());
        }
    };
    let msg = if req.message_type() == "protobuf_test_messages.proto3.TestAllTypesProto3" {
        let mut msg = TestAllTypesProto3::new();
        if let Err(e) = msg.merge_from_bytes(payload.as_slice()) {
            resp.set_parse_error(format!("{}", e));
            return Ok(());
        }
        Box::new(msg)
    } else {
        resp.set_skipped(format!("unsupport message type: {}", req.message_type()));
        return Ok(());
    };
    match req.requested_output_format() {
        WireFormat::Protobuf => {
            let mut bytes = Vec::with_capacity(msg.len());
            msg.write_to_vec(&mut bytes)?;
            resp.set_protobuf_payload(bytes);
        },
        WireFormat::Json => {
            resp.set_skipped("JSON not supported.".to_owned());
        }
        WireFormat::TextFormat => {
            resp.set_skipped("TEXT format not suppored.".to_owned());
        }
        _ => {
            resp.set_skipped("Output not set.".to_owned());
        }
    };
    Ok(())
}

pub fn do_test_io() -> Result<bool> {
    let mut len_bytes: [u8; 4] = [0; 4];
    let mut stdin = stdin();
    match stdin.read_exact(&mut len_bytes) {
        Ok(_) => (),
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(true),
        Err(e) => return Err(e.into()),
    }
    let length = u32::from_le_bytes(len_bytes) as usize;
    let mut bytes = vec![0; length];
    stdin.read_exact(&mut bytes)?;
    let mut req = conformance_pb::ConformanceRequest::default();
    req.merge_from_bytes(bytes.as_slice())?;

    let mut resp = ConformanceResponse::new();
    if let Err(e) = do_test(req, &mut resp) {
        resp.set_runtime_error(format!("{:?}", e));
    }

    let len = resp.len();
    let mut res = Vec::with_capacity(len);
    resp.write_to_vec(&mut res)?;
    assert_eq!(len, res.len());
    let mut stdout = stdout();
    stdout.write_all(unsafe { mem::transmute::<_, &[u8; 4]>(&len) })?;
    stdout.write_all(&res)?;
    stdout.flush()?;
    Ok(false)
}

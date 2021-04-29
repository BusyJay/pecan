mod conformance;
mod google;

use bytes::{Bytes, BytesMut};
use google::protobuf::{
    test_messages_proto2_pb::TestAllTypesProto2, test_messages_proto3_pb::TestAllTypesProto3,
};
use pecan::prelude::*;
use std::{
    io::Write,
    io::{ErrorKind, Read, Result},
    mem, u32,
};

use self::conformance::conformance_pb::*;

fn do_test(req: ConformanceRequest, resp: &mut ConformanceResponse) -> Result<()> {
    if req.message_type == "conformance.FailureSet" {
        let mut buf = BytesMut::default();
        FailureSet::new().write_to_buf(&mut buf).unwrap();
        resp.set_protobuf_payload(buf.freeze());
        return Ok(());
    }
    let msg = match req.payload {
        ConformanceRequest_Payload::ProtobufPayload(mut payload) => {
            let mut msg: Box<dyn BufMessage<Bytes, BytesMut>> =
                if req.message_type == "protobuf_test_messages.proto3.TestAllTypesProto3" {
                    Box::new(TestAllTypesProto3::new())
                } else if req.message_type == "protobuf_test_messages.proto2.TestAllTypesProto2" {
                    Box::new(TestAllTypesProto2::new())
                } else {
                    resp.set_skipped(format!("{} is not supported", req.message_type));
                    return Ok(());
                };

            if let Err(e) = msg.merge_from_buf(&mut payload) {
                resp.set_parse_error(format!("{:?}", e));
                return Ok(());
            }
            msg
        }
        payload => {
            resp.set_skipped(format!("{:?} is not supported", payload));
            return Ok(());
        }
    };

    match req.requested_output_format {
        WireFormat::PROTOBUF => {
            let mut buf = BytesMut::with_capacity(msg.size_buf() as usize);
            msg.write_to_buf(&mut buf).unwrap();
            resp.set_protobuf_payload(buf.freeze());
        }
        format => {
            resp.set_skipped(format!("{:?} is not supported", format));
            return Ok(());
        }
    }

    Ok(())
}

pub fn do_test_io(mut stdin: impl Read, mut stdout: impl Write) -> Result<bool> {
    let mut size_bytes = [0; 4];
    if let Err(e) = stdin.read_exact(&mut size_bytes) {
        if e.kind() == ErrorKind::UnexpectedEof {
            return Ok(false);
        }
        return Err(e);
    }
    let s = unsafe { mem::transmute::<_, u32>(size_bytes) as usize };
    let mut data = vec![0; s];
    stdin.read_exact(&mut data)?;
    let mut request = ConformanceRequest::new();
    request.merge_from_buf(&mut data.as_slice()).unwrap();
    let mut response = ConformanceResponse::new();
    do_test(request, &mut response)?;
    let l = response.size();
    let mut bytes = Vec::with_capacity(l as usize + 4);
    let bits = (l as u32).to_ne_bytes();
    bytes.extend_from_slice(&bits);
    response.write_to_buf(&mut bytes).unwrap();
    assert_eq!(bytes.len(), l as usize + 4);
    stdout.write_all(&bytes)?;
    stdout.flush().unwrap();
    Ok(true)
}

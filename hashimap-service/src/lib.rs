use serde::{Deserialize, Serialize};
#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::http::incoming_handler::{Guest, IncomingRequest, ResponseOutparam};
use bindings::macovedj::shapes::hashimap::Hashimap;
use bindings::wasi::http::types::{Fields, OutgoingBody, OutgoingResponse};

#[derive(Debug, Serialize, Deserialize)]
struct Body {
    function: String,
    key: Option<String>,
}
struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let map = Hashimap::new();
        let req_body = request.consume().unwrap();
        let stream = req_body.stream().unwrap();
        let body: Body = serde_json::from_slice(&stream.blocking_read(100).unwrap()).unwrap();

        dbg!("HAHSIMAP SERVICE");
        if body.function == "get" {
            let res = if let Some(key) = body.key {
                map.get(&key)
            } else {
                None
            };
            let hdrs = Fields::new();
            let resp = OutgoingResponse::new(hdrs);
            let body = resp.body().expect("outgoing response");

            ResponseOutparam::set(response_out, Ok(resp));

            let out = body.write().expect("outgoing stream");
            if let Some(res) = res {
                out.blocking_write_and_flush(&res.as_bytes())
                    .expect("writing response");
            }

            drop(out);
            OutgoingBody::finish(body, None).unwrap();
            return;
        } else if body.function == "keys" {
            let res = map.keys();
            let hdrs = Fields::new();
            let resp = OutgoingResponse::new(hdrs);
            let body = resp.body().expect("outgoing response");

            ResponseOutparam::set(response_out, Ok(resp));

            let out = body.write().expect("outgoing stream");
            out.blocking_write_and_flush(&serde_json::to_string(&res).unwrap().as_bytes())
                .expect("writing response");

            drop(out);
            OutgoingBody::finish(body, None).unwrap();
            return;
        }
    }
}

bindings::export!(Component with_types_in bindings);

#[allow(warnings)]
mod bindings;

use bindings::exports::macovedj::shapes::hashimap::{Guest, GuestHashimap};
use bindings::wasi::http::outgoing_handler::{handle, OutgoingRequest};
use bindings::wasi::http::types::{Fields, Method, Scheme};
use bindings::wasi::io::{poll, streams::InputStream};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ReqBody {
    function: String,
    key: Option<String>,
}
struct Component;

struct Hashimap;

impl GuestHashimap for Hashimap {
    fn new() -> Self {
        Self
    }
    fn get(&self, name: String) -> Option<String> {
        let fields = Fields::new();
        let req = OutgoingRequest::new(fields);
        req.set_method(&Method::Post).unwrap();
        req.set_scheme(Some(&Scheme::Http)).unwrap();
        req.set_path_with_query(Some("/")).unwrap();
        req.set_authority(Some("localhost:8080")).unwrap();
        let body = req.body().unwrap();
        let stream = body.write().unwrap();
        let to_write = ReqBody {
            function: "get".to_string(),
            key: Some(name),
        };
        stream.blocking_write_and_flush(serde_json::to_string(&to_write).unwrap().as_bytes());
        let res = handle(req, None).unwrap();
        let pollable = res.subscribe();
        let polled = poll::poll(&[&pollable]);

        let res = res.get().unwrap().unwrap().unwrap();
        let body = res.consume().unwrap();
        let stream = body.stream().unwrap();
        let bytes = &stream.blocking_read(100).unwrap();
        let thing = std::str::from_utf8(bytes).unwrap();
        let owned = thing.to_owned();
        Some(owned)
    }
    fn keys(&self) -> Vec<String> {
        let fields = Fields::new();
        let req = OutgoingRequest::new(fields);
        req.set_method(&Method::Post).unwrap();
        req.set_scheme(Some(&Scheme::Http)).unwrap();
        req.set_path_with_query(Some("/")).unwrap();
        req.set_authority(Some("localhost:8080")).unwrap();
        let body = req.body().unwrap();
        let stream = body.write().unwrap();
        let to_write = ReqBody {
            function: "keys".to_string(),
            key: None,
        };
        stream.blocking_write_and_flush(serde_json::to_string(&to_write).unwrap().as_bytes());
        let res = handle(req, None).unwrap();
        let pollable = res.subscribe();
        poll::poll(&[&pollable]);

        let res = res.get().unwrap().unwrap().unwrap();
        let body = res.consume().unwrap();
        let stream = body.stream().unwrap();
        let bytes = &stream.blocking_read(100).unwrap();
        let keys: Vec<String> = serde_json::from_slice(bytes).unwrap();
        keys.to_owned()
    }
}

impl Guest for Component {
    type Hashimap = Hashimap;
}

bindings::export!(Component with_types_in bindings);

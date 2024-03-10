use serde::{Deserialize, Serialize};
#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::http::incoming_handler::{Guest, IncomingRequest, ResponseOutparam};
use bindings::wasi::http::outgoing_handler::handle;
use bindings::wasi::http::types::{
    Fields, Method, OutgoingBody, OutgoingRequest, OutgoingResponse, Scheme,
};
use bindings::wasi::io::poll;
struct Component;

#[derive(Debug, Serialize, Deserialize)]
struct ReqBody {
    function: String,
    values: (String, String),
}
impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        // let map = Goodmap::new();
        let req_body = request.consume().unwrap();
        let stream = req_body.stream().unwrap();
        let in_body: ReqBody = serde_json::from_slice(&stream.blocking_read(100).unwrap()).unwrap();

        let fields = Fields::new();
        let out_req = OutgoingRequest::new(fields);
        out_req.set_method(&Method::Post).unwrap();
        out_req.set_scheme(Some(&Scheme::Http)).unwrap();
        out_req.set_path_with_query(Some("/")).unwrap();
        out_req.set_authority(Some("localhost:8080")).unwrap();
        let out_body = out_req.body().unwrap();
        let stream = out_body.write().unwrap();
        stream.blocking_write_and_flush(serde_json::to_string(&in_body).unwrap().as_bytes());

        let res = handle(out_req, None).unwrap();
        let pollable = res.subscribe();
        let polled = poll::poll(&[&pollable]);
        let res = res.get().unwrap().unwrap().unwrap();
        let body = res.consume().unwrap();
        let stream = body.stream().unwrap();
        let bytes = &stream.blocking_read(100).unwrap();
        dbg!("PREFUNCCHECK");
        dbg!(&in_body);
        if in_body.function == "keys" {
            let mut thing: Vec<String> = serde_json::from_slice(bytes).unwrap();
            thing.sort();
            let hdrs = Fields::new();
            let resp = OutgoingResponse::new(hdrs);
            let body = resp.body().expect("outgoing response");

            ResponseOutparam::set(response_out, Ok(resp));

            let out = body.write().expect("outgoing stream");
            out.blocking_write_and_flush(&serde_json::to_string(&thing).unwrap().as_bytes())
                .expect("writing response");

            drop(out);
            OutgoingBody::finish(body, None).unwrap();
        } else if in_body.function == "get" {
            let mut thing: String = std::str::from_utf8(bytes).unwrap().to_string();
            let hdrs = Fields::new();
            let resp = OutgoingResponse::new(hdrs);
            let body = resp.body().expect("outgoing response");

            ResponseOutparam::set(response_out, Ok(resp));

            let out = body.write().expect("outgoing stream");
            out.blocking_write_and_flush(&thing.as_bytes())
                .expect("writing response");

            drop(out);
            OutgoingBody::finish(body, None).unwrap();
        }

        // if body.function == "insert" {
        //     if let Some(v) = body.values {
        //         let key = v.0;
        //         let val = v.1;
        //     }
        // if body.function == "get" {}
        //     let res = if let Some(v) = body.values {
        //         let key = v.0;
        //         map.get(&key)
        //     } else {
        //         None
        //     };
        //     let hdrs = Fields::new();
        //     let resp = OutgoingResponse::new(hdrs);
        //     let body = resp.body().expect("outgoing response");

        //     ResponseOutparam::set(outparam, Ok(resp));

        //     let out = body.write().expect("outgoing stream");
        //     if let Some(res) = res {
        //         out.blocking_write_and_flush(&res.as_bytes())
        //             .expect("writing response");
        //     }

        //     drop(out);
        //     OutgoingBody::finish(body, None).unwrap();
        //     return;
        // } else if body.function == "keys" {
        // if body.function == "keys" {
        // dbg!("INSIDE GET BLOCK");
        // // let res = map.keys();
        // let hdrs = Fields::new();
        // let resp = OutgoingResponse::new(hdrs);
        // let body = resp.body().expect("outgoing response");

        // dbg!("WILL CALL SET");
        // ResponseOutparam::set(response_out, Ok(resp));

        // let out = body.write().expect("outgoing stream");
        // // if let Some(res) = res {
        // out.blocking_write_and_flush(&serde_json::to_string(&thing).unwrap().as_bytes())
        //     .expect("writing response");
        // // }

        // drop(out);
        // OutgoingBody::finish(body, None).unwrap();
        // return;
        // }
        // let hdrs = Fields::new();
        // let resp = OutgoingResponse::new(hdrs);
        // let body = resp.body().expect("outgoing response");

        // ResponseOutparam::set(response_out, Ok(resp));

        // let out = body.write().expect("outgoing stream");
        // // out.blocking_write_and_flush(foo.foo.as_bytes())
        // out.blocking_write_and_flush("hello world".as_bytes())
        //     .expect("writing response");

        // drop(out);
        // OutgoingBody::finish(body, None).unwrap();
    }
}

bindings::export!(Component with_types_in bindings);

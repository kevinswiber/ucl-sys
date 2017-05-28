#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate curl_sys;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn parse_and_emit_json() {
        let data = CString::new("section { key = \"value\" }").unwrap();

        let parser = unsafe { ucl_parser_new(0) };
        let _ = unsafe {
            ucl_parser_add_chunk(parser, data.as_ptr() as *const u8, data.to_bytes().len())
        };

        let obj = unsafe { ucl_parser_get_object(parser) };

        let json_ptr = unsafe { ucl_object_emit(obj, ucl_emitter::UCL_EMIT_JSON_COMPACT) };
        let json = unsafe { CStr::from_ptr(json_ptr as *const i8).to_str().unwrap() };

        assert_eq!("{\"section\":{\"key\":\"value\"}}", json);
    }
}

extern crate encoding;

extern crate cqpsdk;

use std::ffi::{CStr, CString};

use encoding::all::{GBK, UTF_8};
use encoding::{DecoderTrap, EncoderTrap, Encoding};

use cqpsdk::cqpapi;

// Some macros for convenience.
// Should have been integrated in evshiron/cqpsdk-rust.
// But just leave here for easy reading.

macro_rules! gbk {
    ( $x: expr ) => {
        CString::new(GBK.encode($x, EncoderTrap::Ignore).unwrap())
            .unwrap()
            .into_raw()
    };
}

macro_rules! utf8 {
    ( $x: expr ) => {
        &GBK.decode(CStr::from_ptr($x).to_bytes(), DecoderTrap::Ignore)
            .unwrap()[..]
    };
}

static mut client: cqpsdk::Client = cqpsdk::Client::new("com.mi.test");

// https://github.com/rust-lang/rust/issues/17806

#[export_name = "AppInfo"]
pub extern "stdcall" fn app_info() -> *const i8 {
    unsafe{
        return gbk!(client.app_info().as_str());
    }
}

#[export_name = "Initialize"]
pub extern "stdcall" fn initialize(AuthCode: i32) -> i32 {
    unsafe {
        client.initialize(AuthCode);
    }
    return cqpsdk::EVENT_IGNORE;
}

#[export_name = "PrivateMessageHandler"]
pub extern "stdcall" fn private_message_handler(
    subType: i32,
    sendTime: i32,
    qqNum: i64,
    msg: *const i8,
    font: i32,
) -> i32 {
    unsafe{
        return client.send_private_message(qqNum, utf8!(msg));
    }
}

#[export_name = "GroupMessageHandler"]
pub extern "stdcall" fn group_message_handler(
    subType: i32,
    sendTime: i32,
    groupNum: i64,
    qqNum: i64,
    anonymousName: *const i8,
    msg: *const i8,
    font: i32,
) -> i32 {
    return cqpsdk::EVENT_IGNORE;
}

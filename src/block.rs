use crate::context::Context;
use std::ffi::CString;
use std::os::raw::c_int;
use std::os::raw::c_long;
use std::ptr;

use crate::from_byte_string;
use crate::native_types::RedisType;
use crate::raw;
use crate::redismodule::REDIS_OK;
use crate::RedisValue;
use crate::RedisResult;
use crate::RedisString;

pub struct BlockedClient {
    blocked_client: *mut raw::RedisModuleBlockedClient,
}

impl Context {

    pub fn is_blocked_reply_request(&self) -> bool {
        unsafe { raw::RedisModule_IsBlockedReplyRequest.unwrap()(self.ctx) != 0 }
    }

    pub fn is_blocked_timeout_request(&self) -> bool {
        unsafe { raw::RedisModule_IsBlockedTimeoutRequest.unwrap()(self.ctx) != 0 }
    }

}

impl BlockedClient {
    pub fn new(ctx: &Context, timeout: i64) -> Self {
        let blocked_client: *mut raw::RedisModuleBlockedClient = unsafe {
            raw::RedisModule_BlockClient.unwrap()(
                ctx.ctx,
                Some(callback),
                Some(callback),
                None,
                timeout,
            )
        };
        BlockedClient { blocked_client }
    }
    pub fn unblock(&self) {
        unsafe { raw::RedisModule_UnblockClient.unwrap()(self.blocked_client, ptr::null_mut()) };
    }
}

unsafe impl std::marker::Send for BlockedClient {

}

unsafe extern "C" fn callback(
    ctx: *mut raw::RedisModuleCtx,
    argv: *mut *mut raw::RedisModuleString,
    argc: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    let ctx = Context::new(ctx);
    if ctx.is_blocked_reply_request() {
        ctx.log(crate::LogLevel::Notice, "reply callback invoked");
        ctx.reply(RedisResult::Ok(RedisValue::Integer(10)));
    } else if ctx.is_blocked_timeout_request() {
        ctx.log(crate::LogLevel::Notice, "timeout callback invoked");
        ctx.reply(RedisResult::Ok(RedisValue::Integer(-10)));
    }
    0
}

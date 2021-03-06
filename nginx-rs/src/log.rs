#[macro_export]
macro_rules! ngx_log_debug {
    ( $level:expr, $log:expr, $($arg:tt)* ) => {
        let log_level = unsafe { (*$log).log_level };
        if log_level & $level as usize != 0 {
            let level = $crate::bindings::NGX_LOG_DEBUG as $crate::bindings::ngx_uint_t;
            let fmt = ::std::ffi::CString::new("%s").unwrap();
            let c_message = ::std::ffi::CString::new(format!($($arg)*)).unwrap();
            unsafe {
                ngx_log_error_core(level, $log, 0, fmt.as_ptr(), c_message.as_ptr());
            }
        }
    }
}

#[macro_export]
macro_rules! ngx_log_debug_http {
    ( $request:expr, $($arg:tt)* ) => {
        let log = unsafe { (*$request.connection()).log };
        $crate::ngx_log_debug!(NGX_LOG_DEBUG_HTTP, log, $($arg)*);
    }
}

// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.33.

// Section: imports

use super::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate_io!();

// Section: dart2rust

impl CstDecode<flutter_rust_bridge::for_generated::anyhow::Error>
    for *mut wire_cst_list_prim_u_8_strict
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> flutter_rust_bridge::for_generated::anyhow::Error {
        unimplemented!()
    }
}
impl CstDecode<String> for *mut wire_cst_list_prim_u_8_strict {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> String {
        let vec: Vec<u8> = self.cst_decode();
        String::from_utf8(vec).unwrap()
    }
}
impl CstDecode<crate::model::ConnectRequest> for *mut wire_cst_connect_request {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::ConnectRequest {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::model::ConnectRequest>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::model::GetInfoRequest> for *mut wire_cst_get_info_request {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::GetInfoRequest {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::model::GetInfoRequest>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::model::PrepareReceiveRequest> for *mut wire_cst_prepare_receive_request {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::PrepareReceiveRequest {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::model::PrepareReceiveRequest>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::model::PrepareReceiveResponse> for *mut wire_cst_prepare_receive_response {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::PrepareReceiveResponse {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::model::PrepareReceiveResponse>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::model::PrepareSendRequest> for *mut wire_cst_prepare_send_request {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::PrepareSendRequest {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::model::PrepareSendRequest>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::model::PrepareSendResponse> for *mut wire_cst_prepare_send_response {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::PrepareSendResponse {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::model::PrepareSendResponse>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::model::RestoreRequest> for *mut wire_cst_restore_request {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::RestoreRequest {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::model::RestoreRequest>::cst_decode(*wrap).into()
    }
}
impl CstDecode<u64> for *mut u64 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u64 {
        unsafe { *flutter_rust_bridge::for_generated::box_from_leak_ptr(self) }
    }
}
impl CstDecode<crate::model::ConnectRequest> for wire_cst_connect_request {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::ConnectRequest {
        crate::model::ConnectRequest {
            mnemonic: self.mnemonic.cst_decode(),
            data_dir: self.data_dir.cst_decode(),
            network: self.network.cst_decode(),
        }
    }
}
impl CstDecode<crate::model::GetInfoRequest> for wire_cst_get_info_request {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::GetInfoRequest {
        crate::model::GetInfoRequest {
            with_scan: self.with_scan.cst_decode(),
        }
    }
}
impl CstDecode<crate::model::GetInfoResponse> for wire_cst_get_info_response {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::GetInfoResponse {
        crate::model::GetInfoResponse {
            balance_sat: self.balance_sat.cst_decode(),
            pending_send_sat: self.pending_send_sat.cst_decode(),
            pending_receive_sat: self.pending_receive_sat.cst_decode(),
            pubkey: self.pubkey.cst_decode(),
        }
    }
}
impl CstDecode<Vec<crate::model::Payment>> for *mut wire_cst_list_payment {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<crate::model::Payment> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(CstDecode::cst_decode).collect()
    }
}
impl CstDecode<Vec<u8>> for *mut wire_cst_list_prim_u_8_strict {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl CstDecode<crate::model::Payment> for wire_cst_payment {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::Payment {
        crate::model::Payment {
            tx_id: self.tx_id.cst_decode(),
            swap_id: self.swap_id.cst_decode(),
            timestamp: self.timestamp.cst_decode(),
            amount_sat: self.amount_sat.cst_decode(),
            fees_sat: self.fees_sat.cst_decode(),
            payment_type: self.payment_type.cst_decode(),
            status: self.status.cst_decode(),
        }
    }
}
impl CstDecode<crate::error::PaymentError> for wire_cst_payment_error {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::error::PaymentError {
        match self.tag {
            0 => crate::error::PaymentError::AlreadyClaimed,
            1 => crate::error::PaymentError::AmountOutOfRange,
            2 => {
                let ans = unsafe { self.kind.Generic };
                crate::error::PaymentError::Generic {
                    err: ans.err.cst_decode(),
                }
            }
            3 => crate::error::PaymentError::InvalidOrExpiredFees,
            4 => crate::error::PaymentError::InsufficientFunds,
            5 => crate::error::PaymentError::InvalidInvoice,
            6 => crate::error::PaymentError::InvalidPreimage,
            7 => {
                let ans = unsafe { self.kind.LwkError };
                crate::error::PaymentError::LwkError {
                    err: ans.err.cst_decode(),
                }
            }
            8 => crate::error::PaymentError::PairsNotFound,
            9 => crate::error::PaymentError::PersistError,
            10 => {
                let ans = unsafe { self.kind.Refunded };
                crate::error::PaymentError::Refunded {
                    err: ans.err.cst_decode(),
                    refund_tx_id: ans.refund_tx_id.cst_decode(),
                }
            }
            11 => {
                let ans = unsafe { self.kind.SendError };
                crate::error::PaymentError::SendError {
                    err: ans.err.cst_decode(),
                }
            }
            12 => {
                let ans = unsafe { self.kind.SignerError };
                crate::error::PaymentError::SignerError {
                    err: ans.err.cst_decode(),
                }
            }
            _ => unreachable!(),
        }
    }
}
impl CstDecode<crate::model::PrepareReceiveRequest> for wire_cst_prepare_receive_request {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::PrepareReceiveRequest {
        crate::model::PrepareReceiveRequest {
            payer_amount_sat: self.payer_amount_sat.cst_decode(),
        }
    }
}
impl CstDecode<crate::model::PrepareReceiveResponse> for wire_cst_prepare_receive_response {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::PrepareReceiveResponse {
        crate::model::PrepareReceiveResponse {
            payer_amount_sat: self.payer_amount_sat.cst_decode(),
            fees_sat: self.fees_sat.cst_decode(),
        }
    }
}
impl CstDecode<crate::model::PrepareSendRequest> for wire_cst_prepare_send_request {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::PrepareSendRequest {
        crate::model::PrepareSendRequest {
            invoice: self.invoice.cst_decode(),
        }
    }
}
impl CstDecode<crate::model::PrepareSendResponse> for wire_cst_prepare_send_response {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::PrepareSendResponse {
        crate::model::PrepareSendResponse {
            invoice: self.invoice.cst_decode(),
            fees_sat: self.fees_sat.cst_decode(),
        }
    }
}
impl CstDecode<crate::model::ReceivePaymentResponse> for wire_cst_receive_payment_response {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::ReceivePaymentResponse {
        crate::model::ReceivePaymentResponse {
            id: self.id.cst_decode(),
            invoice: self.invoice.cst_decode(),
        }
    }
}
impl CstDecode<crate::model::RestoreRequest> for wire_cst_restore_request {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::RestoreRequest {
        crate::model::RestoreRequest {
            backup_path: self.backup_path.cst_decode(),
        }
    }
}
impl CstDecode<crate::model::SendPaymentResponse> for wire_cst_send_payment_response {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::model::SendPaymentResponse {
        crate::model::SendPaymentResponse {
            txid: self.txid.cst_decode(),
        }
    }
}
impl NewWithNullPtr for wire_cst_connect_request {
    fn new_with_null_ptr() -> Self {
        Self {
            mnemonic: core::ptr::null_mut(),
            data_dir: core::ptr::null_mut(),
            network: Default::default(),
        }
    }
}
impl Default for wire_cst_connect_request {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_get_info_request {
    fn new_with_null_ptr() -> Self {
        Self {
            with_scan: Default::default(),
        }
    }
}
impl Default for wire_cst_get_info_request {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_get_info_response {
    fn new_with_null_ptr() -> Self {
        Self {
            balance_sat: Default::default(),
            pending_send_sat: Default::default(),
            pending_receive_sat: Default::default(),
            pubkey: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_cst_get_info_response {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_payment {
    fn new_with_null_ptr() -> Self {
        Self {
            tx_id: core::ptr::null_mut(),
            swap_id: core::ptr::null_mut(),
            timestamp: Default::default(),
            amount_sat: Default::default(),
            fees_sat: core::ptr::null_mut(),
            payment_type: Default::default(),
            status: Default::default(),
        }
    }
}
impl Default for wire_cst_payment {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_payment_error {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: PaymentErrorKind { nil__: () },
        }
    }
}
impl Default for wire_cst_payment_error {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_prepare_receive_request {
    fn new_with_null_ptr() -> Self {
        Self {
            payer_amount_sat: Default::default(),
        }
    }
}
impl Default for wire_cst_prepare_receive_request {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_prepare_receive_response {
    fn new_with_null_ptr() -> Self {
        Self {
            payer_amount_sat: Default::default(),
            fees_sat: Default::default(),
        }
    }
}
impl Default for wire_cst_prepare_receive_response {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_prepare_send_request {
    fn new_with_null_ptr() -> Self {
        Self {
            invoice: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_cst_prepare_send_request {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_prepare_send_response {
    fn new_with_null_ptr() -> Self {
        Self {
            invoice: core::ptr::null_mut(),
            fees_sat: Default::default(),
        }
    }
}
impl Default for wire_cst_prepare_send_response {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_receive_payment_response {
    fn new_with_null_ptr() -> Self {
        Self {
            id: core::ptr::null_mut(),
            invoice: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_cst_receive_payment_response {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_restore_request {
    fn new_with_null_ptr() -> Self {
        Self {
            backup_path: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_cst_restore_request {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_send_payment_response {
    fn new_with_null_ptr() -> Self {
        Self {
            txid: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_cst_send_payment_response {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_wire_backup(port_: i64) {
    wire_backup_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_wire_connect(port_: i64, req: *mut wire_cst_connect_request) {
    wire_connect_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_wire_empty_wallet_cache(port_: i64) {
    wire_empty_wallet_cache_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_wire_get_info(
    port_: i64,
    req: *mut wire_cst_get_info_request,
) {
    wire_get_info_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_wire_list_payments(port_: i64) {
    wire_list_payments_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_wire_prepare_receive_payment(
    port_: i64,
    req: *mut wire_cst_prepare_receive_request,
) {
    wire_prepare_receive_payment_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_wire_prepare_send_payment(
    port_: i64,
    req: *mut wire_cst_prepare_send_request,
) {
    wire_prepare_send_payment_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_wire_receive_payment(
    port_: i64,
    req: *mut wire_cst_prepare_receive_response,
) {
    wire_receive_payment_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_wire_restore(port_: i64, req: *mut wire_cst_restore_request) {
    wire_restore_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_wire_send_payment(
    port_: i64,
    req: *mut wire_cst_prepare_send_response,
) {
    wire_send_payment_impl(port_, req)
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_cst_new_box_autoadd_connect_request(
) -> *mut wire_cst_connect_request {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_connect_request::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_cst_new_box_autoadd_get_info_request(
) -> *mut wire_cst_get_info_request {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_get_info_request::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_request(
) -> *mut wire_cst_prepare_receive_request {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_prepare_receive_request::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_cst_new_box_autoadd_prepare_receive_response(
) -> *mut wire_cst_prepare_receive_response {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_prepare_receive_response::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_cst_new_box_autoadd_prepare_send_request(
) -> *mut wire_cst_prepare_send_request {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_prepare_send_request::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_cst_new_box_autoadd_prepare_send_response(
) -> *mut wire_cst_prepare_send_response {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_prepare_send_response::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_cst_new_box_autoadd_restore_request(
) -> *mut wire_cst_restore_request {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_restore_request::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_cst_new_box_autoadd_u_64(value: u64) -> *mut u64 {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_cst_new_list_payment(len: i32) -> *mut wire_cst_list_payment {
    let wrap = wire_cst_list_payment {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(
            <wire_cst_payment>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn frbgen_breez_liquid_cst_new_list_prim_u_8_strict(
    len: i32,
) -> *mut wire_cst_list_prim_u_8_strict {
    let ans = wire_cst_list_prim_u_8_strict {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_connect_request {
    mnemonic: *mut wire_cst_list_prim_u_8_strict,
    data_dir: *mut wire_cst_list_prim_u_8_strict,
    network: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_get_info_request {
    with_scan: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_get_info_response {
    balance_sat: u64,
    pending_send_sat: u64,
    pending_receive_sat: u64,
    pubkey: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_payment {
    ptr: *mut wire_cst_payment,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_prim_u_8_strict {
    ptr: *mut u8,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_payment {
    tx_id: *mut wire_cst_list_prim_u_8_strict,
    swap_id: *mut wire_cst_list_prim_u_8_strict,
    timestamp: u32,
    amount_sat: u64,
    fees_sat: *mut u64,
    payment_type: i32,
    status: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_payment_error {
    tag: i32,
    kind: PaymentErrorKind,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PaymentErrorKind {
    Generic: wire_cst_PaymentError_Generic,
    LwkError: wire_cst_PaymentError_LwkError,
    Refunded: wire_cst_PaymentError_Refunded,
    SendError: wire_cst_PaymentError_SendError,
    SignerError: wire_cst_PaymentError_SignerError,
    nil__: (),
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PaymentError_Generic {
    err: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PaymentError_LwkError {
    err: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PaymentError_Refunded {
    err: *mut wire_cst_list_prim_u_8_strict,
    refund_tx_id: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PaymentError_SendError {
    err: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_PaymentError_SignerError {
    err: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_prepare_receive_request {
    payer_amount_sat: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_prepare_receive_response {
    payer_amount_sat: u64,
    fees_sat: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_prepare_send_request {
    invoice: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_prepare_send_response {
    invoice: *mut wire_cst_list_prim_u_8_strict,
    fees_sat: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_receive_payment_response {
    id: *mut wire_cst_list_prim_u_8_strict,
    invoice: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_restore_request {
    backup_path: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_send_payment_response {
    txid: *mut wire_cst_list_prim_u_8_strict,
}

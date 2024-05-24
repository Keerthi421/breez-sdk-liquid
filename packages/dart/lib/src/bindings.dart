// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.35.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'error.dart';
import 'frb_generated.dart';
import 'model.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<BindingLiquidSdk> connect({required ConnectRequest req, dynamic hint}) =>
    RustLib.instance.api.crateBindingsConnect(req: req, hint: hint);

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<BindingLiquidSdk>>
@sealed
class BindingLiquidSdk extends RustOpaque {
  BindingLiquidSdk.dcoDecode(List<dynamic> wire) : super.dcoDecode(wire, _kStaticData);

  BindingLiquidSdk.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api.rust_arc_increment_strong_count_BindingLiquidSdk,
    rustArcDecrementStrongCount: RustLib.instance.api.rust_arc_decrement_strong_count_BindingLiquidSdk,
    rustArcDecrementStrongCountPtr: RustLib.instance.api.rust_arc_decrement_strong_count_BindingLiquidSdkPtr,
  );

  Future<void> backup({dynamic hint}) =>
      RustLib.instance.api.crateBindingsBindingLiquidSdkBackup(that: this, hint: hint);

  Future<void> emptyWalletCache({dynamic hint}) =>
      RustLib.instance.api.crateBindingsBindingLiquidSdkEmptyWalletCache(that: this, hint: hint);

  Future<GetInfoResponse> getInfo({required GetInfoRequest req, dynamic hint}) =>
      RustLib.instance.api.crateBindingsBindingLiquidSdkGetInfo(that: this, req: req, hint: hint);

  Future<List<Payment>> listPayments({dynamic hint}) =>
      RustLib.instance.api.crateBindingsBindingLiquidSdkListPayments(that: this, hint: hint);

  Future<PrepareReceiveResponse> prepareReceivePayment({required PrepareReceiveRequest req, dynamic hint}) =>
      RustLib.instance.api
          .crateBindingsBindingLiquidSdkPrepareReceivePayment(that: this, req: req, hint: hint);

  Future<PrepareSendResponse> prepareSendPayment({required PrepareSendRequest req, dynamic hint}) =>
      RustLib.instance.api.crateBindingsBindingLiquidSdkPrepareSendPayment(that: this, req: req, hint: hint);

  Future<ReceivePaymentResponse> receivePayment({required PrepareReceiveResponse req, dynamic hint}) =>
      RustLib.instance.api.crateBindingsBindingLiquidSdkReceivePayment(that: this, req: req, hint: hint);

  Future<void> restore({required RestoreRequest req, dynamic hint}) =>
      RustLib.instance.api.crateBindingsBindingLiquidSdkRestore(that: this, req: req, hint: hint);

  Future<SendPaymentResponse> sendPayment({required PrepareSendResponse req, dynamic hint}) =>
      RustLib.instance.api.crateBindingsBindingLiquidSdkSendPayment(that: this, req: req, hint: hint);

  Future<void> sync({dynamic hint}) =>
      RustLib.instance.api.crateBindingsBindingLiquidSdkSync(that: this, hint: hint);
}

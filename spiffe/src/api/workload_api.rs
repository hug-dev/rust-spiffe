// This file is generated by rust-protobuf 2.0.5. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct X509SVIDRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl X509SVIDRequest {
    pub fn new() -> X509SVIDRequest {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for X509SVIDRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> X509SVIDRequest {
        X509SVIDRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<X509SVIDRequest>(
                    "X509SVIDRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static X509SVIDRequest {
        static mut instance: ::protobuf::lazy::Lazy<X509SVIDRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const X509SVIDRequest,
        };
        unsafe {
            instance.get(X509SVIDRequest::new)
        }
    }
}

impl ::protobuf::Clear for X509SVIDRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for X509SVIDRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for X509SVIDRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct X509SVIDResponse {
    // message fields
    pub svids: ::protobuf::RepeatedField<X509SVID>,
    pub crl: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub federated_bundles: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl X509SVIDResponse {
    pub fn new() -> X509SVIDResponse {
        ::std::default::Default::default()
    }

    // repeated .X509SVID svids = 1;

    pub fn clear_svids(&mut self) {
        self.svids.clear();
    }

    // Param is passed by value, moved
    pub fn set_svids(&mut self, v: ::protobuf::RepeatedField<X509SVID>) {
        self.svids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_svids(&mut self) -> &mut ::protobuf::RepeatedField<X509SVID> {
        &mut self.svids
    }

    // Take field
    pub fn take_svids(&mut self) -> ::protobuf::RepeatedField<X509SVID> {
        ::std::mem::replace(&mut self.svids, ::protobuf::RepeatedField::new())
    }

    pub fn get_svids(&self) -> &[X509SVID] {
        &self.svids
    }

    // repeated bytes crl = 2;

    pub fn clear_crl(&mut self) {
        self.crl.clear();
    }

    // Param is passed by value, moved
    pub fn set_crl(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.crl = v;
    }

    // Mutable pointer to the field.
    pub fn mut_crl(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.crl
    }

    // Take field
    pub fn take_crl(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.crl, ::protobuf::RepeatedField::new())
    }

    pub fn get_crl(&self) -> &[::std::vec::Vec<u8>] {
        &self.crl
    }

    // repeated .X509SVIDResponse.FederatedBundlesEntry federated_bundles = 3;

    pub fn clear_federated_bundles(&mut self) {
        self.federated_bundles.clear();
    }

    // Param is passed by value, moved
    pub fn set_federated_bundles(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>>) {
        self.federated_bundles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_federated_bundles(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>> {
        &mut self.federated_bundles
    }

    // Take field
    pub fn take_federated_bundles(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.federated_bundles, ::std::collections::HashMap::new())
    }

    pub fn get_federated_bundles(&self) -> &::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>> {
        &self.federated_bundles
    }
}

impl ::protobuf::Message for X509SVIDResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.svids {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.svids)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.crl)?;
                },
                3 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeBytes>(wire_type, is, &mut self.federated_bundles)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.svids {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.crl {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeBytes>(3, &self.federated_bundles);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.svids {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.crl {
            os.write_bytes(2, &v)?;
        };
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeBytes>(3, &self.federated_bundles, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> X509SVIDResponse {
        X509SVIDResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<X509SVID>>(
                    "svids",
                    |m: &X509SVIDResponse| { &m.svids },
                    |m: &mut X509SVIDResponse| { &mut m.svids },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "crl",
                    |m: &X509SVIDResponse| { &m.crl },
                    |m: &mut X509SVIDResponse| { &mut m.crl },
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeBytes>(
                    "federated_bundles",
                    |m: &X509SVIDResponse| { &m.federated_bundles },
                    |m: &mut X509SVIDResponse| { &mut m.federated_bundles },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<X509SVIDResponse>(
                    "X509SVIDResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static X509SVIDResponse {
        static mut instance: ::protobuf::lazy::Lazy<X509SVIDResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const X509SVIDResponse,
        };
        unsafe {
            instance.get(X509SVIDResponse::new)
        }
    }
}

impl ::protobuf::Clear for X509SVIDResponse {
    fn clear(&mut self) {
        self.clear_svids();
        self.clear_crl();
        self.clear_federated_bundles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for X509SVIDResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for X509SVIDResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct X509SVID {
    // message fields
    pub spiffe_id: ::std::string::String,
    pub x509_svid: ::std::vec::Vec<u8>,
    pub x509_svid_key: ::std::vec::Vec<u8>,
    pub bundle: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl X509SVID {
    pub fn new() -> X509SVID {
        ::std::default::Default::default()
    }

    // string spiffe_id = 1;

    pub fn clear_spiffe_id(&mut self) {
        self.spiffe_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_spiffe_id(&mut self, v: ::std::string::String) {
        self.spiffe_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spiffe_id(&mut self) -> &mut ::std::string::String {
        &mut self.spiffe_id
    }

    // Take field
    pub fn take_spiffe_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.spiffe_id, ::std::string::String::new())
    }

    pub fn get_spiffe_id(&self) -> &str {
        &self.spiffe_id
    }

    // bytes x509_svid = 2;

    pub fn clear_x509_svid(&mut self) {
        self.x509_svid.clear();
    }

    // Param is passed by value, moved
    pub fn set_x509_svid(&mut self, v: ::std::vec::Vec<u8>) {
        self.x509_svid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_x509_svid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.x509_svid
    }

    // Take field
    pub fn take_x509_svid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.x509_svid, ::std::vec::Vec::new())
    }

    pub fn get_x509_svid(&self) -> &[u8] {
        &self.x509_svid
    }

    // bytes x509_svid_key = 3;

    pub fn clear_x509_svid_key(&mut self) {
        self.x509_svid_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_x509_svid_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.x509_svid_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_x509_svid_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.x509_svid_key
    }

    // Take field
    pub fn take_x509_svid_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.x509_svid_key, ::std::vec::Vec::new())
    }

    pub fn get_x509_svid_key(&self) -> &[u8] {
        &self.x509_svid_key
    }

    // bytes bundle = 4;

    pub fn clear_bundle(&mut self) {
        self.bundle.clear();
    }

    // Param is passed by value, moved
    pub fn set_bundle(&mut self, v: ::std::vec::Vec<u8>) {
        self.bundle = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bundle(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.bundle
    }

    // Take field
    pub fn take_bundle(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.bundle, ::std::vec::Vec::new())
    }

    pub fn get_bundle(&self) -> &[u8] {
        &self.bundle
    }
}

impl ::protobuf::Message for X509SVID {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.spiffe_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.x509_svid)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.x509_svid_key)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.bundle)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.spiffe_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.spiffe_id);
        }
        if !self.x509_svid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.x509_svid);
        }
        if !self.x509_svid_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.x509_svid_key);
        }
        if !self.bundle.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.bundle);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.spiffe_id.is_empty() {
            os.write_string(1, &self.spiffe_id)?;
        }
        if !self.x509_svid.is_empty() {
            os.write_bytes(2, &self.x509_svid)?;
        }
        if !self.x509_svid_key.is_empty() {
            os.write_bytes(3, &self.x509_svid_key)?;
        }
        if !self.bundle.is_empty() {
            os.write_bytes(4, &self.bundle)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> X509SVID {
        X509SVID::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "spiffe_id",
                    |m: &X509SVID| { &m.spiffe_id },
                    |m: &mut X509SVID| { &mut m.spiffe_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "x509_svid",
                    |m: &X509SVID| { &m.x509_svid },
                    |m: &mut X509SVID| { &mut m.x509_svid },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "x509_svid_key",
                    |m: &X509SVID| { &m.x509_svid_key },
                    |m: &mut X509SVID| { &mut m.x509_svid_key },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bundle",
                    |m: &X509SVID| { &m.bundle },
                    |m: &mut X509SVID| { &mut m.bundle },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<X509SVID>(
                    "X509SVID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static X509SVID {
        static mut instance: ::protobuf::lazy::Lazy<X509SVID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const X509SVID,
        };
        unsafe {
            instance.get(X509SVID::new)
        }
    }
}

impl ::protobuf::Clear for X509SVID {
    fn clear(&mut self) {
        self.clear_spiffe_id();
        self.clear_x509_svid();
        self.clear_x509_svid_key();
        self.clear_bundle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for X509SVID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for X509SVID {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1asrc/api/workload_api.proto\"\x11\n\x0fX509SVIDRequest\"\xe0\x01\n\
    \x10X509SVIDResponse\x12\x1f\n\x05svids\x18\x01\x20\x03(\x0b2\t.X509SVID\
    R\x05svids\x12\x10\n\x03crl\x18\x02\x20\x03(\x0cR\x03crl\x12T\n\x11feder\
    ated_bundles\x18\x03\x20\x03(\x0b2'.X509SVIDResponse.FederatedBundlesEnt\
    ryR\x10federatedBundles\x1aC\n\x15FederatedBundlesEntry\x12\x10\n\x03key\
    \x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05\
    value:\x028\x01\"\x80\x01\n\x08X509SVID\x12\x1b\n\tspiffe_id\x18\x01\x20\
    \x01(\tR\x08spiffeId\x12\x1b\n\tx509_svid\x18\x02\x20\x01(\x0cR\x08x509S\
    vid\x12\"\n\rx509_svid_key\x18\x03\x20\x01(\x0cR\x0bx509SvidKey\x12\x16\
    \n\x06bundle\x18\x04\x20\x01(\x0cR\x06bundle2K\n\x11SpiffeWorkloadAPI\
    \x126\n\rFetchX509SVID\x12\x10.X509SVIDRequest\x1a\x11.X509SVIDResponse0\
    \x01J\xdb\x0e\n\x06\x12\x04\0\00\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\
    \n\x02\x04\0\x12\x03\x02\0\x1c\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x17\n\
    \n\n\x02\x06\0\x12\x04\x04\0\n\x01\n\n\n\x03\x06\0\x01\x12\x03\x04\x08\
    \x19\n\xd9\x01\n\x04\x06\0\x02\0\x12\x03\t\x04I\x1a\xcb\x01\x20X.509-SVI\
    D\x20Profile\n\x20Fetch\x20all\x20SPIFFE\x20identities\x20the\x20workloa\
    d\x20is\x20entitled\x20to,\x20as\n\x20well\x20as\x20related\x20informati\
    on\x20like\x20trust\x20bundles\x20and\x20CRLs.\x20As\n\x20this\x20inform\
    ation\x20changes,\x20subsequent\x20messages\x20will\x20be\x20sent.\n\n\
    \x0c\n\x05\x06\0\x02\0\x01\x12\x03\t\x08\x15\n\x0c\n\x05\x06\0\x02\0\x02\
    \x12\x03\t\x16%\n\x0c\n\x05\x06\0\x02\0\x06\x12\x03\t06\n\x0c\n\x05\x06\
    \0\x02\0\x03\x12\x03\t7G\n\xd3\x01\n\x02\x04\x01\x12\x04\x0f\0\x1c\x01\
    \x1a\xc6\x01\x20The\x20X509SVIDResponse\x20message\x20carries\x20a\x20se\
    t\x20of\x20X.509\x20SVIDs\x20and\x20their\n\x20associated\x20information\
    .\x20It\x20also\x20carries\x20a\x20set\x20of\x20global\x20CRLs,\x20and\
    \x20a\n\x20TTL\x20to\x20inform\x20the\x20workload\x20when\x20it\x20shoul\
    d\x20check\x20back\x20next.\n\n\n\n\x03\x04\x01\x01\x12\x03\x0f\x08\x18\
    \n\x9a\x01\n\x04\x04\x01\x02\0\x12\x03\x13\x04\x20\x1a\x8c\x01\x20A\x20l\
    ist\x20of\x20X509SVID\x20messages,\x20each\x20of\x20which\x20includes\
    \x20a\x20single\n\x20SPIFFE\x20Verifiable\x20Identity\x20Document,\x20al\
    ong\x20with\x20its\x20private\x20key\n\x20and\x20bundle.\n\n\x0c\n\x05\
    \x04\x01\x02\0\x04\x12\x03\x13\x04\x0c\n\x0c\n\x05\x04\x01\x02\0\x06\x12\
    \x03\x13\r\x15\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x13\x16\x1b\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03\x13\x1e\x1f\n\x20\n\x04\x04\x01\x02\x01\
    \x12\x03\x16\x04\x1b\x1a\x13\x20ASN.1\x20DER\x20encoded\n\n\x0c\n\x05\
    \x04\x01\x02\x01\x04\x12\x03\x16\x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x05\
    \x12\x03\x16\r\x12\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x16\x13\x16\n\
    \x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x16\x19\x1a\n\xb8\x01\n\x04\x04\
    \x01\x02\x02\x12\x03\x1b\x04-\x1a\xaa\x01\x20CA\x20certificate\x20bundle\
    s\x20belonging\x20to\x20foreign\x20Trust\x20Domains\x20that\x20the\n\x20\
    workload\x20should\x20trust,\x20keyed\x20by\x20the\x20SPIFFE\x20ID\x20of\
    \x20the\x20foreign\n\x20domain.\x20Bundles\x20are\x20ASN.1\x20DER\x20enc\
    oded.\n\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\x1b\x04\x16\x1b\n\x0c\n\
    \x05\x04\x01\x02\x02\x06\x12\x03\x1b\x04\x16\n\x0c\n\x05\x04\x01\x02\x02\
    \x01\x12\x03\x1b\x17(\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x1b+,\no\n\
    \x02\x04\x02\x12\x04\x20\00\x01\x1ac\x20The\x20X509SVID\x20message\x20ca\
    rries\x20a\x20single\x20SVID\x20and\x20all\x20associated\n\x20informatio\
    n,\x20including\x20CA\x20bundles.\n\n\n\n\x03\x04\x02\x01\x12\x03\x20\
    \x08\x10\ny\n\x04\x04\x02\x02\0\x12\x03#\x04\x19\x1al\x20The\x20SPIFFE\
    \x20ID\x20of\x20the\x20SVID\x20in\x20this\x20entry.\x20MUST\x20match\x20\
    the\x20SPIFFE\x20ID\n\x20encoded\x20in\x20the\x20`x509_svid`\x20certific\
    ate.\n\n\r\n\x05\x04\x02\x02\0\x04\x12\x04#\x04\x20\x12\n\x0c\n\x05\x04\
    \x02\x02\0\x05\x12\x03#\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03#\x0b\
    \x14\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03#\x17\x18\n\x86\x01\n\x04\x04\
    \x02\x02\x01\x12\x03'\x04\x18\x1ay\x20ASN.1\x20DER\x20encoded\x20certifi\
    cate\x20chain.\x20MAY\x20include\x20intermediates,\n\x20the\x20leaf\x20c\
    ertificate\x20(or\x20SVID\x20itself)\x20MUST\x20come\x20first.\n\n\r\n\
    \x05\x04\x02\x02\x01\x04\x12\x04'\x04#\x19\n\x0c\n\x05\x04\x02\x02\x01\
    \x05\x12\x03'\x04\t\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03'\n\x13\n\x0c\
    \n\x05\x04\x02\x02\x01\x03\x12\x03'\x16\x17\nI\n\x04\x04\x02\x02\x02\x12\
    \x03*\x04\x1c\x1a<\x20ASN.1\x20DER\x20encoded\x20PKCS#8\x20private\x20ke\
    y.\x20MUST\x20be\x20unencrypted.\n\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\
    *\x04'\x18\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03*\x04\t\n\x0c\n\x05\
    \x04\x02\x02\x02\x01\x12\x03*\n\x17\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\
    \x03*\x1a\x1b\nO\n\x04\x04\x02\x02\x03\x12\x03.\x04\x15\x1aB\x20CA\x20ce\
    rtificates\x20belonging\x20to\x20the\x20Trust\x20Domain\n\x20ASN.1\x20DE\
    R\x20encoded\n\n\r\n\x05\x04\x02\x02\x03\x04\x12\x04.\x04*\x1c\n\x0c\n\
    \x05\x04\x02\x02\x03\x05\x12\x03.\x04\t\n\x0c\n\x05\x04\x02\x02\x03\x01\
    \x12\x03.\n\x10\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03.\x13\x14b\x06pro\
    to3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

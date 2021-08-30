// This file is generated. Do not edit
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
pub struct LoginRequest {
    // message fields
    pub username: ::std::string::String,
    pub password: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginRequest {}

impl LoginRequest {
    pub fn new() -> LoginRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginRequest {
        static mut instance: ::protobuf::lazy::Lazy<LoginRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginRequest,
        };
        unsafe {
            instance.get(LoginRequest::new)
        }
    }

    // string username = 1;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.username, ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    fn get_username_for_reflect(&self) -> &::std::string::String {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // string password = 2;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    fn get_password_for_reflect(&self) -> &::std::string::String {
        &self.password
    }

    fn mut_password_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }
}

impl ::protobuf::Message for LoginRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.username)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
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
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.username);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.username.is_empty() {
            os.write_string(1, &self.username)?;
        }
        if !self.password.is_empty() {
            os.write_string(2, &self.password)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LoginRequest {
    fn new() -> LoginRequest {
        LoginRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    LoginRequest::get_username_for_reflect,
                    LoginRequest::mut_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "password",
                    LoginRequest::get_password_for_reflect,
                    LoginRequest::mut_password_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginRequest>(
                    "LoginRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginRequest {
    fn clear(&mut self) {
        self.clear_username();
        self.clear_password();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginResponse {
    // message fields
    pub result: i32,
    pub connToken: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginResponse {}

impl LoginResponse {
    pub fn new() -> LoginResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginResponse {
        static mut instance: ::protobuf::lazy::Lazy<LoginResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginResponse,
        };
        unsafe {
            instance.get(LoginResponse::new)
        }
    }

    // int32 result = 1;

    pub fn clear_result(&mut self) {
        self.result = 0;
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: i32) {
        self.result = v;
    }

    pub fn get_result(&self) -> i32 {
        self.result
    }

    fn get_result_for_reflect(&self) -> &i32 {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut i32 {
        &mut self.result
    }

    // int64 connToken = 2;

    pub fn clear_connToken(&mut self) {
        self.connToken = 0;
    }

    // Param is passed by value, moved
    pub fn set_connToken(&mut self, v: i64) {
        self.connToken = v;
    }

    pub fn get_connToken(&self) -> i64 {
        self.connToken
    }

    fn get_connToken_for_reflect(&self) -> &i64 {
        &self.connToken
    }

    fn mut_connToken_for_reflect(&mut self) -> &mut i64 {
        &mut self.connToken
    }
}

impl ::protobuf::Message for LoginResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.result = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.connToken = tmp;
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
        if self.result != 0 {
            my_size += ::protobuf::rt::value_size(1, self.result, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.connToken != 0 {
            my_size += ::protobuf::rt::value_size(2, self.connToken, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.result != 0 {
            os.write_int32(1, self.result)?;
        }
        if self.connToken != 0 {
            os.write_int64(2, self.connToken)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LoginResponse {
    fn new() -> LoginResponse {
        LoginResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "result",
                    LoginResponse::get_result_for_reflect,
                    LoginResponse::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "connToken",
                    LoginResponse::get_connToken_for_reflect,
                    LoginResponse::mut_connToken_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginResponse>(
                    "LoginResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_connToken();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0blogin.proto\"F\n\x0cLoginRequest\x12\x1a\n\x08username\x18\x01\x20\
    \x01(\tR\x08username\x12\x1a\n\x08password\x18\x02\x20\x01(\tR\x08passwo\
    rd\"E\n\rLoginResponse\x12\x16\n\x06result\x18\x01\x20\x01(\x05R\x06resu\
    lt\x12\x1c\n\tconnToken\x18\x02\x20\x01(\x03R\tconnToken26\n\x0cLoginSer\
    vice\x12&\n\x05Login\x12\r.LoginRequest\x1a\x0e.LoginResponseJ\xed\x02\n\
    \x06\x12\x04\0\0\x0e\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\
    \x12\x04\x02\0\x05\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x14\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\x03\x02\x14\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x03\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\t\x11\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x03\x12\x13\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x04\x02\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x02\x08\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03\x04\t\x11\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x04\x12\x13\n\n\n\x02\x04\x01\x12\x04\x07\0\n\x01\n\n\n\x03\x04\
    \x01\x01\x12\x03\x07\x08\x15\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x08\x02\
    \x11\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x08\x02\x07\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x03\x08\x08\x0e\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\
    \x08\x0f\x10\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\t\x02\x14\n\x0c\n\x05\
    \x04\x01\x02\x01\x05\x12\x03\t\x02\x07\n\x0c\n\x05\x04\x01\x02\x01\x01\
    \x12\x03\t\x08\x11\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\t\x12\x13\n\n\
    \n\x02\x06\0\x12\x04\x0c\0\x0e\x01\n\n\n\x03\x06\0\x01\x12\x03\x0c\x08\
    \x14\n\x0b\n\x04\x06\0\x02\0\x12\x03\r\x023\n\x0c\n\x05\x06\0\x02\0\x01\
    \x12\x03\r\x06\x0b\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\r\r\x19\n\x0c\n\
    \x05\x06\0\x02\0\x03\x12\x03\r$1b\x06proto3\
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

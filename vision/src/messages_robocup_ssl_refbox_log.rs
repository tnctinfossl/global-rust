// This file is generated by rust-protobuf 2.9.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `messages_robocup_ssl_refbox_log.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_9_0;

#[derive(PartialEq,Clone,Default)]
pub struct Log_Frame {
    // message fields
    pub frame: ::protobuf::SingularPtrField<super::messages_robocup_ssl_detection::SSL_DetectionFrame>,
    refbox_cmd: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Log_Frame {
    fn default() -> &'a Log_Frame {
        <Log_Frame as ::protobuf::Message>::default_instance()
    }
}

impl Log_Frame {
    pub fn new() -> Log_Frame {
        ::std::default::Default::default()
    }

    // required .SSL_DetectionFrame frame = 1;


    pub fn get_frame(&self) -> &super::messages_robocup_ssl_detection::SSL_DetectionFrame {
        self.frame.as_ref().unwrap_or_else(|| <super::messages_robocup_ssl_detection::SSL_DetectionFrame as ::protobuf::Message>::default_instance())
    }
    pub fn clear_frame(&mut self) {
        self.frame.clear();
    }

    pub fn has_frame(&self) -> bool {
        self.frame.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frame(&mut self, v: super::messages_robocup_ssl_detection::SSL_DetectionFrame) {
        self.frame = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_frame(&mut self) -> &mut super::messages_robocup_ssl_detection::SSL_DetectionFrame {
        if self.frame.is_none() {
            self.frame.set_default();
        }
        self.frame.as_mut().unwrap()
    }

    // Take field
    pub fn take_frame(&mut self) -> super::messages_robocup_ssl_detection::SSL_DetectionFrame {
        self.frame.take().unwrap_or_else(|| super::messages_robocup_ssl_detection::SSL_DetectionFrame::new())
    }

    // required string refbox_cmd = 2;


    pub fn get_refbox_cmd(&self) -> &str {
        match self.refbox_cmd.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_refbox_cmd(&mut self) {
        self.refbox_cmd.clear();
    }

    pub fn has_refbox_cmd(&self) -> bool {
        self.refbox_cmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_refbox_cmd(&mut self, v: ::std::string::String) {
        self.refbox_cmd = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_refbox_cmd(&mut self) -> &mut ::std::string::String {
        if self.refbox_cmd.is_none() {
            self.refbox_cmd.set_default();
        }
        self.refbox_cmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_refbox_cmd(&mut self) -> ::std::string::String {
        self.refbox_cmd.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for Log_Frame {
    fn is_initialized(&self) -> bool {
        if self.frame.is_none() {
            return false;
        }
        if self.refbox_cmd.is_none() {
            return false;
        }
        for v in &self.frame {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.frame)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.refbox_cmd)?;
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
        if let Some(ref v) = self.frame.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.refbox_cmd.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.frame.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.refbox_cmd.as_ref() {
            os.write_string(2, &v)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Log_Frame {
        Log_Frame::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::rt::make_option_accessor::<_, ::protobuf::reflect::types::ProtobufTypeMessage<super::messages_robocup_ssl_detection::SSL_DetectionFrame>, _>(
                "frame",
                |m: &Log_Frame| { &m.frame },
                |m: &mut Log_Frame| { &mut m.frame },
            ));
            fields.push(::protobuf::reflect::rt::make_option_get_ref_accessor::<_, ::protobuf::reflect::types::ProtobufTypeString, _>(
                "refbox_cmd",
                |m: &Log_Frame| { &m.refbox_cmd },
                |m: &mut Log_Frame| { &mut m.refbox_cmd },
                Log_Frame::get_refbox_cmd,
            ));
            ::protobuf::reflect::MessageDescriptor::new::<Log_Frame>(
                "Log_Frame",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Log_Frame {
        static instance: ::protobuf::lazy::Lazy<Log_Frame> = ::protobuf::lazy::Lazy::INIT;
        instance.get(Log_Frame::new)
    }
}

impl ::protobuf::Clear for Log_Frame {
    fn clear(&mut self) {
        self.frame.clear();
        self.refbox_cmd.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Log_Frame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Log_Frame {
}

#[derive(PartialEq,Clone,Default)]
pub struct Refbox_Log {
    // message fields
    pub log: ::protobuf::RepeatedField<Log_Frame>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Refbox_Log {
    fn default() -> &'a Refbox_Log {
        <Refbox_Log as ::protobuf::Message>::default_instance()
    }
}

impl Refbox_Log {
    pub fn new() -> Refbox_Log {
        ::std::default::Default::default()
    }

    // repeated .Log_Frame log = 1;


    pub fn get_log(&self) -> &[Log_Frame] {
        &self.log
    }
    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::protobuf::RepeatedField<Log_Frame>) {
        self.log = v;
    }

    // Mutable pointer to the field.
    pub fn mut_log(&mut self) -> &mut ::protobuf::RepeatedField<Log_Frame> {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::protobuf::RepeatedField<Log_Frame> {
        ::std::mem::replace(&mut self.log, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Refbox_Log {
    fn is_initialized(&self) -> bool {
        for v in &self.log {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.log)?;
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
        for value in &self.log {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.log {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Refbox_Log {
        Refbox_Log::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::rt::make_repeated_field_accessor::<_, ::protobuf::reflect::types::ProtobufTypeMessage<Log_Frame>>(
                "log",
                |m: &Refbox_Log| { &m.log },
                |m: &mut Refbox_Log| { &mut m.log },
            ));
            ::protobuf::reflect::MessageDescriptor::new::<Refbox_Log>(
                "Refbox_Log",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Refbox_Log {
        static instance: ::protobuf::lazy::Lazy<Refbox_Log> = ::protobuf::lazy::Lazy::INIT;
        instance.get(Refbox_Log::new)
    }
}

impl ::protobuf::Clear for Refbox_Log {
    fn clear(&mut self) {
        self.log.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Refbox_Log {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Refbox_Log {
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%messages_robocup_ssl_refbox_log.proto\x1a$messages_robocup_ssl_detect\
    ion.proto\"U\n\tLog_Frame\x12)\n\x05frame\x18\x01\x20\x02(\x0b2\x13.SSL_\
    DetectionFrameR\x05frame\x12\x1d\n\nrefbox_cmd\x18\x02\x20\x02(\tR\trefb\
    oxCmd\"*\n\nRefbox_Log\x12\x1c\n\x03log\x18\x01\x20\x03(\x0b2\n.Log_Fram\
    eR\x03log\
";

static file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}

// This file is generated by rust-protobuf 2.8.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
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
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `src/proto/grSim_Replacement.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_1;

#[derive(PartialEq,Clone,Default)]
pub struct grSim_RobotReplacement {
    // message fields
    x: ::std::option::Option<f64>,
    y: ::std::option::Option<f64>,
    dir: ::std::option::Option<f64>,
    id: ::std::option::Option<u32>,
    yellowteam: ::std::option::Option<bool>,
    turnon: ::std::option::Option<bool>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a grSim_RobotReplacement {
    fn default() -> &'a grSim_RobotReplacement {
        <grSim_RobotReplacement as ::protobuf::Message>::default_instance()
    }
}

impl grSim_RobotReplacement {
    pub fn new() -> grSim_RobotReplacement {
        ::std::default::Default::default()
    }

    // required double x = 1;


    pub fn get_x(&self) -> f64 {
        self.x.unwrap_or(0.)
    }
    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f64) {
        self.x = ::std::option::Option::Some(v);
    }

    // required double y = 2;


    pub fn get_y(&self) -> f64 {
        self.y.unwrap_or(0.)
    }
    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f64) {
        self.y = ::std::option::Option::Some(v);
    }

    // required double dir = 3;


    pub fn get_dir(&self) -> f64 {
        self.dir.unwrap_or(0.)
    }
    pub fn clear_dir(&mut self) {
        self.dir = ::std::option::Option::None;
    }

    pub fn has_dir(&self) -> bool {
        self.dir.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dir(&mut self, v: f64) {
        self.dir = ::std::option::Option::Some(v);
    }

    // required uint32 id = 4;


    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }
    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    // required bool yellowteam = 5;


    pub fn get_yellowteam(&self) -> bool {
        self.yellowteam.unwrap_or(false)
    }
    pub fn clear_yellowteam(&mut self) {
        self.yellowteam = ::std::option::Option::None;
    }

    pub fn has_yellowteam(&self) -> bool {
        self.yellowteam.is_some()
    }

    // Param is passed by value, moved
    pub fn set_yellowteam(&mut self, v: bool) {
        self.yellowteam = ::std::option::Option::Some(v);
    }

    // optional bool turnon = 6;


    pub fn get_turnon(&self) -> bool {
        self.turnon.unwrap_or(false)
    }
    pub fn clear_turnon(&mut self) {
        self.turnon = ::std::option::Option::None;
    }

    pub fn has_turnon(&self) -> bool {
        self.turnon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_turnon(&mut self, v: bool) {
        self.turnon = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for grSim_RobotReplacement {
    fn is_initialized(&self) -> bool {
        if self.x.is_none() {
            return false;
        }
        if self.y.is_none() {
            return false;
        }
        if self.dir.is_none() {
            return false;
        }
        if self.id.is_none() {
            return false;
        }
        if self.yellowteam.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.dir = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.yellowteam = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.turnon = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += 9;
        }
        if let Some(v) = self.y {
            my_size += 9;
        }
        if let Some(v) = self.dir {
            my_size += 9;
        }
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.yellowteam {
            my_size += 2;
        }
        if let Some(v) = self.turnon {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_double(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_double(2, v)?;
        }
        if let Some(v) = self.dir {
            os.write_double(3, v)?;
        }
        if let Some(v) = self.id {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.yellowteam {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.turnon {
            os.write_bool(6, v)?;
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

    fn new() -> grSim_RobotReplacement {
        grSim_RobotReplacement::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "x",
                    |m: &grSim_RobotReplacement| { &m.x },
                    |m: &mut grSim_RobotReplacement| { &mut m.x },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "y",
                    |m: &grSim_RobotReplacement| { &m.y },
                    |m: &mut grSim_RobotReplacement| { &mut m.y },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "dir",
                    |m: &grSim_RobotReplacement| { &m.dir },
                    |m: &mut grSim_RobotReplacement| { &mut m.dir },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    |m: &grSim_RobotReplacement| { &m.id },
                    |m: &mut grSim_RobotReplacement| { &mut m.id },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "yellowteam",
                    |m: &grSim_RobotReplacement| { &m.yellowteam },
                    |m: &mut grSim_RobotReplacement| { &mut m.yellowteam },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "turnon",
                    |m: &grSim_RobotReplacement| { &m.turnon },
                    |m: &mut grSim_RobotReplacement| { &mut m.turnon },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<grSim_RobotReplacement>(
                    "grSim_RobotReplacement",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static grSim_RobotReplacement {
        static mut instance: ::protobuf::lazy::Lazy<grSim_RobotReplacement> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const grSim_RobotReplacement,
        };
        unsafe {
            instance.get(grSim_RobotReplacement::new)
        }
    }
}

impl ::protobuf::Clear for grSim_RobotReplacement {
    fn clear(&mut self) {
        self.x = ::std::option::Option::None;
        self.y = ::std::option::Option::None;
        self.dir = ::std::option::Option::None;
        self.id = ::std::option::Option::None;
        self.yellowteam = ::std::option::Option::None;
        self.turnon = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for grSim_RobotReplacement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for grSim_RobotReplacement {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct grSim_BallReplacement {
    // message fields
    x: ::std::option::Option<f64>,
    y: ::std::option::Option<f64>,
    vx: ::std::option::Option<f64>,
    vy: ::std::option::Option<f64>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a grSim_BallReplacement {
    fn default() -> &'a grSim_BallReplacement {
        <grSim_BallReplacement as ::protobuf::Message>::default_instance()
    }
}

impl grSim_BallReplacement {
    pub fn new() -> grSim_BallReplacement {
        ::std::default::Default::default()
    }

    // required double x = 1;


    pub fn get_x(&self) -> f64 {
        self.x.unwrap_or(0.)
    }
    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f64) {
        self.x = ::std::option::Option::Some(v);
    }

    // required double y = 2;


    pub fn get_y(&self) -> f64 {
        self.y.unwrap_or(0.)
    }
    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f64) {
        self.y = ::std::option::Option::Some(v);
    }

    // required double vx = 3;


    pub fn get_vx(&self) -> f64 {
        self.vx.unwrap_or(0.)
    }
    pub fn clear_vx(&mut self) {
        self.vx = ::std::option::Option::None;
    }

    pub fn has_vx(&self) -> bool {
        self.vx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vx(&mut self, v: f64) {
        self.vx = ::std::option::Option::Some(v);
    }

    // required double vy = 4;


    pub fn get_vy(&self) -> f64 {
        self.vy.unwrap_or(0.)
    }
    pub fn clear_vy(&mut self) {
        self.vy = ::std::option::Option::None;
    }

    pub fn has_vy(&self) -> bool {
        self.vy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vy(&mut self, v: f64) {
        self.vy = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for grSim_BallReplacement {
    fn is_initialized(&self) -> bool {
        if self.x.is_none() {
            return false;
        }
        if self.y.is_none() {
            return false;
        }
        if self.vx.is_none() {
            return false;
        }
        if self.vy.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.vx = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.vy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += 9;
        }
        if let Some(v) = self.y {
            my_size += 9;
        }
        if let Some(v) = self.vx {
            my_size += 9;
        }
        if let Some(v) = self.vy {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_double(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_double(2, v)?;
        }
        if let Some(v) = self.vx {
            os.write_double(3, v)?;
        }
        if let Some(v) = self.vy {
            os.write_double(4, v)?;
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

    fn new() -> grSim_BallReplacement {
        grSim_BallReplacement::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "x",
                    |m: &grSim_BallReplacement| { &m.x },
                    |m: &mut grSim_BallReplacement| { &mut m.x },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "y",
                    |m: &grSim_BallReplacement| { &m.y },
                    |m: &mut grSim_BallReplacement| { &mut m.y },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "vx",
                    |m: &grSim_BallReplacement| { &m.vx },
                    |m: &mut grSim_BallReplacement| { &mut m.vx },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "vy",
                    |m: &grSim_BallReplacement| { &m.vy },
                    |m: &mut grSim_BallReplacement| { &mut m.vy },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<grSim_BallReplacement>(
                    "grSim_BallReplacement",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static grSim_BallReplacement {
        static mut instance: ::protobuf::lazy::Lazy<grSim_BallReplacement> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const grSim_BallReplacement,
        };
        unsafe {
            instance.get(grSim_BallReplacement::new)
        }
    }
}

impl ::protobuf::Clear for grSim_BallReplacement {
    fn clear(&mut self) {
        self.x = ::std::option::Option::None;
        self.y = ::std::option::Option::None;
        self.vx = ::std::option::Option::None;
        self.vy = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for grSim_BallReplacement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for grSim_BallReplacement {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct grSim_Replacement {
    // message fields
    ball: ::protobuf::SingularPtrField<grSim_BallReplacement>,
    robots: ::protobuf::RepeatedField<grSim_RobotReplacement>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a grSim_Replacement {
    fn default() -> &'a grSim_Replacement {
        <grSim_Replacement as ::protobuf::Message>::default_instance()
    }
}

impl grSim_Replacement {
    pub fn new() -> grSim_Replacement {
        ::std::default::Default::default()
    }

    // optional .grSim_BallReplacement ball = 1;


    pub fn get_ball(&self) -> &grSim_BallReplacement {
        self.ball.as_ref().unwrap_or_else(|| grSim_BallReplacement::default_instance())
    }
    pub fn clear_ball(&mut self) {
        self.ball.clear();
    }

    pub fn has_ball(&self) -> bool {
        self.ball.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ball(&mut self, v: grSim_BallReplacement) {
        self.ball = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ball(&mut self) -> &mut grSim_BallReplacement {
        if self.ball.is_none() {
            self.ball.set_default();
        }
        self.ball.as_mut().unwrap()
    }

    // Take field
    pub fn take_ball(&mut self) -> grSim_BallReplacement {
        self.ball.take().unwrap_or_else(|| grSim_BallReplacement::new())
    }

    // repeated .grSim_RobotReplacement robots = 2;


    pub fn get_robots(&self) -> &[grSim_RobotReplacement] {
        &self.robots
    }
    pub fn clear_robots(&mut self) {
        self.robots.clear();
    }

    // Param is passed by value, moved
    pub fn set_robots(&mut self, v: ::protobuf::RepeatedField<grSim_RobotReplacement>) {
        self.robots = v;
    }

    // Mutable pointer to the field.
    pub fn mut_robots(&mut self) -> &mut ::protobuf::RepeatedField<grSim_RobotReplacement> {
        &mut self.robots
    }

    // Take field
    pub fn take_robots(&mut self) -> ::protobuf::RepeatedField<grSim_RobotReplacement> {
        ::std::mem::replace(&mut self.robots, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for grSim_Replacement {
    fn is_initialized(&self) -> bool {
        for v in &self.ball {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.robots {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ball)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.robots)?;
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
        if let Some(ref v) = self.ball.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.robots {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ball.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.robots {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> grSim_Replacement {
        grSim_Replacement::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<grSim_BallReplacement>>(
                    "ball",
                    |m: &grSim_Replacement| { &m.ball },
                    |m: &mut grSim_Replacement| { &mut m.ball },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<grSim_RobotReplacement>>(
                    "robots",
                    |m: &grSim_Replacement| { &m.robots },
                    |m: &mut grSim_Replacement| { &mut m.robots },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<grSim_Replacement>(
                    "grSim_Replacement",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static grSim_Replacement {
        static mut instance: ::protobuf::lazy::Lazy<grSim_Replacement> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const grSim_Replacement,
        };
        unsafe {
            instance.get(grSim_Replacement::new)
        }
    }
}

impl ::protobuf::Clear for grSim_Replacement {
    fn clear(&mut self) {
        self.ball.clear();
        self.robots.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for grSim_Replacement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for grSim_Replacement {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!src/proto/grSim_Replacement.proto\"\x8e\x01\n\x16grSim_RobotReplaceme\
    nt\x12\x0c\n\x01x\x18\x01\x20\x02(\x01R\x01x\x12\x0c\n\x01y\x18\x02\x20\
    \x02(\x01R\x01y\x12\x10\n\x03dir\x18\x03\x20\x02(\x01R\x03dir\x12\x0e\n\
    \x02id\x18\x04\x20\x02(\rR\x02id\x12\x1e\n\nyellowteam\x18\x05\x20\x02(\
    \x08R\nyellowteam\x12\x16\n\x06turnon\x18\x06\x20\x01(\x08R\x06turnon\"S\
    \n\x15grSim_BallReplacement\x12\x0c\n\x01x\x18\x01\x20\x02(\x01R\x01x\
    \x12\x0c\n\x01y\x18\x02\x20\x02(\x01R\x01y\x12\x0e\n\x02vx\x18\x03\x20\
    \x02(\x01R\x02vx\x12\x0e\n\x02vy\x18\x04\x20\x02(\x01R\x02vy\"p\n\x11grS\
    im_Replacement\x12*\n\x04ball\x18\x01\x20\x01(\x0b2\x16.grSim_BallReplac\
    ementR\x04ball\x12/\n\x06robots\x18\x02\x20\x03(\x0b2\x17.grSim_RobotRep\
    lacementR\x06robots\
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

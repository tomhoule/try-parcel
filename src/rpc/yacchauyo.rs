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
pub struct Text {
    // message fields
    pub id: ::std::string::String,
    pub title: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Text {}

impl Text {
    pub fn new() -> Text {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Text {
        static mut instance: ::protobuf::lazy::Lazy<Text> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Text,
        };
        unsafe {
            instance.get(Text::new)
        }
    }

    // string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // string title = 2;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }
}

impl ::protobuf::Message for Text {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.title);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.title.is_empty() {
            os.write_string(2, &self.title)?;
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

impl ::protobuf::MessageStatic for Text {
    fn new() -> Text {
        Text::new()
    }

    fn descriptor_static(_: ::std::option::Option<Text>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Text::get_id_for_reflect,
                    Text::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    Text::get_title_for_reflect,
                    Text::mut_title_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Text>(
                    "Text",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Text {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_title();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Text {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Text {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Texts {
    // message fields
    pub texts: ::protobuf::RepeatedField<Text>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Texts {}

impl Texts {
    pub fn new() -> Texts {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Texts {
        static mut instance: ::protobuf::lazy::Lazy<Texts> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Texts,
        };
        unsafe {
            instance.get(Texts::new)
        }
    }

    // repeated .Text texts = 1;

    pub fn clear_texts(&mut self) {
        self.texts.clear();
    }

    // Param is passed by value, moved
    pub fn set_texts(&mut self, v: ::protobuf::RepeatedField<Text>) {
        self.texts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_texts(&mut self) -> &mut ::protobuf::RepeatedField<Text> {
        &mut self.texts
    }

    // Take field
    pub fn take_texts(&mut self) -> ::protobuf::RepeatedField<Text> {
        ::std::mem::replace(&mut self.texts, ::protobuf::RepeatedField::new())
    }

    pub fn get_texts(&self) -> &[Text] {
        &self.texts
    }

    fn get_texts_for_reflect(&self) -> &::protobuf::RepeatedField<Text> {
        &self.texts
    }

    fn mut_texts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Text> {
        &mut self.texts
    }
}

impl ::protobuf::Message for Texts {
    fn is_initialized(&self) -> bool {
        for v in &self.texts {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.texts)?;
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
        for value in &self.texts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.texts {
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

impl ::protobuf::MessageStatic for Texts {
    fn new() -> Texts {
        Texts::new()
    }

    fn descriptor_static(_: ::std::option::Option<Texts>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Text>>(
                    "texts",
                    Texts::get_texts_for_reflect,
                    Texts::mut_texts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Texts>(
                    "Texts",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Texts {
    fn clear(&mut self) {
        self.clear_texts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Texts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Texts {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TextsQuery {
    // message fields
    pub title: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TextsQuery {}

impl TextsQuery {
    pub fn new() -> TextsQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TextsQuery {
        static mut instance: ::protobuf::lazy::Lazy<TextsQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TextsQuery,
        };
        unsafe {
            instance.get(TextsQuery::new)
        }
    }

    // string title = 1;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_for_reflect(&self) -> &::std::string::String {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }
}

impl ::protobuf::Message for TextsQuery {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
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
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.title);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.title.is_empty() {
            os.write_string(1, &self.title)?;
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

impl ::protobuf::MessageStatic for TextsQuery {
    fn new() -> TextsQuery {
        TextsQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<TextsQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    TextsQuery::get_title_for_reflect,
                    TextsQuery::mut_title_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TextsQuery>(
                    "TextsQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TextsQuery {
    fn clear(&mut self) {
        self.clear_title();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TextsQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TextsQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15proto/yacchauyo.proto\",\n\x04Text\x12\x0e\n\x02id\x18\x01\x20\x01\
    (\tR\x02id\x12\x14\n\x05title\x18\x02\x20\x01(\tR\x05title\"$\n\x05Texts\
    \x12\x1b\n\x05texts\x18\x01\x20\x03(\x0b2\x05.TextR\x05texts\"\"\n\nText\
    sQuery\x12\x14\n\x05title\x18\x01\x20\x01(\tR\x05title2.\n\tYacchauyo\
    \x12!\n\nTextsIndex\x12\x0b.TextsQuery\x1a\x06.TextsJ\xc0\x03\n\x06\x12\
    \x04\0\0\x11\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\
    \x02\0\x05\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x0c\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\x03\x02\x10\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x03\x02\x02\
    \x0e\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x02\x08\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x03\t\x0b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x0e\
    \x0f\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x02\x13\n\r\n\x05\x04\0\x02\
    \x01\x04\x12\x04\x04\x02\x03\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\
    \x04\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\t\x0e\n\x0c\n\x05\
    \x04\0\x02\x01\x03\x12\x03\x04\x11\x12\n\n\n\x02\x04\x01\x12\x04\x07\0\t\
    \x01\n\n\n\x03\x04\x01\x01\x12\x03\x07\x08\r\n\x0b\n\x04\x04\x01\x02\0\
    \x12\x03\x08\x02\x1a\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x08\x02\n\n\
    \x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x08\x0b\x0f\n\x0c\n\x05\x04\x01\x02\
    \0\x01\x12\x03\x08\x10\x15\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x08\x18\
    \x19\n\n\n\x02\x04\x02\x12\x04\x0b\0\r\x01\n\n\n\x03\x04\x02\x01\x12\x03\
    \x0b\x08\x12\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x0c\x02\x13\n\r\n\x05\x04\
    \x02\x02\0\x04\x12\x04\x0c\x02\x0b\x14\n\x0c\n\x05\x04\x02\x02\0\x05\x12\
    \x03\x0c\x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x0c\t\x0e\n\x0c\n\
    \x05\x04\x02\x02\0\x03\x12\x03\x0c\x11\x12\n\n\n\x02\x06\0\x12\x04\x0f\0\
    \x11\x01\n\n\n\x03\x06\0\x01\x12\x03\x0f\x08\x11\n\x0b\n\x04\x06\0\x02\0\
    \x12\x03\x10\x02.\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x10\x06\x10\n\x0c\
    \n\x05\x06\0\x02\0\x02\x12\x03\x10\x12\x1c\n\x0c\n\x05\x06\0\x02\0\x03\
    \x12\x03\x10',b\x06proto3\
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

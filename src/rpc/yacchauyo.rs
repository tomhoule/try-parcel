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
    pub slug: ::std::string::String,
    pub authors: ::std::string::String,
    pub description: ::std::string::String,
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

    // string slug = 3;

    pub fn clear_slug(&mut self) {
        self.slug.clear();
    }

    // Param is passed by value, moved
    pub fn set_slug(&mut self, v: ::std::string::String) {
        self.slug = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slug(&mut self) -> &mut ::std::string::String {
        &mut self.slug
    }

    // Take field
    pub fn take_slug(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.slug, ::std::string::String::new())
    }

    pub fn get_slug(&self) -> &str {
        &self.slug
    }

    fn get_slug_for_reflect(&self) -> &::std::string::String {
        &self.slug
    }

    fn mut_slug_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.slug
    }

    // string authors = 4;

    pub fn clear_authors(&mut self) {
        self.authors.clear();
    }

    // Param is passed by value, moved
    pub fn set_authors(&mut self, v: ::std::string::String) {
        self.authors = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_authors(&mut self) -> &mut ::std::string::String {
        &mut self.authors
    }

    // Take field
    pub fn take_authors(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.authors, ::std::string::String::new())
    }

    pub fn get_authors(&self) -> &str {
        &self.authors
    }

    fn get_authors_for_reflect(&self) -> &::std::string::String {
        &self.authors
    }

    fn mut_authors_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.authors
    }

    // string description = 5;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
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
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.slug)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.authors)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
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
        if !self.slug.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.slug);
        }
        if !self.authors.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.authors);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.description);
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
        if !self.slug.is_empty() {
            os.write_string(3, &self.slug)?;
        }
        if !self.authors.is_empty() {
            os.write_string(4, &self.authors)?;
        }
        if !self.description.is_empty() {
            os.write_string(5, &self.description)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "slug",
                    Text::get_slug_for_reflect,
                    Text::mut_slug_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "authors",
                    Text::get_authors_for_reflect,
                    Text::mut_authors_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    Text::get_description_for_reflect,
                    Text::mut_description_for_reflect,
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
        self.clear_slug();
        self.clear_authors();
        self.clear_description();
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
    pub id: ::std::string::String,
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

impl ::protobuf::Message for TextsQuery {
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
                    "id",
                    TextsQuery::get_id_for_reflect,
                    TextsQuery::mut_id_for_reflect,
                ));
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
        self.clear_id();
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
    \n\x15proto/yacchauyo.proto\"|\n\x04Text\x12\x0e\n\x02id\x18\x01\x20\x01\
    (\tR\x02id\x12\x14\n\x05title\x18\x02\x20\x01(\tR\x05title\x12\x12\n\x04\
    slug\x18\x03\x20\x01(\tR\x04slug\x12\x18\n\x07authors\x18\x04\x20\x01(\t\
    R\x07authors\x12\x20\n\x0bdescription\x18\x05\x20\x01(\tR\x0bdescription\
    \"$\n\x05Texts\x12\x1b\n\x05texts\x18\x01\x20\x03(\x0b2\x05.TextR\x05tex\
    ts\"2\n\nTextsQuery\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x14\n\
    \x05title\x18\x02\x20\x01(\tR\x05title2e\n\tYacchauyo\x12!\n\nTextsIndex\
    \x12\x0b.TextsQuery\x1a\x06.Texts\x12\x1a\n\nCreateText\x12\x05.Text\x1a\
    \x05.Text\x12\x19\n\tPatchText\x12\x05.Text\x1a\x05.TextJ\xc6\x06\n\x06\
    \x12\x04\0\0\x17\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x06\0\x12\
    \x04\x02\0\x06\x01\n\n\n\x03\x06\0\x01\x12\x03\x02\x08\x11\n\x0b\n\x04\
    \x06\0\x02\0\x12\x03\x03\x02.\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x03\
    \x06\x10\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x03\x12\x1c\n\x0c\n\x05\x06\
    \0\x02\0\x03\x12\x03\x03',\n\x0b\n\x04\x06\0\x02\x01\x12\x03\x04\x02'\n\
    \x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x04\x06\x10\n\x0c\n\x05\x06\0\x02\
    \x01\x02\x12\x03\x04\x12\x16\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x04!%\
    \n\x0b\n\x04\x06\0\x02\x02\x12\x03\x05\x02&\n\x0c\n\x05\x06\0\x02\x02\
    \x01\x12\x03\x05\x06\x0f\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03\x05\x11\
    \x15\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03\x05\x20$\n\n\n\x02\x04\0\x12\
    \x04\x08\0\x0e\x01\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x0c\n\x0b\n\x04\
    \x04\0\x02\0\x12\x03\t\x02\x10\n\r\n\x05\x04\0\x02\0\x04\x12\x04\t\x02\
    \x08\x0e\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\t\x02\x08\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\t\t\x0b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\t\x0e\x0f\
    \n\x0b\n\x04\x04\0\x02\x01\x12\x03\n\x02\x13\n\r\n\x05\x04\0\x02\x01\x04\
    \x12\x04\n\x02\t\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\n\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\n\t\x0e\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03\n\x11\x12\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0b\x02\x12\n\r\
    \n\x05\x04\0\x02\x02\x04\x12\x04\x0b\x02\n\x13\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x03\x0b\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0b\t\r\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0b\x10\x11\n\x0b\n\x04\x04\0\x02\
    \x03\x12\x03\x0c\x02\x15\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x0c\x02\x0b\
    \x12\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x0c\x02\x08\n\x0c\n\x05\x04\0\
    \x02\x03\x01\x12\x03\x0c\t\x10\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x0c\
    \x13\x14\n\x0b\n\x04\x04\0\x02\x04\x12\x03\r\x02\x19\n\r\n\x05\x04\0\x02\
    \x04\x04\x12\x04\r\x02\x0c\x15\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\r\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\r\t\x14\n\x0c\n\x05\x04\0\
    \x02\x04\x03\x12\x03\r\x17\x18\n\n\n\x02\x04\x01\x12\x04\x10\0\x12\x01\n\
    \n\n\x03\x04\x01\x01\x12\x03\x10\x08\r\n\x0b\n\x04\x04\x01\x02\0\x12\x03\
    \x11\x02\x1a\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x11\x02\n\n\x0c\n\x05\
    \x04\x01\x02\0\x06\x12\x03\x11\x0b\x0f\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x03\x11\x10\x15\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x11\x18\x19\n\n\n\
    \x02\x04\x02\x12\x04\x14\0\x17\x01\n\n\n\x03\x04\x02\x01\x12\x03\x14\x08\
    \x12\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x15\x02\x10\n\r\n\x05\x04\x02\x02\
    \0\x04\x12\x04\x15\x02\x14\x14\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x15\
    \x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x15\t\x0b\n\x0c\n\x05\x04\
    \x02\x02\0\x03\x12\x03\x15\x0e\x0f\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\
    \x16\x02\x13\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x16\x02\x15\x10\n\x0c\
    \n\x05\x04\x02\x02\x01\x05\x12\x03\x16\x02\x08\n\x0c\n\x05\x04\x02\x02\
    \x01\x01\x12\x03\x16\t\x0e\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x16\
    \x11\x12b\x06proto3\
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

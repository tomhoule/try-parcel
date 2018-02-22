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

#[derive(PartialEq,Clone,Default)]
pub struct Schema {
    // message fields
    pub id: ::std::string::String,
    pub text_id: ::std::string::String,
    pub paths: ::protobuf::RepeatedField<::std::string::String>,
    pub created_at: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Schema {}

impl Schema {
    pub fn new() -> Schema {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Schema {
        static mut instance: ::protobuf::lazy::Lazy<Schema> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Schema,
        };
        unsafe {
            instance.get(Schema::new)
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

    // string text_id = 2;

    pub fn clear_text_id(&mut self) {
        self.text_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_text_id(&mut self, v: ::std::string::String) {
        self.text_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text_id(&mut self) -> &mut ::std::string::String {
        &mut self.text_id
    }

    // Take field
    pub fn take_text_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.text_id, ::std::string::String::new())
    }

    pub fn get_text_id(&self) -> &str {
        &self.text_id
    }

    fn get_text_id_for_reflect(&self) -> &::std::string::String {
        &self.text_id
    }

    fn mut_text_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.text_id
    }

    // repeated string paths = 3;

    pub fn clear_paths(&mut self) {
        self.paths.clear();
    }

    // Param is passed by value, moved
    pub fn set_paths(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.paths = v;
    }

    // Mutable pointer to the field.
    pub fn mut_paths(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.paths
    }

    // Take field
    pub fn take_paths(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.paths, ::protobuf::RepeatedField::new())
    }

    pub fn get_paths(&self) -> &[::std::string::String] {
        &self.paths
    }

    fn get_paths_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.paths
    }

    fn mut_paths_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.paths
    }

    // .google.protobuf.Timestamp created_at = 4;

    pub fn clear_created_at(&mut self) {
        self.created_at.clear();
    }

    pub fn has_created_at(&self) -> bool {
        self.created_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_created_at(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.created_at = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_created_at(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.created_at.is_none() {
            self.created_at.set_default();
        }
        self.created_at.as_mut().unwrap()
    }

    // Take field
    pub fn take_created_at(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.created_at.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    pub fn get_created_at(&self) -> &::protobuf::well_known_types::Timestamp {
        self.created_at.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::default_instance())
    }

    fn get_created_at_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp> {
        &self.created_at
    }

    fn mut_created_at_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp> {
        &mut self.created_at
    }
}

impl ::protobuf::Message for Schema {
    fn is_initialized(&self) -> bool {
        for v in &self.created_at {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.text_id)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.paths)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.created_at)?;
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
        if !self.text_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.text_id);
        }
        for value in &self.paths {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if let Some(ref v) = self.created_at.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.text_id.is_empty() {
            os.write_string(2, &self.text_id)?;
        }
        for v in &self.paths {
            os.write_string(3, &v)?;
        };
        if let Some(ref v) = self.created_at.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for Schema {
    fn new() -> Schema {
        Schema::new()
    }

    fn descriptor_static(_: ::std::option::Option<Schema>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Schema::get_id_for_reflect,
                    Schema::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text_id",
                    Schema::get_text_id_for_reflect,
                    Schema::mut_text_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "paths",
                    Schema::get_paths_for_reflect,
                    Schema::mut_paths_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                    "created_at",
                    Schema::get_created_at_for_reflect,
                    Schema::mut_created_at_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Schema>(
                    "Schema",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Schema {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_text_id();
        self.clear_paths();
        self.clear_created_at();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Schema {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Schema {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Fragment {
    // message fields
    pub id: ::std::string::String,
    pub schema_path: ::std::string::String,
    pub text_id: ::std::string::String,
    pub created_at: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Fragment {}

impl Fragment {
    pub fn new() -> Fragment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Fragment {
        static mut instance: ::protobuf::lazy::Lazy<Fragment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Fragment,
        };
        unsafe {
            instance.get(Fragment::new)
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

    // string schema_path = 2;

    pub fn clear_schema_path(&mut self) {
        self.schema_path.clear();
    }

    // Param is passed by value, moved
    pub fn set_schema_path(&mut self, v: ::std::string::String) {
        self.schema_path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_schema_path(&mut self) -> &mut ::std::string::String {
        &mut self.schema_path
    }

    // Take field
    pub fn take_schema_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.schema_path, ::std::string::String::new())
    }

    pub fn get_schema_path(&self) -> &str {
        &self.schema_path
    }

    fn get_schema_path_for_reflect(&self) -> &::std::string::String {
        &self.schema_path
    }

    fn mut_schema_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.schema_path
    }

    // string text_id = 3;

    pub fn clear_text_id(&mut self) {
        self.text_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_text_id(&mut self, v: ::std::string::String) {
        self.text_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text_id(&mut self) -> &mut ::std::string::String {
        &mut self.text_id
    }

    // Take field
    pub fn take_text_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.text_id, ::std::string::String::new())
    }

    pub fn get_text_id(&self) -> &str {
        &self.text_id
    }

    fn get_text_id_for_reflect(&self) -> &::std::string::String {
        &self.text_id
    }

    fn mut_text_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.text_id
    }

    // .google.protobuf.Timestamp created_at = 4;

    pub fn clear_created_at(&mut self) {
        self.created_at.clear();
    }

    pub fn has_created_at(&self) -> bool {
        self.created_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_created_at(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.created_at = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_created_at(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.created_at.is_none() {
            self.created_at.set_default();
        }
        self.created_at.as_mut().unwrap()
    }

    // Take field
    pub fn take_created_at(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.created_at.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    pub fn get_created_at(&self) -> &::protobuf::well_known_types::Timestamp {
        self.created_at.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::default_instance())
    }

    fn get_created_at_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp> {
        &self.created_at
    }

    fn mut_created_at_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp> {
        &mut self.created_at
    }

    // string value = 5;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::string::String {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }
}

impl ::protobuf::Message for Fragment {
    fn is_initialized(&self) -> bool {
        for v in &self.created_at {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.schema_path)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.text_id)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.created_at)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
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
        if !self.schema_path.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.schema_path);
        }
        if !self.text_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.text_id);
        }
        if let Some(ref v) = self.created_at.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.schema_path.is_empty() {
            os.write_string(2, &self.schema_path)?;
        }
        if !self.text_id.is_empty() {
            os.write_string(3, &self.text_id)?;
        }
        if let Some(ref v) = self.created_at.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.value.is_empty() {
            os.write_string(5, &self.value)?;
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

impl ::protobuf::MessageStatic for Fragment {
    fn new() -> Fragment {
        Fragment::new()
    }

    fn descriptor_static(_: ::std::option::Option<Fragment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Fragment::get_id_for_reflect,
                    Fragment::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "schema_path",
                    Fragment::get_schema_path_for_reflect,
                    Fragment::mut_schema_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text_id",
                    Fragment::get_text_id_for_reflect,
                    Fragment::mut_text_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                    "created_at",
                    Fragment::get_created_at_for_reflect,
                    Fragment::mut_created_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    Fragment::get_value_for_reflect,
                    Fragment::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Fragment>(
                    "Fragment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Fragment {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_schema_path();
        self.clear_text_id();
        self.clear_created_at();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Fragment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Fragment {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FragmentsQuery {
    // message fields
    pub fragments: ::protobuf::RepeatedField<Fragment>,
    pub prefix: ::std::string::String,
    pub text_id: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FragmentsQuery {}

impl FragmentsQuery {
    pub fn new() -> FragmentsQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FragmentsQuery {
        static mut instance: ::protobuf::lazy::Lazy<FragmentsQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FragmentsQuery,
        };
        unsafe {
            instance.get(FragmentsQuery::new)
        }
    }

    // repeated .Fragment fragments = 1;

    pub fn clear_fragments(&mut self) {
        self.fragments.clear();
    }

    // Param is passed by value, moved
    pub fn set_fragments(&mut self, v: ::protobuf::RepeatedField<Fragment>) {
        self.fragments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fragments(&mut self) -> &mut ::protobuf::RepeatedField<Fragment> {
        &mut self.fragments
    }

    // Take field
    pub fn take_fragments(&mut self) -> ::protobuf::RepeatedField<Fragment> {
        ::std::mem::replace(&mut self.fragments, ::protobuf::RepeatedField::new())
    }

    pub fn get_fragments(&self) -> &[Fragment] {
        &self.fragments
    }

    fn get_fragments_for_reflect(&self) -> &::protobuf::RepeatedField<Fragment> {
        &self.fragments
    }

    fn mut_fragments_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Fragment> {
        &mut self.fragments
    }

    // string prefix = 2;

    pub fn clear_prefix(&mut self) {
        self.prefix.clear();
    }

    // Param is passed by value, moved
    pub fn set_prefix(&mut self, v: ::std::string::String) {
        self.prefix = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prefix(&mut self) -> &mut ::std::string::String {
        &mut self.prefix
    }

    // Take field
    pub fn take_prefix(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.prefix, ::std::string::String::new())
    }

    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }

    fn get_prefix_for_reflect(&self) -> &::std::string::String {
        &self.prefix
    }

    fn mut_prefix_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.prefix
    }

    // string text_id = 3;

    pub fn clear_text_id(&mut self) {
        self.text_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_text_id(&mut self, v: ::std::string::String) {
        self.text_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text_id(&mut self) -> &mut ::std::string::String {
        &mut self.text_id
    }

    // Take field
    pub fn take_text_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.text_id, ::std::string::String::new())
    }

    pub fn get_text_id(&self) -> &str {
        &self.text_id
    }

    fn get_text_id_for_reflect(&self) -> &::std::string::String {
        &self.text_id
    }

    fn mut_text_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.text_id
    }
}

impl ::protobuf::Message for FragmentsQuery {
    fn is_initialized(&self) -> bool {
        for v in &self.fragments {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.fragments)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.prefix)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.text_id)?;
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
        for value in &self.fragments {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.prefix.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.prefix);
        }
        if !self.text_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.text_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.fragments {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.prefix.is_empty() {
            os.write_string(2, &self.prefix)?;
        }
        if !self.text_id.is_empty() {
            os.write_string(3, &self.text_id)?;
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

impl ::protobuf::MessageStatic for FragmentsQuery {
    fn new() -> FragmentsQuery {
        FragmentsQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<FragmentsQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Fragment>>(
                    "fragments",
                    FragmentsQuery::get_fragments_for_reflect,
                    FragmentsQuery::mut_fragments_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "prefix",
                    FragmentsQuery::get_prefix_for_reflect,
                    FragmentsQuery::mut_prefix_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text_id",
                    FragmentsQuery::get_text_id_for_reflect,
                    FragmentsQuery::mut_text_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FragmentsQuery>(
                    "FragmentsQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FragmentsQuery {
    fn clear(&mut self) {
        self.clear_fragments();
        self.clear_prefix();
        self.clear_text_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FragmentsQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FragmentsQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15proto/yacchauyo.proto\x1a\x1fgoogle/protobuf/timestamp.proto\"|\n\
    \x04Text\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x14\n\x05title\
    \x18\x02\x20\x01(\tR\x05title\x12\x12\n\x04slug\x18\x03\x20\x01(\tR\x04s\
    lug\x12\x18\n\x07authors\x18\x04\x20\x01(\tR\x07authors\x12\x20\n\x0bdes\
    cription\x18\x05\x20\x01(\tR\x0bdescription\"$\n\x05Texts\x12\x1b\n\x05t\
    exts\x18\x01\x20\x03(\x0b2\x05.TextR\x05texts\"2\n\nTextsQuery\x12\x0e\n\
    \x02id\x18\x01\x20\x01(\tR\x02id\x12\x14\n\x05title\x18\x02\x20\x01(\tR\
    \x05title\"\x82\x01\n\x06Schema\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02i\
    d\x12\x17\n\x07text_id\x18\x02\x20\x01(\tR\x06textId\x12\x14\n\x05paths\
    \x18\x03\x20\x03(\tR\x05paths\x129\n\ncreated_at\x18\x04\x20\x01(\x0b2\
    \x1a.google.protobuf.TimestampR\tcreatedAt\"\xa5\x01\n\x08Fragment\x12\
    \x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x1f\n\x0bschema_path\x18\x02\
    \x20\x01(\tR\nschemaPath\x12\x17\n\x07text_id\x18\x03\x20\x01(\tR\x06tex\
    tId\x129\n\ncreated_at\x18\x04\x20\x01(\x0b2\x1a.google.protobuf.Timesta\
    mpR\tcreatedAt\x12\x14\n\x05value\x18\x05\x20\x01(\tR\x05value\"j\n\x0eF\
    ragmentsQuery\x12'\n\tfragments\x18\x01\x20\x03(\x0b2\t.FragmentR\tfragm\
    ents\x12\x16\n\x06prefix\x18\x02\x20\x01(\tR\x06prefix\x12\x17\n\x07text\
    _id\x18\x03\x20\x01(\tR\x06textId2\xde\x01\n\tYacchauyo\x12!\n\nTextsInd\
    ex\x12\x0b.TextsQuery\x1a\x06.Texts\x12\x1a\n\nCreateText\x12\x05.Text\
    \x1a\x05.Text\x12\x19\n\tPatchText\x12\x05.Text\x1a\x05.Text\x12\"\n\nTe\
    xtSchema\x12\x0b.TextsQuery\x1a\x07.Schema\x12\x1f\n\x0bPatchSchema\x12\
    \x07.Schema\x1a\x07.Schema\x122\n\x0eQueryFragments\x12\x0f.FragmentsQue\
    ry\x1a\x0f.FragmentsQueryJ\x84\x0f\n\x06\x12\x04\0\03\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07(\n\n\n\x02\x06\0\
    \x12\x04\x04\0\r\x01\n\n\n\x03\x06\0\x01\x12\x03\x04\x08\x11\n\x0b\n\x04\
    \x06\0\x02\0\x12\x03\x05\x02.\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x05\
    \x06\x10\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x05\x12\x1c\n\x0c\n\x05\x06\
    \0\x02\0\x03\x12\x03\x05',\n\x0b\n\x04\x06\0\x02\x01\x12\x03\x06\x02'\n\
    \x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x06\x06\x10\n\x0c\n\x05\x06\0\x02\
    \x01\x02\x12\x03\x06\x12\x16\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x06!%\
    \n\x0b\n\x04\x06\0\x02\x02\x12\x03\x07\x02&\n\x0c\n\x05\x06\0\x02\x02\
    \x01\x12\x03\x07\x06\x0f\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03\x07\x11\
    \x15\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03\x07\x20$\n\x0b\n\x04\x06\0\
    \x02\x03\x12\x03\t\x02/\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x03\t\x06\x10\
    \n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03\t\x12\x1c\n\x0c\n\x05\x06\0\x02\
    \x03\x03\x12\x03\t'-\n\x0b\n\x04\x06\0\x02\x04\x12\x03\n\x02,\n\x0c\n\
    \x05\x06\0\x02\x04\x01\x12\x03\n\x06\x11\n\x0c\n\x05\x06\0\x02\x04\x02\
    \x12\x03\n\x13\x19\n\x0c\n\x05\x06\0\x02\x04\x03\x12\x03\n$*\n\x0b\n\x04\
    \x06\0\x02\x05\x12\x03\x0c\x02?\n\x0c\n\x05\x06\0\x02\x05\x01\x12\x03\
    \x0c\x06\x14\n\x0c\n\x05\x06\0\x02\x05\x02\x12\x03\x0c\x16$\n\x0c\n\x05\
    \x06\0\x02\x05\x03\x12\x03\x0c/=\n\n\n\x02\x04\0\x12\x04\x0f\0\x15\x01\n\
    \n\n\x03\x04\0\x01\x12\x03\x0f\x08\x0c\n\x0b\n\x04\x04\0\x02\0\x12\x03\
    \x10\x02\x10\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x10\x02\x0f\x0e\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03\x10\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x10\t\x0b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x10\x0e\x0f\n\x0b\n\
    \x04\x04\0\x02\x01\x12\x03\x11\x02\x13\n\r\n\x05\x04\0\x02\x01\x04\x12\
    \x04\x11\x02\x10\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x11\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x11\t\x0e\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03\x11\x11\x12\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x12\x02\x12\n\
    \r\n\x05\x04\0\x02\x02\x04\x12\x04\x12\x02\x11\x13\n\x0c\n\x05\x04\0\x02\
    \x02\x05\x12\x03\x12\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x12\t\
    \r\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x12\x10\x11\n\x0b\n\x04\x04\0\
    \x02\x03\x12\x03\x13\x02\x15\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x13\x02\
    \x12\x12\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x13\x02\x08\n\x0c\n\x05\
    \x04\0\x02\x03\x01\x12\x03\x13\t\x10\n\x0c\n\x05\x04\0\x02\x03\x03\x12\
    \x03\x13\x13\x14\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x14\x02\x19\n\r\n\x05\
    \x04\0\x02\x04\x04\x12\x04\x14\x02\x13\x15\n\x0c\n\x05\x04\0\x02\x04\x05\
    \x12\x03\x14\x02\x08\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x14\t\x14\n\
    \x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x14\x17\x18\n\n\n\x02\x04\x01\x12\
    \x04\x17\0\x19\x01\n\n\n\x03\x04\x01\x01\x12\x03\x17\x08\r\n\x0b\n\x04\
    \x04\x01\x02\0\x12\x03\x18\x02\x1a\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\
    \x18\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x18\x0b\x0f\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03\x18\x10\x15\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03\x18\x18\x19\n\n\n\x02\x04\x02\x12\x04\x1b\0\x1e\x01\n\n\n\x03\x04\
    \x02\x01\x12\x03\x1b\x08\x12\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1c\x02\
    \x10\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x1c\x02\x1b\x14\n\x0c\n\x05\x04\
    \x02\x02\0\x05\x12\x03\x1c\x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\
    \x1c\t\x0b\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x1c\x0e\x0f\n\x0b\n\x04\
    \x04\x02\x02\x01\x12\x03\x1d\x02\x13\n\r\n\x05\x04\x02\x02\x01\x04\x12\
    \x04\x1d\x02\x1c\x10\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x1d\x02\x08\
    \n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x1d\t\x0e\n\x0c\n\x05\x04\x02\
    \x02\x01\x03\x12\x03\x1d\x11\x12\n\n\n\x02\x04\x03\x12\x04\x20\0%\x01\n\
    \n\n\x03\x04\x03\x01\x12\x03\x20\x08\x0e\n\x0b\n\x04\x04\x03\x02\0\x12\
    \x03!\x02\x10\n\r\n\x05\x04\x03\x02\0\x04\x12\x04!\x02\x20\x10\n\x0c\n\
    \x05\x04\x03\x02\0\x05\x12\x03!\x02\x08\n\x0c\n\x05\x04\x03\x02\0\x01\
    \x12\x03!\t\x0b\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03!\x0e\x0f\n\x0b\n\
    \x04\x04\x03\x02\x01\x12\x03\"\x02\x15\n\r\n\x05\x04\x03\x02\x01\x04\x12\
    \x04\"\x02!\x10\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03\"\x02\x08\n\x0c\
    \n\x05\x04\x03\x02\x01\x01\x12\x03\"\t\x10\n\x0c\n\x05\x04\x03\x02\x01\
    \x03\x12\x03\"\x13\x14\n\x0b\n\x04\x04\x03\x02\x02\x12\x03#\x02\x1c\n\
    \x0c\n\x05\x04\x03\x02\x02\x04\x12\x03#\x02\n\n\x0c\n\x05\x04\x03\x02\
    \x02\x05\x12\x03#\x0b\x11\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03#\x12\
    \x17\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03#\x1a\x1b\n\x0b\n\x04\x04\
    \x03\x02\x03\x12\x03$\x02+\n\r\n\x05\x04\x03\x02\x03\x04\x12\x04$\x02#\
    \x1c\n\x0c\n\x05\x04\x03\x02\x03\x06\x12\x03$\x02\x1b\n\x0c\n\x05\x04\
    \x03\x02\x03\x01\x12\x03$\x1c&\n\x0c\n\x05\x04\x03\x02\x03\x03\x12\x03$)\
    *\n\n\n\x02\x04\x04\x12\x04'\0-\x01\n\n\n\x03\x04\x04\x01\x12\x03'\x08\
    \x10\n\x0b\n\x04\x04\x04\x02\0\x12\x03(\x02\x10\n\r\n\x05\x04\x04\x02\0\
    \x04\x12\x04(\x02'\x12\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03(\x02\x08\n\
    \x0c\n\x05\x04\x04\x02\0\x01\x12\x03(\t\x0b\n\x0c\n\x05\x04\x04\x02\0\
    \x03\x12\x03(\x0e\x0f\n\x0b\n\x04\x04\x04\x02\x01\x12\x03)\x02\x19\n\r\n\
    \x05\x04\x04\x02\x01\x04\x12\x04)\x02(\x10\n\x0c\n\x05\x04\x04\x02\x01\
    \x05\x12\x03)\x02\x08\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03)\t\x14\n\
    \x0c\n\x05\x04\x04\x02\x01\x03\x12\x03)\x17\x18\n\x0b\n\x04\x04\x04\x02\
    \x02\x12\x03*\x02\x15\n\r\n\x05\x04\x04\x02\x02\x04\x12\x04*\x02)\x19\n\
    \x0c\n\x05\x04\x04\x02\x02\x05\x12\x03*\x02\x08\n\x0c\n\x05\x04\x04\x02\
    \x02\x01\x12\x03*\t\x10\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03*\x13\x14\
    \n\x0b\n\x04\x04\x04\x02\x03\x12\x03+\x02+\n\r\n\x05\x04\x04\x02\x03\x04\
    \x12\x04+\x02*\x15\n\x0c\n\x05\x04\x04\x02\x03\x06\x12\x03+\x02\x1b\n\
    \x0c\n\x05\x04\x04\x02\x03\x01\x12\x03+\x1c&\n\x0c\n\x05\x04\x04\x02\x03\
    \x03\x12\x03+)*\n\x0b\n\x04\x04\x04\x02\x04\x12\x03,\x02\x13\n\r\n\x05\
    \x04\x04\x02\x04\x04\x12\x04,\x02++\n\x0c\n\x05\x04\x04\x02\x04\x05\x12\
    \x03,\x02\x08\n\x0c\n\x05\x04\x04\x02\x04\x01\x12\x03,\t\x0e\n\x0c\n\x05\
    \x04\x04\x02\x04\x03\x12\x03,\x11\x12\n\n\n\x02\x04\x05\x12\x04/\03\x01\
    \n\n\n\x03\x04\x05\x01\x12\x03/\x08\x16\n\x0b\n\x04\x04\x05\x02\0\x12\
    \x030\x02\"\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x030\x02\n\n\x0c\n\x05\x04\
    \x05\x02\0\x06\x12\x030\x0b\x13\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x030\
    \x14\x1d\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x030\x20!\n\x0b\n\x04\x04\x05\
    \x02\x01\x12\x031\x02\x14\n\r\n\x05\x04\x05\x02\x01\x04\x12\x041\x020\"\
    \n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x031\x02\x08\n\x0c\n\x05\x04\x05\
    \x02\x01\x01\x12\x031\t\x0f\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x031\x12\
    \x13\n\x0b\n\x04\x04\x05\x02\x02\x12\x032\x02\x15\n\r\n\x05\x04\x05\x02\
    \x02\x04\x12\x042\x021\x14\n\x0c\n\x05\x04\x05\x02\x02\x05\x12\x032\x02\
    \x08\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x032\t\x10\n\x0c\n\x05\x04\x05\
    \x02\x02\x03\x12\x032\x13\x14b\x06proto3\
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

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
pub struct GetEditionsParams {
    // message fields
    pub languages: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEditionsParams {}

impl GetEditionsParams {
    pub fn new() -> GetEditionsParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEditionsParams {
        static mut instance: ::protobuf::lazy::Lazy<GetEditionsParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEditionsParams,
        };
        unsafe {
            instance.get(GetEditionsParams::new)
        }
    }

    // string languages = 1;

    pub fn clear_languages(&mut self) {
        self.languages.clear();
    }

    // Param is passed by value, moved
    pub fn set_languages(&mut self, v: ::std::string::String) {
        self.languages = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_languages(&mut self) -> &mut ::std::string::String {
        &mut self.languages
    }

    // Take field
    pub fn take_languages(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.languages, ::std::string::String::new())
    }

    pub fn get_languages(&self) -> &str {
        &self.languages
    }

    fn get_languages_for_reflect(&self) -> &::std::string::String {
        &self.languages
    }

    fn mut_languages_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.languages
    }
}

impl ::protobuf::Message for GetEditionsParams {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.languages)?;
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
        if !self.languages.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.languages);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.languages.is_empty() {
            os.write_string(1, &self.languages)?;
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

impl ::protobuf::MessageStatic for GetEditionsParams {
    fn new() -> GetEditionsParams {
        GetEditionsParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEditionsParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "languages",
                    GetEditionsParams::get_languages_for_reflect,
                    GetEditionsParams::mut_languages_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEditionsParams>(
                    "GetEditionsParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEditionsParams {
    fn clear(&mut self) {
        self.clear_languages();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEditionsParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEditionsParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Edition {
    // message fields
    pub title: ::std::string::String,
    pub slug: ::std::string::String,
    pub editor: ::std::string::String,
    pub year: i32,
    pub language_code: ::std::string::String,
    pub created_at: ::std::string::String,
    pub updated_at: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Edition {}

impl Edition {
    pub fn new() -> Edition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Edition {
        static mut instance: ::protobuf::lazy::Lazy<Edition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Edition,
        };
        unsafe {
            instance.get(Edition::new)
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

    // string slug = 2;

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

    // string editor = 3;

    pub fn clear_editor(&mut self) {
        self.editor.clear();
    }

    // Param is passed by value, moved
    pub fn set_editor(&mut self, v: ::std::string::String) {
        self.editor = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_editor(&mut self) -> &mut ::std::string::String {
        &mut self.editor
    }

    // Take field
    pub fn take_editor(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.editor, ::std::string::String::new())
    }

    pub fn get_editor(&self) -> &str {
        &self.editor
    }

    fn get_editor_for_reflect(&self) -> &::std::string::String {
        &self.editor
    }

    fn mut_editor_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.editor
    }

    // int32 year = 4;

    pub fn clear_year(&mut self) {
        self.year = 0;
    }

    // Param is passed by value, moved
    pub fn set_year(&mut self, v: i32) {
        self.year = v;
    }

    pub fn get_year(&self) -> i32 {
        self.year
    }

    fn get_year_for_reflect(&self) -> &i32 {
        &self.year
    }

    fn mut_year_for_reflect(&mut self) -> &mut i32 {
        &mut self.year
    }

    // string language_code = 5;

    pub fn clear_language_code(&mut self) {
        self.language_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_language_code(&mut self, v: ::std::string::String) {
        self.language_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_language_code(&mut self) -> &mut ::std::string::String {
        &mut self.language_code
    }

    // Take field
    pub fn take_language_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.language_code, ::std::string::String::new())
    }

    pub fn get_language_code(&self) -> &str {
        &self.language_code
    }

    fn get_language_code_for_reflect(&self) -> &::std::string::String {
        &self.language_code
    }

    fn mut_language_code_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.language_code
    }

    // string created_at = 6;

    pub fn clear_created_at(&mut self) {
        self.created_at.clear();
    }

    // Param is passed by value, moved
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_created_at(&mut self) -> &mut ::std::string::String {
        &mut self.created_at
    }

    // Take field
    pub fn take_created_at(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.created_at, ::std::string::String::new())
    }

    pub fn get_created_at(&self) -> &str {
        &self.created_at
    }

    fn get_created_at_for_reflect(&self) -> &::std::string::String {
        &self.created_at
    }

    fn mut_created_at_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.created_at
    }

    // string updated_at = 7;

    pub fn clear_updated_at(&mut self) {
        self.updated_at.clear();
    }

    // Param is passed by value, moved
    pub fn set_updated_at(&mut self, v: ::std::string::String) {
        self.updated_at = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_updated_at(&mut self) -> &mut ::std::string::String {
        &mut self.updated_at
    }

    // Take field
    pub fn take_updated_at(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.updated_at, ::std::string::String::new())
    }

    pub fn get_updated_at(&self) -> &str {
        &self.updated_at
    }

    fn get_updated_at_for_reflect(&self) -> &::std::string::String {
        &self.updated_at
    }

    fn mut_updated_at_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.updated_at
    }
}

impl ::protobuf::Message for Edition {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.slug)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.editor)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.year = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.language_code)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.created_at)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.updated_at)?;
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
        if !self.slug.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.slug);
        }
        if !self.editor.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.editor);
        }
        if self.year != 0 {
            my_size += ::protobuf::rt::value_size(4, self.year, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.language_code.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.language_code);
        }
        if !self.created_at.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.created_at);
        }
        if !self.updated_at.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.updated_at);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.title.is_empty() {
            os.write_string(1, &self.title)?;
        }
        if !self.slug.is_empty() {
            os.write_string(2, &self.slug)?;
        }
        if !self.editor.is_empty() {
            os.write_string(3, &self.editor)?;
        }
        if self.year != 0 {
            os.write_int32(4, self.year)?;
        }
        if !self.language_code.is_empty() {
            os.write_string(5, &self.language_code)?;
        }
        if !self.created_at.is_empty() {
            os.write_string(6, &self.created_at)?;
        }
        if !self.updated_at.is_empty() {
            os.write_string(7, &self.updated_at)?;
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

impl ::protobuf::MessageStatic for Edition {
    fn new() -> Edition {
        Edition::new()
    }

    fn descriptor_static(_: ::std::option::Option<Edition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    Edition::get_title_for_reflect,
                    Edition::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "slug",
                    Edition::get_slug_for_reflect,
                    Edition::mut_slug_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "editor",
                    Edition::get_editor_for_reflect,
                    Edition::mut_editor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "year",
                    Edition::get_year_for_reflect,
                    Edition::mut_year_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "language_code",
                    Edition::get_language_code_for_reflect,
                    Edition::mut_language_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "created_at",
                    Edition::get_created_at_for_reflect,
                    Edition::mut_created_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "updated_at",
                    Edition::get_updated_at_for_reflect,
                    Edition::mut_updated_at_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Edition>(
                    "Edition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Edition {
    fn clear(&mut self) {
        self.clear_title();
        self.clear_slug();
        self.clear_editor();
        self.clear_year();
        self.clear_language_code();
        self.clear_created_at();
        self.clear_updated_at();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Edition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Edition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Editions {
    // message fields
    pub data: ::protobuf::RepeatedField<Edition>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Editions {}

impl Editions {
    pub fn new() -> Editions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Editions {
        static mut instance: ::protobuf::lazy::Lazy<Editions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Editions,
        };
        unsafe {
            instance.get(Editions::new)
        }
    }

    // repeated .repository.Edition data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::protobuf::RepeatedField<Edition>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    pub fn mut_data(&mut self) -> &mut ::protobuf::RepeatedField<Edition> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::protobuf::RepeatedField<Edition> {
        ::std::mem::replace(&mut self.data, ::protobuf::RepeatedField::new())
    }

    pub fn get_data(&self) -> &[Edition] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::protobuf::RepeatedField<Edition> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Edition> {
        &mut self.data
    }
}

impl ::protobuf::Message for Editions {
    fn is_initialized(&self) -> bool {
        for v in &self.data {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.data)?;
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
        for value in &self.data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.data {
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

impl ::protobuf::MessageStatic for Editions {
    fn new() -> Editions {
        Editions::new()
    }

    fn descriptor_static(_: ::std::option::Option<Editions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Edition>>(
                    "data",
                    Editions::get_data_for_reflect,
                    Editions::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Editions>(
                    "Editions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Editions {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Editions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Editions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetSchemaParams {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetSchemaParams {}

impl GetSchemaParams {
    pub fn new() -> GetSchemaParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetSchemaParams {
        static mut instance: ::protobuf::lazy::Lazy<GetSchemaParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetSchemaParams,
        };
        unsafe {
            instance.get(GetSchemaParams::new)
        }
    }
}

impl ::protobuf::Message for GetSchemaParams {
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetSchemaParams {
    fn new() -> GetSchemaParams {
        GetSchemaParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetSchemaParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetSchemaParams>(
                    "GetSchemaParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetSchemaParams {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetSchemaParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetSchemaParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EthicsSchema {
    // message fields
    pub parts: ::protobuf::RepeatedField<EthicsSchema_Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EthicsSchema {}

impl EthicsSchema {
    pub fn new() -> EthicsSchema {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EthicsSchema {
        static mut instance: ::protobuf::lazy::Lazy<EthicsSchema> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EthicsSchema,
        };
        unsafe {
            instance.get(EthicsSchema::new)
        }
    }

    // repeated .repository.EthicsSchema.Node parts = 1;

    pub fn clear_parts(&mut self) {
        self.parts.clear();
    }

    // Param is passed by value, moved
    pub fn set_parts(&mut self, v: ::protobuf::RepeatedField<EthicsSchema_Node>) {
        self.parts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_parts(&mut self) -> &mut ::protobuf::RepeatedField<EthicsSchema_Node> {
        &mut self.parts
    }

    // Take field
    pub fn take_parts(&mut self) -> ::protobuf::RepeatedField<EthicsSchema_Node> {
        ::std::mem::replace(&mut self.parts, ::protobuf::RepeatedField::new())
    }

    pub fn get_parts(&self) -> &[EthicsSchema_Node] {
        &self.parts
    }

    fn get_parts_for_reflect(&self) -> &::protobuf::RepeatedField<EthicsSchema_Node> {
        &self.parts
    }

    fn mut_parts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EthicsSchema_Node> {
        &mut self.parts
    }
}

impl ::protobuf::Message for EthicsSchema {
    fn is_initialized(&self) -> bool {
        for v in &self.parts {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.parts)?;
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
        for value in &self.parts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.parts {
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

impl ::protobuf::MessageStatic for EthicsSchema {
    fn new() -> EthicsSchema {
        EthicsSchema::new()
    }

    fn descriptor_static(_: ::std::option::Option<EthicsSchema>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EthicsSchema_Node>>(
                    "parts",
                    EthicsSchema::get_parts_for_reflect,
                    EthicsSchema::mut_parts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EthicsSchema>(
                    "EthicsSchema",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EthicsSchema {
    fn clear(&mut self) {
        self.clear_parts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EthicsSchema {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EthicsSchema {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EthicsSchema_Node {
    // message fields
    pub node_type: EthicsSchema_NodeType,
    pub children: ::protobuf::RepeatedField<EthicsSchema_Node>,
    // message oneof groups
    identifier: ::std::option::Option<EthicsSchema_Node_oneof_identifier>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EthicsSchema_Node {}

#[derive(Clone,PartialEq)]
pub enum EthicsSchema_Node_oneof_identifier {
    num(i32),
    title(::std::string::String),
}

impl EthicsSchema_Node {
    pub fn new() -> EthicsSchema_Node {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EthicsSchema_Node {
        static mut instance: ::protobuf::lazy::Lazy<EthicsSchema_Node> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EthicsSchema_Node,
        };
        unsafe {
            instance.get(EthicsSchema_Node::new)
        }
    }

    // .repository.EthicsSchema.NodeType node_type = 1;

    pub fn clear_node_type(&mut self) {
        self.node_type = EthicsSchema_NodeType::UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_node_type(&mut self, v: EthicsSchema_NodeType) {
        self.node_type = v;
    }

    pub fn get_node_type(&self) -> EthicsSchema_NodeType {
        self.node_type
    }

    fn get_node_type_for_reflect(&self) -> &EthicsSchema_NodeType {
        &self.node_type
    }

    fn mut_node_type_for_reflect(&mut self) -> &mut EthicsSchema_NodeType {
        &mut self.node_type
    }

    // int32 num = 2;

    pub fn clear_num(&mut self) {
        self.identifier = ::std::option::Option::None;
    }

    pub fn has_num(&self) -> bool {
        match self.identifier {
            ::std::option::Option::Some(EthicsSchema_Node_oneof_identifier::num(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_num(&mut self, v: i32) {
        self.identifier = ::std::option::Option::Some(EthicsSchema_Node_oneof_identifier::num(v))
    }

    pub fn get_num(&self) -> i32 {
        match self.identifier {
            ::std::option::Option::Some(EthicsSchema_Node_oneof_identifier::num(v)) => v,
            _ => 0,
        }
    }

    // string title = 3;

    pub fn clear_title(&mut self) {
        self.identifier = ::std::option::Option::None;
    }

    pub fn has_title(&self) -> bool {
        match self.identifier {
            ::std::option::Option::Some(EthicsSchema_Node_oneof_identifier::title(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.identifier = ::std::option::Option::Some(EthicsSchema_Node_oneof_identifier::title(v))
    }

    // Mutable pointer to the field.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(EthicsSchema_Node_oneof_identifier::title(_)) = self.identifier {
        } else {
            self.identifier = ::std::option::Option::Some(EthicsSchema_Node_oneof_identifier::title(::std::string::String::new()));
        }
        match self.identifier {
            ::std::option::Option::Some(EthicsSchema_Node_oneof_identifier::title(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        if self.has_title() {
            match self.identifier.take() {
                ::std::option::Option::Some(EthicsSchema_Node_oneof_identifier::title(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_title(&self) -> &str {
        match self.identifier {
            ::std::option::Option::Some(EthicsSchema_Node_oneof_identifier::title(ref v)) => v,
            _ => "",
        }
    }

    // repeated .repository.EthicsSchema.Node children = 4;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<EthicsSchema_Node>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<EthicsSchema_Node> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<EthicsSchema_Node> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[EthicsSchema_Node] {
        &self.children
    }

    fn get_children_for_reflect(&self) -> &::protobuf::RepeatedField<EthicsSchema_Node> {
        &self.children
    }

    fn mut_children_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EthicsSchema_Node> {
        &mut self.children
    }
}

impl ::protobuf::Message for EthicsSchema_Node {
    fn is_initialized(&self) -> bool {
        for v in &self.children {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.node_type = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.identifier = ::std::option::Option::Some(EthicsSchema_Node_oneof_identifier::num(is.read_int32()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.identifier = ::std::option::Option::Some(EthicsSchema_Node_oneof_identifier::title(is.read_string()?));
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.children)?;
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
        if self.node_type != EthicsSchema_NodeType::UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(1, self.node_type);
        }
        for value in &self.children {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let ::std::option::Option::Some(ref v) = self.identifier {
            match v {
                &EthicsSchema_Node_oneof_identifier::num(v) => {
                    my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &EthicsSchema_Node_oneof_identifier::title(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.node_type != EthicsSchema_NodeType::UNSPECIFIED {
            os.write_enum(1, self.node_type.value())?;
        }
        for v in &self.children {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let ::std::option::Option::Some(ref v) = self.identifier {
            match v {
                &EthicsSchema_Node_oneof_identifier::num(v) => {
                    os.write_int32(2, v)?;
                },
                &EthicsSchema_Node_oneof_identifier::title(ref v) => {
                    os.write_string(3, v)?;
                },
            };
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

impl ::protobuf::MessageStatic for EthicsSchema_Node {
    fn new() -> EthicsSchema_Node {
        EthicsSchema_Node::new()
    }

    fn descriptor_static(_: ::std::option::Option<EthicsSchema_Node>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EthicsSchema_NodeType>>(
                    "node_type",
                    EthicsSchema_Node::get_node_type_for_reflect,
                    EthicsSchema_Node::mut_node_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor::<_>(
                    "num",
                    EthicsSchema_Node::has_num,
                    EthicsSchema_Node::get_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "title",
                    EthicsSchema_Node::has_title,
                    EthicsSchema_Node::get_title,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EthicsSchema_Node>>(
                    "children",
                    EthicsSchema_Node::get_children_for_reflect,
                    EthicsSchema_Node::mut_children_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EthicsSchema_Node>(
                    "EthicsSchema_Node",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EthicsSchema_Node {
    fn clear(&mut self) {
        self.clear_node_type();
        self.clear_num();
        self.clear_title();
        self.clear_children();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EthicsSchema_Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EthicsSchema_Node {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EthicsSchema_NodeType {
    UNSPECIFIED = 0,
    ANONYMOUS_FRAGMENT = 1,
    ALITER = 2,
    APPENDIX = 3,
    AXIOMA = 4,
    CAPUT = 5,
    COROLLARIUM = 6,
    DEFINITIO = 7,
    DEMONSTRATIO = 8,
    EXPLICATIO = 9,
    LEMMA = 11,
    PARS = 12,
    POSTULATUM = 13,
    PRAEFATIO = 14,
    PROPOSITIO = 15,
    SCHOLIUM = 16,
}

impl ::protobuf::ProtobufEnum for EthicsSchema_NodeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EthicsSchema_NodeType> {
        match value {
            0 => ::std::option::Option::Some(EthicsSchema_NodeType::UNSPECIFIED),
            1 => ::std::option::Option::Some(EthicsSchema_NodeType::ANONYMOUS_FRAGMENT),
            2 => ::std::option::Option::Some(EthicsSchema_NodeType::ALITER),
            3 => ::std::option::Option::Some(EthicsSchema_NodeType::APPENDIX),
            4 => ::std::option::Option::Some(EthicsSchema_NodeType::AXIOMA),
            5 => ::std::option::Option::Some(EthicsSchema_NodeType::CAPUT),
            6 => ::std::option::Option::Some(EthicsSchema_NodeType::COROLLARIUM),
            7 => ::std::option::Option::Some(EthicsSchema_NodeType::DEFINITIO),
            8 => ::std::option::Option::Some(EthicsSchema_NodeType::DEMONSTRATIO),
            9 => ::std::option::Option::Some(EthicsSchema_NodeType::EXPLICATIO),
            11 => ::std::option::Option::Some(EthicsSchema_NodeType::LEMMA),
            12 => ::std::option::Option::Some(EthicsSchema_NodeType::PARS),
            13 => ::std::option::Option::Some(EthicsSchema_NodeType::POSTULATUM),
            14 => ::std::option::Option::Some(EthicsSchema_NodeType::PRAEFATIO),
            15 => ::std::option::Option::Some(EthicsSchema_NodeType::PROPOSITIO),
            16 => ::std::option::Option::Some(EthicsSchema_NodeType::SCHOLIUM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EthicsSchema_NodeType] = &[
            EthicsSchema_NodeType::UNSPECIFIED,
            EthicsSchema_NodeType::ANONYMOUS_FRAGMENT,
            EthicsSchema_NodeType::ALITER,
            EthicsSchema_NodeType::APPENDIX,
            EthicsSchema_NodeType::AXIOMA,
            EthicsSchema_NodeType::CAPUT,
            EthicsSchema_NodeType::COROLLARIUM,
            EthicsSchema_NodeType::DEFINITIO,
            EthicsSchema_NodeType::DEMONSTRATIO,
            EthicsSchema_NodeType::EXPLICATIO,
            EthicsSchema_NodeType::LEMMA,
            EthicsSchema_NodeType::PARS,
            EthicsSchema_NodeType::POSTULATUM,
            EthicsSchema_NodeType::PRAEFATIO,
            EthicsSchema_NodeType::PROPOSITIO,
            EthicsSchema_NodeType::SCHOLIUM,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EthicsSchema_NodeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EthicsSchema_NodeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EthicsSchema_NodeType {
}

impl ::std::default::Default for EthicsSchema_NodeType {
    fn default() -> Self {
        EthicsSchema_NodeType::UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for EthicsSchema_NodeType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10repository.proto\x12\nrepository\x1a\x1cgoogle/api/annotations.pro\
    to\"1\n\x11GetEditionsParams\x12\x1c\n\tlanguages\x18\x01\x20\x01(\tR\tl\
    anguages\"\xc2\x01\n\x07Edition\x12\x14\n\x05title\x18\x01\x20\x01(\tR\
    \x05title\x12\x12\n\x04slug\x18\x02\x20\x01(\tR\x04slug\x12\x16\n\x06edi\
    tor\x18\x03\x20\x01(\tR\x06editor\x12\x12\n\x04year\x18\x04\x20\x01(\x05\
    R\x04year\x12#\n\rlanguage_code\x18\x05\x20\x01(\tR\x0clanguageCode\x12\
    \x1d\n\ncreated_at\x18\x06\x20\x01(\tR\tcreatedAt\x12\x1d\n\nupdated_at\
    \x18\x07\x20\x01(\tR\tupdatedAt\"3\n\x08Editions\x12'\n\x04data\x18\x01\
    \x20\x03(\x0b2\x13.repository.EditionR\x04data\"\x11\n\x0fGetSchemaParam\
    s\"\xfc\x03\n\x0cEthicsSchema\x123\n\x05parts\x18\x01\x20\x03(\x0b2\x1d.\
    repository.EthicsSchema.NodeR\x05parts\x1a\xbb\x01\n\x04Node\x12>\n\tnod\
    e_type\x18\x01\x20\x01(\x0e2!.repository.EthicsSchema.NodeTypeR\x08nodeT\
    ype\x12\x12\n\x03num\x18\x02\x20\x01(\x05H\0R\x03num\x12\x16\n\x05title\
    \x18\x03\x20\x01(\tH\0R\x05title\x129\n\x08children\x18\x04\x20\x03(\x0b\
    2\x1d.repository.EthicsSchema.NodeR\x08childrenB\x0c\n\nidentifier\"\xf8\
    \x01\n\x08NodeType\x12\x0f\n\x0bUNSPECIFIED\x10\0\x12\x16\n\x12ANONYMOUS\
    _FRAGMENT\x10\x01\x12\n\n\x06ALITER\x10\x02\x12\x0c\n\x08APPENDIX\x10\
    \x03\x12\n\n\x06AXIOMA\x10\x04\x12\t\n\x05CAPUT\x10\x05\x12\x0f\n\x0bCOR\
    OLLARIUM\x10\x06\x12\r\n\tDEFINITIO\x10\x07\x12\x10\n\x0cDEMONSTRATIO\
    \x10\x08\x12\x0e\n\nEXPLICATIO\x10\t\x12\t\n\x05LEMMA\x10\x0b\x12\x08\n\
    \x04PARS\x10\x0c\x12\x0e\n\nPOSTULATUM\x10\r\x12\r\n\tPRAEFATIO\x10\x0e\
    \x12\x0e\n\nPROPOSITIO\x10\x0f\x12\x0c\n\x08SCHOLIUM\x10\x102\xd2\x01\n\
    \x10EthicsRepository\x12]\n\tGetSchema\x12\x1b.repository.GetSchemaParam\
    s\x1a\x18.repository.EthicsSchema\"\x19\x82\xd3\xe4\x93\x02\x13\x12\x11/\
    v1/ethics/schema\x12_\n\x0bGetEditions\x12\x1d.repository.GetEditionsPar\
    ams\x1a\x14.repository.Editions\"\x1b\x82\xd3\xe4\x93\x02\x15\x12\x13/v1\
    /ethics/editionsJ\xc7\x12\n\x06\x12\x04\0\0E\x01\n\x08\n\x01\x0c\x12\x03\
    \0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x12\n\t\n\x02\x03\0\x12\x03\x03\
    \x07%\n\n\n\x02\x06\0\x12\x04\x05\0\x10\x01\n\n\n\x03\x06\0\x01\x12\x03\
    \x05\x08\x18\n\x0c\n\x04\x06\0\x02\0\x12\x04\x06\x02\n\x03\n\x0c\n\x05\
    \x06\0\x02\0\x01\x12\x03\x06\x06\x0f\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\
    \x06\x11\x20\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x06+7\n\r\n\x05\x06\0\
    \x02\0\x04\x12\x04\x07\x04\t\x06\n\x10\n\x08\x06\0\x02\0\x04\xe7\x07\0\
    \x12\x04\x07\x04\t\x06\n\x10\n\t\x06\0\x02\0\x04\xe7\x07\0\x02\x12\x03\
    \x07\x0b\x1c\n\x11\n\n\x06\0\x02\0\x04\xe7\x07\0\x02\0\x12\x03\x07\x0b\
    \x1c\n\x12\n\x0b\x06\0\x02\0\x04\xe7\x07\0\x02\0\x01\x12\x03\x07\x0c\x1b\
    \n\x11\n\t\x06\0\x02\0\x04\xe7\x07\0\x08\x12\x04\x07\x1f\t\x05\n\x0c\n\
    \x04\x06\0\x02\x01\x12\x04\x0b\x02\x0f\x03\n\x0c\n\x05\x06\0\x02\x01\x01\
    \x12\x03\x0b\x06\x11\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x0b\x13$\n\
    \x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x0b/7\n\r\n\x05\x06\0\x02\x01\x04\
    \x12\x04\x0c\x04\x0e\x06\n\x10\n\x08\x06\0\x02\x01\x04\xe7\x07\0\x12\x04\
    \x0c\x04\x0e\x06\n\x10\n\t\x06\0\x02\x01\x04\xe7\x07\0\x02\x12\x03\x0c\
    \x0b\x1c\n\x11\n\n\x06\0\x02\x01\x04\xe7\x07\0\x02\0\x12\x03\x0c\x0b\x1c\
    \n\x12\n\x0b\x06\0\x02\x01\x04\xe7\x07\0\x02\0\x01\x12\x03\x0c\x0c\x1b\n\
    \x11\n\t\x06\0\x02\x01\x04\xe7\x07\0\x08\x12\x04\x0c\x1f\x0e\x05\n\n\n\
    \x02\x04\0\x12\x04\x12\0\x14\x01\n\n\n\x03\x04\0\x01\x12\x03\x12\x08\x19\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\x13\x02\x17\n\r\n\x05\x04\0\x02\0\x04\
    \x12\x04\x13\x02\x12\x1b\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x13\x02\x08\
    \n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x13\t\x12\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x13\x15\x16\n\n\n\x02\x04\x01\x12\x04\x16\0\x1e\x01\n\n\n\
    \x03\x04\x01\x01\x12\x03\x16\x08\x0f\n\x0b\n\x04\x04\x01\x02\0\x12\x03\
    \x17\x02\x13\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x17\x02\x16\x11\n\x0c\n\
    \x05\x04\x01\x02\0\x05\x12\x03\x17\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\
    \x12\x03\x17\t\x0e\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x17\x11\x12\n\
    \x0b\n\x04\x04\x01\x02\x01\x12\x03\x18\x02\x12\n\r\n\x05\x04\x01\x02\x01\
    \x04\x12\x04\x18\x02\x17\x13\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x18\
    \x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x18\t\r\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03\x18\x10\x11\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\
    \x19\x02\x14\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\x19\x02\x18\x12\n\x0c\
    \n\x05\x04\x01\x02\x02\x05\x12\x03\x19\x02\x08\n\x0c\n\x05\x04\x01\x02\
    \x02\x01\x12\x03\x19\t\x0f\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x19\
    \x12\x13\n\x0b\n\x04\x04\x01\x02\x03\x12\x03\x1a\x02\x11\n\r\n\x05\x04\
    \x01\x02\x03\x04\x12\x04\x1a\x02\x19\x14\n\x0c\n\x05\x04\x01\x02\x03\x05\
    \x12\x03\x1a\x02\x07\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x1a\x08\x0c\
    \n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x1a\x0f\x10\n\x0b\n\x04\x04\x01\
    \x02\x04\x12\x03\x1b\x02\x1b\n\r\n\x05\x04\x01\x02\x04\x04\x12\x04\x1b\
    \x02\x1a\x11\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\x03\x1b\x02\x08\n\x0c\n\
    \x05\x04\x01\x02\x04\x01\x12\x03\x1b\t\x16\n\x0c\n\x05\x04\x01\x02\x04\
    \x03\x12\x03\x1b\x19\x1a\n\x0b\n\x04\x04\x01\x02\x05\x12\x03\x1c\x02\x18\
    \n\r\n\x05\x04\x01\x02\x05\x04\x12\x04\x1c\x02\x1b\x1b\n\x0c\n\x05\x04\
    \x01\x02\x05\x05\x12\x03\x1c\x02\x08\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\
    \x03\x1c\t\x13\n\x0c\n\x05\x04\x01\x02\x05\x03\x12\x03\x1c\x16\x17\n\x0b\
    \n\x04\x04\x01\x02\x06\x12\x03\x1d\x02\x18\n\r\n\x05\x04\x01\x02\x06\x04\
    \x12\x04\x1d\x02\x1c\x18\n\x0c\n\x05\x04\x01\x02\x06\x05\x12\x03\x1d\x02\
    \x08\n\x0c\n\x05\x04\x01\x02\x06\x01\x12\x03\x1d\t\x13\n\x0c\n\x05\x04\
    \x01\x02\x06\x03\x12\x03\x1d\x16\x17\n\n\n\x02\x04\x02\x12\x04\x20\0\"\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03\x20\x08\x10\n\x0b\n\x04\x04\x02\x02\0\
    \x12\x03!\x02\x1c\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03!\x02\n\n\x0c\n\
    \x05\x04\x02\x02\0\x06\x12\x03!\x0b\x12\n\x0c\n\x05\x04\x02\x02\0\x01\
    \x12\x03!\x13\x17\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03!\x1a\x1b\n\t\n\
    \x02\x04\x03\x12\x03$\0\x1a\n\n\n\x03\x04\x03\x01\x12\x03$\x08\x17\n\n\n\
    \x02\x04\x04\x12\x04&\0E\x01\n\n\n\x03\x04\x04\x01\x12\x03&\x08\x14\n\
    \x0c\n\x04\x04\x04\x04\0\x12\x04'\x028\x03\n\x0c\n\x05\x04\x04\x04\0\x01\
    \x12\x03'\x07\x0f\n\r\n\x06\x04\x04\x04\0\x02\0\x12\x03(\x04\x14\n\x0e\n\
    \x07\x04\x04\x04\0\x02\0\x01\x12\x03(\x04\x0f\n\x0e\n\x07\x04\x04\x04\0\
    \x02\0\x02\x12\x03(\x12\x13\n\r\n\x06\x04\x04\x04\0\x02\x01\x12\x03)\x04\
    \x1b\n\x0e\n\x07\x04\x04\x04\0\x02\x01\x01\x12\x03)\x04\x16\n\x0e\n\x07\
    \x04\x04\x04\0\x02\x01\x02\x12\x03)\x19\x1a\n\r\n\x06\x04\x04\x04\0\x02\
    \x02\x12\x03*\x04\x0f\n\x0e\n\x07\x04\x04\x04\0\x02\x02\x01\x12\x03*\x04\
    \n\n\x0e\n\x07\x04\x04\x04\0\x02\x02\x02\x12\x03*\r\x0e\n\r\n\x06\x04\
    \x04\x04\0\x02\x03\x12\x03+\x04\x11\n\x0e\n\x07\x04\x04\x04\0\x02\x03\
    \x01\x12\x03+\x04\x0c\n\x0e\n\x07\x04\x04\x04\0\x02\x03\x02\x12\x03+\x0f\
    \x10\n\r\n\x06\x04\x04\x04\0\x02\x04\x12\x03,\x04\x0f\n\x0e\n\x07\x04\
    \x04\x04\0\x02\x04\x01\x12\x03,\x04\n\n\x0e\n\x07\x04\x04\x04\0\x02\x04\
    \x02\x12\x03,\r\x0e\n\r\n\x06\x04\x04\x04\0\x02\x05\x12\x03-\x04\x0e\n\
    \x0e\n\x07\x04\x04\x04\0\x02\x05\x01\x12\x03-\x04\t\n\x0e\n\x07\x04\x04\
    \x04\0\x02\x05\x02\x12\x03-\x0c\r\n\r\n\x06\x04\x04\x04\0\x02\x06\x12\
    \x03.\x04\x14\n\x0e\n\x07\x04\x04\x04\0\x02\x06\x01\x12\x03.\x04\x0f\n\
    \x0e\n\x07\x04\x04\x04\0\x02\x06\x02\x12\x03.\x12\x13\n\r\n\x06\x04\x04\
    \x04\0\x02\x07\x12\x03/\x04\x12\n\x0e\n\x07\x04\x04\x04\0\x02\x07\x01\
    \x12\x03/\x04\r\n\x0e\n\x07\x04\x04\x04\0\x02\x07\x02\x12\x03/\x10\x11\n\
    \r\n\x06\x04\x04\x04\0\x02\x08\x12\x030\x04\x15\n\x0e\n\x07\x04\x04\x04\
    \0\x02\x08\x01\x12\x030\x04\x10\n\x0e\n\x07\x04\x04\x04\0\x02\x08\x02\
    \x12\x030\x13\x14\n\r\n\x06\x04\x04\x04\0\x02\t\x12\x031\x04\x13\n\x0e\n\
    \x07\x04\x04\x04\0\x02\t\x01\x12\x031\x04\x0e\n\x0e\n\x07\x04\x04\x04\0\
    \x02\t\x02\x12\x031\x11\x12\n\r\n\x06\x04\x04\x04\0\x02\n\x12\x032\x04\
    \x0f\n\x0e\n\x07\x04\x04\x04\0\x02\n\x01\x12\x032\x04\t\n\x0e\n\x07\x04\
    \x04\x04\0\x02\n\x02\x12\x032\x0c\x0e\n\r\n\x06\x04\x04\x04\0\x02\x0b\
    \x12\x033\x04\x0e\n\x0e\n\x07\x04\x04\x04\0\x02\x0b\x01\x12\x033\x04\x08\
    \n\x0e\n\x07\x04\x04\x04\0\x02\x0b\x02\x12\x033\x0b\r\n\r\n\x06\x04\x04\
    \x04\0\x02\x0c\x12\x034\x04\x14\n\x0e\n\x07\x04\x04\x04\0\x02\x0c\x01\
    \x12\x034\x04\x0e\n\x0e\n\x07\x04\x04\x04\0\x02\x0c\x02\x12\x034\x11\x13\
    \n\r\n\x06\x04\x04\x04\0\x02\r\x12\x035\x04\x13\n\x0e\n\x07\x04\x04\x04\
    \0\x02\r\x01\x12\x035\x04\r\n\x0e\n\x07\x04\x04\x04\0\x02\r\x02\x12\x035\
    \x10\x12\n\r\n\x06\x04\x04\x04\0\x02\x0e\x12\x036\x04\x14\n\x0e\n\x07\
    \x04\x04\x04\0\x02\x0e\x01\x12\x036\x04\x0e\n\x0e\n\x07\x04\x04\x04\0\
    \x02\x0e\x02\x12\x036\x11\x13\n\r\n\x06\x04\x04\x04\0\x02\x0f\x12\x037\
    \x04\x12\n\x0e\n\x07\x04\x04\x04\0\x02\x0f\x01\x12\x037\x04\x0c\n\x0e\n\
    \x07\x04\x04\x04\0\x02\x0f\x02\x12\x037\x0f\x11\n\x0c\n\x04\x04\x04\x03\
    \0\x12\x04:\x02B\x03\n\x0c\n\x05\x04\x04\x03\0\x01\x12\x03:\n\x0e\n\r\n\
    \x06\x04\x04\x03\0\x02\0\x12\x03;\x04\x1b\n\x0f\n\x07\x04\x04\x03\0\x02\
    \0\x04\x12\x04;\x04:\x10\n\x0e\n\x07\x04\x04\x03\0\x02\0\x06\x12\x03;\
    \x04\x0c\n\x0e\n\x07\x04\x04\x03\0\x02\0\x01\x12\x03;\r\x16\n\x0e\n\x07\
    \x04\x04\x03\0\x02\0\x03\x12\x03;\x19\x1a\n\x0e\n\x06\x04\x04\x03\0\x08\
    \0\x12\x04<\x04?\x05\n\x0e\n\x07\x04\x04\x03\0\x08\0\x01\x12\x03<\n\x14\
    \n'\n\x06\x04\x04\x03\0\x02\x01\x12\x03=\x06\x14\"\x18\x20for\x20numbere\
    d\x20fragments\n\n\x0e\n\x07\x04\x04\x03\0\x02\x01\x05\x12\x03=\x06\x0b\
    \n\x0e\n\x07\x04\x04\x03\0\x02\x01\x01\x12\x03=\x0c\x0f\n\x0e\n\x07\x04\
    \x04\x03\0\x02\x01\x03\x12\x03=\x12\x13\n\x1b\n\x06\x04\x04\x03\0\x02\
    \x02\x12\x03>\x06\x17\"\x0c\x20for\x20scopes\n\n\x0e\n\x07\x04\x04\x03\0\
    \x02\x02\x05\x12\x03>\x06\x0c\n\x0e\n\x07\x04\x04\x03\0\x02\x02\x01\x12\
    \x03>\r\x12\n\x0e\n\x07\x04\x04\x03\0\x02\x02\x03\x12\x03>\x15\x16\n\r\n\
    \x06\x04\x04\x03\0\x02\x03\x12\x03@\x04\x1f\n\x0e\n\x07\x04\x04\x03\0\
    \x02\x03\x04\x12\x03@\x04\x0c\n\x0e\n\x07\x04\x04\x03\0\x02\x03\x06\x12\
    \x03@\r\x11\n\x0e\n\x07\x04\x04\x03\0\x02\x03\x01\x12\x03@\x12\x1a\n\x0e\
    \n\x07\x04\x04\x03\0\x02\x03\x03\x12\x03@\x1d\x1e\n\x0b\n\x04\x04\x04\
    \x02\0\x12\x03D\x02\x1a\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03D\x02\n\n\
    \x0c\n\x05\x04\x04\x02\0\x06\x12\x03D\x0b\x0f\n\x0c\n\x05\x04\x04\x02\0\
    \x01\x12\x03D\x10\x15\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03D\x18\x19b\
    \x06proto3\
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

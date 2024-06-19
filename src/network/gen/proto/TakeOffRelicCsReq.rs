// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `TakeOffRelicCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TakeOffRelicCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TakeOffRelicCsReq {
    // message fields
    // @@protoc_insertion_point(field:TakeOffRelicCsReq.slot_list)
    pub slot_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:TakeOffRelicCsReq.equip_avatar_id)
    pub equip_avatar_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TakeOffRelicCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TakeOffRelicCsReq {
    fn default() -> &'a TakeOffRelicCsReq {
        <TakeOffRelicCsReq as ::protobuf::Message>::default_instance()
    }
}

impl TakeOffRelicCsReq {
    pub fn new() -> TakeOffRelicCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "slot_list",
            |m: &TakeOffRelicCsReq| { &m.slot_list },
            |m: &mut TakeOffRelicCsReq| { &mut m.slot_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "equip_avatar_id",
            |m: &TakeOffRelicCsReq| { &m.equip_avatar_id },
            |m: &mut TakeOffRelicCsReq| { &mut m.equip_avatar_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TakeOffRelicCsReq>(
            "TakeOffRelicCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TakeOffRelicCsReq {
    const NAME: &'static str = "TakeOffRelicCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.slot_list)?;
                },
                64 => {
                    self.slot_list.push(is.read_uint32()?);
                },
                8 => {
                    self.equip_avatar_id = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.slot_list {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        if self.equip_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.equip_avatar_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.slot_list {
            os.write_uint32(8, *v)?;
        };
        if self.equip_avatar_id != 0 {
            os.write_uint32(1, self.equip_avatar_id)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> TakeOffRelicCsReq {
        TakeOffRelicCsReq::new()
    }

    fn clear(&mut self) {
        self.slot_list.clear();
        self.equip_avatar_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TakeOffRelicCsReq {
        static instance: TakeOffRelicCsReq = TakeOffRelicCsReq {
            slot_list: ::std::vec::Vec::new(),
            equip_avatar_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TakeOffRelicCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TakeOffRelicCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TakeOffRelicCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TakeOffRelicCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17TakeOffRelicCsReq.proto\"X\n\x11TakeOffRelicCsReq\x12\x1b\n\tslot_\
    list\x18\x08\x20\x03(\rR\x08slotList\x12&\n\x0fequip_avatar_id\x18\x01\
    \x20\x01(\rR\requipAvatarIdB\x15\n\x13emu.lunarcore.protob\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TakeOffRelicCsReq::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}

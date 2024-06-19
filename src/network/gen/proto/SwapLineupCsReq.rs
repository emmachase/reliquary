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

//! Generated file from `SwapLineupCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SwapLineupCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SwapLineupCsReq {
    // message fields
    // @@protoc_insertion_point(field:SwapLineupCsReq.extra_lineup_type)
    pub extra_lineup_type: ::protobuf::EnumOrUnknown<super::ExtraLineupType::ExtraLineupType>,
    // @@protoc_insertion_point(field:SwapLineupCsReq.plane_id)
    pub plane_id: u32,
    // @@protoc_insertion_point(field:SwapLineupCsReq.index)
    pub index: u32,
    // @@protoc_insertion_point(field:SwapLineupCsReq.src_slot)
    pub src_slot: u32,
    // @@protoc_insertion_point(field:SwapLineupCsReq.is_virtual)
    pub is_virtual: bool,
    // @@protoc_insertion_point(field:SwapLineupCsReq.dst_slot)
    pub dst_slot: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SwapLineupCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SwapLineupCsReq {
    fn default() -> &'a SwapLineupCsReq {
        <SwapLineupCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SwapLineupCsReq {
    pub fn new() -> SwapLineupCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "extra_lineup_type",
            |m: &SwapLineupCsReq| { &m.extra_lineup_type },
            |m: &mut SwapLineupCsReq| { &mut m.extra_lineup_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "plane_id",
            |m: &SwapLineupCsReq| { &m.plane_id },
            |m: &mut SwapLineupCsReq| { &mut m.plane_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "index",
            |m: &SwapLineupCsReq| { &m.index },
            |m: &mut SwapLineupCsReq| { &mut m.index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "src_slot",
            |m: &SwapLineupCsReq| { &m.src_slot },
            |m: &mut SwapLineupCsReq| { &mut m.src_slot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_virtual",
            |m: &SwapLineupCsReq| { &m.is_virtual },
            |m: &mut SwapLineupCsReq| { &mut m.is_virtual },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dst_slot",
            |m: &SwapLineupCsReq| { &m.dst_slot },
            |m: &mut SwapLineupCsReq| { &mut m.dst_slot },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SwapLineupCsReq>(
            "SwapLineupCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SwapLineupCsReq {
    const NAME: &'static str = "SwapLineupCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.extra_lineup_type = is.read_enum_or_unknown()?;
                },
                64 => {
                    self.plane_id = is.read_uint32()?;
                },
                32 => {
                    self.index = is.read_uint32()?;
                },
                24 => {
                    self.src_slot = is.read_uint32()?;
                },
                80 => {
                    self.is_virtual = is.read_bool()?;
                },
                72 => {
                    self.dst_slot = is.read_uint32()?;
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
        if self.extra_lineup_type != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.extra_lineup_type.value());
        }
        if self.plane_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.plane_id);
        }
        if self.index != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.index);
        }
        if self.src_slot != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.src_slot);
        }
        if self.is_virtual != false {
            my_size += 1 + 1;
        }
        if self.dst_slot != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.dst_slot);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.extra_lineup_type != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.extra_lineup_type))?;
        }
        if self.plane_id != 0 {
            os.write_uint32(8, self.plane_id)?;
        }
        if self.index != 0 {
            os.write_uint32(4, self.index)?;
        }
        if self.src_slot != 0 {
            os.write_uint32(3, self.src_slot)?;
        }
        if self.is_virtual != false {
            os.write_bool(10, self.is_virtual)?;
        }
        if self.dst_slot != 0 {
            os.write_uint32(9, self.dst_slot)?;
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

    fn new() -> SwapLineupCsReq {
        SwapLineupCsReq::new()
    }

    fn clear(&mut self) {
        self.extra_lineup_type = ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE);
        self.plane_id = 0;
        self.index = 0;
        self.src_slot = 0;
        self.is_virtual = false;
        self.dst_slot = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SwapLineupCsReq {
        static instance: SwapLineupCsReq = SwapLineupCsReq {
            extra_lineup_type: ::protobuf::EnumOrUnknown::from_i32(0),
            plane_id: 0,
            index: 0,
            src_slot: 0,
            is_virtual: false,
            dst_slot: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SwapLineupCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SwapLineupCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SwapLineupCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SwapLineupCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15SwapLineupCsReq.proto\x1a\x15ExtraLineupType.proto\"\xd5\x01\n\x0f\
    SwapLineupCsReq\x12<\n\x11extra_lineup_type\x18\x0c\x20\x01(\x0e2\x10.Ex\
    traLineupTypeR\x0fextraLineupType\x12\x19\n\x08plane_id\x18\x08\x20\x01(\
    \rR\x07planeId\x12\x14\n\x05index\x18\x04\x20\x01(\rR\x05index\x12\x19\n\
    \x08src_slot\x18\x03\x20\x01(\rR\x07srcSlot\x12\x1d\n\nis_virtual\x18\n\
    \x20\x01(\x08R\tisVirtual\x12\x19\n\x08dst_slot\x18\t\x20\x01(\rR\x07dst\
    SlotB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::ExtraLineupType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SwapLineupCsReq::generated_message_descriptor_data());
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

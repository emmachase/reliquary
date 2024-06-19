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

//! Generated file from `RogueHandbookAeon.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueHandbookAeon)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueHandbookAeon {
    // message fields
    // @@protoc_insertion_point(field:RogueHandbookAeon.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:RogueHandbookAeon.level)
    pub level: u32,
    // @@protoc_insertion_point(field:RogueHandbookAeon.aeon_id)
    pub aeon_id: u32,
    // @@protoc_insertion_point(field:RogueHandbookAeon.taken_reward_list)
    pub taken_reward_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:RogueHandbookAeon.max_level)
    pub max_level: u32,
    // @@protoc_insertion_point(field:RogueHandbookAeon.archive_unlock_list)
    pub archive_unlock_list: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueHandbookAeon.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueHandbookAeon {
    fn default() -> &'a RogueHandbookAeon {
        <RogueHandbookAeon as ::protobuf::Message>::default_instance()
    }
}

impl RogueHandbookAeon {
    pub fn new() -> RogueHandbookAeon {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &RogueHandbookAeon| { &m.exp },
            |m: &mut RogueHandbookAeon| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &RogueHandbookAeon| { &m.level },
            |m: &mut RogueHandbookAeon| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "aeon_id",
            |m: &RogueHandbookAeon| { &m.aeon_id },
            |m: &mut RogueHandbookAeon| { &mut m.aeon_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "taken_reward_list",
            |m: &RogueHandbookAeon| { &m.taken_reward_list },
            |m: &mut RogueHandbookAeon| { &mut m.taken_reward_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_level",
            |m: &RogueHandbookAeon| { &m.max_level },
            |m: &mut RogueHandbookAeon| { &mut m.max_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "archive_unlock_list",
            |m: &RogueHandbookAeon| { &m.archive_unlock_list },
            |m: &mut RogueHandbookAeon| { &mut m.archive_unlock_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueHandbookAeon>(
            "RogueHandbookAeon",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueHandbookAeon {
    const NAME: &'static str = "RogueHandbookAeon";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.exp = is.read_uint32()?;
                },
                64 => {
                    self.level = is.read_uint32()?;
                },
                96 => {
                    self.aeon_id = is.read_uint32()?;
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.taken_reward_list)?;
                },
                120 => {
                    self.taken_reward_list.push(is.read_uint32()?);
                },
                40 => {
                    self.max_level = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.archive_unlock_list)?;
                },
                16 => {
                    self.archive_unlock_list.push(is.read_uint32()?);
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
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.exp);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.level);
        }
        if self.aeon_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.aeon_id);
        }
        for value in &self.taken_reward_list {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        if self.max_level != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.max_level);
        }
        for value in &self.archive_unlock_list {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.exp != 0 {
            os.write_uint32(13, self.exp)?;
        }
        if self.level != 0 {
            os.write_uint32(8, self.level)?;
        }
        if self.aeon_id != 0 {
            os.write_uint32(12, self.aeon_id)?;
        }
        for v in &self.taken_reward_list {
            os.write_uint32(15, *v)?;
        };
        if self.max_level != 0 {
            os.write_uint32(5, self.max_level)?;
        }
        for v in &self.archive_unlock_list {
            os.write_uint32(2, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RogueHandbookAeon {
        RogueHandbookAeon::new()
    }

    fn clear(&mut self) {
        self.exp = 0;
        self.level = 0;
        self.aeon_id = 0;
        self.taken_reward_list.clear();
        self.max_level = 0;
        self.archive_unlock_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueHandbookAeon {
        static instance: RogueHandbookAeon = RogueHandbookAeon {
            exp: 0,
            level: 0,
            aeon_id: 0,
            taken_reward_list: ::std::vec::Vec::new(),
            max_level: 0,
            archive_unlock_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueHandbookAeon {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueHandbookAeon").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueHandbookAeon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueHandbookAeon {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17RogueHandbookAeon.proto\"\xcd\x01\n\x11RogueHandbookAeon\x12\x10\n\
    \x03exp\x18\r\x20\x01(\rR\x03exp\x12\x14\n\x05level\x18\x08\x20\x01(\rR\
    \x05level\x12\x17\n\x07aeon_id\x18\x0c\x20\x01(\rR\x06aeonId\x12*\n\x11t\
    aken_reward_list\x18\x0f\x20\x03(\rR\x0ftakenRewardList\x12\x1b\n\tmax_l\
    evel\x18\x05\x20\x01(\rR\x08maxLevel\x12.\n\x13archive_unlock_list\x18\
    \x02\x20\x03(\rR\x11archiveUnlockListB\x15\n\x13emu.lunarcore.protob\x06\
    proto3\
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
            messages.push(RogueHandbookAeon::generated_message_descriptor_data());
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

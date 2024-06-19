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

//! Generated file from `StartPartialChallengeCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:StartPartialChallengeCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartPartialChallengeCsReq {
    // message fields
    // @@protoc_insertion_point(field:StartPartialChallengeCsReq.challenge_id)
    pub challenge_id: u32,
    // @@protoc_insertion_point(field:StartPartialChallengeCsReq.buff_id)
    pub buff_id: u32,
    // @@protoc_insertion_point(field:StartPartialChallengeCsReq.is_first_half)
    pub is_first_half: bool,
    // special fields
    // @@protoc_insertion_point(special_field:StartPartialChallengeCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartPartialChallengeCsReq {
    fn default() -> &'a StartPartialChallengeCsReq {
        <StartPartialChallengeCsReq as ::protobuf::Message>::default_instance()
    }
}

impl StartPartialChallengeCsReq {
    pub fn new() -> StartPartialChallengeCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_id",
            |m: &StartPartialChallengeCsReq| { &m.challenge_id },
            |m: &mut StartPartialChallengeCsReq| { &mut m.challenge_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "buff_id",
            |m: &StartPartialChallengeCsReq| { &m.buff_id },
            |m: &mut StartPartialChallengeCsReq| { &mut m.buff_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_first_half",
            |m: &StartPartialChallengeCsReq| { &m.is_first_half },
            |m: &mut StartPartialChallengeCsReq| { &mut m.is_first_half },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartPartialChallengeCsReq>(
            "StartPartialChallengeCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartPartialChallengeCsReq {
    const NAME: &'static str = "StartPartialChallengeCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.challenge_id = is.read_uint32()?;
                },
                120 => {
                    self.buff_id = is.read_uint32()?;
                },
                96 => {
                    self.is_first_half = is.read_bool()?;
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
        if self.challenge_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.challenge_id);
        }
        if self.buff_id != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.buff_id);
        }
        if self.is_first_half != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.challenge_id != 0 {
            os.write_uint32(6, self.challenge_id)?;
        }
        if self.buff_id != 0 {
            os.write_uint32(15, self.buff_id)?;
        }
        if self.is_first_half != false {
            os.write_bool(12, self.is_first_half)?;
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

    fn new() -> StartPartialChallengeCsReq {
        StartPartialChallengeCsReq::new()
    }

    fn clear(&mut self) {
        self.challenge_id = 0;
        self.buff_id = 0;
        self.is_first_half = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartPartialChallengeCsReq {
        static instance: StartPartialChallengeCsReq = StartPartialChallengeCsReq {
            challenge_id: 0,
            buff_id: 0,
            is_first_half: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartPartialChallengeCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartPartialChallengeCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartPartialChallengeCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartPartialChallengeCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20StartPartialChallengeCsReq.proto\"|\n\x1aStartPartialChallengeCsRe\
    q\x12!\n\x0cchallenge_id\x18\x06\x20\x01(\rR\x0bchallengeId\x12\x17\n\
    \x07buff_id\x18\x0f\x20\x01(\rR\x06buffId\x12\"\n\ris_first_half\x18\x0c\
    \x20\x01(\x08R\x0bisFirstHalfB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            messages.push(StartPartialChallengeCsReq::generated_message_descriptor_data());
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

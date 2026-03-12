#[cfg(test)]
use crate::{
    Create, Cursor, Entry, FileType, Metadata, Permit, RequestClose, RequestCopy, RequestIdentify,
    RequestList, RequestMake, RequestMetadata, RequestOpen, RequestRead, RequestRemove,
    RequestSeek, RequestSetmeta, RequestTell, RequestWrite, ResponseClose, ResponseCopy,
    ResponseError, ResponseIdentify, ResponseList, ResponseMake, ResponseMetadata, ResponseOpen,
    ResponseRead, ResponseRemove, ResponseSeek, ResponseSetmeta, ResponseTell, ResponseWrite,
    Timestamp, Write, RequestLink, ResponseLink, RequestDrop, ResponseDrop,
};
#[cfg(test)]
use core::marker::PhantomData;
#[cfg(test)]
macro_rules! test {
    ($test_name:ident, $type:ty, $constructor:expr) => {
        #[test]
        fn $test_name() {
            let original: $type = $constructor;
            let mut buf = [0u8; 1024];
            let serialised = original.to_slice(&mut buf).unwrap();
            let deserialised = <$type>::from_bytes(serialised).unwrap();
            assert_eq!(original, deserialised);
        }
    };
}
#[cfg(test)]
const TEST_PERMIT: Permit = Permit {
    read: true,
    write: false,
    append: true,
    resize: false,
    permit: true,
    read_accessed: false,
    read_modified: true,
    read_created: false,
    write_accessed: true,
    write_modified: false,
    write_created: true,
    link_before: false,
    link_after: true,
};
#[cfg(test)]
const TEST_METADATA: Metadata = Metadata {
    permit: TEST_PERMIT,
    is_file: None,
    accessed: Some(Timestamp {
        secs: 12345,
        nanos: 23456,
    }),
    modified: None,
    created: Some(Timestamp {
        secs: 34567,
        nanos: 45678,
    }),
};
#[cfg(test)]
const TEST_CONTENT: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
#[cfg(test)]
test!(response_error, ResponseError, ResponseError {});
#[cfg(test)]
test!(
    request_identify,
    RequestIdentify<&str>,
    RequestIdentify {
        path: "/foo/bar/baz.txt"
    }
);
#[cfg(test)]
test!(
    response_identify,
    ResponseIdentify,
    ResponseIdentify {
        file_type: FileType::File
    }
);
#[cfg(test)]
test!(
    request_open,
    RequestOpen<&str>,
    RequestOpen {
        expect: FileType::Dir,
        read: true,
        write: Some((
            Write::Append,
            Create::Any {
                permit: TEST_PERMIT,
                truncate: false
            }
        )),
        path: "/foo/bar/baz.txt"
    }
);
#[cfg(test)]
test!(
    response_open,
    ResponseOpen,
    ResponseOpen { descriptor: 56789 }
);
#[cfg(test)]
test!(
    request_close,
    RequestClose,
    RequestClose { descriptor: 67890 }
);
#[cfg(test)]
test!(response_close, ResponseClose, ResponseClose {});
#[cfg(test)]
test!(
    request_metadata,
    RequestMetadata,
    RequestMetadata { descriptor: 78901 }
);
#[cfg(test)]
test!(
    response_metadata,
    ResponseMetadata,
    ResponseMetadata {
        metadata: TEST_METADATA
    }
);
#[cfg(test)]
test!(
    request_setmeta,
    RequestSetmeta,
    RequestSetmeta {
        descriptor: 89012,
        metadata: TEST_METADATA
    }
);
#[cfg(test)]
test!(response_setmeta, ResponseSetmeta, ResponseSetmeta {});
#[cfg(test)]
test!(
    request_list,
    RequestList,
    RequestList {
        descriptor: 90123,
        length: 01234
    }
);
#[cfg(test)]
test!(response_list, ResponseList<&str, [Entry<&str>; 2]>, ResponseList { entries: [Entry { metadata: TEST_METADATA, name: "fizz.txt" }, Entry { metadata: TEST_METADATA, name: "buzz.txt" }], _phantom_n: PhantomData });
#[cfg(test)]
test!(
    request_make,
    RequestMake<&str>,
    RequestMake {
        descriptor: 13579,
        file_type: FileType::File,
        permit: TEST_PERMIT,
        name: "lorem.ipsum"
    }
);
#[cfg(test)]
test!(response_make, ResponseMake, ResponseMake {});
#[cfg(test)]
test!(
    request_remove,
    RequestRemove<&str>,
    RequestRemove {
        descriptor: 24680,
        name: "ipsum.lorem"
    }
);
#[cfg(test)]
test!(response_remove, ResponseRemove, ResponseRemove {});
#[cfg(test)]
test!(
    request_read,
    RequestRead,
    RequestRead {
        descriptor: 35791,
        length: 46802
    }
);
#[cfg(test)]
test!(
    response_read,
    ResponseRead<[u8; 10]>,
    ResponseRead {
        content: TEST_CONTENT
    }
);
#[cfg(test)]
test!(
    request_write,
    RequestWrite<[u8; 10]>,
    RequestWrite {
        descriptor: 57913,
        content: TEST_CONTENT
    }
);
#[cfg(test)]
test!(
    response_write,
    ResponseWrite,
    ResponseWrite { length: 68024 }
);
#[cfg(test)]
test!(
    request_seek,
    RequestSeek,
    RequestSeek {
        descriptor: 79135,
        cursor: Cursor::Backward,
        offset: 80246
    }
);
#[cfg(test)]
test!(response_seek, ResponseSeek, ResponseSeek {});
#[cfg(test)]
test!(request_tell, RequestTell, RequestTell { descriptor: 91357 });
#[cfg(test)]
test!(response_tell, ResponseTell, ResponseTell { offset: 02468 });
#[cfg(test)]
test!(
    request_copy,
    RequestCopy,
    RequestCopy {
        from: 14703,
        to: 25814,
        length: 36925
    }
);
#[cfg(test)]
test!(response_copy, ResponseCopy, ResponseCopy { length: 47036 });
#[cfg(test)]
test!(
    request_link,
    RequestLink<&str>,
    RequestLink {
        permit: TEST_PERMIT,
        above: false,
        from: "/test/from",
        to: "/test/to"
    }
);
#[cfg(test)]
test!(response_link, ResponseLink, ResponseLink {});
#[cfg(test)]
test!(request_drop, RequestDrop, RequestDrop {});
#[cfg(test)]
test!(response_drop, ResponseDrop, ResponseDrop {});

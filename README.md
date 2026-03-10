# Floreum Parser

Serialisation/deserialisation primitives for Floreum messages. Floreum is a protocol for accessing a
filesystem over a serialised connection. It was developed for the Floreum operating system, where it
is used as the mechanism of interprocess communication. The protocol consists of a number of
messages encoding possible filesystem operations, and is serialised to the postcard data format.

## Terminology

### Client

A program requesting to access resources managed by a `device`.

### Device

A filesystem exported by a program, a la Unix devices.

### Descriptor

A handle to an open file. A client requests to open a file with some specific permissions, e.g.
`read`ing and `append`ing, and the device responds with an arbitrary `descriptor` (u64) that the
client will use to reference the file when it `read`s or `append`s data, until the client requests
to `close` the file, and the descriptor is destroyed.

### Cursor

A descriptor's position within an open file. For example, a client may request to open
`"/user/essay.txt"` for `read`ing, receive descriptor 9 (chosen by the `device`), and request to
read 10 bytes. The device will then respond with the first 10 bytes of the file, and record that the
descriptor is now 10 bytes deep into the file. If the client then reads another 5 bytes, the device
will return the next 5 bytes (bytes 10..15), and record another 5 bytes against the descriptor's
cursor.

### Link

This is locked behind the `link` feature, and is not supported by most `device`s (designed exclusively
for the Floreum system). `Link`s allow referencing one location in a filesystem from a different
path, transparent to both the device and the client. These links may be placed `above` the
destination, overriding all of the device's files, the links `below` the device, and any previous
`above` links, or placed `below` the destination, not overriding any of the device's files, any
`above` links, or previous `below` links (this will however override future `below` links). Links
live as long as their destination device, and become "broken" if their source device is dropped, or
their source device no longer serves something at that path. A link has an associated `Permit`,
which will mask the permissions of the source device (e.g. a read-only link will not allow writes
regardless of what the source device allows), and can safely point outside of the namespace root.

### Namespace

A `namespace` is a view of a filesystem. The "real" namespace might include:
- a "/docs/" directory, a
- "/code/" directory, and
- a "/code/project/" directory.
A namespace rooted at the "/" here sees 3 directories, with one "project/" inside of "/code/",
however a namespace rooted at "/code/" sees only one directory, named "/project/".

### TOCTOU

Time-of-check-to-time-of-use. Do not, in a security-critical context, e.g. check that a file exists
and assume it won't be:
- instantly deleted;
- deleted and replaced with a directory;
- deleted and replaced with a completely different file;
- replaced with something critical that absolutely should not be shared;
- assume the device won't change something critical about what the path refers to;
- etc.
This vulnerability exists anywhere that involves checking something and separately doing something
assuming check still holds. Neither the client machine, the protocol, the device, or the device's
machine can be assumed to ensure that two consecutive operations are not separated by something else
between.
## Structures

### FileType

A file may be either a true `File` or a `Dir` (-ectory). Symbolic links will not be supported as
they substantially complicate cross-device multiplexing, and cannot elegantly handle references to
files above the namespace root. Hardlinks are not supported for similar reasons, but are supplanted
in the Floreum system by `Link`s.

### Cursor

Controls the seeking/jumping of file content and directory entry cursors, relative to the current
cursor position or the start/end of the file. Can be set to:
- `Forward` (move forward relative to the cursor's current position),
- `Backward` (move backwards, similar but opposite to `Forward`),
- `Start` (move to an absolute position relative to the start of the file), or
- `End` (move relative to the end of the file, similar but opposite to `Start`).

### Create

Controls the behaviour for when an operation to modify a file finds an existing one at the specified
path. Can be set to:
- `Existing` (open only if the file already exists),
- `New` (open only if the file doesn't already exist), or
- `Any` (open if it does exist, or create it if it doesn't).
For `Existing` and `Any`, the client may also choose whether or not to `truncate` (erase) the
existing content, or append to the end of it. If a new file must be created, it will use the
provided `Permit`.

### Write

Controls the behaviour for writes to a descriptor. Can be either of:
- `Append` (writes append content to the end of the file), or
- `Overwrite` (writes replace the next bytes after the cursor, extending the file if necessary).
Note that `Append`ing does not set the cursor to the end of the file, meaning `read`s will still
return bytes.

### Permit

A particular descriptor's permissions for a particular file. Setting these permissions via the
Permit message at least temporarily changes the file's permissions for this descriptor, but whether
this is permanent, visible to other descriptors, visible ten minutes from now, or even visible
immediately, depends on how the device interprets the request. A disk format device, which stores
its files permanently, might consider a "read-only" more permanent than a TCP socket device, which
will close and forget all of its state when the device's host machine shuts down.

### Timestamp

A Unix timestamp relative to midnight on the 1st of January, 1970, in UTC. Stores seconds and
nanoseconds, but is not supported by all devices (will sometimes show up as None in metadata).
Timestamps are for Educational Purposes Only, and cannot be trusted to:
- measure the correct time;
- measure the correct time for either the client or the device;
- measure an even remotely correct time;
- measure a time in the past;
- measure a time valid in any time zone;
- measure a time that can be represented using a Gregorian calendar;
- obey daylight savings;
- not obey daylight savings;
- progress at one second per second;
- progress at one second per second in this inertial reference frame;
- progress at a consistent rate;
- progress at all;
- progress faster than one second per century;
- progress slower than one century per second;
- not vascillate between these two extremes;
- not progress backwards;
- measure a time similar to the time measured by a simultaneous operation;
- measure a time before the heat death of the universe; or
- measure a time after the formation of the electromagnetic force.

### Metadata

The non-content, non-name data associated with a file. Stores whether it is a file (and therefore
has an associated length), a Permit, and may store 0..3 timestamps recording the times it was
**allegedly** created, last modified, and last accessed.

### Entry

An entry in a directory. Stores a name, and associated Metadata.

## Messages

There are multiple types of messages, classed into requests and responses, such that a device will
response to a RequestX with a ResponseX, or a ResponseError.

### Exists(path: String) -> bool

Find whether a file exists at `path`. Requires `read` permissions from all of the intermediate
directories.

### Open(expect: FileType, read: bool, write: Option<(Write, Create)>, path: String) -> descriptor

Open a file, expected to be of type `expect`, accessible from `path`. if `read`, the file will be
readable, and if `write` is Some, the `Write` will determine the behaviour of writing and the
`Create` will determine what to do with an unexpectedly existing/nonexistant file.

### Close(descriptor: u64) -> ()

Close `descriptor`. The actual number may or may not later be reassigned to a different descriptor.
Does not require any particular permissions from `descriptor`.

### Metadata(descriptor: u64) -> Metadata

Get the metadata for a file. Does not require any particular permissions from `descriptor`.

### Permit(descriptor: u64, permit: Permit) -> ()

Change the permissions for a file. Requires `permit` permissions from `descriptor`.

### List(descriptor: u64, length: u64) -> \[Entry\]

Read `length` directory entries from `descriptor`. Requires that `descriptor` holds read permissions,
and is opened to expect a directory. Moves `descriptor`'s cursor forward.

### Make(descriptor: u64, file_type: FileType, permit: Permit, name: String) -> ()

Add a child of type `file_type` to `descriptor` with permissions `permit`, named `name`. Requires
`append` permissions from `descriptor`, and that it is opened to expect a directory.

### Remove(descriptor: u64, name: N) -> 

## AI Policy

Use of generative AI is not restricted, but contributors will be held accountable for the quality of
code submitted.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

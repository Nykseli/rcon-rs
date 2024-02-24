// User generated types
#[derive(Debug)]
pub struct RAuth;
const SERVERDATA_AUTH: i32 = 3;
#[derive(Debug)]
pub struct RExec;
const SERVERDATA_EXECCOMMAND: i32 = 2;

// Server generated types
#[derive(Debug)]
pub struct RAuthRes;
const SERVERDATA_AUTH_RESPONSE: i32 = 2;
#[derive(Debug)]
pub struct RExecRes;
const SERVERDATA_RESPONSE_VALUE: i32 = 0;

/// RCON TCP packet. Response and Reqest both use it.
///
/// Basic structure:
/// |    Field     |                 Type                |        Value       |
/// |--------------|-------------------------------------|--------------------|
/// |     Size     | 32-bit little-endian Signed Integer | Varies, see below. |
/// |      ID      | 32-bit little-endian Signed Integer | Varies, see below. |
/// |     Type     | 32-bit little-endian Signed Integer | Varies, see below. |
/// |     Body     | Null-terminated ASCII String        | Varies, see below. |
/// | Empty String | Null-terminated ASCII String        | 0x00               |
///
/// Note that this structure doesn't contain the "Empty String" field as it's
/// constant value of '\0'.
#[derive(Debug)]
pub struct RconPacket<T> {
    /// The packet size field is a signed 32-bit little endian integer,
    /// representing the length of the request in bytes.
    /// Note that the packet size field itself is not included when determining
    /// the size of the packet, so the value of this field is always 4 less
    /// than the packet's actual length. The minimum possible value for packet size is 10:
    ///
    /// |      Size       |             Containing          |
    /// |-----------------|---------------------------------|
    /// |     4 Bytes     |             ID Field            |
    /// |     4 Bytes     |            Type Field           |
    /// | At least 1 Byte | Packet body (potentially empty) |
    /// |     1 Bytes     |     Empty string terminator     |
    ///
    /// Since the only one of these values that can change in length is the body,
    /// an easy way to calculate the size of a packet is to find the byte-length of the packet body, then add 10 to it.
    /// The maximum value of packet size is 4096.
    /// If the response is too large to fit into one packet, it will be split and sent as multiple packets.
    size: i32,

    /// The packet id field is a signed 32-bit little endian integer chosen by the client for each request.
    /// It may be set to any positive integer. When the server responds to the request,
    /// the response packet will have the same packet id as the original request
    /// (unless it is a failed SERVERDATA_AUTH_RESPONSE packet - see below.)
    /// It need not be unique, but if a unique packet id is assigned,
    /// it can be used to match incoming responses to their corresponding requests.
    id: i32,

    /// The packet type field is a signed 32-bit little endian integer, which indicates the purpose of the packet.
    /// Its value will always be either 0, 2, or 3, depending on which of the following request/response types the packet represents:
    /// | Value |     String Descriptor      |
    /// |-------| ---------------------------|
    /// |   3   |      SERVERDATA_AUTH       |
    /// |   2   |  SERVERDATA_AUTH_RESPONSE  |
    /// |   2   |   SERVERDATA_EXECCOMMAND   |
    /// |   0   | SERVERDATA_RESPONSE_VALUE  |
    ///
    /// Note that the repetition in the above table is not an error:
    /// SERVERDATA_AUTH_RESPONSE and SERVERDATA_EXECCOMMAND both have a numeric value of 2.
    packet_type: i32,

    /// The packet body field is a null-terminated string encoded in ASCII (i.e. ASCIIZ).
    /// Depending on the packet type, it may contain either the RCON password for the server,
    /// the command to be executed, or the server's response to a request.
    body: String,

    /// PhantomData for the type of RconPacket
    type_: std::marker::PhantomData<T>,
}

impl<T> RconPacket<T> {
    fn packet_size(body: &str) -> i32 {
        // TODO: handle sizes over 4096
        // + 10 is structure size
        body.len() as i32 + 10
    }

    fn from_data(_packet_type: i32, data: [u8; 4096]) -> Self {
        // TODO: error if not expected package type
        let size = i32::from_le_bytes(data[0..4].try_into().unwrap());
        let id = i32::from_le_bytes(data[4..8].try_into().unwrap());
        let type_ = i32::from_le_bytes(data[8..12].try_into().unwrap());
        let body_size = size as usize - 10;
        let body = String::from_utf8(data[12..body_size + 12].into()).unwrap();
        Self {
            body,
            size,
            packet_type: type_,
            id,
            type_: std::marker::PhantomData::<T>,
        }
    }

    pub fn packet_data(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend(self.size.to_le_bytes().iter());
        data.extend(self.id.to_le_bytes().iter());
        data.extend(self.packet_type.to_le_bytes().iter());
        data.extend(self.body.as_bytes().iter());
        // ASCIIZ null termination
        data.push(b'\0');
        // Empty String field
        data.push(b'\0');
        data
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}

impl RconPacket<RAuth> {
    pub fn new(id: i32, body: String) -> Self {
        let size = Self::packet_size(&body);
        Self {
            size,
            body,
            id,
            packet_type: SERVERDATA_AUTH,
            type_: std::marker::PhantomData::<RAuth>,
        }
    }
}

impl RconPacket<RExec> {
    pub fn new(id: i32, body: String) -> Self {
        let size = Self::packet_size(&body);
        Self {
            size,
            body,
            id,
            packet_type: SERVERDATA_EXECCOMMAND,
            type_: std::marker::PhantomData::<RExec>,
        }
    }
}

impl RconPacket<RAuthRes> {
    pub fn new(data: [u8; 4096]) -> Self {
        Self::from_data(SERVERDATA_AUTH_RESPONSE, data)
    }

    pub fn is_logged_in(&self) -> bool {
        self.id != -1
    }
}

impl RconPacket<RExecRes> {
    pub fn new(data: [u8; 4096]) -> Self {
        Self::from_data(SERVERDATA_RESPONSE_VALUE, data)
    }
}

/// Structure for creating Rcon Packets with unique IDs
pub struct RconPacketGen {
    current_id: i32,
}

impl RconPacketGen {
    pub fn new() -> Self {
        Self { current_id: 0 }
    }

    fn next_id(&mut self) -> i32 {
        self.current_id += 1;
        self.current_id
    }

    pub fn gen_auth(&mut self, body: String) -> RconPacket<RAuth> {
        RconPacket::<RAuth>::new(self.next_id(), body)
    }

    pub fn gen_exec(&mut self, body: String) -> RconPacket<RExec> {
        RconPacket::<RExec>::new(self.next_id(), body)
    }
}

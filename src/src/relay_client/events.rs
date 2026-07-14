use nodetunnel_protocol::packet::RoomInfo;
use nodetunnel_protocol::ClientId;
use crate::transport::common::Channel;

#[derive(Debug)]
pub enum RelayEvent {
    ConnectedToServer,
    Authenticated,
    RoomsReceived { rooms: Vec<RoomInfo> },
    RoomJoined { room_id: String, peer_id: i32 },
    PeerJoinAttempt { client_id: ClientId, metadata: String },
    PeerJoinedRoom { peer_id: i32 },
    GameDataReceived { channel: Channel, from_peer: i32, data: Vec<u8> },
    PeerLeftRoom { peer_id: i32 },
    ForceDisconnect,
    Error { error_code: i32, error_message: String },
}
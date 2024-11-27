use crate::{
    mqtt::{decode::decode, types::ControlPackets},
    ping::PingResPacket,
    pubsub::{
        PubackPacket, PubcompPacket, PublishPacket, PublishPacketGet, PubrecPacket, SubackPacket,
        UnsubackPacket,
    },
};

pub fn handle_packet(packet: Vec<u8>) {
    let get_packet = ControlPackets::try_from(packet[0]).unwrap();
    match get_packet {
        ControlPackets::Connect => {}
        ControlPackets::Connack => {}
        ControlPackets::Publish => decode(PublishPacketGet, packet),
        ControlPackets::Puback => decode(PubackPacket, packet),
        ControlPackets::Pubrec => decode(PubrecPacket, packet),
        ControlPackets::Pubrel => {}
        ControlPackets::Pubcomp => decode(PubcompPacket, packet),
        ControlPackets::Subscribe => {}
        ControlPackets::Suback => decode(SubackPacket, packet),
        ControlPackets::Unsubscribe => {}
        ControlPackets::Unsuback => decode(UnsubackPacket, packet),
        ControlPackets::Pingreq => {}
        ControlPackets::Pingresp => decode(PingResPacket, packet),
        ControlPackets::Disconnect => {}
        ControlPackets::Auth => {}
    }
}

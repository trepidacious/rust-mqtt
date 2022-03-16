/*
 * MIT License
 *
 * Copyright (c) [2022] [Ondrej Babec <ond.babec@gmail.com>]
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use crate::packet::v5::mqtt_packet::Packet;
use crate::packet::v5::packet_type::PacketType;
use crate::packet::v5::property::Property;
use crate::packet::v5::pubcomp_packet::PubcompPacket;
use crate::utils::buffer_reader::BuffReader;
use crate::utils::types::EncodedString;

use heapless::Vec;

#[test]
fn test_encode() {
    let mut buffer: [u8; 14] = [0; 14];
    let mut packet = PubcompPacket::<1>::new();
    packet.fixed_header = PacketType::Pubcomp.into();
    packet.packet_identifier = 35420;
    packet.reason_code = 0x00;
    let mut str = EncodedString::new();
    str.string = "Wheel";
    str.len = 5;
    let mut props = Vec::<Property, 1>::new();
    props.push(Property::ReasonString(str));
    packet.property_len = packet.add_properties(&props);
    let res = packet.encode(&mut buffer, 14);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 14);
    assert_eq!(
        buffer,
        [0x70, 0x0C, 0x8A, 0x5C, 0x00, 0x08, 0x1F, 0x00, 0x05, 0x57, 0x68, 0x65, 0x65, 0x6c]
    )
}

#[test]
fn test_decode() {
    let buffer: [u8; 14] = [
        0x70, 0x0C, 0x8A, 0x5C, 0x00, 0x08, 0x1F, 0x00, 0x05, 0x57, 0x68, 0x65, 0x65, 0x6c,
    ];
    let mut packet = PubcompPacket::<1>::new();
    let res = packet.decode(&mut BuffReader::new(&buffer, 14));
    assert!(res.is_ok());
    assert_eq!(packet.fixed_header, PacketType::Pubcomp.into());
    assert_eq!(packet.packet_identifier, 35420);
    assert_eq!(packet.reason_code, 0x00);
    assert_eq!(packet.property_len, 8);
    let prop = packet.properties.get(0);
    assert!(prop.is_some());
    assert_eq!(<&Property as Into<u8>>::into(prop.unwrap()), 0x1F);
    if let Property::ReasonString(u) = (*prop.unwrap()).clone() {
        assert_eq!(u.string, "Wheel");
    }
}

const { expect } = require('chai');
const {
    RpcCodec,
    RpcMsgType,
    PeerType
} = require('../../lib');

it('should encode handshake frame', () => {
    const codec = new RpcCodec();
    const buf = codec.encode({
        id: 100,
        msg_type: RpcMsgType.HANDSHAKE,
        req: {
            peer_type: PeerType.WALLET
        }
    });
    expect(buf).to.eql(Buffer.from([
        0x00, 0x00, 0x00, 0x0a, // Message length
        0x00, 0x00, 0x00, 0x64, // Payload ID
        0x02, // Message type
        0x01, // Peer type
    ]));
});

it('should decode handshake frame', () => {
    const codec = new RpcCodec();
    codec.update(Buffer.from([
        0x00, 0x00, 0x00, 0x0A, // Message length
        0x00, 0x00, 0x00, 0x64, // Payload ID
        0x02, // Message type
        0x00, // Peer type
    ]));
    const data = codec.decode();
    expect(data).to.eql({
        id: 100,
        msg_type: RpcMsgType.HANDSHAKE,
        req: {
            peer_type: PeerType.NODE
        }
    });
});

it('should throw on invalid peer type', () => {
    const codec = new RpcCodec();
    codec.update(Buffer.from([
        0x00, 0x00, 0x00, 0x0A, // Message length
        0x00, 0x00, 0x00, 0x64, // Payload ID
        0x02, // Message type
        0xFF, // Peer type
    ]));
    expect(() => {
        codec.decode();
    }).to.throw(Error, 'invalid peer type');
});

const { expect } = require('chai');
const {
    RpcCodec,
    RpcMsgType,
    PeerType
} = require('../lib');

it('should encode handshake frame', () => {
    const codec = new RpcCodec();
    const buf = codec.encode({
        id: 100,
        msg_type: RpcMsgType.HANDSHAKE,
        data: {
            peer_type: PeerType.WALLET
        }
    });
    expect(buf).to.eql(Buffer.from([
        0x00, 0x00, 0x00, 0x0a, // Message length
        0x00, 0x00, 0x00, 0x64, // Payload ID
        0x00, // Message type
        0x01, // Peer type
    ]));
});

it('should decode handshake frame', () => {
    const codec = new RpcCodec();
    const data = codec.decode(Buffer.from([
        0x00, 0x00, 0x00, 0x0A, // Message length
        0x00, 0x00, 0x00, 0x64, // Payload ID
        0x00, // Message type
        0x00, // Peer type
    ]));
    expect(data).to.eql({
        id: 100,
        msg_type: RpcMsgType.HANDSHAKE,
        data: {
            peer_type: PeerType.NODE
        }
    });
});

it('should throw on invalid peer type', () => {
    const codec = new RpcCodec();
    expect(() => {
        codec.decode(Buffer.from([
            0x00, 0x00, 0x00, 0x0A, // Message length
            0x00, 0x00, 0x00, 0x64, // Payload ID
            0x00, // Message type
            0xFF, // Peer type
        ]));
    }).to.throw(Error, 'invalid peer type');
});

it('should decode multipart frame', () => {
    const codec = new RpcCodec();

    let data = codec.decode(Buffer.from([0x00, 0x00, 0x00]));
    expect(data).to.be.undefined;

    data = codec.decode(Buffer.from([0x0A, 0x00, 0x00]));
    expect(data).to.be.undefined;

    data = codec.decode(Buffer.from([0x00, 0x64]));
    expect(data).to.be.undefined;

    data = codec.decode(Buffer.from([0x00, 0x01]));
    expect(data).to.eql({
        id: 100,
        msg_type: RpcMsgType.HANDSHAKE,
        data: {
            peer_type: PeerType.WALLET
        }
    });
});

it('should throw on invalid frame size decoding', () => {
    const codec = new RpcCodec();
    expect(() => {
        codec.decode(Buffer.from([0xFF, 0xFF, 0xFF, 0xFF]));
    }).to.throw(Error, 'payload must be <=5242880 bytes');
});

it('should throw on invalid frame decoding', () => {
    const codec = new RpcCodec();
    expect(() => {
        codec.decode(Buffer.from([
            0x00, 0x00, 0x00, 0x09,
            0x00, 0x00, 0x00, 0x00,
            0xFF,
        ]));
    }).to.throw(Error, 'invalid msg type');
});

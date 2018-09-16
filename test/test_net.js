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
    codec.update(Buffer.from([
        0x00, 0x00, 0x00, 0x0A, // Message length
        0x00, 0x00, 0x00, 0x64, // Payload ID
        0x00, // Message type
        0x00, // Peer type
    ]));
    const data = codec.decode();
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
    codec.update(Buffer.from([
        0x00, 0x00, 0x00, 0x0A, // Message length
        0x00, 0x00, 0x00, 0x64, // Payload ID
        0x00, // Message type
        0xFF, // Peer type
    ]));
    expect(() => {
        codec.decode();
    }).to.throw(Error, 'invalid peer type');
});

it('should decode multipart frame', () => {
    const codec = new RpcCodec();

    let data = codec.decode();
    expect(data).to.be.undefined;

    codec.update(Buffer.from([0x00, 0x00, 0x00]));
    data = codec.decode();
    expect(data).to.be.undefined;

    codec.update(Buffer.from([0x0A, 0x00, 0x00]));
    data = codec.decode();
    expect(data).to.be.undefined;

    codec.update(Buffer.from([0x00, 0x64]));
    data = codec.decode();
    expect(data).to.be.undefined;

    codec.update(Buffer.from([0x00, 0x01]));
    data = codec.decode();
    expect(data).to.eql({
        id: 100,
        msg_type: RpcMsgType.HANDSHAKE,
        data: {
            peer_type: PeerType.WALLET
        }
    });
});

it('should decode multiple multipart frames', () => {
    const codec = new RpcCodec();

    codec.update(Buffer.from([
        0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x64, 0x00, 0x00, // Frame 1
        0x00, 0x00, 0x00, 0x0A, 0x00, 0x00 // Part of frame 2
    ]));
    let data = codec.decode();
    expect(data).to.eql({
        id: 100,
        msg_type: RpcMsgType.HANDSHAKE,
        data: {
            peer_type: PeerType.NODE
        }
    });

    codec.update(Buffer.from([
        0x00, 0x01, 0x00, 0x01 // End of frame 2
    ]));
    data = codec.decode();
    expect(data).to.eql({
        id: 1,
        msg_type: RpcMsgType.HANDSHAKE,
        data: {
            peer_type: PeerType.WALLET
        }
    });
});

it('should decode multiple frames in a single chunk', () => {
    const codec = new RpcCodec();

    codec.update(Buffer.from([
        0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, // Frame 1
        0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, // Frame 2
    ]));

    expect(codec.decode()).to.eql({
        id: 1,
        msg_type: RpcMsgType.HANDSHAKE,
        data: {
            peer_type: PeerType.NODE
        }
    });

    expect(codec.decode()).to.eql({
        id: 2,
        msg_type: RpcMsgType.HANDSHAKE,
        data: {
            peer_type: PeerType.WALLET
        }
    });

    expect(codec.decode()).to.be.undefined;
});

it('should throw on invalid frame size decoding', () => {
    const codec = new RpcCodec();
    codec.update(Buffer.from([0xFF, 0xFF, 0xFF, 0xFF]));
    expect(() => {
        codec.decode();
    }).to.throw(Error, 'payload must be <=5242880 bytes');
});

it('should throw on invalid frame decoding', () => {
    const codec = new RpcCodec();
    codec.update(Buffer.from([
        0x00, 0x00, 0x00, 0x09,
        0x00, 0x00, 0x00, 0x00,
        0xFF
    ]));
    expect(() => {
        codec.decode();
    }).to.throw(Error, 'invalid msg type');
});

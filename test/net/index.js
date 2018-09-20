const { expect } = require('chai');
const {
    RpcCodec,
    RpcMsgType,
    PeerType
} = require('../../lib');

describe('Handshake', () => {
    require('./test_handshake');
});

describe('Event', () => {
    require('./test_event');
});

describe('Broadcast', () => {
    require('./test_broadcast');
});

describe('Block', () => {
    require('./test_block');
});

describe('Properties', () => {
    require('./test_properties');
})

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

    codec.update(Buffer.from([0x02, 0x01]));
    data = codec.decode();
    expect(data).to.eql({
        id: 100,
        msg_type: RpcMsgType.HANDSHAKE,
        req: {
            peer_type: PeerType.WALLET
        }
    });
});

it('should decode multiple multipart frames', () => {
    const codec = new RpcCodec();

    codec.update(Buffer.from([
        0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x64, 0x02, 0x00, // Frame 1
        0x00, 0x00, 0x00, 0x0A, 0x00, 0x00 // Part of frame 2
    ]));
    let data = codec.decode();
    expect(data).to.eql({
        id: 100,
        msg_type: RpcMsgType.HANDSHAKE,
        req: {
            peer_type: PeerType.NODE
        }
    });

    codec.update(Buffer.from([
        0x00, 0x01, 0x02, 0x01 // End of frame 2
    ]));
    data = codec.decode();
    expect(data).to.eql({
        id: 1,
        msg_type: RpcMsgType.HANDSHAKE,
        req: {
            peer_type: PeerType.WALLET
        }
    });
});

it('should decode multiple frames in a single chunk', () => {
    const codec = new RpcCodec();

    codec.update(Buffer.from([
        0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, // Frame 1
        0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x02, 0x02, 0x01, // Frame 2
    ]));

    expect(codec.decode()).to.eql({
        id: 1,
        msg_type: RpcMsgType.HANDSHAKE,
        req: {
            peer_type: PeerType.NODE
        }
    });

    expect(codec.decode()).to.eql({
        id: 2,
        msg_type: RpcMsgType.HANDSHAKE,
        req: {
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

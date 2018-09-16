const addon = require('./native');

const RpcMsgType = {
    NONE: 0,
    HANDSHAKE: 1,
    PROPERTIES: 2,
    0: 'NONE',
    1: 'HANDSHAKE',
    2: 'PROPERTIES'
};

const PeerType = {
    NODE: 0,
    WALLET: 1,
    0: 'NODE',
    1: 'WALLET'
};

class RpcCodec {
    constructor() {
        this.decoder = new addon.Net_RpcDecoder();
    }

    encode(obj) {
        return addon.Net_rpc_encoder(obj);
    }

    update(buf) {
        return this.decoder.update(buf);
    }

    decode() {
        return this.decoder.decode();
    }
}

module.exports = {
    RpcCodec,
    RpcMsgType,
    PeerType
};

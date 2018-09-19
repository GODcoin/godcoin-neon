const addon = require('./native');
const { SignedBlock } = require('./block');
const { Tx } = require('./tx');

const RpcMsgType = {
    NONE: -1,
    ERROR: 0,
    EVENT: 1,
    HANDSHAKE: 2,
    BROADCAST: 3,
    PROPERTIES: 4,
    BLOCK: 5,
    BALANCE: 6,
    TOTAL_FEE: 7,
    '-1': 'NONE',
    0: 'HANDSHAKE',
    1: 'PROPERTIES',
    2: 'EVENT',
    3: 'BROADCAST',
    4: 'PROPERTIES',
    5: 'BLOCK',
    6: 'BALANCE',
    7: 'TOTAL_FEE'
};

const RpcEventType = {
    TX: 'tx',
    BLOCK: 'block',
    block: 'BLOCK',
    tx: 'TX'
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
        const frame = this.decoder.decode();
        if (frame
                && frame.msg_type === RpcMsgType.EVENT
                && frame.req.subscribe !== true) {
            const eventData = frame.req;
            switch (eventData.type) {
                case RpcEventType.TX:
                    eventData.data = Tx.objToTx(eventData.data);
                    break;
                case RpcEventType.BLOCK:
                    eventData.data = SignedBlock.objToBlock(eventData.data);
                    break;
                default:
                    throw new Error("invalid event type: " + eventData.type);
            }
        }
        return frame;
    }
}

module.exports = {
    RpcCodec,
    RpcMsgType,
    RpcEventType,
    PeerType
};

const { expect } = require('chai');
const {
    RpcCodec,
    RpcMsgType,
    PrivateKey,
    Asset
} = require('../../lib');

it('should encode and decode total fee request frames', () => {
    const key = PrivateKey.genKeyPair()[0];
    const data = {
        id: 0,
        msg_type: RpcMsgType.TOTAL_FEE,
        req: key
    };
    const codec = new RpcCodec();
    codec.update(codec.encode(data));
    expect(codec.decode()).to.eql(data);
});

it('should encode and decode total fee response frames', () => {
    const data = {
        id: 1,
        msg_type: RpcMsgType.TOTAL_FEE,
        res: [
            Asset.fromString('0.00001 GOLD'),
            Asset.fromString('0.001 SILVER')
        ]
    };
    const codec = new RpcCodec();
    codec.update(codec.encode(data));
    expect(codec.decode()).to.eql(data);
});

const { expect } = require('chai');
const {
    RpcCodec,
    RpcMsgType,
    Asset
} = require('../../lib');

it('should encode and decode property request frames', () => {
    const data = {
        id: 0,
        msg_type: RpcMsgType.PROPERTIES,
        req: {}
    };
    const codec = new RpcCodec();
    codec.update(codec.encode(data));
    expect(codec.decode()).to.eql(data);
});

it('should encode and decode property response frames', () => {
    const data = {
        id: 100,
        msg_type: RpcMsgType.PROPERTIES,
        res: {
            height: 1000,
            token_supply: [
                Asset.fromString('100.0000 GOLD'),
                Asset.fromString('1000.000 SILVER')
            ]
        }
    };
    const codec = new RpcCodec();
    codec.update(codec.encode(data));
    expect(codec.decode()).to.eql(data);
});

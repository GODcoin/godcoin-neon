const { expect } = require('chai');
const { RpcCodec, RpcMsgType } = require('../../lib');

it('should encode and decode error frames', () => {
    const data = {
        id: 5,
        msg_type: RpcMsgType.ERROR,
        res: {
            error: 'an error has occurred'
        }
    };
    const codec = new RpcCodec();
    codec.update(codec.encode(data));
    expect(codec.decode()).to.eql(data);
});

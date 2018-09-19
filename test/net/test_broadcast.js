const { expect } = require('chai');
const {
    RpcCodec,
    RpcMsgType,
    PrivateKey,
    BondTx,
    Asset
} = require('../../lib');

it('should encode and decode broadcast frames', () => {
    const data = {
        id: 100,
        msg_type: RpcMsgType.BROADCAST,
        req: new BondTx({
            timestamp: new Date(),
            fee: Asset.EMPTY_GOLD,
            minter: PrivateKey.genKeyPair()[0],
            staker: PrivateKey.genKeyPair()[0],
            bond_fee: Asset.EMPTY_GOLD,
            stake_amt: Asset.EMPTY_GOLD,
            signature_pairs: []
        })
    };
    const codec = new RpcCodec();
    codec.update(codec.encode(data));
    expect(codec.decode()).to.eql(data);
});

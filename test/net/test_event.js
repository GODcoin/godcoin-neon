const { expect } = require('chai');
const {
    RpcCodec,
    RpcMsgType,
    RpcEventType,
    PrivateKey,
    Block,
    RewardTx,
    BondTx,
    Asset
} = require('../../lib');

it('should encode and decode tx events', () => {
    let data = {
        id: 15,
        msg_type: RpcMsgType.EVENT,
        data: {
            type: RpcEventType.TX,
            data: new BondTx({
                timestamp: new Date(),
                fee: Asset.EMPTY_GOLD,
                minter: PrivateKey.genKeyPair()[0],
                staker: PrivateKey.genKeyPair()[0],
                bond_fee: Asset.EMPTY_GOLD,
                stake_amt: Asset.EMPTY_GOLD,
                signature_pairs: []
            })
        }
    };

    const codec = new RpcCodec();
    codec.update(codec.encode(data));
    expect(data).to.eql(codec.decode());
});

it('should encode and decode block events', () => {
    const keys = PrivateKey.genKeyPair();
    const genesisTs = new Date();
    const genesisBlock = new Block({
      height: 1,
      timestamp: genesisTs,
      previous_hash: Buffer.alloc(32),
      transactions: [
        new RewardTx({
          timestamp: genesisTs,
          fee: Asset.fromString('0 GOLD'),
          to: keys[0],
          rewards: [
            Asset.fromString('1 GOLD'),
            Asset.fromString('1 SILVER')
          ],
          signature_pairs: []
        })
      ]
    }).sign(keys);

    const data = {
        id: 15,
        msg_type: RpcMsgType.EVENT,
        data: {
            type: RpcEventType.BLOCK,
            data: genesisBlock
        }
    };

    const codec = new RpcCodec();
    codec.update(codec.encode(data));
    expect(data).to.eql(codec.decode());
});

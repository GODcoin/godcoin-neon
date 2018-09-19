const { expect } = require('chai');
const {
    RpcCodec,
    RpcMsgType,
    PrivateKey,
    RewardTx,
    Block,
    Asset
} = require('../../lib');

it('should encode and decode block request frames', () => {
    const data = {
        id: 0,
        msg_type: RpcMsgType.BLOCK,
        req: {
            height: 10
        }
    };
    const codec = new RpcCodec();
    codec.update(codec.encode(data));
    expect(codec.decode()).to.eql(data);
});

it('should encode and decode block response frames', () => {
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
        id: 100,
        msg_type: RpcMsgType.BLOCK,
        res: genesisBlock
    };
    const codec = new RpcCodec();
    codec.update(codec.encode(data));
    expect(codec.decode()).to.eql(data);
});

it('should encode and decode empty block response frames', () => {
    const data = {
        id: 100,
        msg_type: RpcMsgType.BLOCK,
        res: undefined
    };
    const codec = new RpcCodec();
    codec.update(codec.encode(data));
    expect(codec.decode()).to.eql(data);
});

const { expect } = require('chai');
const {
    PrivateKey,
    Block,
    SignedBlock,
    RewardTx,
    Asset
} = require('../lib');

it('should serialize blocks', () => {
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

    const encoded = genesisBlock.encodeWithTx();
    const recBlock = SignedBlock.decodeWithTx(encoded);
    expect(recBlock).to.eql(genesisBlock);
    expect(recBlock).to.be.an.instanceOf(SignedBlock);
    expect(recBlock.transactions[0]).to.be.an.instanceof(RewardTx);
});

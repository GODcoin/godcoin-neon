const { expect } = require('chai');
const {
  PrivateKey,
  Asset,
  TxType,
  Tx,
  RewardTx,
  BondTx,
  TransferTx
} = require('../lib');

it('should encode bond transactions', () => {
  const tx = new BondTx({
    timestamp: new Date(),
    fee: Asset.EMPTY_GOLD,
    minter: PrivateKey.genKeyPair()[0],
    staker: PrivateKey.genKeyPair()[0],
    bond_fee: Asset.EMPTY_GOLD,
    stake_amt: Asset.EMPTY_GOLD,
    signature_pairs: []
  });

  const buf = tx.encodeWithSigs();
  const recTx = Tx.decodeWithSigs(buf);
  expect(recTx).to.eql(tx);
});

it('should encode reward transactions', () => {
  const tx = new RewardTx({
    timestamp: new Date(),
    fee: Asset.EMPTY_GOLD,
    to: PrivateKey.genKeyPair()[0],
    rewards: [Asset.fromString('10 GOLD'), Asset.fromString('100 SILVER')],
    signature_pairs: []
  });

  const buf = tx.encodeWithSigs();
  const recTx = Tx.decodeWithSigs(buf);
  expect(recTx).to.eql(tx);
});

it('should encode transfer transactions', () => {
  const from = PrivateKey.genKeyPair();
  const tx = new TransferTx({
    timestamp: new Date(),
    fee: Asset.fromString('0.00000001 GOLD'),
    signature_pairs: [],
    from: from[0],
    to: PrivateKey.genKeyPair()[0],
    amount: Asset.fromString('10 GOLD'),
    memo: Buffer.from('test 123')
  }).appendSign(from);

  const buf = tx.encodeWithSigs();
  const recTx = Tx.decodeWithSigs(buf);
  expect(recTx).to.eql(tx);
  expect(recTx.toString()).to.eql(tx.toString());
});

it('should encode transfer transactions with empty memo', () => {
  const from = PrivateKey.genKeyPair();
  const tx = new TransferTx({
    timestamp: new Date(),
    fee: Asset.fromString('0.00000001 GOLD'),
    signature_pairs: [],
    from: from[0],
    to: PrivateKey.genKeyPair()[0],
    amount: Asset.fromString('10 GOLD'),
    memo: undefined
  }).appendSign(from);

  const buf = tx.encodeWithSigs();
  const recTx = Tx.decodeWithSigs(buf);
  expect(recTx).to.eql(tx);
  expect(recTx.toString()).to.eql(tx.toString());
});

it('should fail on invalid transactions', () => {
  expect(TxType[255]).to.not.exist;
  const tx = new RewardTx({
    tx_type: 255,
    timestamp: new Date(),
    fee: Asset.fromString('0 GOLD'),
    to: PrivateKey.genKeyPair()[0],
    rewards: [],
    signature_pairs: []
  });

  expect(() => {
    tx.encodeWithSigs();
  }).to.throw(Error, 'invalid tx_type');
});

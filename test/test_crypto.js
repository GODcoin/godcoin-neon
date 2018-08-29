const bs58 = require('bs58');
const { expect } = require('chai');
const {
  PrivateKey,
  PublicKey
} = require('../lib');

it('should create keys', () => {
  const keys = PrivateKey.genKeyPair();
  expect(keys[0] instanceof PublicKey).to.be.true;
  expect(keys[1] instanceof PrivateKey).to.be.true;
});

it('should import keys', () => {
  const keys = PrivateKey.genKeyPair();
  const publicWif = keys[0].toWif();
  const privateWif = keys[1].toWif();

  const pubKey = PublicKey.fromWif(publicWif);
  const recKeys = PrivateKey.fromWif(privateWif);

  expect(pubKey.toWif()).is.eq(publicWif);
  expect(recKeys[0].toWif()).is.eq(publicWif);
  expect(recKeys[1].toWif()).is.eq(privateWif);

  const pair = PrivateKey.fromWif('3GAD3otqozDorfu1iDpMQJ1gzWp8PRFEjVHZivZdedKW3i3KtM');
  expect(pair[0].toWif()).to.eq('GOD52QZDBUStV5CudxvKf6bPsQeN7oeKTkEm2nAU1vAUqNVexGTb8');
  expect(pair[1].toWif()).to.eq('3GAD3otqozDorfu1iDpMQJ1gzWp8PRFEjVHZivZdedKW3i3KtM');
});

it('should compare keys', () => {
  const a = PrivateKey.genKeyPair();
  const b = PrivateKey.genKeyPair();
  expect(a[0].equals(a[0])).to.be.true;
  expect(a[0].equals(b[0])).to.be.false;
});

it('should throw on invalid key', () => {
  expect(() => {
    PrivateKey.fromWif('');
  }).to.throw(Error, 'invalid length');

  const keys = PrivateKey.genKeyPair();
  expect(() => {
    const buf = bs58.decode(keys[1].toWif());
    buf[0] = 0;
    PrivateKey.fromWif(bs58.encode(buf));
  }).to.throw(Error, 'invalid prefix');

  expect(() => {
    // Private key and public key has a different prefix
    const buf = bs58.decode(keys[1].toWif());
    PublicKey.fromWif('GOD' + bs58.encode(buf));
  }).to.throw(Error, 'invalid prefix');

  expect(() => {
    const wif = keys[0].toWif().slice(PublicKey.ADDR_PREFIX.length);
    PublicKey.fromWif(wif);
  }).to.throw(Error, 'invalid prefix');

  expect(() => {
    const buf = bs58.decode(keys[1].toWif());
    for (let i = 0; i < 4; ++i) buf[buf.length - i - 1] = 0;
    PrivateKey.fromWif(bs58.encode(buf));
  }).to.throw(Error, 'invalid checksum');
});

it('should properly sign and validate', () => {
  const keys = PrivateKey.genKeyPair();
  const msg = Buffer.from('Hello world!');
  const sig = keys[1].sign(msg);
  expect(keys[0].verify(sig, msg)).is.true;

  const badKeys = PrivateKey.genKeyPair();
  expect(badKeys[0].verify(sig, msg)).is.false;
});

it('should throw on invalid key lengths', () => {
  expect(() => {
    new PrivateKey(new ArrayBuffer(32), new ArrayBuffer(32));
  }).to.throw(Error, 'invalid secret len');

  expect(() => {
    new PrivateKey(new ArrayBuffer(64), new ArrayBuffer(16));
  }).to.throw(Error, 'invalid seed len');

  expect(() => {
    new PublicKey(new ArrayBuffer(64));
  }).to.throw(Error, 'invalid buffer len');
});

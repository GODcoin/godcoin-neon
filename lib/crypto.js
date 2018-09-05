const addon = require('./native');
const crypto = require('crypto');

const PublicKey = addon.PublicKey;
{
    PublicKey.ADDR_PREFIX = 'GOD';
    PublicKey.fromWif = addon.PublicKey_from_wif;

    PublicKey.prototype.toWif = addon.PublicKey.prototype.to_wif;
}

const PrivateKey = addon.PrivateKey;
{
    PrivateKey.fromWif = addon.PrivateKey_from_wif;
    PrivateKey.genKeyPair = addon.PrivateKey_gen_key_pair;

    PrivateKey.prototype.toWif = addon.PrivateKey.prototype.to_wif;
}

function doubleSha256(val) {
  return sha256(sha256(val));
}

function sha256(val) {
  return crypto.createHash('sha256').update(val).digest();
}

module.exports = {
    PublicKey,
    PrivateKey,
    doubleSha256
};

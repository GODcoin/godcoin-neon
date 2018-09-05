const addon = require('./native');

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

module.exports = {
    PublicKey,
    PrivateKey
};

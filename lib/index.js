const addon = require('../native');
addon.init();

const AssetSymbol = {
    GOLD: 0,
    SILVER: 1,
    0: 'GOLD',
    1: 'SILVER'
};

const Asset = addon.Asset;
{
    Asset.EMPTY_GOLD = new Asset(0, 0, AssetSymbol.GOLD);
    Asset.EMPTY_SILVER = new Asset(0, 0, AssetSymbol.SILVER);
    Asset.MAX_STR_LEN = 32;
    Asset.MAX_PRECISION = 8;

    Asset.fromString = addon.Asset_from_string;

    Asset.prototype.toString = addon.Asset.prototype.to_string;
}

module.exports = {
    Asset,
    AssetSymbol
};

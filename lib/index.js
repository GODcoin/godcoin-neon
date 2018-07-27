const addon = require('../native');

const AssetSymbol = {
    GOLD: 0,
    SILVER: 1,
    0: 'GOLD',
    1: 'SILVER'
};

const Asset = addon.Asset;
Asset.fromString = addon.Asset_from_string;
Asset.prototype.toString = addon.Asset.prototype.to_string;

module.exports = {
    Asset,
    AssetSymbol
};

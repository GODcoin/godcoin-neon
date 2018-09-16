const addon = require('./native');
addon.init();

module.exports = {
    ...require('./tx'),
    ...require('./asset'),
    ...require('./crypto'),
    ...require('./block'),
    ...require('./net')
};

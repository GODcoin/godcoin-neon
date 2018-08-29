const addon = require('../native');
addon.init();

module.exports = {
    ...require('./asset'),
    ...require('./crypto')
};

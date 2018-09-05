const addon = require('./native');
const { Asset } = require('./asset');
const { PublicKey } = require('./crypto');

const TxType = {
    REWARD: 0,
    TRANSFER: 1,
    BOND: 2,
    0: 'REWARD',
    1: 'TRANSFER',
    2: 'BOND'
};

class Tx {
    static decodeWithSigs(buffer) {
        let tx = addon.Tx_decode_with_sigs(buffer);
        return Tx.objToTx(tx);
    }

    static objToTx(obj) {
        switch (obj.tx_type) {
            case TxType.REWARD:
                return new RewardTx(obj);
            case TxType.BOND:
                return new BondTx(obj);
            case TxType.TRANSFER:
                return new TransferTx(obj);
        }
        return null;
    }

    constructor(type, data) {
        this.tx_type = type;
        Object.assign(this, data);
        if (typeof(this.timestamp) === 'number') {
            this.timestamp = new Date(this.timestamp * 1000);
        } else {
            const truncated = Math.floor(this.timestamp.getTime() / 1000) * 1000;
            this.timestamp = new Date(truncated);
        }
    }

    appendSign(keyPair) {
        const buf = this.encode();
        const sig = keyPair[1].sign(buf);
        this.signature_pairs.push([keyPair[0], sig]);
        return this;
    }

    encode() {
        return addon.Tx_encode(this);
    }

    encodeWithSigs() {
        return addon.Tx_encode_with_sigs(this);
    }

    toString() {
        const data = {};
        Object.getOwnPropertyNames(this).forEach(name => {
          if (name === 'tx_type') {
            data[name] = TxType[this[name]];
          } else {
            data[name] = stringify(this[name]);
          }
        });
        return JSON.stringify(data, undefined, 2);
    }
}

class RewardTx extends Tx {
    constructor(data) {
        super(TxType.REWARD, data);
    }
}

class BondTx extends Tx {
    constructor(data) {
        super(TxType.BOND, data);
    }
}

class TransferTx extends Tx {
    constructor(data) {
        super(TxType.TRANSFER, data);
        if (this.memo === undefined) this.memo = Buffer.alloc(0);
    }
}

function stringify(obj) {
    if (obj instanceof Array) {
      const arr = [];
      for (const o of obj) arr.push(stringify(o));
      return arr;
    } else if (obj instanceof Asset) {
      return obj.toString();
    } else if (obj instanceof Buffer) {
      return obj.toString('hex');
    } else if (obj instanceof PublicKey) {
      return obj.toWif();
    } else if (obj && obj.public_key && obj.signature) {
      return {
        public_key: obj.public_key.toWif(),
        signature: obj.signature.toString('hex')
      };
    }
    return obj;
}

module.exports = {
    TxType,
    Tx,
    RewardTx,
    BondTx,
    TransferTx
};

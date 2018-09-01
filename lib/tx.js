const addon = require('../native');

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
        switch (tx.tx_type) {
            case TxType.REWARD:
                return new RewardTx(tx);
            case TxType.BOND:
                return new BondTx(tx);
            case TxType.TRANSFER:
                return new TransferTx(tx);
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

module.exports = {
    TxType,
    Tx,
    RewardTx,
    BondTx,
    TransferTx
};

const { doubleSha256 } = require('./crypto');
const native = require('./native');
const { Tx } = require('./tx');

class Block {
    static calcMerkleRoot(txArr) {
        return native.Block_calc_tx_merkle_root(txArr);
    }

    constructor(data) {
        Object.assign(this, data);
        if (typeof(this.timestamp) === 'number') {
            this.timestamp = new Date(this.timestamp * 1000);
        } else {
            const truncated = Math.floor(this.timestamp.getTime() / 1000) * 1000;
            this.timestamp = new Date(truncated);
        }
        if (!this.tx_merkle_root) {
            this.tx_merkle_root = native.Block_calc_tx_merkle_root(this.transactions);
        }
    }

    verifyMerkleRoot() {
        const root = Block.calcMerkleRoot(this.transactions);
        return this.tx_merkle_root.equals(root);
    }

    encodeHeader() {
        return native.Block_encode_header(this);
    }

    calcHash() {
        return doubleSha256(this.encodeHeader());
    }

    sign(keyPair) {
        const header = this.encodeHeader();
        const sb = new SignedBlock(this);
        sb.sig_pair = [keyPair[0], keyPair[1].sign(header)];
        return sb;
    }

    toString() {
        return JSON.stringify({
            height: this.height.toString(),
            previous_hash: this.previous_hash.toString('hex'),
            timestamp: this.timestamp,
            transactions: this.transactions.map(val => JSON.parse(val.toString())),
            tx_merkle_root: this.tx_merkle_root.toString('hex'),
            sig_pair: this.sig_pair ? {
                public_key: this.sig_pair[0].toWif(),
                signature: this.sig_pair[1].toString('hex')
            } : undefined
        }, undefined, 2);
    }
}

class SignedBlock extends Block {
    static decodeWithTx(buf) {
        const data = native.SignedBlock_decode_with_tx(buf);
        const txs = [];
        for (const tx of data.transactions) {
            txs.push(Tx.objToTx(tx));
        }
        data.transactions = txs;
        return new SignedBlock(data);
    }

    constructor(data) {
        super(data);
    }

    encodeWithTx() {
        return native.SignedBlock_encode_with_tx(this);
    }
}

module.exports = {
    Block,
    SignedBlock
}

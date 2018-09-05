import { KeyPair, SigPair } from './crypto';
import { Tx } from './tx';

export class Block {
    static calcMerkleRoot(txArray: Tx[]): Buffer;

    height: number;
    previous_hash: Buffer;
    timestamp: Date;
    transactions: Tx[];
    tx_merkle_root: Buffer;

    constructor(data: any);

    verifyMerkleRoot(): boolean;
    encodeHeader(): Buffer;
    calcHash(): Buffer;
    sign(pair: KeyPair): SignedBlock;
    toString(): string;
}

export class SignedBlock extends Block {
    static decodeWithTx(buf: Buffer): SignedBlock;

    sig_pair: SigPair;

    constructor(data: any);

    encodeWithTx(): Buffer;
}

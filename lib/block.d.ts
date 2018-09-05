import { KeyPair, SigPair } from './crypto';
import { Tx } from './tx';

export interface BlockData {
    height: number;
    previous_hash: Buffer;
    timestamp: Date;
    transactions: Tx[];
    tx_merkle_root?: Buffer;
}

export class Block implements BlockData {
    static calcMerkleRoot(txArray: Tx[]): Buffer;

    height: number;
    previous_hash: Buffer;
    timestamp: Date;
    transactions: Tx[];
    tx_merkle_root: Buffer;

    constructor(data: BlockData);

    verifyMerkleRoot(): boolean;
    encodeHeader(): Buffer;
    calcHash(): Buffer;
    sign(pair: KeyPair): SignedBlock;
    toString(): string;
}

export interface SignedBlockData extends BlockData {
    sig_pair: SigPair;
}

export class SignedBlock extends Block implements BlockData {
    static decodeWithTx(buf: Buffer): SignedBlock;

    sig_pair: SigPair;

    constructor(data: SignedBlockData);

    encodeWithTx(): Buffer;
}

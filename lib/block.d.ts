import { KeyPair } from './crypto';

export class Block {
    static calcMerkleRoot(txArray: Tx[]): Buffer;

    height: number;
    previous_hash: Buffer;
    timestamp: Date;
    tx_merkle_root: Buffer;

    private constructor(data: any);

    verifyMerkleRoot(): boolean;
    encodeHeader(): Buffer;
    sign(pair: KeyPair): SignedBlock;
    toString(): string;
}

export class SignedBlock extends Block {
    static decodeWithTx(buf: Buffer): SignedBlock;

    sig_pair: Buffer;

    private constructor(data: any);

    encodeWithTx(): Buffer;
}

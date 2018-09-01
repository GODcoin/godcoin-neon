import { KeyPair, SigPair, PublicKey } from './crypto';
import { Asset } from './asset';

export enum TxType {
    REWARD = 0,
    TRANSFER = 1,
    BOND = 2
}

export interface TxData {
    timestamp: Date;
    fee: Asset;
    signature_pairs: SigPair[];
}

export class Tx {
    static decodeWithSigs<T extends Tx>(buffer): T|null;

    private constructor(type: TxType, data: any);

    appendSign(keyPair: KeyPair): Tx;

    encode(): Buffer;
    encodeWithSigs(): Buffer;
}

export interface RewardTxData extends TxData {
    to: PublicKey;
    rewards: Asset[];
}

export class RewardTx extends Tx {
    constructor(data: RewardTxData);
}

export interface BondTxData extends TxData {
    minter: PublicKey;
    staker: PublicKey;
    stake_amt: Asset;
    bond_fee: Asset;
}

export class BondTx extends Tx {
    constructor(data: BondTxData);
}

export interface TransferTxData extends TxData {
    from: PublicKey;
    to: PublicKey;
    amount: Asset;
    memo: Buffer;
}

export class TransferTx extends Tx {
    constructor(data: TransferTxData);
}

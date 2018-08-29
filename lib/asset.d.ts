export enum AssetSymbol {
    GOLD = 0,
    SILVER = 1
}

export class Asset {
    static readonly MAX_STR_LEN: number;
    static readonly MAX_PRECISION: number;

    static readonly EMPTY_GOLD: Asset;
    static readonly EMPTY_SILVER: Asset;

    readonly amount: number;
    readonly decimals: number;
    readonly symbol: AssetSymbol;

    static fromString(s: string): Asset;

    constructor(amt: number, decimals: number, symbol: AssetSymbol);

    toString(): string;

    add(other: Asset): Asset;
    sub(other: Asset): Asset;
    mul(other: Asset, decimals: number): Asset;
    div(other: Asset, decimals: number): Asset;
    pow(num: number, decimals: number): Asset;

    gt(other: Asset): boolean;
    geq(other: Asset): boolean;
    eq(other: Asset): boolean;
    leq(other: Asset): boolean;
    lt(other: Asset): boolean;
}

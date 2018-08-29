export type KeyPair = [PublicKey, PrivateKey];

export class PublicKey {
  static fromWif(wif: string): PublicKey;

  constructor(key: ArrayBuffer);

  verify(sig: Buffer, msg: Buffer): boolean;
  equals(other: PublicKey): boolean;
  toWif(): string;
}

export class PrivateKey {
  static fromWif(wif: string): KeyPair;
  static genKeyPair(): KeyPair;

  constructor(seed: ArrayBuffer, secret: ArrayBuffer);

  sign(msg: Buffer): Buffer;
  toWif(): string;
}

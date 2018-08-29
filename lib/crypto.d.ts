export type KeyPair = [PublicKey, PrivateKey];
export type SigPair = [PublicKey, Buffer];

export class PublicKey {
  static fromWif(wif: string): PublicKey;

  constructor(key: Buffer);

  verify(sig: Buffer, msg: Buffer): boolean;
  equals(other: PublicKey): boolean;
  toWif(): string;
}

export class PrivateKey {
  static fromWif(wif: string): KeyPair;
  static genKeyPair(): KeyPair;

  constructor(seed: Buffer, secret: Buffer);

  sign(msg: Buffer): Buffer;
  toWif(): string;
}

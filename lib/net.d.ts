import { SignedBlock } from './block';
import { Tx } from './tx';

export enum RpcMsgType {
  NONE = 0,
  HANDSHAKE = 1,
  PROPERTIES = 2,
  EVENT = 3
}

export enum RpcEventType {
  TX = 'tx',
  BLOCK = 'block'
}

export enum PeerType {
  NODE = 0,
  WALLET = 1
}

export class RpcCodec {
  readonly buffer: Buffer;

  encode(obj: RpcPayload<RpcMsg>): Buffer;

  update(buf: Buffer): void;
  decode(): RpcPayload<RpcMsg>;
}

export interface RpcPayload<T extends RpcMsg> {
  id: number;
  msg_type?: RpcMsgType;
  data?: T;
}

export interface RpcMsg {}

export interface RpcMsgHandshake extends RpcMsg {
  peer_type: PeerType;
}

export interface RpcMsgProperties extends RpcMsg {
  height: number;
}

export interface RpcMsgEvent extends RpcMsg {
  type: 'tx' | 'block';
  subscribe?: boolean;
  data?: Tx | SignedBlock;
}

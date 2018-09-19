import { SignedBlock } from './block';
import { Tx } from './tx';

export enum RpcMsgType {
  NONE = -1,
  ERROR = 0,
  EVENT = 1,
  HANDSHAKE = 2,
  BROADCAST = 3,
  PROPERTIES = 4,
  BLOCK = 5,
  BALANCE = 6,
  TOTAL_FEE = 7
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

  encode(obj: RpcPayload): Buffer;

  update(buf: Buffer): void;
  decode(): RpcPayload;
}

export interface RpcPayload {
  id: number;
  msg_type: RpcMsgType;
  req?: any;
  res?: any;
}

export interface RpcMsgReqHandshake {
  peer_type: PeerType;
}

export interface RpcMsgResProperties {
  height: number;
}

export interface RpcMsgEvent {
  type: RpcEventType;
  data: Tx | SignedBlock;
}

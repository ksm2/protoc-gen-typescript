import type { BinaryReader, BinaryWriter } from 'google-protobuf';

export interface GrpcService {
  unary<R extends Deserializable>(
    rpc: string,
    req: Serializable,
    res: Class<R>,
    sig?: AbortSignal
  ): Promise<R>;

  serverStreaming<R extends Deserializable>(
    rpc: string,
    req: Serializable,
    res: Class<R>,
    sig?: AbortSignal
  ): AsyncIterable<R>;

  clientStreaming<R extends Deserializable>(
    rpc: string,
    req: AsyncIterable<Serializable>,
    res: Class<R>,
    sig?: AbortSignal
  ): Promise<R>;

  bidiStreaming<R extends Deserializable>(
    rpc: string,
    req: AsyncIterable<Serializable>,
    res: Class<R>,
    sig?: AbortSignal
  ): AsyncIterable<R>;
}

export interface Serializable {
  serialize(writer: BinaryWriter): void;
}

export interface Deserializable {
  deserialize(reader: BinaryReader): void;
}

export interface Class<T> {
  new (): T;
}

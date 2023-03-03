import { BinaryReader, BinaryWriter } from 'google-protobuf';

export interface Serializable {
  serialize(writer: BinaryWriter): void;
}

export interface Deserializable {
  deserialize(reader: BinaryReader): void;
}

export interface Class<T> {
  new (): T;
}

export function serialize(ser: Serializable): Uint8Array {
  const writer = new BinaryWriter();
  ser.serialize(writer);
  return writer.getResultBuffer();
}

export function deserialize<D extends Deserializable>(clazz: Class<D>, binary: Uint8Array): D {
  const reader = new BinaryReader(binary);
  const obj = new clazz();
  obj.deserialize(reader);
  return obj;
}

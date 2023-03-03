import { UInt32Value } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('UInt32Value', () => {
  it('should serialize UInt32Value', async () => {
    const int = UInt32Value.of(42);
    const binary = serialize(int);

    const str = await decode('google.protobuf.UInt32Value', binary);

    expect(str).toEqual({ value: 42 });
  });

  it('should deserialize UInt32Value', async () => {
    const binary = await encode('google.protobuf.UInt32Value', {
      value: 42,
    });

    const int = deserialize(UInt32Value, binary);

    expect(int.value).toStrictEqual(42);
  });
});

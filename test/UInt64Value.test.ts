import { UInt64Value } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('UInt64Value', () => {
  it('should serialize UInt64Value', async () => {
    const int = UInt64Value.of(42n);
    const binary = serialize(int);

    const str = await decode('google.protobuf.UInt64Value', binary);

    expect(str).toEqual({ value: 42 });
  });

  it('should deserialize UInt64Value', async () => {
    const binary = await encode('google.protobuf.UInt64Value', {
      value: 42,
    });

    const int = deserialize(UInt64Value, binary);

    expect(int.value).toBe(42n);
  });
});

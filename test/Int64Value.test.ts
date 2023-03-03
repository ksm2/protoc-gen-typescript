import { Int64Value } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('Int64Value', () => {
  it('should serialize Int64Value', async () => {
    const int = Int64Value.of(42n);
    const binary = serialize(int);

    const str = await decode('google.protobuf.Int64Value', binary);

    expect(str).toEqual({ value: 42 });
  });

  it('should deserialize Int64Value', async () => {
    const binary = await encode('google.protobuf.Int64Value', {
      value: 42,
    });

    const int = deserialize(Int64Value, binary);

    expect(int.value).toBe(42n);
  });
});

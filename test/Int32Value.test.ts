import { Int32Value } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('Int32Value', () => {
  it('should serialize Int32Value', async () => {
    const int = Int32Value.of(42);
    const binary = serialize(int);

    const str = await decode('google.protobuf.Int32Value', binary);

    expect(str).toEqual({ value: 42 });
  });

  it('should deserialize Int32Value', async () => {
    const binary = await encode('google.protobuf.Int32Value', {
      value: 42,
    });

    const int = deserialize(Int32Value, binary);

    expect(int.value).toBe(42);
  });
});

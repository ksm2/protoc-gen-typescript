import { DoubleValue } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('DoubleValue', () => {
  it('should serialize DoubleValue', async () => {
    const double = DoubleValue.of(Math.PI);
    const binary = serialize(double);

    const str = await decode('google.protobuf.DoubleValue', binary);

    expect(str).toEqual({ value: Math.PI });
  });

  it('should deserialize DoubleValue', async () => {
    const binary = await encode('google.protobuf.DoubleValue', {
      value: Math.PI,
    });

    const double = deserialize(DoubleValue, binary);

    expect(double.value).toBe(Math.PI);
  });
});

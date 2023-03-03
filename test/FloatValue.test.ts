import { FloatValue } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('FloatValue', () => {
  it('should serialize FloatValue', async () => {
    const float = FloatValue.of(12.25);
    const binary = serialize(float);

    const str = await decode('google.protobuf.FloatValue', binary);

    expect(str).toEqual({ value: 12.25 });
  });

  it('should deserialize FloatValue', async () => {
    const binary = await encode('google.protobuf.FloatValue', {
      value: 12.25,
    });

    const float = deserialize(FloatValue, binary);

    expect(float.value).toBe(12.25);
  });
});

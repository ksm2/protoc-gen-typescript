import { BoolValue } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('BoolValue', () => {
  beforeAll(() => {
    jest.useFakeTimers();
  });

  it('should serialize BoolValue', async () => {
    const bool = BoolValue.of(true);
    const binary = serialize(bool);

    const str = await decode('google.protobuf.BoolValue', binary);

    expect(str).toEqual({ value: true });
  });

  it('should deserialize BoolValue', async () => {
    const binary = await encode('google.protobuf.BoolValue', {
      value: true,
    });

    const bool = deserialize(BoolValue, binary);

    expect(bool.value).toBe(true);
  });
});

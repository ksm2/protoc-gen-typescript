import { BytesValue } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('BytesValue', () => {
  it('should serialize BytesValue', async () => {
    const bytes = BytesValue.of(new Uint8Array([0x41, 0x42, 0x43, 0x44]));
    const binary = serialize(bytes);

    const str = await decode('google.protobuf.BytesValue', binary);

    expect(str).toEqual({ value: 'ABCD' });
  });

  it('should deserialize BytesValue', async () => {
    const binary = await encode('google.protobuf.BytesValue', {
      value: 'ABCD',
    });

    const bytes = deserialize(BytesValue, binary);

    expect(bytes.value).toStrictEqual(new Uint8Array([0x41, 0x42, 0x43, 0x44]));
  });
});

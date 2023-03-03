import { StringValue } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('StringValue', () => {
  it('should serialize StringValue', async () => {
    const string = StringValue.of('hello world');
    const binary = serialize(string);

    const str = await decode('google.protobuf.StringValue', binary);

    expect(str).toEqual({ value: 'hello world' });
  });

  it('should deserialize StringValue', async () => {
    const binary = await encode('google.protobuf.StringValue', {
      value: 'hello world',
    });

    const string = deserialize(StringValue, binary);

    expect(string.value).toBe('hello world');
  });
});

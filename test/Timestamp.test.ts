import { Timestamp } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('Timestamp', () => {
  beforeAll(() => {
    jest.useFakeTimers();
    jest.setSystemTime(new Date('2021-02-03T23:59:59.678Z'));
  });

  it('should serialize Timestamp', async () => {
    const foo = Timestamp.fromDate(new Date());
    const binary = serialize(foo);

    const str = await decode('google.protobuf.Timestamp', binary);

    expect(str).toEqual({
      seconds: 1612396799,
      nanos: 678000000,
    });
  });

  it('should deserialize Timestamp', async () => {
    const binary = await encode('google.protobuf.Timestamp', {
      seconds: 1612396799,
      nanos: 678000000,
    });

    const foo = deserialize(Timestamp, binary);

    expect(foo.toDate()).toStrictEqual(new Date());
  });
});

import { Duration } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('Duration', () => {
  beforeAll(() => {
    jest.useFakeTimers();
  });

  it('should serialize Duration', async () => {
    const dur = Duration.between(
      new Date('2021-01-01T00:00:00.000Z'),
      new Date('2021-01-01T01:00:00.987Z')
    );
    const binary = serialize(dur);

    const str = await decode('google.protobuf.Duration', binary);

    expect(str).toEqual({
      seconds: 3600,
      nanos: 987_000_000,
    });
  });

  it('should deserialize Duration', async () => {
    const binary = await encode('google.protobuf.Duration', {
      seconds: 3600,
      nanos: 987_000_000,
    });

    const foo = deserialize(Duration, binary);

    expect(foo.toMillis()).toStrictEqual(3600_987);
  });
});

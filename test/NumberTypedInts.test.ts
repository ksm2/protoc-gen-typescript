import { NumberTypedInts } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('NumberTypedInts', () => {
  const MIN_INT64 = BigInt(Number.MIN_SAFE_INTEGER);
  const MAX_INT64 = BigInt(Number.MAX_SAFE_INTEGER);

  it('should serialize NumberTypedInts', async () => {
    const ints = new NumberTypedInts();
    ints.testInt64 = Number(MAX_INT64);
    ints.testUint64 = Number(MAX_INT64);
    ints.testFixed64 = Number(MAX_INT64);
    ints.testSint64 = Number(MIN_INT64);

    const binary = serialize(ints);

    const str = await decode('NumberTypedInts', binary);
    expect(str).toEqual({
      test_int64: MAX_INT64,
      test_uint64: MAX_INT64,
      test_fixed64: MAX_INT64,
      test_sint64: MIN_INT64,
    });
  });

  it('should deserialize NumberTypedInts', async () => {
    const binary = await encode('NumberTypedInts', {
      test_int64: MAX_INT64,
      test_uint64: MAX_INT64,
      test_fixed64: MAX_INT64,
      test_sint64: MIN_INT64,
    });

    const ints = deserialize(NumberTypedInts, binary);

    expect(ints.testInt64).toBe(Number(MAX_INT64));
    expect(ints.testUint64).toBe(Number(MAX_INT64));
    expect(ints.testFixed64).toBe(Number(MAX_INT64));
    expect(ints.testSint64).toBe(Number(MIN_INT64));
  });
});

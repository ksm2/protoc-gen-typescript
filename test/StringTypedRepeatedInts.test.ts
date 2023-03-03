import { StringTypedRepeatedInts } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('StringTypedRepeatedInts', () => {
  const MIN_INT64 = -9223372036854775808n;
  const MAX_INT64 = 9223372036854775807n;
  const MAX_UINT64 = 18446744073709551615n;

  it('should serialize StringTypedRepeatedInts', async () => {
    const ints = new StringTypedRepeatedInts();
    ints.testInt64 = [MIN_INT64.toString(10), MAX_INT64.toString(10)];
    ints.testUint64 = [MAX_UINT64.toString(10), '0'];
    ints.testFixed64 = [MAX_INT64.toString(10)];
    ints.testSint64 = [MIN_INT64.toString(10)];

    const binary = serialize(ints);

    const str = await decode('StringTypedRepeatedInts', binary);
    expect(str).toEqual({
      test_int64: [MIN_INT64, MAX_INT64],
      test_uint64: [MAX_UINT64, 0],
      test_fixed64: MAX_INT64,
      test_sint64: MIN_INT64,
    });
  });

  it('should deserialize StringTypedRepeatedInts', async () => {
    const binary = await encode('StringTypedRepeatedInts', {
      test_int64: [MIN_INT64, MAX_INT64],
      test_uint64: [MAX_UINT64, 0n],
      test_fixed64: MAX_INT64,
      test_sint64: MIN_INT64,
    });

    const ints = deserialize(StringTypedRepeatedInts, binary);

    expect(ints.testInt64).toStrictEqual([MIN_INT64.toString(10), MAX_INT64.toString(10)]);
    expect(ints.testUint64).toStrictEqual([MAX_UINT64.toString(10), '0']);
    expect(ints.testFixed64).toStrictEqual([MAX_INT64.toString(10)]);
    expect(ints.testSint64).toStrictEqual([MIN_INT64.toString(10)]);
  });
});

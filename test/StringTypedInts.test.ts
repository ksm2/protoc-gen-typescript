import { BinaryReader, BinaryWriter } from 'google-protobuf';
import { StringTypedInts } from '../gen';
import { decode, encode } from './protoc';

describe('StringTypedInts', () => {
  const MIN_INT64 = -9223372036854775808n;
  const MAX_INT64 = 9223372036854775807n;
  const MAX_UINT64 = 18446744073709551615n;

  it('should serialize StringTypedInts', async () => {
    const ints = new StringTypedInts();
    ints.testInt64 = MAX_INT64.toString(10);
    ints.testUint64 = MAX_UINT64.toString(10);
    ints.testFixed64 = MAX_INT64.toString(10);
    ints.testSint64 = MIN_INT64.toString(10);

    const writer = new BinaryWriter();
    ints.serialize(writer);
    const binary = writer.getResultBuffer();

    const str = await decode('StringTypedInts', binary);
    expect(str).toEqual({
      test_int64: MAX_INT64,
      test_uint64: MAX_UINT64,
      test_fixed64: MAX_INT64,
      test_sint64: MIN_INT64,
    });
  });

  it('should deserialize StringTypedInts', async () => {
    const binary = await encode('StringTypedInts', {
      test_int64: MAX_INT64,
      test_uint64: MAX_UINT64,
      test_fixed64: MAX_INT64,
      test_sint64: MIN_INT64,
    });

    const reader = new BinaryReader(binary);
    const ints = new StringTypedInts();
    ints.deserialize(reader);

    expect(ints.testInt64).toBe(MAX_INT64.toString(10));
    expect(ints.testUint64).toBe(MAX_UINT64.toString(10));
    expect(ints.testFixed64).toBe(MAX_INT64.toString(10));
    expect(ints.testSint64).toBe(MIN_INT64.toString(10));
  });
});

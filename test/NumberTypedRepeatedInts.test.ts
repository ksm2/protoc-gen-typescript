import { BinaryReader, BinaryWriter } from 'google-protobuf';
import { NumberTypedRepeatedInts } from '../gen';
import { decode, encode } from './protoc';

describe('NumberTypedRepeatedInts', () => {
  const MIN_INT64 = BigInt(Number.MIN_SAFE_INTEGER);
  const MAX_INT64 = BigInt(Number.MAX_SAFE_INTEGER);

  it('should serialize NumberTypedRepeatedInts', async () => {
    const ints = new NumberTypedRepeatedInts();
    ints.testInt64 = [Number(MIN_INT64), Number(MAX_INT64)];
    ints.testUint64 = [Number(MAX_INT64)];
    ints.testFixed64 = [Number(MAX_INT64)];
    ints.testSint64 = [Number(MIN_INT64)];

    const writer = new BinaryWriter();
    ints.serialize(writer);
    const binary = writer.getResultBuffer();

    const str = await decode('NumberTypedRepeatedInts', binary);
    expect(str).toEqual({
      test_int64: [MIN_INT64, MAX_INT64],
      test_uint64: MAX_INT64,
      test_fixed64: MAX_INT64,
      test_sint64: MIN_INT64,
    });
  });

  it('should deserialize NumberTypedRepeatedInts', async () => {
    const binary = await encode('NumberTypedRepeatedInts', {
      test_int64: [MIN_INT64, MAX_INT64],
      test_uint64: MAX_INT64,
      test_fixed64: MAX_INT64,
      test_sint64: MIN_INT64,
    });

    const reader = new BinaryReader(binary);
    const ints = new NumberTypedRepeatedInts();
    ints.deserialize(reader);

    expect(ints.testInt64).toStrictEqual([Number(MIN_INT64), Number(MAX_INT64)]);
    expect(ints.testUint64).toStrictEqual([Number(MAX_INT64)]);
    expect(ints.testFixed64).toStrictEqual([Number(MAX_INT64)]);
    expect(ints.testSint64).toStrictEqual([Number(MIN_INT64)]);
  });
});

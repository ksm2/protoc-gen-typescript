import { BinaryReader, BinaryWriter } from 'google-protobuf';
import { Nested, RepeatedMessage, Status } from '../gen';
import { decode, encode } from './protoc';

describe('RepeatedMessage', () => {
  it('should serialize RepeatedMessage', async () => {
    const foo = new RepeatedMessage();
    foo.testString = ['str1', 'str2'];
    foo.testBool = [true, false, true];
    foo.testUnpackedBool = [true, false, true];
    foo.testEnum = [Status.ONE, Status.TWO];
    foo.testUnpackedEnum = [Status.ONE, Status.TWO];

    const nested = new Nested();
    nested.nestedStr = 'Hello World';
    foo.testNested.push(nested);

    const writer = new BinaryWriter();
    foo.serialize(writer);
    const binary = writer.getResultBuffer();

    const str = await decode('RepeatedMessage', binary);
    expect(str).toEqual({
      test_string: ['str1', 'str2'],
      test_bool: [true, false, true],
      test_unpacked_bool: [true, false, true],
      test_enum: ['ONE', 'TWO'],
      test_unpacked_enum: ['ONE', 'TWO'],
      test_nested: {
        nested_str: 'Hello World',
      },
    });
  });

  it('should deserialize RepeatedMessage', async () => {
    const binary = await encode('RepeatedMessage', {
      test_string: ['str1', 'str2'],
      test_bool: [false, true, false],
      test_unpacked_bool: [false, true, false],
      test_enum: [2, 1],
      test_unpacked_enum: [2, 1],
      test_nested: {
        nested_str: 'Hello World',
      },
    });

    const reader = new BinaryReader(binary);
    const foo = new RepeatedMessage();
    foo.deserialize(reader);

    expect(foo.testString).toStrictEqual(['str1', 'str2']);
    expect(foo.testBool).toStrictEqual([false, true, false]);
    expect(foo.testUnpackedBool).toStrictEqual([false, true, false]);
    expect(foo.testEnum).toStrictEqual([Status.TWO, Status.ONE]);
    expect(foo.testUnpackedEnum).toStrictEqual([Status.TWO, Status.ONE]);
    expect(foo.testNested).toHaveLength(1);
    expect(foo.testNested[0].nestedStr).toBe('Hello World');
  });
});

import { BinaryReader, BinaryWriter } from 'google-protobuf';
import { Foo, Nested, Status } from '../gen';
import { decode, encode } from './protoc';

describe('Foo', () => {
  it('should serialize Foo', async () => {
    const foo = new Foo();
    foo.testString = 'Lorem ipsum dolor sit amet';
    foo.testBool = true;
    foo.testEnum = Status.TWO;
    foo.testNested = new Nested();
    foo.testNested.nestedStr = 'Hello World';

    const writer = new BinaryWriter();
    foo.serialize(writer);
    const binary = writer.getResultBuffer();

    const str = await decode('Foo', binary);
    expect(str).toEqual({
      test_string: 'Lorem ipsum dolor sit amet',
      test_bool: true,
      test_enum: 'TWO',
      test_nested: {
        nested_str: 'Hello World',
      },
    });
  });

  it('should deserialize Foo', async () => {
    const binary = await encode('Foo', {
      test_string: 'Lorem ipsum dolor sit amet',
      test_bool: true,
      test_enum: 1,
      test_nested: {
        nested_str: 'Hello World',
      },
    });

    const reader = new BinaryReader(binary);
    const foo = new Foo();
    foo.deserialize(reader);

    expect(foo.testString).toBe('Lorem ipsum dolor sit amet');
    expect(foo.testBool).toBe(true);
    expect(foo.testEnum).toBe(Status.ONE);
    expect(foo.testNested?.nestedStr).toBe('Hello World');
  });
});

import { Foo, Nested, Status } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('Foo', () => {
  it('should serialize Foo', async () => {
    const foo = new Foo();
    foo.testString = 'Lorem ipsum dolor sit amet';
    foo.testBool = true;
    foo.testEnum = Status.TWO;
    foo.testNested = new Nested();
    foo.testNested.nestedStr = 'Hello World';

    const binary = serialize(foo);

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

    const foo = deserialize(Foo, binary);

    expect(foo.testString).toBe('Lorem ipsum dolor sit amet');
    expect(foo.testBool).toBe(true);
    expect(foo.testEnum).toBe(Status.ONE);
    expect(foo.testNested?.nestedStr).toBe('Hello World');
  });
});

import { Message, Nested, Status } from '../gen';
import { decode, encode } from './protoc';
import { deserialize, serialize } from './serde';

describe('Message', () => {
  it('should serialize Message', async () => {
    const msg = new Message();
    msg.testString = 'Lorem ipsum dolor sit amet';
    msg.testBytes = new Uint8Array([0x41, 0x42, 0x43, 0x44]);
    msg.testBool = true;
    msg.testInt32 = -42;
    msg.testInt64 = -1337n;
    msg.testUint32 = 42;
    msg.testUint64 = 1337n;
    msg.testSint32 = -1;
    msg.testSint64 = 1n;
    msg.testFixed32 = 1;
    msg.testFixed64 = 1n;
    msg.testFloat = 1234.5;
    msg.testDouble = Math.PI;
    msg.testEnum = Status.TWO;
    msg.testNested = new Nested();
    msg.testNested.nestedStr = 'Hello World';

    const binary = serialize(msg);

    const str = await decode('Message', binary);
    expect(str).toEqual({
      test_string: 'Lorem ipsum dolor sit amet',
      test_bytes: 'ABCD',
      test_bool: true,
      test_int32: -42,
      test_int64: -1337,
      test_uint32: 42,
      test_uint64: 1337,
      test_sint32: -1,
      test_sint64: 1,
      test_fixed32: 1,
      test_fixed64: 1,
      test_float: 1234.5,
      test_double: Math.PI,
      test_enum: 'TWO',
      test_nested: {
        nested_str: 'Hello World',
      },
    });
  });

  it('should deserialize Message', async () => {
    const binary = await encode('Message', {
      test_string: 'Lorem ipsum dolor sit amet',
      test_bytes: 'ABCD',
      test_bool: true,
      test_int32: -42,
      test_int64: -1337,
      test_uint32: 42,
      test_uint64: 1337,
      test_sint32: -1,
      test_sint64: 1,
      test_fixed32: 1,
      test_fixed64: 1,
      test_float: 1234.5,
      test_double: Math.PI,
      test_enum: Status.TWO,
      test_nested: {
        nested_str: 'Hello World',
      },
    });

    const msg = deserialize(Message, binary);

    expect(msg.testString).toBe('Lorem ipsum dolor sit amet');
    expect(msg.testBytes).toStrictEqual(new Uint8Array([0x41, 0x42, 0x43, 0x44]));
    expect(msg.testBool).toBe(true);
    expect(msg.testInt32).toBe(-42);
    expect(msg.testInt64).toBe(-1337n);
    expect(msg.testUint32).toBe(42);
    expect(msg.testUint64).toBe(1337n);
    expect(msg.testSint32).toBe(-1);
    expect(msg.testSint64).toBe(1n);
    expect(msg.testFixed32).toBe(1);
    expect(msg.testFixed64).toBe(1n);
    expect(msg.testFloat).toBe(1234.5);
    expect(msg.testDouble).toBe(Math.PI);
    expect(msg.testEnum).toBe(Status.TWO);
    expect(msg.testNested?.nestedStr).toBe('Hello World');
  });
});

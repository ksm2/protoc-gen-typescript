import childProcess from 'node:child_process';
import { jsonToProtoText, protoTextToJSON } from './proto-text';

export async function decode(typeName: string, binary: Uint8Array): Promise<unknown> {
  const buf = await exec(`protoc --decode=${typeName} -Iexample example/test.proto`, binary);
  const str = buf.toString();
  return protoTextToJSON(str);
}

export async function encode(typeName: string, obj: unknown): Promise<Buffer> {
  const str = jsonToProtoText(obj);
  const buf = Buffer.from(str);
  return exec(`protoc --encode=${typeName} -Iexample example/test.proto`, buf);
}

function exec(command: string, stdin: Uint8Array): Promise<Buffer> {
  return new Promise((resolve, reject) => {
    const proc = childProcess.exec(command, { encoding: 'buffer' }, (error, stdout) => {
      if (error) {
        reject(error);
      } else {
        resolve(stdout);
      }
    });

    proc.stdin!.write(stdin);
    proc.stdin!.end();
  });
}

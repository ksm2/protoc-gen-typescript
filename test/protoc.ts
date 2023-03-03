import childProcess from 'node:child_process';
import { jsonToProtoText, protoTextToJSON } from './proto-text';

export async function decode(typeName: string, binary: Uint8Array): Promise<unknown> {
  const buf = await protoc(`--decode=${typeName}`, binary);
  const str = buf.toString();
  return protoTextToJSON(str);
}

export async function encode(typeName: string, obj: unknown): Promise<Buffer> {
  const str = jsonToProtoText(obj);
  const buf = Buffer.from(str);
  return protoc(`--encode=${typeName}`, buf);
}

function protoc(command: string, stdin: Uint8Array): Promise<Buffer> {
  const protoFiles = [
    'include/test.proto',
    'include/duration.proto',
    'include/timestamp.proto',
    'include/wrappers.proto',
  ];
  return exec(`protoc ${command} -Iinclude ${protoFiles.join(' ')}`, stdin);
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

export function protoTextToJSON(text: string): unknown {
  const lines = text.trimEnd().split(/\r\n|[\r\n]/g);
  return protoTextToJSONLines(lines);
}

function protoTextToJSONLines(lines: string[]): unknown {
  const output: Record<string, unknown> = {};
  let line: string | undefined;
  while ((line = lines.shift()) !== undefined) {
    if (line.includes(':')) {
      let [prop, val] = line.split(': ') as [prop: string, val: any];
      if (val.match(/^[A-Z_]+$/)) {
        val = `"${val}"`;
      }
      if (val.match(/^-?[0-9]{16,}$/)) {
        val = BigInt(val);
      } else {
        val = JSON.parse(val);
      }

      add(output, prop.trimStart(), val);
    } else if (line.endsWith('{')) {
      const prop = line.slice(0, -2);
      add(output, prop, protoTextToJSONLines(lines));
    } else if (line.endsWith('}')) {
      return output;
    }
  }
  return output;
}

function add(output: Record<string, unknown>, prop: string, value: unknown) {
  if (Array.isArray(output[prop])) {
    (output[prop] as unknown[]).push(value);
  } else if (output[prop] !== undefined) {
    output[prop] = [output[prop], value];
  } else {
    output[prop] = value;
  }
}

export function jsonToProtoText(obj: unknown): string {
  return jsonToProtoTextIndent(obj, 0);
}

function jsonToProtoTextIndent(obj: unknown, indent: number): string {
  let output = '';
  for (const [key, val] of Object.entries(obj as object)) {
    const indentation = ' '.repeat(indent);
    const value = Array.isArray(val) ? val : [val];
    for (const val of value) {
      if (typeof val !== 'object') {
        output += `${indentation}${key}: ${encode(val)}\n`;
      } else {
        const nested = jsonToProtoTextIndent(val, indent + 2);
        output += `${indentation}${key} {\n${nested}${indentation}}\n`;
      }
    }
  }
  return output;
}

function encode(val: unknown): string {
  if (typeof val === 'bigint') {
    return val.toString(10);
  }
  return JSON.stringify(val);
}

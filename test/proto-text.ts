export function protoTextToJSON(text: string): unknown {
  const lines = text.trimEnd().split(/\r\n|[\r\n]/g);
  let output = '{';
  for (const line of lines) {
    if (line.includes(':')) {
      let [prop, val] = line.split(': ');
      if (val.match(/^[A-Z_]+$/)) {
        val = `"${val}"`;
      }
      output += `"${prop.trimStart()}":${val},`;
    } else if (line.endsWith('{')) {
      output += `"${line.slice(0, -2)}": {`;
    } else if (line.endsWith('}')) {
      if (output.endsWith(',')) {
        output = output.slice(0, -1);
      }
      output += '}';
    }
  }
  output += '}';
  return JSON.parse(output);
}

export function jsonToProtoText(obj: unknown): string {
  return jsonToProtoTextIndent(obj, 0);
}

function jsonToProtoTextIndent(obj: unknown, indent: number): string {
  let output = '';
  for (const [key, val] of Object.entries(obj as object)) {
    const indentation = ' '.repeat(indent);
    if (typeof val !== 'object') {
      output += `${indentation}${key}: ${JSON.stringify(val)}\n`;
    } else {
      const nested = jsonToProtoTextIndent(val, indent + 2);
      output += `${indentation}${key} {\n${nested}${indentation}}\n`;
    }
  }
  return output;
}

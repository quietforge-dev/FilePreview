import { readFile } from '../../api/file';

const decoder = new TextDecoder('utf-8', { fatal: false });

const imageMimeTypes: Record<string, string> = {
  gif: 'image/gif',
  jpeg: 'image/jpeg',
  jpg: 'image/jpeg',
  png: 'image/png',
  svg: 'image/svg+xml',
  webp: 'image/webp',
};

export const imageMimeType = (extension: string) => imageMimeTypes[extension];

export const readTextFile = async (path: string) => decoder.decode(await readFile(path));

export const readBinaryFile = (path: string) => readFile(path);

export const toArrayBuffer = (bytes: Uint8Array): ArrayBuffer => {
  const copy = new Uint8Array(bytes.byteLength);
  copy.set(bytes);
  return copy.buffer;
};

export const fileExtension = (name: string) => name.split('.').at(-1)?.toLowerCase() ?? '';

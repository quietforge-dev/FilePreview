const windowsExtendedPrefix = '\\\\?\\';
const remoteResourcePattern = /^(?:https?|data|blob|mailto|javascript|about|vscode-webview):/i;

const decodePath = (value: string) => {
  try {
    return decodeURIComponent(value);
  } catch {
    return value;
  }
};

const isWindowsStylePath = (path: string) =>
  path.includes('\\') || /^[a-zA-Z]:/.test(path) || path.startsWith('\\\\');

const stripWindowsExtendedPrefix = (path: string) =>
  path.startsWith(windowsExtendedPrefix) ? path.slice(windowsExtendedPrefix.length) : path;

const collapsePathSegments = (segments: string[]) => {
  const result: string[] = [];
  for (const segment of segments) {
    if (!segment || segment === '.') continue;
    if (segment === '..') {
      if (result.length) result.pop();
      continue;
    }
    result.push(segment);
  }
  return result;
};

const fromFileUrl = (source: string) => {
  try {
    const url = new URL(source);
    if (url.protocol !== 'file:') return null;
    const pathname = decodePath(url.pathname);
    if (url.hostname && url.hostname !== 'localhost') {
      return `\\\\${url.hostname}${pathname.replaceAll('/', '\\')}`;
    }
    if (/^\/[a-zA-Z]:/.test(pathname)) {
      return pathname.slice(1).replaceAll('/', '\\');
    }
    return pathname;
  } catch {
    return null;
  }
};

const parentDirectory = (path: string) => {
  const end = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'));
  return end < 0 ? null : path.slice(0, end);
};

const joinWindowsPath = (documentPath: string, resourcePath: string) => {
  const document = stripWindowsExtendedPrefix(documentPath);
  const resource = stripWindowsExtendedPrefix(resourcePath);
  if (/^[a-zA-Z]:/.test(resource)) {
    const drive = resource.slice(0, 2);
    const rest = collapsePathSegments(resource.slice(2).split(/[\\/]/));
    return `${drive}\\${rest.join('\\')}`;
  }
  if (resource.startsWith('\\\\')) {
    return `\\\\${collapsePathSegments(resource.split(/[\\/]/)).join('\\')}`;
  }
  const directory = parentDirectory(document);
  if (directory == null) return null;
  if (directory.startsWith('\\\\')) {
    return `\\\\${collapsePathSegments([
      ...directory.split(/[\\/]/),
      ...resource.split(/[\\/]/),
    ]).join('\\')}`;
  }
  const drive = directory.match(/^([a-zA-Z]:)(.*)$/);
  if (!drive) {
    return collapsePathSegments([...directory.split(/[\\/]/), ...resource.split(/[\\/]/)]).join(
      '\\',
    );
  }
  return `${drive[1]}\\${collapsePathSegments([
    ...drive[2].split(/[\\/]/),
    ...resource.split(/[\\/]/),
  ]).join('\\')}`;
};

const joinPosixPath = (documentPath: string, resourcePath: string) => {
  if (resourcePath.startsWith('/')) {
    return `/${collapsePathSegments(resourcePath.split('/')).join('/')}`;
  }
  const directory = parentDirectory(documentPath);
  if (directory == null) return null;
  const segments = collapsePathSegments([...directory.split('/'), ...resourcePath.split('/')]);
  return documentPath.startsWith('/') ? `/${segments.join('/')}` : segments.join('/');
};

export const localResourcePath = (documentPath: string, source: string) => {
  const trimmed = source.trim();
  if (!trimmed || trimmed.startsWith('#') || remoteResourcePattern.test(trimmed)) return null;
  if (/^\/\/[^\\]/.test(trimmed)) return null;

  const resourcePath = /^file:/i.test(trimmed)
    ? fromFileUrl(trimmed)
    : decodePath(trimmed.split(/[?#]/, 1)[0] ?? '');
  if (!resourcePath) return null;

  return isWindowsStylePath(documentPath) || isWindowsStylePath(resourcePath)
    ? joinWindowsPath(documentPath, resourcePath)
    : joinPosixPath(documentPath, resourcePath);
};

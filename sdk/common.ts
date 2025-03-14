import { writeFileSync } from "fs";

const SUPPORTED_OS = {
  darwin: "macOS",
  linux: "linux",
  win32: "windows",
} as const;

const SUFFIX = {
  macOS: "dylib",
  linux: "so",
  windows: "dll",
} as const;

const SUPPORTED_ARCH = {
  x64: "amd64",
  arm: "arm64",
  arm64: "arm64",
} as const;

/** Attaches the library filename (w.r.t platform) to the given directory.
 *
 * @param dir the directory to attach the library filename to
 * @returns the path to the library
 * @example
 * // e.g. on macOS Intel chip:
 * const path = getLibPath(".");
 * // path = "./libcircomkit_ffi-macOS-amd64.dylib"
 */
export function getLibPath(dir: string) {
  return `${dir}/${getLibFilename()}`;
}

/**
 * Returns the filename of the release library for this machine.
 * Throws an error if the architecture or OS is not supported.
 *
 * Supported OS & ARCH:
 * - **MacOS** Intel & Apple Silion
 * - **Linux** AMD64 & ARM64
 * - **Windows** x64
 */
export function getLibFilename() {
  // check if the current CPU architecture is supported
  if (!(process.arch in SUPPORTED_ARCH)) {
    throw new Error(`Unsupported ARCH: ${process.arch}`);
  }
  const os = SUPPORTED_OS[process.platform as keyof typeof SUPPORTED_OS];

  // check if the current OS is supported
  if (!(process.platform in SUPPORTED_OS)) {
    throw new Error(`Unsupported OS (platform): ${process.platform}`);
  }
  const arch = SUPPORTED_ARCH[process.arch as keyof typeof SUPPORTED_ARCH];

  // suffix is derived by OS
  const suffix = SUFFIX[os];

  return `libcircomkit_ffi-${os}-${arch}.${suffix}`;
}

/** Downloads the latest release for this machine under the given `dir` directory.
 *
 * Supported OS & ARCH:
 * - macOS (amd64, arm64)
 * - linux (amd64, arm64)
 * - windows (x64)
 *
 * @param dir the directory to save the release to
 * @returns the path to the downloaded release
 */
export async function downloadRelease(dir: string): Promise<string> {
  const filename = getLibFilename();
  const url = `https://github.com/erhant/circomkit-ffi/releases/latest/download/${filename}`;

  // download the file
  console.log(`Downloading library ${filename} at ${url}...`);
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to download ${url}: ${response.statusText}`);
  }

  // save the file
  const buffer = new Uint8Array(await response.arrayBuffer());
  const path = `${dir}/${filename}`;

  console.log(`Saving library to ${path}...`);
  writeFileSync(path, buffer);

  return path;
}

/** Returns whether the current environment is Bun or not. */
export function isBun(): boolean {
  // ignore is required as `Bun` is not defined in Node
  // @ts-ignore
  return typeof Bun !== typeof undefined;
}

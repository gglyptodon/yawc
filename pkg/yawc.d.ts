/* tslint:disable */
/* eslint-disable */
/**
*/
export function newGame(): void;
/**
* @returns {string}
*/
export function getState(): string;
/**
* @returns {string}
*/
export function showAttempts(): string;
/**
* @returns {string}
*/
export function showAttemptsRes(): string;
/**
* @returns {number}
*/
export function getAttemptsRemaining(): number;
/**
* @returns {string}
*/
export function getWord(): string;
/**
* @param {string} word
*/
export function attemptWord(word: string): void;
/**
* @param {string} character
* @returns {boolean}
*/
export function getKbUsed(character: string): boolean;
/**
* @param {string} character
* @returns {boolean}
*/
export function getKbIncorrect(character: string): boolean;
/**
* @param {string} character
* @returns {boolean}
*/
export function getKbCorrect(character: string): boolean;
/**
* @returns {boolean}
*/
export function isWon(): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly newGame: () => void;
  readonly getState: (a: number) => void;
  readonly showAttempts: (a: number) => void;
  readonly showAttemptsRes: (a: number) => void;
  readonly getAttemptsRemaining: () => number;
  readonly getWord: (a: number) => void;
  readonly attemptWord: (a: number, b: number) => void;
  readonly getKbUsed: (a: number) => number;
  readonly getKbIncorrect: (a: number) => number;
  readonly getKbCorrect: (a: number) => number;
  readonly isWon: () => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;

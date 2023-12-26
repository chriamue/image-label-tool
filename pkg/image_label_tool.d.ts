/* tslint:disable */
/* eslint-disable */
/**
* init label tool and start app on given root html element
* @param {Element} root
* @param {LabelTool | undefined} [label_tool]
* @param {string | undefined} [canvas_element_id]
* @returns {LabelTool}
*/
export function init_label_tool(root: Element, label_tool?: LabelTool, canvas_element_id?: string): LabelTool;
/**
* Represents an image with associated annotations.
*/
export class AnnotatedImage {
  free(): void;
/**
* constructor of AnnotatedImages for wasm
*/
  constructor();
}
/**
* struct of label tool that manages annotated images
*/
export class LabelTool {
  free(): void;
/**
* constructor of new label tool
*/
  constructor();
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init_label_tool: (a: number, b: number, c: number, d: number) => number;
  readonly __wbg_labeltool_free: (a: number) => void;
  readonly labeltool_new: () => number;
  readonly __wbg_annotatedimage_free: (a: number) => void;
  readonly annotatedimage_constructor: () => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__Fn___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hca4226cc2262bf34: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures__invoke1_mut__hc9d9f19cf25b6a26: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;

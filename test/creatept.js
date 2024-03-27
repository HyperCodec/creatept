import * as wasm from "./creatept_bg.wasm";
import { __wbg_set_wasm } from "./creatept_bg.js";
__wbg_set_wasm(wasm);
export * from "./creatept_bg.js";

wasm.__wbindgen_start();

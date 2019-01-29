/* tslint:disable */
import * as wasm from './wasm_calls_bg';

const lTextEncoder = typeof TextEncoder === 'undefined' ? require('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

let WASM_VECTOR_LEN = 0;

function passStringToWasm(arg) {

    const buf = cachedTextEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
/**
* @param {Login} arg0
* @returns {any}
*/
export function fetch_login(arg0) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return takeObject(wasm.fetch_login(ptr0));
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

export function __widl_f_set_Headers(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        getObject(arg0).set(varg1, varg3);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

export function __widl_f_new_with_str_and_init_Request(arg0, arg1, arg2, exnptr) {
    let varg0 = getStringFromWasm(arg0, arg1);
    try {
        return addHeapObject(new Request(varg0, getObject(arg2)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

export function __widl_f_headers_Request(arg0) {
    return addHeapObject(getObject(arg0).headers);
}

export function __widl_instanceof_Response(idx) {
    return getObject(idx) instanceof Response ? 1 : 0;
}

export function __widl_f_json_Response(arg0, exnptr) {
    try {
        return addHeapObject(getObject(arg0).json());
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

export function __widl_f_text_Response(arg0, exnptr) {
    try {
        return addHeapObject(getObject(arg0).text());
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

export function __widl_instanceof_Window(idx) {
    return getObject(idx) instanceof Window ? 1 : 0;
}

export function __widl_f_fetch_with_request_Window(arg0, arg1) {
    return addHeapObject(getObject(arg0).fetch(getObject(arg1)));
}

export function __wbg_newnoargs_43c5f57b77232284(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
}

export function __wbg_call_7ac13208e630ddeb(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

export function __wbg_call_419dd87667922e6a(arg0, arg1, arg2, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1), getObject(arg2)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

export function __wbg_new_5ed3704e9441a166() {
    return addHeapObject(new Object());
}

export function __wbg_set_a801b819ed807cd8(arg0, arg1, arg2, exnptr) {
    try {
        return Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

export function __wbg_new_5dc66f71c54d38f0(arg0, arg1) {
    let cbarg0 = function(arg0, arg1) {
        let a = this.a;
        this.a = 0;
        try {
            return this.f(a, this.b, addHeapObject(arg0), addHeapObject(arg1));

        } finally {
            this.a = a;

        }

    };
    cbarg0.f = wasm.__wbg_function_table.get(90);
    cbarg0.a = arg0;
    cbarg0.b = arg1;
    try {
        return addHeapObject(new Promise(cbarg0.bind(cbarg0)));
    } finally {
        cbarg0.a = cbarg0.b = 0;

    }
}

export function __wbg_resolve_639c083cf606e81d(arg0) {
    return addHeapObject(Promise.resolve(getObject(arg0)));
}

export function __wbg_then_d3a6d46e815b6c2b(arg0, arg1) {
    return addHeapObject(getObject(arg0).then(getObject(arg1)));
}

export function __wbg_then_7f4acc81aae754bd(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).then(getObject(arg1), getObject(arg2)));
}

export function __wbindgen_object_clone_ref(idx) {
    return addHeapObject(getObject(idx));
}

export function __wbindgen_object_drop_ref(i) { dropObject(i); }

export function __wbindgen_string_new(p, l) {
    return addHeapObject(getStringFromWasm(p, l));
}

export function __wbindgen_debug_string(i, len_ptr) {
    const toString = Object.prototype.toString;
    const debug_str = val => {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debug_str(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debug_str(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
        return `${val.name}: ${val.message}
        ${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
};
const val = getObject(i);
const debug = debug_str(val);
const ptr = passStringToWasm(debug);
getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
return ptr;
}

export function __wbindgen_cb_drop(i) {
    const obj = getObject(i).original;
    dropObject(i);
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return 1;
    }
    return 0;
}

export function __wbindgen_json_parse(ptr, len) {
    return addHeapObject(JSON.parse(getStringFromWasm(ptr, len)));
}

export function __wbindgen_json_serialize(idx, ptrptr) {
    const ptr = passStringToWasm(JSON.stringify(getObject(idx)));
    getUint32Memory()[ptrptr / 4] = ptr;
    return WASM_VECTOR_LEN;
}

export function __wbindgen_closure_wrapper212(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(60);
    const d = wasm.__wbg_function_table.get(61);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, addHeapObject(arg0));

        } finally {
            this.a = a;
            if (this.cnt-- == 1) d(this.a, b);

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}

function freeUser(ptr) {

    wasm.__wbg_user_free(ptr);
}
/**
*/
export class User {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeUser(ptr);
    }

    /**
    * @param {string} arg0
    * @returns {any}
    */
    static fetch(arg0) {
        const ptr0 = passStringToWasm(arg0);
        const len0 = WASM_VECTOR_LEN;
        return takeObject(wasm.user_fetch(ptr0, len0));
    }
}

function freeLogin(ptr) {

    wasm.__wbg_login_free(ptr);
}
/**
*/
export class Login {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeLogin(ptr);
    }

    /**
    * @param {string} arg0
    * @returns {}
    */
    constructor(arg0) {
        const ptr0 = passStringToWasm(arg0);
        const len0 = WASM_VECTOR_LEN;
        this.ptr = wasm.login_new(ptr0, len0);
    }
    /**
    * @returns {any}
    */
    fetch() {
        const ptr = this.ptr;
        this.ptr = 0;
        return takeObject(wasm.login_fetch(ptr));
    }
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}


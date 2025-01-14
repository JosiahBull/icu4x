// generated by diplomat-tool
import { FixedDecimal } from "./FixedDecimal.mjs"
import { FixedDecimalParseError } from "./FixedDecimalParseError.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `PluralOperands`](https://docs.rs/icu/latest/icu/plurals/struct.PluralOperands.html) for more information.
*/

const PluralOperands_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_PluralOperands_destroy_mv1(ptr);
});
export class PluralOperands {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        PluralOperands_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static fromString(s) {
        
        const sSlice = diplomatRuntime.DiplomatBuf.str8(wasm, s);
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.icu4x_PluralOperands_from_string_mv1(diplomat_receive_buffer, sSlice.ptr, sSlice.size);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = FixedDecimalParseError[Array.from(FixedDecimalParseError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('FixedDecimalParseError: ' + cause.value, { cause });
            }
            return new PluralOperands(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            sSlice.free();
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static fromFixedDecimal(x) {
        const result = wasm.icu4x_PluralOperands_from_fixed_decimal_mv1(x.ffiValue);
    
        try {
    
            return new PluralOperands(result, []);
        } finally {
        
        }
    }

    

}
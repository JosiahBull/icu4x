// generated by diplomat-tool
import { DataError } from "./DataError.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { Locale } from "./Locale.mjs"
import { PluralCategories } from "./PluralCategories.mjs"
import { PluralCategory } from "./PluralCategory.mjs"
import { PluralOperands } from "./PluralOperands.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `PluralRules`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html) for more information.
*/

const PluralRules_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_PluralRules_destroy_mv1(ptr);
});
export class PluralRules {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        PluralRules_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static createCardinal(provider, locale) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.icu4x_PluralRules_create_cardinal_mv1(diplomat_receive_buffer, provider.ffiValue, locale.ffiValue);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = DataError[Array.from(DataError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('DataError: ' + cause.value, { cause });
            }
            return new PluralRules(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static createOrdinal(provider, locale) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.icu4x_PluralRules_create_ordinal_mv1(diplomat_receive_buffer, provider.ffiValue, locale.ffiValue);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = DataError[Array.from(DataError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('DataError: ' + cause.value, { cause });
            }
            return new PluralRules(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    categoryFor(op) {
        const result = wasm.icu4x_PluralRules_category_for_mv1(this.ffiValue, op.ffiValue);
    
        try {
    
            return PluralCategory[Array.from(PluralCategory.values.keys())[result]];
        } finally {
        
        }
    }

    get categories() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(6, 1);
        const result = wasm.icu4x_PluralRules_categories_mv1(diplomat_receive_buffer, this.ffiValue);
    
        try {
    
            return new PluralCategories(diplomat_receive_buffer);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 6, 1);
        
        }
    }

    

}
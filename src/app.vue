<template>
    <div id="app">
        <button @click="count++">{{ count }}</button>
        <img src="@assets/navbar-logo-dark.png" alt="rspack logo" width="200" height="200" />
    </div>
</template>

<script>

import wasm from '@wasm-pkg/rust_rspack_template'

wasm().then((wasm) => {
    // console.log(wasm.add(BigInt(1.0), BigInt(2.2))) // 3
    console.log(wasm.add(1, 2)) // 3.2;
    
    }
).catch((err) => {
    console.error('wasm load error', err)
})
export default {
    data() {
        return {
            count: 0
        }
    }
}
</script>

<style></style>
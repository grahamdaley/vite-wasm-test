<script setup lang="ts">
import { onMounted, ref } from 'vue'
import init, { lookup, get_number } from '../hello_wasm/pkg/hello_wasm';
import wasmUrl from '../hello_wasm/pkg/hello_wasm_bg.wasm?url';

defineProps<{ msg: string }>();

const count = ref(0);
const result1Message = ref("");
const result2Message = ref("");

const initAndCallWasm = () => {
  const someComplexValues = [[[74.0078125,136.3125],[75.0078125,136.3125],[123.0078125,136.3125],[176.0078125,136.3125],[202.0078125,136.3125],[209.0078125,136.3125],[209.0078125,136.3125]]];

  init(wasmUrl).then((_) => {
    result1Message.value = "Meaning of life: " + get_number(42);
    result2Message.value = "Result: " + lookup(someComplexValues, 8);
  });
};

onMounted(() => {
  initAndCallWasm();
});
</script>

<template>
  <h1>{{ msg }}</h1>

  <h2>Result</h2>
  <h3>{{ result1Message }}</h3>
  <h3>{{ result2Message }}</h3>

  <div class="card">
    <button type="button" @click="count++">count is {{ count }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank"
      >create-vue</a
    >, the official Vue + Vite starter
  </p>
  <p>
    Install
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
    in your IDE for a better DX
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>

<template>
  <!-- Test 1: Simple unsorted classes -->
  <div class="z-10 p-4 mt-2 bg-white text-gray-900">
    Simple unsorted classes
  </div>

  <!-- Test 2: Classes with variants -->
  <div class="hover:bg-blue-500 bg-red-500 md:text-lg text-base">
    Classes with variants
  </div>

  <!-- Test 3: Component with computed classes -->
  <button
    :class="[
      'z-10 hover:shadow-lg p-4 bg-blue-500 text-white rounded-lg',
      isActive && 'ring-2 ring-blue-300'
    ]"
  >
    Click me
  </button>

  <!-- Test 4: Multiple elements -->
  <section class="flex items-center justify-between w-full h-16 px-4 bg-gray-800 text-white">
    <h1 class="text-2xl font-bold">Header</h1>
    <nav class="space-x-4 flex">
      <a href="#" class="hover:text-blue-400 text-gray-300">Link 1</a>
      <a href="#" class="hover:text-blue-400 text-gray-300">Link 2</a>
    </nav>
  </section>
</template>

<script>
export default {
  data() {
    return {
      isActive: false
    };
  }
};
</script>

<style scoped>
/* Styles here */
</style>

<template>
  <router-view/>
</template>

<script lang="ts" setup>
</script>

<style lang="scss">
@font-face {
  font-family: 'Roboto';
  font-style: normal;
  font-weight: 400;
  src: url('../assets/font/Roboto-Medium.ttf') format('woff2');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}

#app {
  width: 100%;
  height: 100%;
  opacity: 1;
}

body {
  width: 100%;
  height: 100vh;
  margin: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow-x: clip;
  overflow-y: clip;
  font-family: Inter, BlinkMacSystemFont, Segoe UI, Roboto, Oxygen, Ubuntu, Cantarell, Fira Sans, Droid Sans, Helvetica Neue, sans-serif;
}

body::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

body::-webkit-scrollbar-thumb {
  background: #cacaca;
  border-radius: 20px;
}

body::-webkit-scrollbar-thumb:hover {
  background: #ababab;
}

body::-webkit-scrollbar-track {
  background: transparent;
  border-radius: 0;
  box-shadow: inset 0 0 0 0 #F0F0F0;
  margin: 5px 0;
}

</style>

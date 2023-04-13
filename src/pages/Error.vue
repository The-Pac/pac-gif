<template>
  <div class="error-container">
    <h1 class="text-design text-with-shadow text-with-font">Erreur Page introuvable</h1>
  </div>
</template>

<script>
import {defineComponent} from "vue";

export default defineComponent({
  name: "Erreur-Page"
})
</script>

<style scoped>
.error-container {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  text-align: center;
  align-items: center;
}
</style>

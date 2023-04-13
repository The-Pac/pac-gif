<template>
  <div id="home" :style="{'background-color' : start_gif ? '' : 'rgba(255, 255, 255, 0.1)'}">
    <div id="gif-area-container" @mouseup.prevent="mouse_up" @mousemove.prevent="mouse_move"
         @mousedown.prevent="mouse_down">
      <div id="gif-area"
           :style="{
        'left' : gif_start_x + 'px',
        'top' : gif_start_y+ 'px',
        'width' : gif_width+ 'px',
        'height' : gif_height+ 'px',
}">

      </div>
    </div>
  </div>
</template>

<script lang="ts">

import {defineComponent} from "vue";
import {invoke} from "@tauri-apps/api";


export default defineComponent({
  name: "Home",
  data() {
    return {
      gif_start_x: Number(),
      gif_start_y: Number(),
      gif_width: Number(),
      gif_height: Number(),
      start_gif: Boolean()
    }
  },
  methods: {
    mouse_down(event: any): void {
      this.start_gif = true;
      this.gif_start_x = event.clientX;
      this.gif_start_y = event.clientY;
    },
    mouse_up(): void {
      this.start_gif = false;

      invoke('take_gif', {
        'x': this.gif_start_x,
        'y': this.gif_start_y,
        'width': this.gif_width,
        'height': this.gif_height,
      })

      this.reset_mouse_position()
    },
    mouse_move(event: any): void {
      if (this.start_gif) {
        this.gif_width = event.clientX - this.gif_start_x;
        this.gif_height = event.clientY - this.gif_start_y;
      }
    }, reset_mouse_position() {
      this.gif_start_x = 0;
      this.gif_start_y = 0;
      this.gif_width = 0;
      this.gif_height = 0;
    }
  }
})
</script>

<style scoped>
#home {
  height: 100%;
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;

  #gif-area-container {
    width: 100%;
    height: 100%;
    position: relative;

    #gif-area {
      border: #727F00 2px solid;
      position: absolute;
      display: flex;
      justify-content: center;
      align-items: center;
      padding: 0;
      margin: 0;
      border-radius: 2px;
    }
  }
}
</style>
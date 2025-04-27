<script>
    import { invoke } from "@tauri-apps/api";

    /**
     * @type boolean
     */
    let start_gif = false
    /**
     * @type number
     */
    let gif_start_x = 0
    /**
     * @type number
     */
    let gif_start_y = 0
    /**
     * @type number
     */
    let gif_width = 0
    /**
     * @type number
     */
    let gif_height = 0

    /**
     *
     * @param {MouseEvent} event
     */
    function mouse_down(event) {
        start_gif = true;
        gif_start_x = event.clientX;
        gif_start_y = event.clientY;
    }

    function mouse_up() {
        //start_gif = false;

        invoke('take_gif', {
            'x': gif_start_x,
            'y': gif_start_y,
            'width': gif_width,
            'height': gif_height
        });

        //reset_mouse_position();
    }

    /**
     *
     * @param {MouseEvent} event
     */
    function mouse_move(event) {
        if (start_gif) {
            gif_width = event.clientX - gif_start_x;
            gif_height = event.clientY - gif_start_y;
        }
    }


    function reset_mouse_position() {
        gif_start_x = 0;
        gif_start_y = 0;
        gif_width = 0;
        gif_height = 0;
    }

</script>
<div id="home" style="background-color : {start_gif ? '' : 'rgba(255, 255, 255, 0.1)'}">
    <div id="gif-area-container" role="button" tabindex="-1" on:mouseup={mouse_up} on:mousemove={mouse_move}
         on:mousedown={mouse_down}>
        <div id="gif-area"
             style="left : {gif_start_x}px; top : {gif_start_y}px; width :{gif_width}px; height :{gif_height}px;">
        </div>
    </div>
</div>
<style lang="scss">
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
                border: #161616 1px solid;
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
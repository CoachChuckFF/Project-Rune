const rust = import('./pkg/index.js');
const canvas = document.getElementById('gameCanvas');
const gl = canvas.getContext('webgl', { antialias: true });

rust.then(engine => {
    if(!gl) {
        alert('Failed to initialize WebGL');
        return;
    }

    // Play with these -- SET IN RUST
    // gl.enable(gl.BLEND); // Transparency
    // gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

    const FPS_THROTTLE = 1000.0 / 60.0; // milliseconds / frames
    const client = new engine.GameEngineClient();
    const initialTime = performance.now();
    let lastDrawTime = -1; // milliseconds

    function render(){
        window.requestAnimationFrame(render);
        const currentTime = performance.now();

        if(currentTime >= lastDrawTime + FPS_THROTTLE){
            lastDrawTime = currentTime;

            if(window.innerWidth !== canvas.width || window.innerHeight !== canvas.height){
                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;

                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            let elapsedTime = currentTime - initialTime;
            client.update(elapsedTime, window.innerWidth, window.innerHeight);
            client.render();
        }
    }

    client.start(lastDrawTime, window.innerWidth, window.innerHeight);

    render();
}).catch(console.error);
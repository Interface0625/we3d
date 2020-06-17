const rust = import('./pkg');
const canvas = document.getElementById('renderCanvas');
const gl = canvas.getContext('webgl');
const FPS_THROTTLE = 1000.0 / 60.0;
let lastLoopTime = -1;


//canvas.addEventListener("touchstart", onTouch, false);
//canvas.addEventListener("touchend", onTouch, false);
canvas.addEventListener("touchmove", onTouch, false);

function handleResize(){
  const width = window.innerWidth;
  const height = window.innerHeight;
  if(height != canvas.height || width != canvas.width){
    canvas.height = height;
    canvas.clientHeight = height;
    canvas.style.height = height;
    canvas.width = width;
    canvas.clientWidth = width;
    canvas.style.width = width;
    gl.viewport(0, 0, width, height);
  }
  return {width, height}
}
function onTouch(evt) {
  evt.preventDefault();
  if (evt.touches.length > 1 || (evt.type == "touchend" && evt.touches.length > 0))
    return;

  var newEvt = document.createEvent("MouseEvents");
  var type = null;
  var touch = null;

  switch (evt.type) {
    case "touchmove":
      type = "mousemove";
      touch = evt.changedTouches[0];
      break;
    case "touchstart": 
      type = "mousedown";
      touch = evt.changedTouches[0];
      return;
    case "touchend":        
      type = "mouseup";
      touch = evt.changedTouches[0];
      break;
  }

  newEvt.initMouseEvent(type, true, true, evt.originalTarget.ownerDocument.defaultView, 0,
    touch.screenX, touch.screenY, touch.clientX, touch.clientY,
    evt.ctrlKey, evt.altKey, evt.shiftKey, evt.metaKey, 0, null);
  evt.originalTarget.dispatchEvent(newEvt);
}

rust
  .then(async m => {
    const client = new m.RustClient();
    const initialTime = Date.now();
  
    function loop(){
      window.requestAnimationFrame(loop);
      const now = Date.now();
      if(now >= lastLoopTime + FPS_THROTTLE){
        const {width, height} = handleResize();
        const elapsedTime = now - initialTime;
        client.update(elapsedTime, width, height);
        client.render();
        lastLoopTime = now;
      }
    }
    loop();
    client.load_all().then(console.log);
  })
  .catch(console.error);



  



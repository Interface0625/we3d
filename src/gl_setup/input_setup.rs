use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::*;
use super::super::app_state;
//use web_sys::WebGlRenderingContext as GL;


/*
  el.addEventListener("touchstart", handleStart, false);
  el.addEventListener("touchend", handleEnd, false);
  el.addEventListener("touchcancel", handleCancel, false);
  el.addEventListener("touchmove", handleMove, false);

  function handleStart(evt) {
  evt.preventDefault();
  console.log("touchstart.");
  var el = document.getElementById("canvas");
  var ctx = el.getContext("2d");
  var touches = evt.changedTouches;

  evt.preventDefault();
 
  function handleMove(evt) {
  evt.preventDefault();
  var el = document.getElementById("canvas");
  var ctx = el.getContext("2d");
  var touches = evt.changedTouches;

  for (var i = 0; i < touches.length; i++) {
    var color = colorForTouch(touches[i]);
    var idx = ongoingTouchIndexById(touches[i].identifier);

    function handleEnd(evt) {
  evt.preventDefault();
  log("touchend");
  var el = document.getElementById("canvas");
  var ctx = el.getContext("2d");
  var touches = evt.changedTouches;


  function handleCancel(evt) {
  evt.preventDefault();
  console.log("touchcancel.");
  var touches = evt.changedTouches;


  function onTouch(evt) {
  evt.preventDefault();
  if (evt.touches.length > 1 || (evt.type == "touchend" && evt.touches.length > 0))
    return;

  var newEvt = document.createEvent("MouseEvents");
  var type = null;
  var touch = null;

  switch (evt.type) {
    case "touchstart": 
      type = "mousedown";
      touch = evt.changedTouches[0];
      break;
    case "touchmove":
      type = "mousemove";
      touch = evt.changedTouches[0];
      break;
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

*/
pub fn input_setup(canvas: &HtmlCanvasElement){
    let _result = [
      attach_mouse_down_handler(&canvas),
      attach_mouse_up_handler(&canvas), 
      attach_mouse_move_handler(&canvas)
    ];
}

/*
fn attach_touch_down_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
  let handler = move |event: web_sys::TouchEvent| {
      app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, true)
  };
  let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
  canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;
  handler.forget();

  Ok(())
}
 */


fn attach_mouse_down_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, true)
    };
    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_mouse_up_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, false)
    };
    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_mouse_move_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        app_state::update_mouse_position(event.client_x() as f32, event.client_y() as f32)
    };
    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}
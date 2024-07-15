export const afterFrame = () => {
  // Source: https://github.com/andrewiggins/afterframe
  let callbacks = [];
  let channel = new MessageChannel();
  let postMessage = function () {
    this.postMessage(undefined);
  }.bind(channel.port2);
  channel.port1.onmessage = () => {
    let toFlush = callbacks;
    callbacks = [];
    let time = performance.now();
    for (let i = 0; i < toFlush.length; i++) {
      toFlush[i](time);
    }
  };
  channel = null;
  // @ts-ignore: We register the function in the window object
  window.afterFrame = function (callback) {
    if (callbacks.push(callback) === 1) {
      requestAnimationFrame(postMessage);
    }
  };
};

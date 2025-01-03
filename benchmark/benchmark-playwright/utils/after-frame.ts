declare global {
  interface Window {
    afterFrame: (callback: () => void) => void;
  }
}

export const afterFrame = () => {
  const channel = new MessageChannel();
  const callbacks = [];
  const port = channel.port2;
  channel.port1.onmessage = () => {
    callbacks.splice(0, callbacks.length).forEach((cb) => cb());
  };
  const postMessage = () => {
    port.postMessage(null);
  };
  window.afterFrame = (callback) => {
    callbacks.push(callback) === 1 && requestAnimationFrame(postMessage);
  };
};

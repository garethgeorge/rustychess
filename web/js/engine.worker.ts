// The engine search blocks for a while, so it runs in this worker instead of
// on the main thread. That keeps the board interactive while it thinks.
const engine = import("../pkg/index").then(({ ChessEngine }) => {
  let engine = ChessEngine.new();
  postMessage({ ready: true });
  return engine;
});

onmessage = async (e: MessageEvent<string>) => {
  try {
    postMessage({ move: (await engine).select_move(e.data) });
  } catch (err: any) {
    postMessage({ error: err.toString() });
  }
};

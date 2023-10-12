import { Chess, Move } from "chess.js";
import React, { useEffect, useState } from "react";
import { Chessboard } from "react-chessboard";
import { Square } from "react-chessboard/dist/chessboard/types";
import { ChessEngine } from "../pkg/index";

export const App = () => {
  return (
    <>
      <center>
        <h1>RustyChess</h1>

        <p style={{ width: "50%" }}>
          RustyChess is a local-only Chess Engine written in Rust and compiled
          to WASM to run directly in your browser. This means immediate move
          generation with no backend API calls. The AI gets its intelligence
          from a neural network scoring function trained on a dataset of games
          from
          <a href="https://database.lichess.org/">lichess.org</a>. For more info
          see the github{" "}
          <a href="https://github.com/garethgeorge/rustychess">
            github.com/garethgeorge/rustychess
          </a>
        </p>
      </center>
      <RustyBoard />
    </>
  );
};

const RustyBoard = () => {
  const [engine, setEngine] = useState<ChessEngine | null>(null);

  useEffect(() => {
    (async () => {
      const { ChessEngine } = await import("../pkg/index");
      let engine = ChessEngine.new();
      console.log(engine);
      setEngine(engine);
    })();
  }, []);

  if (engine === null) {
    return <p>Loading Chess AI...</p>;
  }

  return (
    <>
      <BoardUI engine={engine} />
    </>
  );
};

export default function BoardUI({ engine }: { engine: ChessEngine }) {
  const [game, setGame] = useState(new Chess());
  const [history, setHistory] = useState<string[]>([]);

  function makeAMove(
    move: { from: Square; to: Square; promotion: string } | string
  ) {
    let copy = new Chess(game.fen());
    copy.move(move);

    let opponent_move = engine.select_move(copy.fen());
    copy.move(opponent_move);

    setGame(copy);
    setHistory([
      ...history,
      ...copy.history({ verbose: true }).map((move) => {
        return `${move.after}, ${
          move.color === "w" ? "white" : "black"
        } moved ${move.piece} from ${move.from} to ${move.to}`;
      }),
    ]);
  }

  function onDrop(sourceSquare: Square, targetSquare: Square) {
    try {
      const move = makeAMove({
        from: sourceSquare,
        to: targetSquare,
        promotion: "q",
      });
    } catch (e: any) {
      alert("invalid move: " + e.toString());
      return false;
    }

    return true;
  }

  return (
    <div
      style={{
        width: "100%",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <div>
        <div
          style={{
            height: "60vh",
            width: "60vh",
          }}
        >
          <Chessboard position={game.fen()} onPieceDrop={onDrop} />
        </div>
        <pre
          style={{
            height: "10em",
            width: "100%",
            overflow: "scroll",
          }}
        >
          {history.join("\n")}
        </pre>
      </div>
    </div>
  );
}

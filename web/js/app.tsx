import { Chess, Move } from "chess.js";
import React, { useEffect, useState } from "react";
import { Chessboard } from "react-chessboard";
import { Square } from "react-chessboard/dist/chessboard/types";
import { ChessEngine } from "../pkg/index";

export const App = () => {
  return (
    <>
      <p>
        A chess AI, written by a rusty chess player in the most appropriate of
        languages: rust.
      </p>
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
    return <p>Loading...</p>;
  }

  return (
    <>
      <p>Loaded.</p>
      <div id="board-container" style={{ width: "50vh", height: "50vh" }}>
        <BoardUI engine={engine} />
      </div>
    </>
  );
};

export default function BoardUI({ engine }: { engine: ChessEngine }) {
  const [game, setGame] = useState(new Chess());

  function makeAMove(
    move: { from: Square; to: Square; promotion: string } | string
  ) {
    const copy = new Chess(game.fen());
    copy.move(move);

    let opponent_move = engine.select_move(copy.fen());
    copy.move(opponent_move);

    setGame(copy);
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

  return <Chessboard position={game.fen()} onPieceDrop={onDrop} />;
}

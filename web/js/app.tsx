import { Chess } from "chess.js";
import React, { useEffect, useRef, useState } from "react";
import { Chessboard } from "react-chessboard";
import {
  CustomSquareStyles,
  Square,
} from "react-chessboard/dist/chessboard/types";
import { ChessEngine } from "../pkg/index";

export const App = () => {
  return (
    <main className="page">
      <header className="header">
        <h1>
          Rusty<span>Chess</span>
        </h1>
        <p>
          A local-only chess engine written in Rust and compiled to WASM, so
          every move is computed right here in your browser. Its intelligence
          comes from a neural network trained on games from{" "}
          <a href="https://database.lichess.org/">lichess.org</a>. More on{" "}
          <a href="https://github.com/garethgeorge/rustychess">GitHub</a>.
        </p>
      </header>
      <RustyBoard />
    </main>
  );
};

const RustyBoard = () => {
  const [engine, setEngine] = useState<ChessEngine | null>(null);

  useEffect(() => {
    (async () => {
      const { ChessEngine } = await import("../pkg/index");
      let engine = ChessEngine.new();
      setEngine(engine);
    })();
  }, []);

  if (engine === null) {
    return (
      <div className="loading">
        <div className="spinner" />
        <p>Loading the chess AI…</p>
      </div>
    );
  }

  return <BoardUI engine={engine} />;
};

// chess.js forgets the move history when created from a FEN, so we copy via
// PGN to keep undo, the move list and repetition draws working.
function copyGame(game: Chess) {
  const copy = new Chess();
  copy.loadPgn(game.pgn());
  return copy;
}

function describe(game: Chess, thinking: boolean) {
  if (game.isCheckmate()) {
    return game.turn() === "w"
      ? "Checkmate. The engine wins."
      : "Checkmate. You win!";
  }
  if (game.isStalemate()) return "Stalemate. It's a draw.";
  if (game.isThreefoldRepetition()) return "Draw by repetition.";
  if (game.isInsufficientMaterial()) return "Draw by insufficient material.";
  if (game.isDraw()) return "It's a draw.";
  if (thinking) return "Engine is thinking";
  if (game.isCheck()) return "You are in check!";
  return "Your move";
}

function squareStyles(game: Chess) {
  const styles: CustomSquareStyles = {};
  const moves = game.history({ verbose: true });
  const last = moves[moves.length - 1];
  if (last) {
    styles[last.from] = { background: "var(--last-move)" };
    styles[last.to] = { background: "var(--last-move)" };
  }
  if (game.isCheck()) {
    const king = game
      .board()
      .flat()
      .find((p) => p && p.type === "k" && p.color === game.turn());
    if (king) {
      styles[king.square] = { background: "var(--check)" };
    }
  }
  return styles;
}

export default function BoardUI({ engine }: { engine: ChessEngine }) {
  const [game, setGame] = useState(new Chess());
  const [thinking, setThinking] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const movesRef = useRef<HTMLOListElement>(null);

  const history = game.history();
  const gameOver = game.isGameOver();

  useEffect(() => {
    const el = movesRef.current;
    el && (el.scrollTop = el.scrollHeight);
  }, [history.length]);

  function makeAMove(move: { from: Square; to: Square; promotion: string }) {
    let copy = copyGame(game);
    copy.move(move);
    setGame(copy);
    setError(null);

    if (copy.isGameOver()) {
      return;
    }

    // The engine runs synchronously on the main thread, so give the browser a
    // moment to animate the player's move and show the thinking state first.
    setThinking(true);
    setTimeout(() => {
      try {
        let reply = copyGame(copy);
        reply.move(engine.select_move(reply.fen()));
        setGame(reply);
      } catch (e: any) {
        setError("The engine failed to move: " + e.toString());
      }
      setThinking(false);
    }, 350);
  }

  function onDrop(sourceSquare: Square, targetSquare: Square, piece: string) {
    try {
      makeAMove({
        from: sourceSquare,
        to: targetSquare,
        promotion: piece[1].toLowerCase(),
      });
    } catch (e) {
      // Illegal move, the piece snaps back.
      return false;
    }

    return true;
  }

  function newGame() {
    setGame(new Chess());
    setError(null);
  }

  function undo() {
    let copy = copyGame(game);
    copy.undo();
    if (copy.turn() === "b") {
      copy.undo();
    }
    setGame(copy);
    setError(null);
  }

  const rows: [string, string | undefined][] = [];
  for (let i = 0; i < history.length; i += 2) {
    rows.push([history[i], history[i + 1]]);
  }

  return (
    <div className="game">
      <div className={"board" + (thinking ? " thinking" : "")}>
        <Chessboard
          position={game.fen()}
          onPieceDrop={onDrop}
          arePiecesDraggable={!thinking && !gameOver}
          customSquareStyles={squareStyles(game)}
          customDarkSquareStyle={{ backgroundColor: "#7b8aa8" }}
          customLightSquareStyle={{ backgroundColor: "#e3e8f2" }}
          customBoardStyle={{ borderRadius: "10px", overflow: "hidden" }}
        />
      </div>

      <aside className="panel">
        <div className={"status" + (gameOver ? " over" : "")}>
          <span className={"dot" + (thinking ? " busy" : "")} />
          <strong>{describe(game, thinking)}</strong>
          {thinking && (
            <span className="dots">
              <i />
              <i />
              <i />
            </span>
          )}
        </div>
        {error && <p className="error">{error}</p>}

        <ol className="moves" ref={movesRef}>
          {rows.length === 0 && (
            <li className="empty">You play white. Drag a piece to begin.</li>
          )}
          {rows.map(([white, black], i) => (
            <li key={i}>
              <span className="num">{i + 1}.</span>
              <span>{white}</span>
              <span>{black}</span>
            </li>
          ))}
        </ol>

        <div className="actions">
          <button onClick={undo} disabled={thinking || history.length === 0}>
            Undo
          </button>
          <button className="primary" onClick={newGame} disabled={thinking}>
            New game
          </button>
        </div>
      </aside>
    </div>
  );
}

import { TicTacToe, Symbol } from 'tictactoe';
import { memory } from "tictactoe/tictactoe_bg";
import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";

const symbolToChar = (symbol) => {
  if ( symbol === Symbol.X ) { return "🦐"; }
  else if ( symbol === Symbol.O ) { return "🪐"; }
  else if ( symbol === Symbol.E ) { return "\n"; }
}

const App = (props) => {
  console.log("--Application--");

  const [newGame, startNewGame] = useState(0);

  const length = 6;
  const ttt = TicTacToe.new(Symbol.X, length, 2);

  return (
    <div>
      <h2>🦐Shrimp vs. Saturn🪐</h2>
      <h4>Can a tiny shrimp beat the all knowing Saturn?</h4>
      <p style={{margin: '1em'}}>Can you, a tiny shrimp, take on the all knowing Saturn? Using the rules of tic-tac-toe,
      the aim of the game is to get 4 in a row.
      Saturn is a 'simu-telligent' being who uses a minimax algorithm and maths to look ahead into the future and calculate the best move. Good luck!</p>
      <Board ttt={ttt} length={length} startNewGame={() => {console.log("start new game!"); startNewGame(!newGame);}} />
    </div>
  );
}

const Board = (props) => {
  console.log("--Rendering Board--");

  const { ttt, length, startNewGame } = props;

  const [ robotConfidence, setRobotConfidence ] = useState(0);
  const [ turn, setTurn ] = useState(0);
  const [ data, setData ] = useState([]);
  const [ time, setTime ] = useState(0);



  const ptr = ttt.board();
  console.log({m: 'main', ptr, data, turn});

  const getDataFromBoard = () => {
    const data = new Uint8Array(memory.buffer, ptr, length * length);
    console.log({m: 'effect', ptr, data});
    setData(data);
  }


  // if ( turn !== Symbol.E )  {
    useEffect(getDataFromBoard, [ptr]);
  // }

  const onClick = (symbol, x, y) => {
    console.log({symbol, x, y, turn});
    if ( turn === Symbol.X )  {
      ttt.make_move(x, y);
      setTurn(ttt.turn());
      setRobotConfidence(0);
    }
  }

  if ( turn === Symbol.O )  {
    const start = Date.now();
    const confidence = ttt.agent_move_2();
    const diff = Date.now() - start;
    console.log({data, diff, confidence});
    setTime(diff);
    setTurn(ttt.turn());
    // setData(null);
    setRobotConfidence(confidence);
  }

  const boardStyle = {
    display: 'flex',
    width: '80%',
    margin: 'auto',
    flexWrap: 'wrap',
    textAlign: 'center',
  }

  const itemStyle = {
    width: `15%`,
    backgroundColor: '#d0d0d0',
    margin: 4,
    height: '72px',
    fontSize: '48px',
    "&:hover": {
      background: "#efefef"
    },
  }

  const statusStyle = {
    display: 'flex',
    width: '80%',
    margin: 'auto',
    flexWrap: 'wrap',
    justifyContent: 'space-between',
    alignItems: 'center',
  }

  const arr = [];
  if (data && data.length)  {
    for ( let i of data.keys()) {
      const x = i % length;
      const y = Math.floor(i / length);
      const item = data[i];
      const clickHandler = () => onClick(item, x, y);

      arr.push(
        <div key={i} style={itemStyle} onClick={clickHandler}>{symbolToChar(item)}</div>
      )
    }
  }

  return (
    <>
      <div style={statusStyle}>
        <button className='success-button' onClick={startNewGame}>NEW GAME</button>
        <h4>{turn === Symbol.E ? "GAME OVER!" : `${symbolToChar(turn)}'s Turn`}</h4>
        <h6>
          {turn === Symbol.E && `Good game Puny Human`}
          {turn === Symbol.X && `Saturn: I'm ${robotConfidence} confident, and took ${time} ms to calculate!`}
          {turn === Symbol.O && 'Saturn: Your turn!'}
        </h6>
      </div>
      <div style={boardStyle}>
        {arr}
      </div>
    </>
  )
}

const wrapper = document.getElementById("canvas");
wrapper ? ReactDOM.render(<App />, wrapper) : false;

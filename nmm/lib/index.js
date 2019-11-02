var addon = require('../native');
const { Manager } = addon;

const man = new Manager();
console.log(Manager);
console.log(man);
console.log(".get_board():\n");
var board = man.get_board();
console.log(board);
console.log("\n.poll():\n");
var gameState = man.poll();
console.log(gameState);
console.log("\ngameState.handle: ");
console.log(gameState.handle);
console.log("\ngameState.trigger: ");
console.log(gameState.trigger);
console.log("\ngameState.board:\n");
console.log(gameState.board);

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

var options = {"user": 1, "opponent": 2, "agent": "human"};
var type = "Piece";
// console.log(man.get_user(options));
console.log(Manager.prototype);
console.log(man.get_user(options));
console.log(man.get_opponent(options));
console.log(man.get_agent(options));
console.log(man.get_req_type(type));
// console.log(man.get_opponent(options));

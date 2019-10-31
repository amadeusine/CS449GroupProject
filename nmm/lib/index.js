var addon = require('../native');
const { Manager } = addon;

const man = new Manager();
console.log(Manager);
console.log(man);
var board = man.get_board();
console.log(board);
console.log("Board at G7: " + board["G7"]);

const express = require('express');
const router = express.Router();

/* GET logic listing. */

module.exports = (bind) => router.get('/', function(req, res, next) {
  bind.board = 0;
  res.send("[]");
});
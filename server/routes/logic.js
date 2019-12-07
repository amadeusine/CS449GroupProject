const express = require('express');
const router = express.Router();

/* GET logic listing. */

module.exports = (bind) => router.get('/', function(req, res, next) {
  res.send(JSON.stringify({resp: bind.board++}));
});
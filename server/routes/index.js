var express = require('express');
var router = express.Router();
var { Board } = require('../public/javascripts/board');

/* GET home page. */
router.get('/', function(req, res, next) {
  
  let options = {
    title: 'Nine Men\'s Morris',
    theme: req.query.theme || 'default',
    validPositions: {
        A: [1, 4, 7],
        B: [2, 4, 6],
        C: [3, 4, 5],
        D: [1, 2, 3, 5, 6, 7],
        E: [3, 4, 5],
        F: [2, 4, 6],
        G: [1, 4, 7]
    },
    toWords: {
      1: "one",
      2: "two",
      3: "three",
      4: "four",
      5: "five",
      6: "six",
      7: "seven",
      8: "eight",
      9: "nine",
    },
    color: ['blue', 'red'],
  }
  res.render('index', options);
});

module.exports = router;
